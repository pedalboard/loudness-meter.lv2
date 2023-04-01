use lv2_core::prelude::*;
use urid::*;

#[derive(PortCollection)]
struct Ports {
    gain: InputPort<Control>,
    input: InputPort<InPlaceAudio>,
    output: OutputPort<InPlaceAudio>,
}

#[uri("https://github.com/pedalboard/db-meter.lv2")]
struct DbMeter;

impl Plugin for DbMeter {
    type Ports = Ports;

    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        Some(Self)
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut (), _: u32) {
        let coef = if *(ports.gain) > -90.0 {
            10.0_f32.powf(*(ports.gain) * 0.05)
        } else {
            0.0
        };

        let input = ports.input.iter();
        let output = ports.output.iter();

        for (input_sample, output_sample) in input.zip(output) {
            output_sample.set(input_sample.get() * coef);
        }
    }
}

lv2_descriptors!(DbMeter);
