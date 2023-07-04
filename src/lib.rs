use ebur128::{EbuR128, Mode};
use lv2::prelude::*;
use std::convert::TryFrom;
use wmidi::*;

#[derive(PortCollection)]
struct Ports {
    in_r: InputPort<InPlaceAudio>,
    in_l: InputPort<InPlaceAudio>,
    loudness_midi: OutputPort<AtomPort>,
    momentary: OutputPort<InPlaceControl>,
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
    buffer: [f32; 2],
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
            buffer: [0.0, 0.0],
            ebu: EbuR128::new(2, sample_rate, Mode::M).unwrap(),
        })
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut (), count: u32) {
        // pass the signal through to outputs
        for (isr, isl) in ports.in_r.iter().zip(ports.in_l.iter()) {
            self.buffer[0] = isr.get();
            self.buffer[1] = isl.get();
            self.ebu.add_frames_f32(&self.buffer).unwrap();
        }

        self.sample_count += count;
        // update the short loudness with 10Hz frequency
        let rate = self.ebu.rate() / 10;
        if self.sample_count > rate {
            let momentary = self.ebu.loudness_momentary().unwrap();
            ports.momentary.set(momentary as f32);

            self.sample_count = self.sample_count.rem_euclid(rate);
            self.houndred_ms_count += 1;

            let mut level_sequence = ports
                .loudness_midi
                .init(
                    self.urids.atom.sequence,
                    TimeStampURID::Frames(self.urids.unit.frame),
                )
                .unwrap();

            let st = momentary.abs().min(127.0).round() as u8;
            let st_message =
                MidiMessage::NoteOff(Channel::Ch4, Note::C1, U7::try_from(st).unwrap());
            level_sequence
                .init(
                    TimeStamp::Frames(self.sample_count as i64),
                    self.urids.midi.wmidi,
                    st_message,
                )
                .unwrap();
        }
    }
}

lv2_descriptors!(LoudnessMeter);
