use crate::builders::error::BuildError;
use crate::dmx::{DMXRange, DMXAddress, Color, Channel};

use std::path::Path;


#[derive(Debug, Clone)]
pub struct FixtureModel {
    pub name: FixtureName,
    pub manufacturer: String,
    pub channel_modes: Vec<FixtureChannelMode>,
}

impl FixtureModel {
    pub fn builder () -> FixtureModelBuilder {
        FixtureModelBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct FixtureModelBuilder {
    name: Option<FixtureName>,
    manufacturer: Option<String>,
    channel_modes: Vec<FixtureChannelMode>,
}

impl FixtureModelBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn model(&mut self, name: FixtureName) -> &mut Self {
        self.name = Some(name);
        self
    }
    pub fn manufacturer(&mut self, manufacturer: String) -> &mut Self {
        self.manufacturer = Some(manufacturer);
        self
    }
    pub fn channel_mode(&mut self, channel_mode: FixtureChannelMode) -> &mut Self {
        self.channel_modes.push(channel_mode);
        self
    }

    pub fn build(&self) -> Result<FixtureModel, BuildError> {
        if self.name.is_none() {
            return Err(BuildError::MissingField("name"));
        }
        if self.manufacturer.is_none() {
            return Err(BuildError::MissingField("manufacturer"));
        }
        if self.channel_modes.is_empty() {
            return Err(BuildError::EmptyField("channel_modes"));
        }
        Ok(FixtureModel {
            name: self.name.clone().unwrap(),
            manufacturer: self.manufacturer.clone().unwrap(),
            channel_modes: self.channel_modes.clone(),
        })
    }
}


#[derive(Debug, Clone)]
pub struct FixtureChannelMode {
    pub total_channels: Channel,
    
    pub operation_modes: Vec<FixtureOperationMode>,
    pub movement: Option<FixtureMovement>,
    pub lights: Option<FixtureLights>,
    
    pub name: Option<String>,
    pub zoom: Option<FixtureZoom>,

    pub custom: Option<Vec<FixtureCustom>>,
}

impl FixtureChannelMode {
    pub fn builder() -> FixtureChannelModeBuilder {
        FixtureChannelModeBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct FixtureChannelModeBuilder {
    total_channels: Option<Channel>,
    operation_modes: Vec<FixtureOperationMode>,
    lights: Option<FixtureLights>,
    name: Option<String>,
    movement: Option<FixtureMovement>,
    zoom: Option<FixtureZoom>,
    custom: Option<Vec<FixtureCustom>>,
}

impl FixtureChannelModeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn total_channels(&mut self, total_channels: Channel) -> &mut Self {
        self.total_channels = Some(total_channels);
        self
    }
    pub fn operation_mode(&mut self, operation_mode: FixtureOperationMode) -> &mut Self {
        self.operation_modes.push(operation_mode);
        self
    }
    pub fn lights(&mut self, lights: FixtureLights) -> &mut Self {
        self.lights = Some(lights);
        self
    }
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    pub fn movement(&mut self, movement: FixtureMovement) -> &mut Self {
        self.movement = Some(movement);
        self
    }
    pub fn zoom(&mut self, zoom: FixtureZoom) -> &mut Self {
        self.zoom = Some(zoom);
        self
    }
    pub fn custom(&mut self, custom: FixtureCustom) -> &mut Self {
        if self.custom.is_none() {
            self.custom = Some(Vec::new());
        }
        self.custom.as_mut().unwrap().push(custom);
        self
    }

    pub fn build(&self) -> Result<FixtureChannelMode, BuildError> {
        if self.total_channels.is_none() {
            return Err(BuildError::MissingField("total_channels"));
        }
        if self.operation_modes.is_empty() {
            return Err(BuildError::MissingField("operation_modes"));
        }
        if self.lights.is_none() {
            return Err(BuildError::MissingField("lights"));
        }
        Ok(FixtureChannelMode {
            total_channels: self.total_channels.unwrap(),
            operation_modes: self.operation_modes.clone(),
            lights: self.lights.clone(),
            name: self.name.clone(),
            movement: self.movement.clone(),
            zoom: self.zoom.clone(),
            custom: self.custom.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FixtureOperationMode {
    pub mode_type: OperationModeType,
    pub address: DMXAddress,
    pub submodes: Vec<FixtureSubOperationMode>,
}

impl FixtureOperationMode {
    pub fn builder() -> FixtureOperationModeBuilder {
        FixtureOperationModeBuilder::default()
    }
    pub fn new(mode_type: OperationModeType, address: DMXAddress, submodes: Vec<FixtureSubOperationMode>) -> Self {
        Self {
            mode_type,
            address,
            submodes,
        }
    }
}

#[derive(Debug, Default)]
pub struct FixtureOperationModeBuilder {
    mode_type: Option<OperationModeType>,
    address: Option<DMXAddress>,
    submodes: Vec<FixtureSubOperationMode>,
}

impl FixtureOperationModeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn mode_type(&mut self, mode_type: OperationModeType) -> &mut Self {
        self.mode_type = Some(mode_type);
        self
    }
    pub fn address(&mut self, address: DMXAddress) -> &mut Self {
        self.address = Some(address);
        self
    }
    pub fn submode(&mut self, submode: FixtureSubOperationMode) -> &mut Self {
        self.submodes.push(submode);
        self
    }
    pub fn build(&mut self) -> Result<FixtureOperationMode, BuildError> {
        if self.mode_type.is_none() {
            return Err(BuildError::MissingField("mode_type"));
        }
        if self.address.is_none() {
            return Err(BuildError::MissingField("address"));
        }
        Ok(FixtureOperationMode {
            mode_type: self.mode_type.clone().unwrap(),
            address: self.address.unwrap(),
            submodes: self.submodes.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FixtureLights {
    pub color_mode: FixtureColorMode,
    pub dimmer: Option<DMXRange>,
}

impl FixtureLights {
    pub fn new(color_mode: FixtureColorMode, dimmer: Option<DMXRange>) -> Self {
        Self {
            color_mode,
            dimmer,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FixtureColorMode {
    Presets(Vec<(Color, DMXAddress)>),
    RGB(DMXRange, DMXRange, DMXRange),
    RGBW(DMXRange, DMXRange, DMXRange, DMXRange),
    CMY(DMXRange, DMXRange, DMXRange),
    CMYW(DMXRange, DMXRange, DMXRange, DMXRange),
    Custom(String, Vec<DMXRange>),
}

#[derive(Debug, Default, Clone)]
pub struct FixtureMovement {
    pub pan: Option<MovementAxis>,
    pub tilt: Option<MovementAxis>,
}

impl FixtureMovement {
    pub fn builder() -> FixtureMovementBuilder {
        FixtureMovementBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct FixtureMovementBuilder {
    pan: Option<MovementAxis>,
    tilt: Option<MovementAxis>,
}

impl FixtureMovementBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn pan(&mut self, pan: MovementAxis) -> &mut Self {
        self.pan = Some(pan);
        self
    }
    pub fn tilt(&mut self, tilt: MovementAxis) -> &mut Self {
        self.tilt = Some(tilt);
        self
    }
    pub fn build(&self) -> Result<FixtureMovement, BuildError> {
        Ok(FixtureMovement {
            pan: self.pan.clone(),
            tilt: self.tilt.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MovementAxis {
    pub range: DMXRange,
    pub reset: Option<DMXAddress>,
}

impl MovementAxis {
    pub fn new(range: DMXRange, reset: Option<DMXAddress>) -> Self {
        Self {
            range,
            reset,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FixtureZoom {
    pub range: DMXRange,
    pub reset: Option<DMXAddress>,
}

impl FixtureZoom {
    pub fn new(range: DMXRange, reset: Option<DMXAddress>) -> Self {
        Self {
            range,
            reset,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FixtureCustom {
    Slider(String, DMXRange),
    Button(String, DMXAddress),
    Stepped(String, Vec<(String, DMXAddress)>),
}

#[derive(Debug, Clone)]
pub struct FixtureSubOperationMode {
    pub name: FixtureName,
    pub address: DMXAddress,
}

impl FixtureSubOperationMode {
    pub fn new(name: FixtureName, address: DMXAddress) -> Self {
        Self {
            name,
            address,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FixtureName {
    name: String,
    icon: Option<Box<Path>>,
}

impl FixtureName {
    pub fn new(name: String) -> Self {
        Self {
            name,
            icon: None,
        }
    }

    pub fn new_with_icon(name: String, icon: Box<Path>) -> Self {
        Self {
            name,
            icon: Some(icon),
        }
    }

    pub fn icon(&mut self, icon: &Path) -> &mut Self {
        self.icon = Some(icon.to_path_buf().into_boxed_path());
        self
    }
}

impl From<String> for FixtureName {
    fn from(name: String) -> Self {
        Self {
            name,
            icon: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum OperationModeType {
    Off,
    On,
    Auto,
    SoundToLight,
    DMX(FixtureName),
}