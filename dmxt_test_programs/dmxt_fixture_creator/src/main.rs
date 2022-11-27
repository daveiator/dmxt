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
                        FixtureCustomOperation::Button(fixture_name!("Circle"), address!(2, 0)),
                        FixtureCustomOperation::Button(fixture_name!("Star"), address!(2, 1)),
                        FixtureCustomOperation::Button(fixture_name!("Heart"), address!(2, 2)),
                        FixtureCustomOperation::Button(fixture_name!("Flower"), address!(2, 3)),
                    ]
                ))
                .operation_mode(operation_mode!(OperationModeType::DMX(fixture_name!("Dynamic pattern")), Some(address!(1, 200)),
                    vec![
                        FixtureCustomOperation::Button(fixture_name!("Circle"), address!(2, 0)),
                        FixtureCustomOperation::Button(fixture_name!("Rectangle"), address!(2, 50)),
                        FixtureCustomOperation::Button(fixture_name!("Triangle"), address!(2, 100)),
                        FixtureCustomOperation::Button(fixture_name!("Star"), address!(2, 150)),
                    ]
                ))
                .lights(lights!(fixture::FixtureColorMode::Presets(
                    vec![
                        (Color::Auto, address!(8, 0)),
                        (Color::Red, address!(8, 1)),
                        (Color::Green, address!(8, 2)),
                        (Color::Blue, address!(8, 3)),
                        (Color::Cyan, address!(8, 4)),
                    ]
                )))
                .zoom(zoom!(range!(address!(5, 11), address!(5, 255))))
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
                                (fixture_name!("Color pulse"), address!(1, 64)),
                                (fixture_name!("Color"), address!(1, 72)),
                                (fixture_name!("Color rev"), address!(1, 80)),
                                (fixture_name!("Color instant"), address!(1, 88)),
                                (fixture_name!("RGB side to side one way"), address!(1, 96)),
                                (fixture_name!("Color instant split"), address!(1, 104)),
                                (fixture_name!("White chaser"), address!(1, 112)),
                                (fixture_name!("Color side to side one way"), address!(1, 120)),
                                (fixture_name!("Color side to side ping pong"), address!(1, 128)),
                                (fixture_name!("Color fade side to side one way"), address!(1, 136)),
                                (fixture_name!("Color fade side to side ping pong"), address!(1, 144)),
                                (fixture_name!("Color to center"), address!(1, 152)),
                                (fixture_name!("Color from center"), address!(1, 160)),
                                (fixture_name!("Rainbow side to side one way"), address!(1, 168)),
                                (fixture_name!("Rainbow stacker side to side"), address!(1, 176)),
                                (fixture_name!("Color stacker side to side"), address!(1, 184)),
                                (fixture_name!("Color to center ping pong"), address!(1, 192)),
                                (fixture_name!("Rainbow to center ping pong"), address!(1, 200)),
                                (fixture_name!("RGB side to side ping pong multiple chasers"), address!(1, 208)),
                                (fixture_name!("Yellow on red side to side one way"), address!(1, 216)),
                                (fixture_name!("Yellow on red side to side ping pong"), address!(1, 224)),
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
