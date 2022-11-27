use crate::builders::error::BuildError;
use crate::dmx::{DMXRange, DMXAddress, Color, Channel};

use std::path::Path;
use std::vec;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Default, Serialize, Deserialize)]
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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureChannelMode {
    pub total_channels: Channel,
    
    pub operation_modes: Vec<FixtureOperationMode>,
    pub movement: Option<FixtureMovement>,
    pub lights: Option<FixtureMatrix>,
    
    pub name: Option<FixtureName>,
    pub zoom: Option<FixtureZoom>,

    pub custom: Option<Vec<FixtureCustomOperation>>,
}

impl FixtureChannelMode {
    pub fn builder() -> FixtureChannelModeBuilder {
        FixtureChannelModeBuilder::default()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FixtureChannelModeBuilder {
    total_channels: Option<Channel>,
    operation_modes: Vec<FixtureOperationMode>,
    lights: Option<FixtureMatrix>,
    name: Option<FixtureName>,
    movement: Option<FixtureMovement>,
    zoom: Option<FixtureZoom>,
    custom: Option<Vec<FixtureCustomOperation>>,
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
        self.lights = Some(FixtureMatrix::new(vec![vec![lights]]));
        self
    }
    pub fn matrix(&mut self, matrix: FixtureMatrix) -> &mut Self {
        self.lights = Some(matrix);
        self
    }
    pub fn name(&mut self, name: FixtureName) -> &mut Self {
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
    pub fn custom(&mut self, custom: FixtureCustomOperation) -> &mut Self {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureOperationMode {
    pub mode_type: OperationModeType,
    pub address: Option<DMXAddress>,
    pub submodes: Vec<FixtureCustomOperation>,
}

impl FixtureOperationMode {
    pub fn builder() -> FixtureOperationModeBuilder {
        FixtureOperationModeBuilder::default()
    }
    pub fn new(mode_type: OperationModeType, address: Option<DMXAddress>, submodes: Vec<FixtureCustomOperation>) -> Self {
        Self {
            mode_type,
            address,
            submodes,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FixtureOperationModeBuilder {
    mode_type: Option<OperationModeType>,
    address: Option<DMXAddress>,
    submodes: Vec<FixtureCustomOperation>,
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
    pub fn submode(&mut self, submode: FixtureCustomOperation) -> &mut Self {
        self.submodes.push(submode);
        self
    }
    pub fn build(&mut self) -> Result<FixtureOperationMode, BuildError> {
        if self.mode_type.is_none() {
            return Err(BuildError::MissingField("mode_type"));
        }
        if self.address.is_none() && self.submodes.is_empty() {
            return Err(BuildError::MissingField("address or submodes"));
        }
        Ok(FixtureOperationMode {
            mode_type: self.mode_type.clone().unwrap(),
            address: self.address.clone(),
            submodes: self.submodes.clone(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureMatrix {
    pub matrix: Vec<Vec<FixtureLights>>,
}

impl FixtureMatrix {
    pub fn new(matrix: Vec<Vec<FixtureLights>>) -> Self {
        Self {
            matrix,
        }
    }

    pub fn builder() -> FixtureMatrixBuilder {
        FixtureMatrixBuilder::default()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FixtureMatrixBuilder {
    matrix: Vec<Vec<FixtureLights>>,
}

impl FixtureMatrixBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn row(&mut self, row: Vec<FixtureLights>) -> &mut Self {
        self.matrix.push(row);
        self
    }
    pub fn build(&self) -> Result<FixtureMatrix, BuildError> {
        if self.matrix.is_empty() {
            return Err(BuildError::MissingField("matrix"));
        }
        Ok(FixtureMatrix {
            matrix: self.matrix.clone(),
        })
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixtureColorMode {
    Presets(Vec<(Color, DMXAddress)>),
    RGB(DMXRange, DMXRange, DMXRange),
    RGBW(DMXRange, DMXRange, DMXRange, DMXRange),
    CMY(DMXRange, DMXRange, DMXRange),
    CMYW(DMXRange, DMXRange, DMXRange, DMXRange),
    RgbTrailingChannels(DMXRange),
    RgbwTrailingChannels(DMXRange),
    CmyTrailingChannels(DMXRange),
    CmywTrailingChannels(DMXRange),
    Custom(String, Vec<DMXRange>),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FixtureMovement {
    pub pan: Option<MovementAxis>,
    pub tilt: Option<MovementAxis>,
}

impl FixtureMovement {
    pub fn builder() -> FixtureMovementBuilder {
        FixtureMovementBuilder::default()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixtureCustomOperation {
    Slider(FixtureName, DMXRange),
    Button(FixtureName, DMXAddress),
    Stepped(FixtureName, Vec<(FixtureName, DMXAddress)>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationModeType {
    Off,
    On,
    Auto,
    SoundToLight,
    DMX(FixtureName),
}