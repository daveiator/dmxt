use std::path::Path;
use std::vec;

use dmxt_lib::builders::fixture::{FixtureModel, FixtureChannelMode, FixtureMovement, OperationModeType, self, FixtureCustomOperation, FixtureMatrix};
use dmxt_lib::dmx::Color;
use dmxt_lib::{address, channel, fixture_name, movement_axis, operation_mode, lights, range, zoom};

use serde_json;

fn main() {
    let _laser = FixtureModel::builder()
        .model(fixture_name!("EL-230RGB", Path::new("laserworld_el-230rgb.png")))
        .manufacturer("Laserworld".into())
        .channel_mode(
            FixtureChannelMode::builder()
                .total_channels(channel!(9))
                .movement(
                    FixtureMovement::builder()
                    .pan(movement_axis!(range!(address!(3, 11), address!(3, 255)), address!(3, 0)))
                    .tilt(movement_axis!(range!(address!(4, 11), address!(4, 255)), address!(4, 0)))
                    .build().unwrap()
                )
                .operation_mode(operation_mode!(OperationModeType::Off, address!(1, 0)))
                .operation_mode(operation_mode!(OperationModeType::SoundToLight, address!(1, 50)))
                .operation_mode(operation_mode!(OperationModeType::Auto, address!(1, 100)))
                .operation_mode(operation_mode!(OperationModeType::DMX(fixture_name!("Static pattern")), Some(address!(1, 150)),
                    vec![
                        FixtureCustomOperation::Button(fixture_name!("Circle"),                             address!(2, 0)),
                        FixtureCustomOperation::Button(fixture_name!("Circle incomplete"),                  address!(2, 5)),
                        FixtureCustomOperation::Button(fixture_name!("Circle dotted"),                      address!(2, 10)),
                        FixtureCustomOperation::Button(fixture_name!("Octagon"),                            address!(2, 15)),
                        FixtureCustomOperation::Button(fixture_name!("3xV Circle"),                         address!(2, 20)),
                        FixtureCustomOperation::Button(fixture_name!("3xV Circle spread small"),            address!(2, 25)),
                        FixtureCustomOperation::Button(fixture_name!("4x Circle"),                          address!(2, 30)),
                        FixtureCustomOperation::Button(fixture_name!("2x Circle incomplete horizontal"),    address!(2, 35)),
                        FixtureCustomOperation::Button(fixture_name!("2x Circle dotted horizonal"),         address!(2, 40)),
                        FixtureCustomOperation::Button(fixture_name!("Line horizontal growing"),            address!(2, 45)),
                        FixtureCustomOperation::Button(fixture_name!("Line horizontal shrinking"),          address!(2, 50)),
                        FixtureCustomOperation::Button(fixture_name!("Line horizontal ping pong"),          address!(2, 55)),
                        FixtureCustomOperation::Button(fixture_name!("Line horizontal color shift"),        address!(2, 60)),
                        FixtureCustomOperation::Button(fixture_name!("Line horizontal dotted"),             address!(2, 65)),
                        FixtureCustomOperation::Button(fixture_name!("Line vertical"),                      address!(2, 70)),
                        FixtureCustomOperation::Button(fixture_name!("Line vertical dotted"),               address!(2, 75)),
                        FixtureCustomOperation::Button(fixture_name!("Line diagonal"),                      address!(2, 80)),
                        FixtureCustomOperation::Button(fixture_name!("Line diagonal dotted"),               address!(2, 85)),
                        FixtureCustomOperation::Button(fixture_name!("2x Line horizontal"),                 address!(2, 90)),
                        FixtureCustomOperation::Button(fixture_name!("2x Line horizontal dotted"),          address!(2, 95)),
                        FixtureCustomOperation::Button(fixture_name!("2x Line vertical"),                   address!(2, 100)),
                        FixtureCustomOperation::Button(fixture_name!("2x Line vertical dotted"),            address!(2, 105)),
                        FixtureCustomOperation::Button(fixture_name!("/\\"),                                address!(2, 110)),
                        FixtureCustomOperation::Button(fixture_name!("/\\ dotted"),                         address!(2, 115)),
                        FixtureCustomOperation::Button(fixture_name!("Triangle"),                           address!(2, 120)),
                        FixtureCustomOperation::Button(fixture_name!("Triangle dotted"),                    address!(2, 125)),
                        FixtureCustomOperation::Button(fixture_name!("Triangle inverted"),                  address!(2, 130)),
                        FixtureCustomOperation::Button(fixture_name!("Triangle inverted dotted"),           address!(2, 135)),
                        FixtureCustomOperation::Button(fixture_name!("X"),                                  address!(2, 140)),
                        FixtureCustomOperation::Button(fixture_name!("X dotted"),                           address!(2, 145)),
                        FixtureCustomOperation::Button(fixture_name!("Square"),                             address!(2, 150)),
                        FixtureCustomOperation::Button(fixture_name!("Square dotted"),                      address!(2, 155)),
                        FixtureCustomOperation::Button(fixture_name!("Rectangle"),                          address!(2, 160)),
                        FixtureCustomOperation::Button(fixture_name!("Rectangle dotted"),                   address!(2, 165)),
                        FixtureCustomOperation::Button(fixture_name!("Stepped diagonal"),                   address!(2, 170)),
                        FixtureCustomOperation::Button(fixture_name!("Stepped /\\"),                        address!(2, 175)),
                        FixtureCustomOperation::Button(fixture_name!("4 Star"),                             address!(2, 180)),
                        FixtureCustomOperation::Button(fixture_name!("4 Star diagonal"),                    address!(2, 185)),
                        FixtureCustomOperation::Button(fixture_name!("5 Star"),                             address!(2, 190)),
                        FixtureCustomOperation::Button(fixture_name!("5 Star inverted"),                    address!(2, 195)),
                        FixtureCustomOperation::Button(fixture_name!("Pentagon"),                           address!(2, 200)),
                        FixtureCustomOperation::Button(fixture_name!("Triangle wave"),                      address!(2, 205)),
                        FixtureCustomOperation::Button(fixture_name!("Triangle wave color cycle"),          address!(2, 210)),
                        FixtureCustomOperation::Button(fixture_name!("Sine wave color pulse"),              address!(2, 215)),
                        FixtureCustomOperation::Button(fixture_name!("Triangle wave color cycle dotted"),   address!(2, 220)),
                        FixtureCustomOperation::Button(fixture_name!("Spiral"),                             address!(2, 225)),
                        ]
                ))
                .operation_mode(operation_mode!(OperationModeType::DMX(fixture_name!("Dynamic pattern")), Some(address!(1, 200)),
                    vec![
                        FixtureCustomOperation::Button(fixture_name!("Circle growing"),                                     address!(2, 0)),
                        FixtureCustomOperation::Button(fixture_name!("Circle growing incomplete"),                          address!(2, 5)),
                        FixtureCustomOperation::Button(fixture_name!("Circle growing dotted"),                              address!(2, 10)),
                        FixtureCustomOperation::Button(fixture_name!("Spiral growing + Cicle growing"),                     address!(2, 15)),
                        FixtureCustomOperation::Button(fixture_name!("Circle moving horizontally"),                         address!(2, 20)),
                        FixtureCustomOperation::Button(fixture_name!("Circle incomplete moving horizontally"),              address!(2, 25)),
                        FixtureCustomOperation::Button(fixture_name!("Circle dotted moving horizontally"),                  address!(2, 30)),
                        FixtureCustomOperation::Button(fixture_name!("Circle spinning"),                                    address!(2, 35)),
                        FixtureCustomOperation::Button(fixture_name!("Circle filling lines"),                               address!(2, 40)),
                        FixtureCustomOperation::Button(fixture_name!("Random Circles"),                                     address!(2, 45)),
                        FixtureCustomOperation::Button(fixture_name!("Spiral ping pong"),                                   address!(2, 50)),
                        FixtureCustomOperation::Button(fixture_name!("Random Circles ping pong"),                           address!(2, 55)),
                        FixtureCustomOperation::Button(fixture_name!("Circle incomplete chasing"),                          address!(2, 60)),
                        FixtureCustomOperation::Button(fixture_name!("2 Circles spinning against each other"),              address!(2, 65)),
                        FixtureCustomOperation::Button(fixture_name!("3 Circles chasing"),                                  address!(2, 70)),
                        FixtureCustomOperation::Button(fixture_name!("Random Diagonals"),                                   address!(2, 75)),
                        FixtureCustomOperation::Button(fixture_name!("Lines horizontal scanning up stepped"),               address!(2, 80)),
                        FixtureCustomOperation::Button(fixture_name!("Lines dotted horizontal scanning up stepped"),        address!(2, 85)),
                        FixtureCustomOperation::Button(fixture_name!("Lines diagonal scanning down"),                       address!(2, 90)),
                        FixtureCustomOperation::Button(fixture_name!("Lines dotted diagonal scanning down"),                address!(2, 95)),
                        FixtureCustomOperation::Button(fixture_name!("Line spinning"),                                      address!(2, 100)),
                        FixtureCustomOperation::Button(fixture_name!("Clock hand spinning"),                                address!(2, 105)),
                        FixtureCustomOperation::Button(fixture_name!("3x Clock hand spinning"),                             address!(2, 110)),
                        FixtureCustomOperation::Button(fixture_name!("Line horizontal scanning ping pong"),                 address!(2, 115)),
                        FixtureCustomOperation::Button(fixture_name!("Line dotted horizontal scanning ping pong"),          address!(2, 120)),
                        FixtureCustomOperation::Button(fixture_name!("2 Lines horizontal scanning ping pong"),              address!(2, 125)),
                        FixtureCustomOperation::Button(fixture_name!("2 Lines dotted horizontal scanning ping pong"),       address!(2, 130)),
                        FixtureCustomOperation::Button(fixture_name!("Line vertical scanning ping pong"),                   address!(2, 135)),
                        FixtureCustomOperation::Button(fixture_name!("Line dotted vertical scanning ping pong"),            address!(2, 140)),
                        FixtureCustomOperation::Button(fixture_name!("2 Lines vertical scanning ping pong"),                address!(2, 145)),
                        FixtureCustomOperation::Button(fixture_name!("2 Lines dotted vertical scanning ping pong"),         address!(2, 150)),
                        FixtureCustomOperation::Button(fixture_name!("Rectangle growing"),                                  address!(2, 155)),
                        FixtureCustomOperation::Button(fixture_name!("Rectangle growing dotted"),                           address!(2, 160)),
                        FixtureCustomOperation::Button(fixture_name!("Rectangle growing width"),                            address!(2, 165)),
                        FixtureCustomOperation::Button(fixture_name!("Rectangle spinning solid / dotted"),                  address!(2, 170)),
                        FixtureCustomOperation::Button(fixture_name!("Rectangle wide spinning"),                            address!(2, 175)),
                        FixtureCustomOperation::Button(fixture_name!("Rectangle chase outline"),                            address!(2, 180)),
                        FixtureCustomOperation::Button(fixture_name!("Rectangle chase outline dotted"),                     address!(2, 185)),
                        FixtureCustomOperation::Button(fixture_name!("Wave spinning ping pong"),                            address!(2, 190)),
                        FixtureCustomOperation::Button(fixture_name!("Wave spinning ping pong dotted"),                     address!(2, 195)),
                        FixtureCustomOperation::Button(fixture_name!("Random horizontal waves"),                            address!(2, 200)),
                        FixtureCustomOperation::Button(fixture_name!("Random horizontal waves dotted"),                     address!(2, 205)),
                        FixtureCustomOperation::Button(fixture_name!("Wave amplitude ping pong"),                           address!(2, 210)),
                        FixtureCustomOperation::Button(fixture_name!("Wave amplitude ping pong dotted"),                    address!(2, 215)),
                        FixtureCustomOperation::Button(fixture_name!("5 Star chasing ccw"),                                 address!(2, 220)),
                        FixtureCustomOperation::Button(fixture_name!("5 Star chasing cw"),                                  address!(2, 225)),
                        FixtureCustomOperation::Button(fixture_name!("Pentagon chasing filling lines"),                     address!(2, 230)),
                        FixtureCustomOperation::Button(fixture_name!("Bird"),                                               address!(2, 235)),
                        FixtureCustomOperation::Button(fixture_name!("Random Vs"),                                          address!(2, 240)),

                    ]
                ))
                .lights(lights!(fixture::FixtureColorMode::Presets(
                    vec![
                        (Color::ColorChange,                                                            address!(8, 0)), 
                        (Color::White,                                                                  address!(8, 2)),
                        (Color::Magenta,                                                                address!(8, 4)),
                        (Color::Cyan,                                                                   address!(8, 6)),
                        (Color::Yellow,                                                                 address!(8, 8)),
                        (Color::Blue,                                                                   address!(8, 10)),
                        (Color::Red,                                                                    address!(8, 12)),
                        (Color::Green,                                                                  address!(8, 14)),
                        (Color::Custom(fixture_name!("White / Magenta")),                               address!(8, 16)),
                        (Color::Custom(fixture_name!("White / Cyan")),                                  address!(8, 18)),
                        (Color::Custom(fixture_name!("White / Yellow")),                                address!(8, 20)),
                        (Color::Custom(fixture_name!("Magenta / Cyan")),                                address!(8, 22)),
                        (Color::Custom(fixture_name!("Magenta / Yellow")),                              address!(8, 24)),
                        (Color::Custom(fixture_name!("Magenta / Blue")),                                address!(8, 26)),
                        (Color::Custom(fixture_name!("Cyan / Yellow")),                                 address!(8, 28)),
                        (Color::Custom(fixture_name!("Cyan / Blue")),                                   address!(8, 30)),
                        (Color::Custom(fixture_name!("Cyan / Red")),                                    address!(8, 32)),
                        (Color::Custom(fixture_name!("Yellow / Blue")),                                 address!(8, 34)),
                        (Color::Custom(fixture_name!("Yellow / Red")),                                  address!(8, 36)),
                        (Color::Custom(fixture_name!("Yellow / Green")),                                address!(8, 38)),
                        (Color::Custom(fixture_name!("Blue / Red")),                                    address!(8, 40)),
                        (Color::Custom(fixture_name!("Blue / Green")),                                  address!(8, 42)),
                        (Color::Custom(fixture_name!("Blue / White")),                                  address!(8, 44)),
                        (Color::Custom(fixture_name!("Red / Green")),                                   address!(8, 46)),
                        (Color::Custom(fixture_name!("Red / Magenta")),                                 address!(8, 48)),
                        (Color::Custom(fixture_name!("Red / White")),                                   address!(8, 50)),
                        (Color::Custom(fixture_name!("Green / Cyan")),                                  address!(8, 52)),
                        (Color::Custom(fixture_name!("Green / Magenta")),                               address!(8, 54)),
                        (Color::Custom(fixture_name!("Green / White")),                                 address!(8, 56)),
                        (Color::Custom(fixture_name!("Magenta / White / Cyan")),                        address!(8, 58)),
                        (Color::Custom(fixture_name!("Magenta / White / Yellow")),                      address!(8, 60)),
                        (Color::Custom(fixture_name!("Magenta / White / Blue")),                        address!(8, 62)),
                        (Color::Custom(fixture_name!("Magenta / White / Red")),                         address!(8, 64)),
                        (Color::Custom(fixture_name!("Magenta / White / Green")),                       address!(8, 66)),
                        (Color::Custom(fixture_name!("Cyan / Magenta / Yellow")),                       address!(8, 68)),
                        (Color::Custom(fixture_name!("Cyan / Magenta / Blue")),                         address!(8, 70)),
                        (Color::Custom(fixture_name!("Cyan / Magenta / Red")),                          address!(8, 72)),
                        (Color::Custom(fixture_name!("Cyan / Magenta / Green")),                        address!(8, 74)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Magenta")),                       address!(8, 76)),
                        (Color::Custom(fixture_name!("Yellow / Cyan / Blue")),                          address!(8, 78)),
                        (Color::Custom(fixture_name!("Yellow / Cyan / Red")),                           address!(8, 80)),
                        (Color::Custom(fixture_name!("Yellow / Cyan / Green")),                         address!(8, 82)),
                        (Color::Custom(fixture_name!("Blue / Cyan / Red")),                             address!(8, 84)),
                        (Color::Custom(fixture_name!("Blue / Cyan / White")),                           address!(8, 86)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Red")),                           address!(8, 88)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Green")),                         address!(8, 90)),
                        (Color::Custom(fixture_name!("Blue / Yellow / White")),                         address!(8, 92)),
                        (Color::Custom(fixture_name!("Cyan / Yellow / White")),                         address!(8, 94)),
                        (Color::Custom(fixture_name!("Magenta / Yellow / Red")),                        address!(8, 96)),
                        (Color::Custom(fixture_name!("Red / Blue / Green")),                            address!(8, 98)),
                        (Color::Custom(fixture_name!("Red / Blue / Magenta")),                          address!(8, 100)),
                        (Color::Custom(fixture_name!("Red / Blue / White")),                            address!(8, 102)),
                        (Color::Custom(fixture_name!("Green / Blue / Magenta")),                        address!(8, 104)),
                        (Color::Custom(fixture_name!("Green / Blue / White")),                          address!(8, 106)),
                        (Color::Custom(fixture_name!("Green / Red / Cyan")),                            address!(8, 108)),
                        (Color::Custom(fixture_name!("Green / Red / Magenta")),                         address!(8, 110)),
                        (Color::Custom(fixture_name!("Green / Red / White")),                           address!(8, 112)),
                        (Color::Custom(fixture_name!("White / Red / Cyan")),                            address!(8, 114)),
                        (Color::Custom(fixture_name!("White / Red / Yellow")),                          address!(8, 116)),
                        (Color::Custom(fixture_name!("White / Green / Cyan")),                          address!(8, 118)),
                        (Color::Custom(fixture_name!("White / Green / Yellow")),                        address!(8, 120)),
                        (Color::Custom(fixture_name!("Magenta / Green / Yellow")),                      address!(8, 122)),
                        (Color::Custom(fixture_name!("Blue / Green / Cyan")),                           address!(8, 124)),
                        (Color::Custom(fixture_name!("Red / Green / Yellow")),                          address!(8, 126)),
                        (Color::Custom(fixture_name!("Cyan / Magenta / White / Yellow")),               address!(8, 128)),
                        (Color::Custom(fixture_name!("Cyan / Magenta / White / Blue")),                 address!(8, 130)),
                        (Color::Custom(fixture_name!("Cyan / Magenta / White / Red")),                  address!(8, 132)),
                        (Color::Custom(fixture_name!("Cyan / Magenta / White / Green")),                address!(8, 134)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Magenta / White")),               address!(8, 136)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Cyan / Magenta")),                address!(8, 138)),
                        (Color::Custom(fixture_name!("Yellow / Cyan / Magenta / Red")),                 address!(8, 140)),
                        (Color::Custom(fixture_name!("Yellow / Cyan / Magenta / Green")),               address!(8, 142)),
                        (Color::Custom(fixture_name!("Blue / Cyan / Magenta / Red")),                   address!(8, 144)),
                        (Color::Custom(fixture_name!("Blue / Cyan / Magenta / Green")),                 address!(8, 146)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Cyan / Red")),                    address!(8, 148)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Cyan / Green")),                  address!(8, 150)),
                        (Color::Custom(fixture_name!("Red / Yellow / Cyan / Green")),                   address!(8, 152)),
                        (Color::Custom(fixture_name!("Red / Yellow / Cyan / White")),                   address!(8, 154)),
                        (Color::Custom(fixture_name!("Red / Yellow / Cyan / Blue")),                    address!(8, 156)),
                        (Color::Custom(fixture_name!("Red / Blue / Yellow / Green")),                   address!(8, 158)),
                        (Color::Custom(fixture_name!("Red / Blue / Yellow / White")),                   address!(8, 160)),
                        (Color::Custom(fixture_name!("Red / Blue / Yellow / Magenta")),                 address!(8, 162)),
                        (Color::Custom(fixture_name!("White / Blue / Yellow / Cyan")),                  address!(8, 164)),
                        (Color::Custom(fixture_name!("Magenta / Blue / Yellow / Cyan")),                address!(8, 166)),
                        (Color::Custom(fixture_name!("Green / Red / Blue / White")),                    address!(8, 168)),
                        (Color::Custom(fixture_name!("Green / Red / Blue / Magenta")),                  address!(8, 170)),
                        (Color::Custom(fixture_name!("Green / Red / Blue / Cyan")),                     address!(8, 172)),
                        (Color::Custom(fixture_name!("White / Red / Blue / Magenta")),                  address!(8, 174)),
                        (Color::Custom(fixture_name!("White / Red / Blue / Cyan")),                     address!(8, 176)),
                        (Color::Custom(fixture_name!("White / Green / Red / Magenta")),                 address!(8, 178)),
                        (Color::Custom(fixture_name!("White / Green / Red / Cyan")),                    address!(8, 180)),
                        (Color::Custom(fixture_name!("White / Green / Red / Yellow")),                  address!(8, 182)),
                        (Color::Custom(fixture_name!("Magenta / Green / Red / Cyan")),                  address!(8, 184)),
                        (Color::Custom(fixture_name!("Magenta / Green / Red / Yellow")),                address!(8, 186)),
                        (Color::Custom(fixture_name!("Magenta / White / Green / Yellow")),              address!(8, 188)),
                        (Color::Custom(fixture_name!("Magenta / White / Green / Blue")),                address!(8, 190)),
                        (Color::Custom(fixture_name!("Magenta / White / Green / Red")),                 address!(8, 192)),
                        (Color::Custom(fixture_name!("Blue / Magenta / Green / Yellow")),               address!(8, 194)),
                        (Color::Custom(fixture_name!("Cyan / White / Green / Blue")),                   address!(8, 196)),
                        (Color::Custom(fixture_name!("Yellow / Cyan / Magenta / White / Blue")),        address!(8, 198)),
                        (Color::Custom(fixture_name!("Yellow / Cyan / Magenta / White / Red")),         address!(8, 200)),
                        (Color::Custom(fixture_name!("Yellow / Cyan / Magenta / White / Green")),       address!(8, 202)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Magenta / Red / Blue")),          address!(8, 204)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Cyan / Magenta / Green")),        address!(8, 206)),
                        (Color::Custom(fixture_name!("Blue / Cyan / Magenta / Green / Red")),           address!(8, 208)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Cyan / Green / Red")),            address!(8, 210)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Cyan / White / Red")),            address!(8, 212)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Cyan / White / Green")),          address!(8, 214)),
                        (Color::Custom(fixture_name!("Blue / Yellow / White / Green / Red")),           address!(8, 216)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Magenta / Green / Red")),         address!(8, 218)),
                        (Color::Custom(fixture_name!("Blue / Yellow / Magenta / White / Red")),         address!(8, 220)),
                        (Color::Custom(fixture_name!("Red / Blue / Magenta / White / Green")),          address!(8, 222)),
                        (Color::Custom(fixture_name!("Red / Blue / Cyan / White / Green")),             address!(8, 224)),
                        (Color::Custom(fixture_name!("Red / Blue / Cyan / Magenta / White")),           address!(8, 226)),
                        (Color::Custom(fixture_name!("Green / Red / Cyan / Magenta / White")),          address!(8, 228)),
                        (Color::Custom(fixture_name!("Green / Red / Yellow / Magenta / White")),        address!(8, 230)),
                        (Color::Custom(fixture_name!("Green / Red / Yellow / Cyan / White")),           address!(8, 232)),
                        (Color::Custom(fixture_name!("White / Green / Blue / Cyan / Magenta")),         address!(8, 234)),
                        (Color::Custom(fixture_name!("White / Green / Blue / Yellow / Magenta")),       address!(8, 236)),
                        (Color::Custom(fixture_name!("Magenta / Green / Red / Yellow / Cyan")),         address!(8, 238)),
                        (Color::Custom(fixture_name!("White / Red / Blue / Yellow / Cyan")),            address!(8, 240)),
                        (Color::Custom(fixture_name!("Magenta / Green / Red / Blue / Yellow")),         address!(8, 242)),
                        (Color::Custom(fixture_name!("Cyan / White / Green / Red / Blue")),             address!(8, 244)),
                        (Color::Custom(fixture_name!("Yellow / Magenta / White / Green / Red")),        address!(8, 246)),
                        (Color::Custom(fixture_name!("Blue / Cyan / Magenta / White / Green")),         address!(8, 248)),
                        (Color::Custom(fixture_name!("Red / Yellow / Cyan / Magenta / White")),         address!(8, 250)),
                        (Color::Custom(fixture_name!("Green / Blue / Yellow / Cyan / Magenta")),        address!(8, 252)),
                        (Color::All,                                                                    address!(8, 254)),

                    ]
                )))
                .custom(FixtureCustomOperation::Slider(fixture_name!("Scanning speed"), range!(address!(5, 255), address!(5, 0))))
                .custom(FixtureCustomOperation::Slider(fixture_name!("Pattern speed"), range!(address!(6, 255), address!(6, 0))))
                .zoom(zoom!(range!(address!(7, 0), address!(7, 255))))
                .custom(FixtureCustomOperation::Stepped(fixture_name!("Color Segments"),
                    vec![
                        (fixture_name!("One"), address!(9, 0)),
                        (fixture_name!("Two"), address!(9, 20)),
                    ]))
            .build().unwrap()
        )
        .build().unwrap();


    let _lightbar = FixtureModel::builder()
        .model(fixture_name!("Led Bar 240/8 RGB DMX 30°", Path::new("led_bar_240_8_rgb_dmx_30.png")))
        .manufacturer("Stairville".into())
        .channel_mode(
            FixtureChannelMode::builder()
                .name(fixture_name!("2 Channel Mode"))
                .total_channels(channel!(2))
                .operation_mode(operation_mode!(OperationModeType::Off, address!(1, 0)))
                .operation_mode(operation_mode!(OperationModeType::DMX(fixture_name!("Constant unicolored pattern")), None, vec![]))
                .operation_mode(operation_mode!(OperationModeType::Auto, None,
                    vec![
                        FixtureCustomOperation::Stepped(fixture_name!("Shows"), 
                            vec![
                                (fixture_name!("Color pulse"),                                  address!(1, 64)),
                                (fixture_name!("Color"),                                        address!(1, 72)),
                                (fixture_name!("Color rev"),                                    address!(1, 80)),
                                (fixture_name!("Color instant"),                                address!(1, 88)),
                                (fixture_name!("RGB side to side one way"),                     address!(1, 96)),
                                (fixture_name!("Color instant split"),                          address!(1, 104)),
                                (fixture_name!("White chaser"),                                 address!(1, 112)),
                                (fixture_name!("Color side to side one way"),                   address!(1, 120)),
                                (fixture_name!("Color side to side ping pong"),                 address!(1, 128)),
                                (fixture_name!("Color fade side to side one way"),              address!(1, 136)),
                                (fixture_name!("Color fade side to side ping pong"),            address!(1, 144)),
                                (fixture_name!("Color to center"),                              address!(1, 152)),
                                (fixture_name!("Color from center"),                            address!(1, 160)),
                                (fixture_name!("Rainbow side to side one way"),                 address!(1, 168)),
                                (fixture_name!("Rainbow stacker side to side"),                 address!(1, 176)),
                                (fixture_name!("Color stacker side to side"),                   address!(1, 184)),
                                (fixture_name!("Color to center ping pong"),                    address!(1, 192)),
                                (fixture_name!("Rainbow to center ping pong"),                  address!(1, 200)),
                                (fixture_name!("RGB side to side ping pong multiple chasers"),  address!(1, 208)),
                                (fixture_name!("Yellow on red side to side one way"),           address!(1, 216)),
                                (fixture_name!("Yellow on red side to side ping pong"),         address!(1, 224)),
                            ]
                        ),
                        FixtureCustomOperation::Slider(fixture_name!("Speed"), range!(address!(2, 0), address!(2, 255)))
                    ]
                ))
                .operation_mode(operation_mode!(OperationModeType::SoundToLight, Some(address!(1, 232)),
                    vec![
                        FixtureCustomOperation::Slider(fixture_name!("Microphone Sensitivity"), range!(address!(2, 0), address!(2, 255)))
                    ]
                ))
                .lights(lights!(fixture::FixtureColorMode::Presets(
                    vec![
                        (Color::Black, address!(1, 0)),
                        (Color::Red, address!(1, 8)),
                        (Color::Yellow, address!(1, 16)),
                        (Color::Blue, address!(1, 24)),
                        (Color::Cyan, address!(1, 32)),
                        (Color::Blue, address!(1, 40)),
                        (Color::Magenta, address!(1, 48)),
                        (Color::White, address!(1, 56)),
                    ]
                )))
                .build().unwrap()
            )
            .channel_mode(
                FixtureChannelMode::builder()
                    .name(fixture_name!("3 Channel Mode"))
                    .total_channels(channel!(3))
                    .lights(lights!(fixture::FixtureColorMode::RGB(
                        range!(address!(1, 0), address!(1, 255)),
                        range!(address!(2, 0), address!(2, 255)),
                        range!(address!(3, 0), address!(3, 255))
                    )))
                    .build().unwrap()
            )
            .channel_mode(
                FixtureChannelMode::builder()
                    .name(fixture_name!("5 Channel Mode"))
                    .total_channels(channel!(5))
                    .lights(lights!(fixture::FixtureColorMode::RGB(
                        range!(address!(1, 0), address!(1, 255)),
                        range!(address!(2, 0), address!(2, 255)),
                        range!(address!(3, 0), address!(3, 255))
                    ), range!(address!(4, 0), address!(4, 255))))
                    .custom(FixtureCustomOperation::Slider(fixture_name!("Strobe"), range!(address!(5, 0), address!(5, 255))))
                    .build().unwrap()
            )
            .channel_mode(
                FixtureChannelMode::builder()
                    .name(fixture_name!("24 Channel Mode"))
                    .total_channels(channel!(24))
                    .matrix(
                        FixtureMatrix::builder()
                            .row(vec![
                                lights!(fixture::FixtureColorMode::RgbTrailingChannels(range!(address!(1, 0), address!(1, 255)))),
                                lights!(fixture::FixtureColorMode::RgbTrailingChannels(range!(address!(4, 0), address!(4, 255)))),
                                lights!(fixture::FixtureColorMode::RgbTrailingChannels(range!(address!(7, 0), address!(7, 255)))),
                                lights!(fixture::FixtureColorMode::RgbTrailingChannels(range!(address!(10, 0), address!(10, 255)))),
                                lights!(fixture::FixtureColorMode::RgbTrailingChannels(range!(address!(13, 0), address!(13, 255)))),
                                lights!(fixture::FixtureColorMode::RgbTrailingChannels(range!(address!(16, 0), address!(16, 255)))),
                                lights!(fixture::FixtureColorMode::RgbTrailingChannels(range!(address!(19, 0), address!(19, 255)))),
                                lights!(fixture::FixtureColorMode::RgbTrailingChannels(range!(address!(22, 0), address!(22, 255)))),
                            ])
                        .build().unwrap()
                    )
                .build().unwrap()
            )
        .build().unwrap();

    let _laser_json = serde_json::to_string_pretty(&_laser).unwrap();
    println!("{:?}", _laser);
    println!("{}", _laser_json);
    let _lightbar_json = serde_json::to_string_pretty(&_lightbar).unwrap();
    println!("{:?}", _lightbar);
    println!("{}", _lightbar_json);
}
