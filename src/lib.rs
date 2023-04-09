use ebur128::{EbuR128, Mode};
use lv2::prelude::*;
use std::convert::TryFrom;
use wmidi::*;

const SAMPLE_RATE: u32 = 48000;

#[derive(PortCollection)]
struct Ports {
    in_r: InputPort<InPlaceAudio>,
    in_l: InputPort<InPlaceAudio>,
    out_r: OutputPort<InPlaceAudio>,
    out_l: OutputPort<InPlaceAudio>,
    loudness_midi: OutputPort<AtomPort>,
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

#[uri("https://github.com/pedalboard/db-meter.lv2")]
struct DbMeter {
    urids: URIDs,
    sample_count: u32,
    rate: f64,
    ebu: ebur128::EbuR128,
}

impl Plugin for DbMeter {
    type Ports = Ports;

    type InitFeatures = Features<'static>;
    type AudioFeatures = ();

    fn new(plugin_info: &PluginInfo, features: &mut Features) -> Option<Self> {
        Some(Self {
            urids: features.map.populate_collection()?,
            rate: plugin_info.sample_rate(),
            sample_count: 0,
            ebu: ebur128::EbuR128::new(2, 48000, ebur128::Mode::S | ebur128::Mode::I).unwrap(),
        })
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut (), count: u32) {
        /*
        for (input_sample, output_sample) in input.zip(output) {
            let value = input_sample.get();
            output_sample.set(value);
        }
        */

        self.sample_count += count;

        if self.sample_count > SAMPLE_RATE {
            ports.short_term.set(self.rate as f32);
            self.sample_count = self.sample_count.rem_euclid(SAMPLE_RATE);

            let mut level_sequence = ports
                .loudness_midi
                .init(
                    self.urids.atom.sequence,
                    TimeStampURID::Frames(self.urids.unit.frame),
                )
                .unwrap();

            let message_to_send =
                MidiMessage::NoteOff(Channel::Ch1, Note::C1, U7::try_from(1u8).unwrap());

            level_sequence
                .init(
                    TimeStamp::Frames(100),
                    self.urids.midi.wmidi,
                    message_to_send,
                )
                .unwrap();
        }
    }
}

lv2_descriptors!(DbMeter);
