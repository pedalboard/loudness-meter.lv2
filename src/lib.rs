use ebur128::{EbuR128, Mode};
use lv2::prelude::*;
use std::convert::TryFrom;
use wmidi::*;

#[derive(PortCollection)]
struct Ports {
    in_r: InputPort<InPlaceAudio>,
    in_l: InputPort<InPlaceAudio>,
    out_r: OutputPort<InPlaceAudio>,
    out_l: OutputPort<InPlaceAudio>,
    loudness_midi: OutputPort<AtomPort>,
    momentary: OutputPort<InPlaceControl>,
    short_term: OutputPort<InPlaceControl>,
    integrated: OutputPort<InPlaceControl>,
}

#[derive(FeatureCollection)]
pub struct Features<'a> {
    map: LV2Map<'a>,
}

#[derive(URIDCollection)]
pub struct URIDs {
    atom: AtomURIDCollection,
    midi: MidiURIDCollection,
    unit: UnitURIDCollection,
}

#[uri("https://github.com/pedalboard/loudness-meter.lv2")]
struct LoudnessMeter {
    urids: URIDs,
    sample_count: u32,
    houndred_ms_count: u32,
    ebu: ebur128::EbuR128,
}

impl Plugin for LoudnessMeter {
    type Ports = Ports;

    type InitFeatures = Features<'static>;
    type AudioFeatures = ();

    fn new(plugin_info: &PluginInfo, features: &mut Features) -> Option<Self> {
        let sample_rate = plugin_info.sample_rate() as u32;
        Some(Self {
            urids: features.map.populate_collection()?,
            sample_count: 0,
            houndred_ms_count: 0,
            ebu: EbuR128::new(2, sample_rate, Mode::S | Mode::I | Mode::M).unwrap(),
        })
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut (), count: u32) {
        let r = ports.in_r.iter().map(|s| s.get()).collect::<Vec<f32>>();
        let l = ports.in_l.iter().map(|s| s.get()).collect::<Vec<f32>>();

        self.ebu.add_frames_planar_f32(&[&l, &r]).unwrap();

        // pass the signal through to outputs
        for (is, os) in ports.in_r.iter().zip(ports.out_r.iter()) {
            os.set(is.get());
        }
        for (is, os) in ports.in_l.iter().zip(ports.out_l.iter()) {
            os.set(is.get());
        }

        self.sample_count += count;
        let momentary = self.ebu.loudness_momentary().unwrap();
        ports.momentary.set(momentary as f32);

        // update the short term loudness with 10Hz frequency
        let rate = self.ebu.rate() / 10;
        if self.sample_count > rate {
            self.sample_count = self.sample_count.rem_euclid(rate);
            self.houndred_ms_count += 1;

            let short_term = self.ebu.loudness_shortterm().unwrap();
            ports.short_term.set(short_term as f32);
            let mut level_sequence = ports
                .loudness_midi
                .init(
                    self.urids.atom.sequence,
                    TimeStampURID::Frames(self.urids.unit.frame),
                )
                .unwrap();

            let st = short_term.abs().min(127.0).round() as u8;
            let st_message =
                MidiMessage::NoteOff(Channel::Ch4, Note::C1, U7::try_from(st).unwrap());
            level_sequence
                .init(
                    TimeStamp::Frames(self.sample_count as i64),
                    self.urids.midi.wmidi,
                    st_message,
                )
                .unwrap();

            // update integrated loudness with 1Hz frequency
            if self.houndred_ms_count == 10 {
                self.houndred_ms_count = 0;
                let integrated = self.ebu.loudness_global().unwrap();
                ports.integrated.set(integrated as f32);
                let int = integrated.abs().min(127.0).round() as u8;
                let int_message =
                    MidiMessage::NoteOff(Channel::Ch4, Note::D1, U7::try_from(int).unwrap());

                level_sequence
                    .init(TimeStamp::Frames(100), self.urids.midi.wmidi, int_message)
                    .unwrap();
            }
        }
    }
}

lv2_descriptors!(LoudnessMeter);
