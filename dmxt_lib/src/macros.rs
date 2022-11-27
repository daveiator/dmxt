#[macro_export]
macro_rules! channel {
    ($channel:expr) => {
        dmxt_lib::dmx::Channel::new($channel).unwrap()
    };
}


#[macro_export]
macro_rules! address {
    ($channel:expr, $value:expr) => {
        dmxt_lib::dmx::DMXAddress::new(channel!($channel), $value)
    };
}

#[macro_export]
macro_rules! fixture_name {
    ($name:expr) => {
        dmxt_lib::builders::fixture::FixtureName::new($name.into())
    };
    ($name:expr, $icon:expr) => {
        dmxt_lib::builders::fixture::FixtureName::new_with_icon($name.into(), $icon.into())
    };
}

#[macro_export]
macro_rules! range {
    ($start:expr, $end:expr) => {
        dmxt_lib::dmx::DMXRange::new($start, $end)
    };
}

#[macro_export]
macro_rules! movement_axis {
    ($range:expr) => {
        dmxt_lib::builders::fixture::MovementAxis::new($range, None)
    };
    ($range:expr, $reset:expr) => {
        dmxt_lib::builders::fixture::MovementAxis::new($range, Some($reset))
    };
}

#[macro_export]
macro_rules! operation_mode {
    ($operation_mode_type:expr, $address:expr) => {
        dmxt_lib::builders::fixture::FixtureOperationMode::new($operation_mode_type, $address, vec![])
    };
    ($operation_mode_type:expr, $address:expr, $sub_operation_modes:expr) => {
        dmxt_lib::builders::fixture::FixtureOperationMode::new($operation_mode_type, $address, $sub_operation_modes)
    };
}

#[macro_export]
macro_rules! lights {
    ($color_mode:expr) => {
        dmxt_lib::builders::fixture::FixtureLights::new($color_mode, None)
    };
    ($color_mode:expr, $dimmer:expr) => {
        dmxt_lib::builders::fixture::FixtureLights::new($color_mode, Some($dimmer))
    };
}

#[macro_export]
macro_rules! zoom {
    ($range:expr) => {
        dmxt_lib::builders::fixture::FixtureZoom::new($range, None)
    };
    ($range:expr, $reset:expr) => {
        dmxt_lib::builders::fixture::FixtureZoom::new($range, Some(address!($reset)))
    };
}