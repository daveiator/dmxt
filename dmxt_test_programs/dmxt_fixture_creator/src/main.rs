use std::path::Path;
use std::vec;

use dmxt_lib::builders::fixture::{FixtureModel, FixtureChannelMode, FixtureMovement, FixtureSubOperationMode, OperationModeType, self, FixtureCustom};
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
                .operation_mode(operation_mode!(OperationModeType::DMX(fixture_name!("Static pattern")), address!(1, 150),
                    vec![
                        FixtureSubOperationMode::new(fixture_name!("Circle"), address!(2, 0)),
                        FixtureSubOperationMode::new(fixture_name!("Star"), address!(2, 1)),
                        FixtureSubOperationMode::new(fixture_name!("Heart"), address!(2, 2)),
                        FixtureSubOperationMode::new(fixture_name!("Flower"), address!(2, 3)),
                    ]
                ))
                .operation_mode(operation_mode!(OperationModeType::DMX(fixture_name!("Dynamic pattern")), address!(1, 200),
                    vec![
                        FixtureSubOperationMode::new(fixture_name!("Circle"), address!(2, 0)),
                        FixtureSubOperationMode::new(fixture_name!("Rectangle"), address!(2, 50)),
                        FixtureSubOperationMode::new(fixture_name!("Triangle"), address!(2, 100)),
                        FixtureSubOperationMode::new(fixture_name!("Star"), address!(2, 150)),
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
                .custom(FixtureCustom::Stepped("Color Segments".into(),
                    vec![
                        ("One".into(), address!(9, 0)),
                        ("Two".into(), address!(9, 20)),
                    ]))
            .build().unwrap()
        )
        .build().unwrap();


    let _laser_json = serde_json::to_string_pretty(&_laser).unwrap();
    println!("{:?}", _laser);
    println!("{}", _laser_json);


    // let lightbar = Fixture.builder()
    //     .model("Led Bar 240/8 RGB DMX 30°")
    //     .manufacturer("Stairville")
    //     .icon("stairville_ledbar2408rgb.png")
    //     .total_channels(5)   
}
