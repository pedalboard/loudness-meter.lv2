use lv2::prelude::*;
use wmidi::*;

const SAMPLE_RATE: u32 = 48000;

#[derive(PortCollection)]
struct Ports {
    input: InputPort<InPlaceAudio>,
    output: OutputPort<InPlaceAudio>,
    level: OutputPort<InPlaceControl>,
    level_midi: OutputPort<AtomPort>,
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
    count: f32,
}

impl Plugin for DbMeter {
    type Ports = Ports;

    type InitFeatures = Features<'static>;
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, features: &mut Features) -> Option<Self> {
        Some(Self {
            urids: features.map.populate_collection()?,
            sample_count: 0,
            count: 0.0,
        })
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut (), count: u32) {
        let input = ports.input.iter();
        let output = ports.output.iter();

        for (input_sample, output_sample) in input.zip(output) {
            output_sample.set(input_sample.get());
        }

        self.sample_count += count;

        if false && self.sample_count > SAMPLE_RATE {
            self.count = self.count + 1.0;
            ports.level.set(self.count);

            self.sample_count = self.sample_count.rem_euclid(SAMPLE_RATE);

            let mut level_sequence = ports
                .level_midi
                .write(self.urids.atom.sequence)
                .unwrap()
                .with_unit(self.urids.unit.frame)
                .unwrap();

            let message_to_send = MidiMessage::NoteOn(Channel::Ch1, Note::C1, Velocity::MAX);

            level_sequence
                .new_event(0, self.urids.midi.wmidi)
                .unwrap()
                .set(message_to_send)
                .unwrap();
        }
    }
}

lv2_descriptors!(DbMeter);
