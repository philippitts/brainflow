/* automatically generated by rust-bindgen 0.59.1 */

#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BrainFlowExitCodes {
    StatusOk = 0,
    PortAlreadyOpenError = 1,
    UnableToOpenPortError = 2,
    SetPortError = 3,
    BoardWriteError = 4,
    IncommingMsgError = 5,
    InitialMsgError = 6,
    BoardNotReadyError = 7,
    StreamAlreadyRunError = 8,
    InvalidBufferSizeError = 9,
    StreamThreadError = 10,
    StreamThreadIsNotRunning = 11,
    EmptyBufferError = 12,
    InvalidArgumentsError = 13,
    UnsupportedBoardError = 14,
    BoardNotCreatedError = 15,
    AnotherBoardIsCreatedError = 16,
    GeneralError = 17,
    SyncTimeoutError = 18,
    JsonNotFoundError = 19,
    NoSuchDataInJsonError = 20,
    ClassifierIsNotPreparedError = 21,
    AnotherClassifierIsPreparedError = 22,
    UnsupportedClassifierAndMetricCombinationError = 23,
}
impl BoardIds {
    pub const FIRST: BoardIds = BoardIds::PlaybackFileBoard;
}
impl BoardIds {
    pub const LAST: BoardIds = BoardIds::BrainaliveBoard;
}
#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BoardIds {
    PlaybackFileBoard = -3,
    StreamingBoard = -2,
    SyntheticBoard = -1,
    CytonBoard = 0,
    GanglionBoard = 1,
    CytonDaisyBoard = 2,
    GaleaBoard = 3,
    GanglionWifiBoard = 4,
    CytonWifiBoard = 5,
    CytonDaisyWifiBoard = 6,
    BrainbitBoard = 7,
    UnicornBoard = 8,
    CallibriEegBoard = 9,
    CallibriEmgBoard = 10,
    CallibriEcgBoard = 11,
    FasciaBoard = 12,
    Notion1Board = 13,
    Notion2Board = 14,
    IronbciBoard = 15,
    GforceProBoard = 16,
    Freeeeg32Board = 17,
    BrainbitBledBoard = 18,
    GforceDualBoard = 19,
    GaleaSerialBoard = 20,
    MuseSBledBoard = 21,
    Muse2BledBoard = 22,
    CrownBoard = 23,
    AntNeuroEe410Board = 24,
    AntNeuroEe411Board = 25,
    AntNeuroEe430Board = 26,
    AntNeuroEe211Board = 27,
    AntNeuroEe212Board = 28,
    AntNeuroEe213Board = 29,
    AntNeuroEe214Board = 30,
    AntNeuroEe215Board = 31,
    AntNeuroEe221Board = 32,
    AntNeuroEe222Board = 33,
    AntNeuroEe223Board = 34,
    AntNeuroEe224Board = 35,
    AntNeuroEe225Board = 36,
    EnophoneBoard = 37,
    Muse2Board = 38,
    MuseSBoard = 39,
    BrainaliveBoard = 40,
}
#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IpProtocolType {
    None = 0,
    Udp = 1,
    Tcp = 2,
}
#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FilterTypes {
    Butterworth = 0,
    ChebyshevType1 = 1,
    Bessel = 2,
}
#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AggOperations {
    Mean = 0,
    Median = 1,
    Each = 2,
}
#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WindowFunctions {
    NoWindow = 0,
    Hanning = 1,
    Hamming = 2,
    BlackmanHarris = 3,
}
#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DetrendOperations {
    None = 0,
    Constant = 1,
    Linear = 2,
}
#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BrainFlowMetrics {
    Relaxation = 0,
    Concentration = 1,
    UserDefined = 2,
}
#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BrainFlowClassifiers {
    Regression = 0,
    Knn = 1,
    Svm = 2,
    Lda = 3,
    DynLibClassifier = 4,
    OnnxClassifier = 5,
}
#[repr(i32)]
#[doc = " LogLevels enum to store all possible log levels"]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LogLevels {
    LevelTrace = 0,
    #[doc = " Trace"]
    LevelDebug = 1,
    #[doc = " Debug"]
    LevelInfo = 2,
    #[doc = " Info"]
    LevelWarn = 3,
    #[doc = " Warn"]
    LevelError = 4,
    #[doc = " Error"]
    LevelCritical = 5,
    #[doc = " Critical"]
    LevelOff = 6,
}
#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum NoiseTypes {
    Fifty = 0,
    Sixty = 1,
}
