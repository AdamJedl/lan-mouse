use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

/*
 * https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input
 * https://download.microsoft.com/download/1/6/1/161ba512-40e2-4cc9-843a-923143f3456c/translate.pdf
 * https://kbd-project.org/docs/scancodes/scancodes-1.html
 */
#[repr(u32)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
pub enum Windows {
    Shutdown = 0xE05E,
    SystemSleep = 0xE05F,
    SystemWakeUp = 0xE063,
    ErrorRollOver = 0x00FF,
    KeyA = 0x001E,
    KeyB = 0x0030,
    KeyC = 0x002E,
    KeyD = 0x0020,
    KeyE = 0x0012,
    KeyF = 0x0021,
    KeyG = 0x0022,
    KeyH = 0x0023,
    KeyI = 0x0017,
    KeyJ = 0x0024,
    KeyK = 0x0025,
    KeyL = 0x0026,
    KeyM = 0x0032,
    KeyN = 0x0031,
    KeyO = 0x0018,
    KeyP = 0x0019,
    KeyQ = 0x0010,
    KeyR = 0x0013,
    KeyS = 0x001F,
    KeyT = 0x0014,
    KeyU = 0x0016,
    KeyV = 0x002F,
    KeyW = 0x0011,
    KeyX = 0x002D,
    KeyY = 0x0015,
    KeyZ = 0x002C,
    Key1 = 0x0002,
    Key2 = 0x0003,
    Key3 = 0x0004,
    Key4 = 0x0005,
    Key5 = 0x0006,
    Key6 = 0x0007,
    Key7 = 0x0008,
    Key8 = 0x0009,
    Key9 = 0x000A,
    Key0 = 0x000B,
    KeyEnter = 0x001C,
    KeyEsc = 0x0001,
    KeyDelete = 0x000E,
    KeyTab = 0x000F,
    KeySpace = 0x0039,
    KeyMinus = 0x000C,
    KeyEqual = 0x000D,
    KeyLeftBrace = 0x001A,
    KeyRightBrace = 0x001B,
    KeyBackslash = 0x002B,
    KeySemiColon = 0x0027,
    KeyApostrophe = 0x0028,
    KeyGrave = 0x0029,
    KeyComma = 0x0033,
    KeyDot = 0x0034,
    KeySlash = 0x0035,
    KeyCapsLock = 0x003A,
    KeyF1 = 0x003B,
    KeyF2 = 0x003C,
    KeyF3 = 0x003D,
    KeyF4 = 0x003E,
    KeyF5 = 0x003F,
    KeyF6 = 0x0040,
    KeyF7 = 0x0041,
    KeyF8 = 0x0042,
    KeyF9 = 0x0043,
    KeyF10 = 0x0044,
    KeyF11 = 0x0057,
    KeyF12 = 0x0058,
    KeyPrintScreen = 0xE037,
    KeyScrollLock = 0x0046,
    KeyPause = 0xE11D45,
    KeyInsert = 0xE052,
    KeyHome = 0xE047,
    KeyPageUp = 0xE049,
    KeyDeleteForward = 0xE053,
    KeyEnd = 0xE04F,
    KeyPageDown = 0xE051,
    KeyRight = 0xE04D,
    KeyLeft = 0xE04B,
    KeyDown = 0xE050,
    KeyUp = 0xE048,
    KeypadNumLock = 0x0045,
    KeypadSlash = 0xE035,
    KeypadStar = 0x0037,
    KeypadDash = 0x004A,
    KeypadPlus = 0x004E,
    KeypadEnter = 0xE01C,
    Keypad1End = 0x004F,
    Keypad2DownArrow = 0x0050,
    Keypad3PageDn = 0x0051,
    Keypad4LeftArrow = 0x004B,
    Keypad5 = 0x004C,
    Keypad6RightArrow = 0x004D,
    Keypad7Home = 0x0047,
    Keypad8UpArrow = 0x0048,
    Keypad9PageUp = 0x0049,
    Keypad0Insert = 0x0052,
    KeypadDot = 0x0053,
    KeyNonUSSlashBar = 0x0056,
    KeyApplication = 0xE05D,
    KeypadEquals = 0x0059,
    KeyF13 = 0x0064,
    KeyF14 = 0x0065,
    KeyF15 = 0x0066,
    KeyF16 = 0x0067,
    KeyF17 = 0x0068,
    KeyF18 = 0x0069,
    KeyF19 = 0x006A,
    KeyF20 = 0x006B,
    KeyF21 = 0x006C,
    KeyF22 = 0x006D,
    KeyF23 = 0x006E,
    KeyF24 = 0x0076, // KeyLANG5
    KeypadComma = 0x007E,
    KeyInternational1 = 0x0073,
    KeyInternational2 = 0x0070,
    KeyInternational3 = 0x007D, // typo in doc -> its Int'l 3 not Int'l 2
    #[allow(dead_code)]
    KeyInternational4 = 0x0079,
    #[allow(dead_code)]
    KeyInternational5 = 0x007B,
    // KeyInternational6 = 0x005C,
    KeyLANG1 = 0x0072,
    KeyLANG2 = 0x0071,
    KeyLANG3 = 0x0078,
    KeyLANG4 = 0x0077,
    // KeyLANG5 = 0x0076,
    KeyLeftCtrl = 0x001D,
    KeyLeftShift = 0x002A,
    KeyLeftAlt = 0x0038,
    KeyLeftGUI = 0xE05B,
    KeyRightCtrl = 0xE01D,
    KeyRightShift = 0x0036,
    KeyFakeRightShift = 0xE036,
    KeyRightAlt = 0xE038,
    KeyRightGUI = 0xE05C,
    KeyScanNextTrack = 0xE019,
    KeyScanPreviousTrack = 0xE010,
    KeyStop = 0xE024,
    KeyPlayPause = 0xE022,
    KeyMute = 0xE020,
    KeyVolumeUp = 0xE030,
    KeyVolumeDown = 0xE02E,
    #[allow(dead_code)]
    ALConsumerControlConfiguration = 0xE06D, // TODO Unused
    ALEmailReader = 0xE06C,
    ALCalculator = 0xE021,
    ALLocalMachineBrowser = 0xE06B,
    ACSearch = 0xE065,
    ACHome = 0xE032,
    ACBack = 0xE06A,
    ACForward = 0xE069,
    ACStop = 0xE068,
    ACRefresh = 0xE067,
    ACBookmarks = 0xE066,
}

/*
 * https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h
 */
#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, Hash, PartialEq, TryFromPrimitive)]
#[allow(dead_code)]
pub enum Linux {
    KeyReserved = 0,
    KeyEsc = 1,
    Key1 = 2,
    Key2 = 3,
    Key3 = 4,
    Key4 = 5,
    Key5 = 6,
    Key6 = 7,
    Key7 = 8,
    Key8 = 9,
    Key9 = 10,
    Key0 = 11,
    KeyMinus = 12,
    KeyEqual = 13,
    KeyBackspace = 14,
    KeyTab = 15,
    KeyQ = 16,
    KeyW = 17,
    KeyE = 18,
    KeyR = 19,
    KeyT = 20,
    KeyY = 21,
    KeyU = 22,
    KeyI = 23,
    KeyO = 24,
    KeyP = 25,
    KeyLeftbrace = 26,
    KeyRightbrace = 27,
    KeyEnter = 28,
    KeyLeftCtrl = 29,
    KeyA = 30,
    KeyS = 31,
    KeyD = 32,
    KeyF = 33,
    KeyG = 34,
    KeyH = 35,
    KeyJ = 36,
    KeyK = 37,
    KeyL = 38,
    KeySemicolon = 39,
    KeyApostrophe = 40,
    KeyGrave = 41,
    KeyLeftShift = 42,
    KeyBackslash = 43,
    KeyZ = 44,
    KeyX = 45,
    KeyC = 46,
    KeyV = 47,
    KeyB = 48,
    KeyN = 49,
    KeyM = 50,
    KeyComma = 51,
    KeyDot = 52,
    KeySlash = 53,
    KeyRightShift = 54,
    KeyKpAsterisk = 55,
    KeyLeftAlt = 56,
    KeySpace = 57,
    KeyCapsLock = 58,
    KeyF1 = 59,
    KeyF2 = 60,
    KeyF3 = 61,
    KeyF4 = 62,
    KeyF5 = 63,
    KeyF6 = 64,
    KeyF7 = 65,
    KeyF8 = 66,
    KeyF9 = 67,
    KeyF10 = 68,
    KeyNumlock = 69,
    KeyScrollLock = 70,
    KeyKp7 = 71,
    KeyKp8 = 72,
    KeyKp9 = 73,
    KeyKpMinus = 74,
    KeyKp4 = 75,
    KeyKp5 = 76,
    KeyKp6 = 77,
    KeyKpplus = 78,
    KeyKp1 = 79,
    KeyKp2 = 80,
    KeyKp3 = 81,
    KeyKp0 = 82,
    KeyKpDot = 83,
    Invalid = 84,
    KeyZenkakuhankaku = 85,
    Key102nd = 86,
    KeyF11 = 87,
    KeyF12 = 88,
    KeyRo = 89,
    KeyKatakana = 90,
    KeyHiragana = 91,
    KeyHenkan = 92,
    KeyKatakanahiragana = 93,
    KeyMuhenkan = 94,
    KeyKpJpComma = 95,
    KeyKpEnter = 96,
    KeyRightCtrl = 97,
    KeyKpslash = 98,
    KeySysrq = 99,
    KeyRightalt = 100,
    KeyLinefeed = 101,
    KeyHome = 102,
    KeyUp = 103,
    KeyPageup = 104,
    KeyLeft = 105,
    KeyRight = 106,
    KeyEnd = 107,
    KeyDown = 108,
    KeyPagedown = 109,
    KeyInsert = 110,
    KeyDelete = 111,
    KeyMacro = 112,
    KeyMute = 113,
    KeyVolumeDown = 114,
    KeyVolumeUp = 115,
    KeyPower = 116, /* SC System Power Down */
    KeyKpequal = 117,
    KeyKpplusminus = 118,
    KeyPause = 119,
    KeyScale = 120, /* AL Compiz Scale (Expose) */
    KeyKpcomma = 121,
    KeyHanguel = 122,
    // KEY_HANGUEL = KeyHangeul,
    KeyHanja = 123,
    KeyYen = 124,
    KeyLeftMeta = 125,
    KeyRightmeta = 126,
    KeyCompose = 127,
    KeyStop = 128, /* AC Stop */
    KeyAgain = 129,
    KeyProps = 130, /* AC Properties */
    KeyUndo = 131,  /* AC Undo */
    KeyFront = 132,
    KeyCopy = 133,  /* AC Copy */
    KeyOpen = 134,  /* AC Open */
    KeyPaste = 135, /* AC Paste */
    KeyFind = 136,  /* AC Search */
    KeyCut = 137,   /* AC Cut */
    KeyHelp = 138,  /* AL Integrated Help Center */
    KeyMenu = 139,  /* Menu (show menu) */
    KeyCalc = 140,  /* AL Calculator */
    KeySetup = 141,
    KeySleep = 142,  /* SC System Sleep */
    KeyWakeup = 143, /* System Wake Up */
    KeyFile = 144,   /* AL Local Machine Browser */
    KeySendfile = 145,
    KeyDeletefile = 146,
    KeyXfer = 147,
    KeyProg1 = 148,
    KeyProg2 = 149,
    KeyWww = 150, /* AL Internet Browser */
    KeyMsdos = 151,
    KeyCoffee = 152, /* AL Terminal Lock/Screensaver */
    // KEY_SCREENLOCK = KeyCoffee,
    KeyRotateDisplay = 153, /* Display orientation for e.g. tablets */
    // KEY_DIRECTION = KeyRotateDisplay,
    KeyCyclewindows = 154,
    KeyMail = 155,
    KeyBookmarks = 156, /* AC Bookmarks */
    KeyComputer = 157,
    KeyBack = 158,    /* AC Back */
    KeyForward = 159, /* AC Forward */
    KeyClosecd = 160,
    KeyEjectcd = 161,
    KeyEjectclosecd = 162,
    KeyNextsong = 163,
    KeyPlaypause = 164,
    KeyPrevioussong = 165,
    KeyStopcd = 166,
    KeyRecord = 167,
    KeyRewind = 168,
    KeyPhone = 169, /* Media Select Telephone */
    KeyIso = 170,
    KeyConfig = 171,   /* AL Consumer Control Configuration */
    KeyHomepage = 172, /* AC Home */
    KeyRefresh = 173,  /* AC Refresh */
    KeyExit = 174,     /* AC Exit */
    KeyMove = 175,
    KeyEdit = 176,
    KeyScrollup = 177,
    KeyScrolldown = 178,
    KeyKpleftparen = 179,
    KeyKprightparen = 180,
    KeyNew = 181,  /* AC New */
    KeyRedo = 182, /* AC Redo/Repeat */
    KeyF13 = 183,
    KeyF14 = 184,
    KeyF15 = 185,
    KeyF16 = 186,
    KeyF17 = 187,
    KeyF18 = 188,
    KeyF19 = 189,
    KeyF20 = 190,
    KeyF21 = 191,
    KeyF22 = 192,
    KeyF23 = 193,
    KeyF24 = 194,
    Invalid1 = 195,
    Invalid2 = 196,
    Invalid3 = 197,
    Invalid4 = 198,
    Invalid5 = 199,
    KeyPlaycd = 200,
    KeyPausecd = 201,
    KeyProg3 = 202,
    KeyProg4 = 203,
    KeyAllApplications = 204, /* AC Desktop Show All Applications */
    // KEY_DASHBOARD = KeyAllApplications,
    KeySuspend = 205,
    KeyClose = 206, /* AC Close */
    KeyPlay = 207,
    KeyFastforward = 208,
    KeyBassboost = 209,
    KeyPrint = 210, /* AC Print */
    KeyHp = 211,
    KeyCamera = 212,
    KeySound = 213,
    KeyQuestion = 214,
    KeyEmail = 215,
    KeyChat = 216,
    KeySearch = 217,
    KeyConnect = 218,
    KeyFinance = 219, /* AL Checkbook/Finance */
    KeySport = 220,
    KeyShop = 221,
    KeyAlterase = 222,
    KeyCancel = 223, /* AC Cancel */
    KeyBrightnessdown = 224,
    KeyBrightnessup = 225,
    KeyMedia = 226,
    KeySwitchvideomode = 227, /* Cycle between available video, outputs (Monitor/LCD/TV-out/etc) */
    KeyKbdillumtoggle = 228,
    KeyKbdillumdown = 229,
    KeyKbdillumup = 230,
    KeySend = 231,        /* AC Send */
    KeyReply = 232,       /* AC Reply */
    KeyForwardmail = 233, /* AC Forward Msg */
    KeySave = 234,        /* AC Save */
    KeyDocuments = 235,
    KeyBattery = 236,
    KeyBluetooth = 237,
    KeyWlan = 238,
    KeyUwb = 239,
    KeyUnknown = 240,
    KeyVideoNext = 241,       /* drive next video source */
    KeyVideoPrev = 242,       /* drive previous video source */
    KeyBrightnessCycle = 243, /* brightness up, after max is min */
    KeyBrightnessAuto = 244, /* Set Auto Brightness: manual, brightness control is off, rely on ambient */
    // KEY_BRIGHTNESS_ZERO=KeyBrightnessAuto,
    KeyDisplayOff = 245, /* display device to off state */
    KeyWwan = 246,       /* Wireless WAN (LTE, UMTS, GSM, etc.) */
    // KEY_WIMAX =	KeyWwan,
    KeyRfkill = 247,  /* Key that controls all radios */
    KeyMicmute = 248, /* Mute / unmute the microphone */
    KeyCount = 249,
}

impl TryFrom<Linux> for Windows {
    type Error = ();

    fn try_from(value: Linux) -> Result<Self, Self::Error> {
        match value {
            Linux::KeyReserved => Err(()),
            Linux::KeyEsc => Ok(Self::KeyEsc),
            Linux::Key1 => Ok(Self::Key1),
            Linux::Key2 => Ok(Self::Key2),
            Linux::Key3 => Ok(Self::Key3),
            Linux::Key4 => Ok(Self::Key4),
            Linux::Key5 => Ok(Self::Key5),
            Linux::Key6 => Ok(Self::Key6),
            Linux::Key7 => Ok(Self::Key7),
            Linux::Key8 => Ok(Self::Key8),
            Linux::Key9 => Ok(Self::Key9),
            Linux::Key0 => Ok(Self::Key0),
            Linux::KeyMinus => Ok(Self::KeyMinus),
            Linux::KeyEqual => Ok(Self::KeyEqual),
            Linux::KeyBackspace => Ok(Self::KeyDelete),
            Linux::KeyTab => Ok(Self::KeyTab),
            Linux::KeyQ => Ok(Self::KeyQ),
            Linux::KeyW => Ok(Self::KeyW),
            Linux::KeyE => Ok(Self::KeyE),
            Linux::KeyR => Ok(Self::KeyR),
            Linux::KeyT => Ok(Self::KeyT),
            Linux::KeyY => Ok(Self::KeyY),
            Linux::KeyU => Ok(Self::KeyU),
            Linux::KeyI => Ok(Self::KeyI),
            Linux::KeyO => Ok(Self::KeyO),
            Linux::KeyP => Ok(Self::KeyP),
            Linux::KeyLeftbrace => Ok(Self::KeyLeftBrace),
            Linux::KeyRightbrace => Ok(Self::KeyRightBrace),
            Linux::KeyEnter => Ok(Self::KeyEnter),
            Linux::KeyLeftCtrl => Ok(Self::KeyLeftCtrl),
            Linux::KeyA => Ok(Self::KeyA),
            Linux::KeyS => Ok(Self::KeyS),
            Linux::KeyD => Ok(Self::KeyD),
            Linux::KeyF => Ok(Self::KeyF),
            Linux::KeyG => Ok(Self::KeyG),
            Linux::KeyH => Ok(Self::KeyH),
            Linux::KeyJ => Ok(Self::KeyJ),
            Linux::KeyK => Ok(Self::KeyK),
            Linux::KeyL => Ok(Self::KeyL),
            Linux::KeySemicolon => Ok(Self::KeySemiColon),
            Linux::KeyApostrophe => Ok(Self::KeyApostrophe),
            Linux::KeyGrave => Ok(Self::KeyGrave),
            Linux::KeyLeftShift => Ok(Self::KeyLeftShift),
            Linux::KeyBackslash => Ok(Self::KeyBackslash),
            Linux::KeyZ => Ok(Self::KeyZ),
            Linux::KeyX => Ok(Self::KeyX),
            Linux::KeyC => Ok(Self::KeyC),
            Linux::KeyV => Ok(Self::KeyV),
            Linux::KeyB => Ok(Self::KeyB),
            Linux::KeyN => Ok(Self::KeyN),
            Linux::KeyM => Ok(Self::KeyM),
            Linux::KeyComma => Ok(Self::KeyComma),
            Linux::KeyDot => Ok(Self::KeyDot),
            Linux::KeySlash => Ok(Self::KeySlash),
            Linux::KeyRightShift => Ok(Self::KeyRightShift),
            Linux::KeyKpAsterisk => Ok(Self::KeypadStar),
            Linux::KeyLeftAlt => Ok(Self::KeyLeftAlt),
            Linux::KeySpace => Ok(Self::KeySpace),
            Linux::KeyCapsLock => Ok(Self::KeyCapsLock),
            Linux::KeyF1 => Ok(Self::KeyF1),
            Linux::KeyF2 => Ok(Self::KeyF2),
            Linux::KeyF3 => Ok(Self::KeyF3),
            Linux::KeyF4 => Ok(Self::KeyF4),
            Linux::KeyF5 => Ok(Self::KeyF5),
            Linux::KeyF6 => Ok(Self::KeyF6),
            Linux::KeyF7 => Ok(Self::KeyF7),
            Linux::KeyF8 => Ok(Self::KeyF8),
            Linux::KeyF9 => Ok(Self::KeyF9),
            Linux::KeyF10 => Ok(Self::KeyF10),
            Linux::KeyNumlock => Ok(Self::KeypadNumLock),
            Linux::KeyScrollLock => Ok(Self::KeyScrollLock),
            Linux::KeyKp7 => Ok(Self::Keypad7Home),
            Linux::KeyKp8 => Ok(Self::Keypad8UpArrow),
            Linux::KeyKp9 => Ok(Self::Keypad9PageUp),
            Linux::KeyKpMinus => Ok(Self::KeypadDash),
            Linux::KeyKp4 => Ok(Self::Keypad4LeftArrow),
            Linux::KeyKp5 => Ok(Self::Keypad5),
            Linux::KeyKp6 => Ok(Self::Keypad6RightArrow),
            Linux::KeyKpplus => Ok(Self::KeypadPlus),
            Linux::KeyKp1 => Ok(Self::Keypad1End),
            Linux::KeyKp2 => Ok(Self::Keypad2DownArrow),
            Linux::KeyKp3 => Ok(Self::Keypad3PageDn),
            Linux::KeyKp0 => Ok(Self::Keypad0Insert),
            Linux::KeyKpDot => Ok(Self::KeypadDot),
            Linux::KeyZenkakuhankaku => Ok(Self::KeyF24), // KeyLANG5
            Linux::Key102nd => Ok(Self::KeyNonUSSlashBar), // TODO unsure
            Linux::KeyF11 => Ok(Self::KeyF11),
            Linux::KeyF12 => Ok(Self::KeyF12),
            Linux::KeyRo => Ok(Self::KeyInternational1),
            Linux::KeyKatakana => Ok(Self::KeyLANG3),
            Linux::KeyHiragana => Ok(Self::KeyLANG4),
            Linux::KeyHenkan => Ok(Self::KeyInternational4),
            Linux::KeyKatakanahiragana => Ok(Self::KeyInternational2),
            Linux::KeyMuhenkan => Ok(Self::KeyInternational5),
            Linux::KeyKpJpComma => Ok(Self::KeypadComma),
            Linux::KeyKpEnter => Ok(Self::KeypadEnter),
            Linux::KeyRightCtrl => Ok(Self::KeyRightCtrl),
            Linux::KeyKpslash => Ok(Self::KeypadSlash),
            Linux::KeySysrq => Ok(Self::KeyPrintScreen), // TODO Windows does not have Sysrq, right?
            Linux::KeyRightalt => Ok(Self::KeyRightAlt),
            Linux::KeyLinefeed => Ok(Self::KeyEnter), // TODO unsure
            Linux::KeyHome => Ok(Self::KeyHome),
            Linux::KeyUp => Ok(Self::KeyUp),
            Linux::KeyPageup => Ok(Self::KeyPageUp),
            Linux::KeyLeft => Ok(Self::KeyLeft),
            Linux::KeyRight => Ok(Self::KeyRight),
            Linux::KeyEnd => Ok(Self::KeyEnd),
            Linux::KeyDown => Ok(Self::KeyDown),
            Linux::KeyPagedown => Ok(Self::KeyPageDown),
            Linux::KeyInsert => Ok(Self::KeyInsert),
            Linux::KeyDelete => Ok(Self::KeyDeleteForward),
            Linux::KeyMacro => Err(()), // TODO
            Linux::KeyMute => Ok(Self::KeyMute),
            Linux::KeyVolumeDown => Ok(Self::KeyVolumeDown),
            Linux::KeyVolumeUp => Ok(Self::KeyVolumeUp),
            Linux::KeyPower => Ok(Self::Shutdown),
            Linux::KeyKpequal => Ok(Self::KeypadEquals),
            Linux::KeyKpplusminus => Ok(Self::KeypadPlus),
            Linux::KeyPause => Ok(Self::KeyPause),
            Linux::KeyScale => Err(()), // TODO
            Linux::KeyKpcomma => Ok(Self::KeypadComma),
            Linux::KeyHanguel => Ok(Self::KeyLANG1), // FIXME should be 00F2?
            Linux::KeyHanja => Ok(Self::KeyLANG2),   // FIXME should be 00F1?
            Linux::KeyYen => Ok(Self::KeyInternational3),
            Linux::KeyLeftMeta => Ok(Self::KeyLeftGUI),
            Linux::KeyRightmeta => Ok(Self::KeyRightGUI),
            Linux::KeyCompose => Ok(Self::KeyApplication),
            Linux::KeyStop => Ok(Self::ACStop),
            Linux::KeyAgain => Err(()),
            Linux::KeyProps => Err(()),
            Linux::KeyUndo => Err(()),
            Linux::KeyFront => Err(()),
            Linux::KeyCopy => Err(()),
            Linux::KeyOpen => Err(()),
            Linux::KeyPaste => Err(()),
            Linux::KeyFind => Ok(Self::ACSearch),
            Linux::KeyCut => Err(()),
            Linux::KeyHelp => Ok(Self::KeyF1), // AL Integrated Help Center?
            Linux::KeyMenu => Ok(Self::KeyApplication),
            Linux::KeyCalc => Ok(Self::ALCalculator),
            Linux::KeySetup => Err(()),
            Linux::KeySleep => Ok(Self::SystemSleep),
            Linux::KeyWakeup => Ok(Self::SystemWakeUp),
            Linux::KeyFile => Ok(Self::ALLocalMachineBrowser),
            Linux::KeySendfile => Err(()),
            Linux::KeyDeletefile => Err(()),
            Linux::KeyXfer => Err(()),
            Linux::KeyProg1 => Err(()),
            Linux::KeyProg2 => Err(()),
            Linux::KeyWww => Ok(Self::ACSearch), // TODO unsure
            Linux::KeyMsdos => Err(()),
            Linux::KeyCoffee => Err(()),
            Linux::KeyRotateDisplay => Err(()),
            Linux::KeyCyclewindows => Err(()),
            Linux::KeyMail => Ok(Self::ALEmailReader),
            Linux::KeyBookmarks => Ok(Self::ACBookmarks),
            Linux::KeyComputer => Ok(Self::ACHome),
            Linux::KeyBack => Ok(Self::ACBack),
            Linux::KeyForward => Ok(Self::ACForward),
            Linux::KeyClosecd => Err(()),
            Linux::KeyEjectcd => Err(()),
            Linux::KeyEjectclosecd => Err(()),
            Linux::KeyNextsong => Ok(Self::KeyScanNextTrack),
            Linux::KeyPlaypause => Ok(Self::KeyPlayPause),
            Linux::KeyPrevioussong => Ok(Self::KeyScanPreviousTrack),
            Linux::KeyStopcd => Ok(Self::KeyStop),
            Linux::KeyRecord => Err(()),
            Linux::KeyRewind => Err(()),
            Linux::KeyPhone => Err(()),
            Linux::KeyIso => Err(()),
            Linux::KeyConfig => Err(()),
            Linux::KeyHomepage => Ok(Self::ACHome),
            Linux::KeyRefresh => Ok(Self::ACRefresh),
            Linux::KeyExit => Err(()),
            Linux::KeyMove => Err(()),
            Linux::KeyEdit => Err(()),
            Linux::KeyScrollup => Err(()),
            Linux::KeyScrolldown => Err(()),
            Linux::KeyKpleftparen => Err(()),
            Linux::KeyKprightparen => Err(()),
            Linux::KeyNew => Err(()),
            Linux::KeyRedo => Err(()),
            Linux::KeyF13 => Ok(Self::KeyF13),
            Linux::KeyF14 => Ok(Self::KeyF14),
            Linux::KeyF15 => Ok(Self::KeyF15),
            Linux::KeyF16 => Ok(Self::KeyF16),
            Linux::KeyF17 => Ok(Self::KeyF17),
            Linux::KeyF18 => Ok(Self::KeyF18),
            Linux::KeyF19 => Ok(Self::KeyF19),
            Linux::KeyF20 => Ok(Self::KeyF20),
            Linux::KeyF21 => Ok(Self::KeyF21),
            Linux::KeyF22 => Ok(Self::KeyF22),
            Linux::KeyF23 => Ok(Self::KeyF23),
            Linux::KeyF24 => Ok(Self::KeyF24),
            Linux::KeyPlaycd => Err(()),
            Linux::KeyPausecd => Err(()),
            Linux::KeyProg3 => Err(()),
            Linux::KeyProg4 => Err(()),
            Linux::KeyAllApplications => Err(()),
            Linux::KeySuspend => Err(()),
            Linux::KeyClose => Err(()),
            Linux::KeyPlay => Err(()),
            Linux::KeyFastforward => Err(()),
            Linux::KeyBassboost => Err(()),
            Linux::KeyPrint => Err(()),
            Linux::KeyHp => Err(()),
            Linux::KeyCamera => Err(()),
            Linux::KeySound => Err(()),
            Linux::KeyQuestion => Err(()),
            Linux::KeyEmail => Err(()),
            Linux::KeyChat => Err(()),
            Linux::KeySearch => Err(()),
            Linux::KeyConnect => Err(()),
            Linux::KeyFinance => Err(()),
            Linux::KeySport => Err(()),
            Linux::KeyShop => Err(()),
            Linux::KeyAlterase => Err(()),
            Linux::KeyCancel => Err(()),
            Linux::KeyBrightnessdown => Err(()),
            Linux::KeyBrightnessup => Err(()),
            Linux::KeyMedia => Err(()),
            Linux::KeySwitchvideomode => Err(()),
            Linux::KeyKbdillumtoggle => Err(()),
            Linux::KeyKbdillumdown => Err(()),
            Linux::KeyKbdillumup => Err(()),
            Linux::KeySend => Err(()),
            Linux::KeyReply => Err(()),
            Linux::KeyForwardmail => Err(()),
            Linux::KeySave => Err(()),
            Linux::KeyDocuments => Err(()),
            Linux::KeyBattery => Err(()),
            Linux::KeyBluetooth => Err(()),
            Linux::KeyWlan => Err(()),
            Linux::KeyUwb => Err(()),
            Linux::KeyUnknown => Err(()),
            Linux::KeyVideoNext => Err(()),
            Linux::KeyVideoPrev => Err(()),
            Linux::KeyBrightnessCycle => Err(()),
            Linux::KeyBrightnessAuto => Err(()),
            Linux::KeyDisplayOff => Err(()),
            Linux::KeyWwan => Err(()),
            Linux::KeyRfkill => Err(()),
            Linux::KeyMicmute => Err(()),
            Linux::KeyCount => Err(()),
            Linux::Invalid => Err(()),
            Linux::Invalid1 => Err(()),
            Linux::Invalid2 => Err(()),
            Linux::Invalid3 => Err(()),
            Linux::Invalid4 => Err(()),
            Linux::Invalid5 => Err(()),
        }
    }
}
impl TryFrom<Windows> for Linux {
    type Error = ();

    fn try_from(value: Windows) -> Result<Self, Self::Error> {
        match value {
            Windows::Shutdown => Ok(Self::KeyPower),
            Windows::SystemSleep => Ok(Self::KeySleep),
            Windows::SystemWakeUp => Ok(Self::KeyWakeup),
            Windows::ErrorRollOver => Ok(Self::KeyRo),
            Windows::KeyA => Ok(Self::KeyA),
            Windows::KeyB => Ok(Self::KeyB),
            Windows::KeyC => Ok(Self::KeyC),
            Windows::KeyD => Ok(Self::KeyD),
            Windows::KeyE => Ok(Self::KeyE),
            Windows::KeyF => Ok(Self::KeyF),
            Windows::KeyG => Ok(Self::KeyG),
            Windows::KeyH => Ok(Self::KeyH),
            Windows::KeyI => Ok(Self::KeyI),
            Windows::KeyJ => Ok(Self::KeyJ),
            Windows::KeyK => Ok(Self::KeyK),
            Windows::KeyL => Ok(Self::KeyL),
            Windows::KeyM => Ok(Self::KeyM),
            Windows::KeyN => Ok(Self::KeyN),
            Windows::KeyO => Ok(Self::KeyO),
            Windows::KeyP => Ok(Self::KeyP),
            Windows::KeyQ => Ok(Self::KeyQ),
            Windows::KeyR => Ok(Self::KeyR),
            Windows::KeyS => Ok(Self::KeyS),
            Windows::KeyT => Ok(Self::KeyT),
            Windows::KeyU => Ok(Self::KeyU),
            Windows::KeyV => Ok(Self::KeyV),
            Windows::KeyW => Ok(Self::KeyW),
            Windows::KeyX => Ok(Self::KeyX),
            Windows::KeyY => Ok(Self::KeyY),
            Windows::KeyZ => Ok(Self::KeyZ),
            Windows::Key1 => Ok(Self::Key1),
            Windows::Key2 => Ok(Self::Key2),
            Windows::Key3 => Ok(Self::Key3),
            Windows::Key4 => Ok(Self::Key4),
            Windows::Key5 => Ok(Self::Key5),
            Windows::Key6 => Ok(Self::Key6),
            Windows::Key7 => Ok(Self::Key7),
            Windows::Key8 => Ok(Self::Key8),
            Windows::Key9 => Ok(Self::Key9),
            Windows::Key0 => Ok(Self::Key0),
            Windows::KeyEnter => Ok(Self::KeyEnter),
            Windows::KeyEsc => Ok(Self::KeyEsc),
            Windows::KeyDelete => Ok(Self::KeyBackspace),
            Windows::KeyTab => Ok(Self::KeyTab),
            Windows::KeySpace => Ok(Self::KeySpace),
            Windows::KeyMinus => Ok(Self::KeyMinus),
            Windows::KeyEqual => Ok(Self::KeyEqual),
            Windows::KeyLeftBrace => Ok(Self::KeyLeftbrace),
            Windows::KeyRightBrace => Ok(Self::KeyRightbrace),
            Windows::KeyBackslash => Ok(Self::KeyBackslash),
            Windows::KeySemiColon => Ok(Self::KeySemicolon),
            Windows::KeyApostrophe => Ok(Self::KeyApostrophe),
            Windows::KeyGrave => Ok(Self::KeyGrave),
            Windows::KeyComma => Ok(Self::KeyComma),
            Windows::KeyDot => Ok(Self::KeyDot),
            Windows::KeySlash => Ok(Self::KeySlash),
            Windows::KeyCapsLock => Ok(Self::KeyCapsLock),
            Windows::KeyF1 => Ok(Self::KeyF1),
            Windows::KeyF2 => Ok(Self::KeyF2),
            Windows::KeyF3 => Ok(Self::KeyF3),
            Windows::KeyF4 => Ok(Self::KeyF4),
            Windows::KeyF5 => Ok(Self::KeyF5),
            Windows::KeyF6 => Ok(Self::KeyF6),
            Windows::KeyF7 => Ok(Self::KeyF7),
            Windows::KeyF8 => Ok(Self::KeyF8),
            Windows::KeyF9 => Ok(Self::KeyF9),
            Windows::KeyF10 => Ok(Self::KeyF10),
            Windows::KeyF11 => Ok(Self::KeyF11),
            Windows::KeyF12 => Ok(Self::KeyF12),
            Windows::KeyPrintScreen => Ok(Self::KeySysrq),
            Windows::KeyScrollLock => Ok(Self::KeyScrollLock),
            Windows::KeyPause => Ok(Self::KeyPause),
            Windows::KeyInsert => Ok(Self::KeyInsert),
            Windows::KeyHome => Ok(Self::KeyHome),
            Windows::KeyPageUp => Ok(Self::KeyPageup),
            Windows::KeyDeleteForward => Ok(Self::KeyDelete),
            Windows::KeyEnd => Ok(Self::KeyEnd),
            Windows::KeyPageDown => Ok(Self::KeyPagedown),
            Windows::KeyRight => Ok(Self::KeyRight),
            Windows::KeyLeft => Ok(Self::KeyLeft),
            Windows::KeyDown => Ok(Self::KeyDown),
            Windows::KeyUp => Ok(Self::KeyUp),
            Windows::KeypadNumLock => Ok(Self::KeyNumlock),
            Windows::KeypadSlash => Ok(Self::KeyKpslash),
            Windows::KeypadStar => Ok(Self::KeyKpAsterisk),
            Windows::KeypadDash => Ok(Self::KeyKpMinus),
            Windows::KeypadPlus => Ok(Self::KeyKpplus),
            Windows::KeypadEnter => Ok(Self::KeyKpEnter),
            Windows::Keypad1End => Ok(Self::KeyKp1),
            Windows::Keypad2DownArrow => Ok(Self::KeyKp2),
            Windows::Keypad3PageDn => Ok(Self::KeyKp3),
            Windows::Keypad4LeftArrow => Ok(Self::KeyKp4),
            Windows::Keypad5 => Ok(Self::KeyKp5),
            Windows::Keypad6RightArrow => Ok(Self::KeyKp6),
            Windows::Keypad7Home => Ok(Self::KeyKp7),
            Windows::Keypad8UpArrow => Ok(Self::KeyKp8),
            Windows::Keypad9PageUp => Ok(Self::KeyKp9),
            Windows::Keypad0Insert => Ok(Self::KeyKp0),
            Windows::KeypadDot => Ok(Self::KeyKpDot),
            Windows::KeyNonUSSlashBar => Ok(Self::Key102nd),
            Windows::KeyApplication => Ok(Self::KeyMenu),
            Windows::KeypadEquals => Ok(Self::KeyKpequal),
            Windows::KeyF13 => Ok(Self::KeyF13),
            Windows::KeyF14 => Ok(Self::KeyF14),
            Windows::KeyF15 => Ok(Self::KeyF15),
            Windows::KeyF16 => Ok(Self::KeyF16),
            Windows::KeyF17 => Ok(Self::KeyF17),
            Windows::KeyF18 => Ok(Self::KeyF18),
            Windows::KeyF19 => Ok(Self::KeyF19),
            Windows::KeyF20 => Ok(Self::KeyF20),
            Windows::KeyF21 => Ok(Self::KeyF21),
            Windows::KeyF22 => Ok(Self::KeyF22),
            Windows::KeyF23 => Ok(Self::KeyF23),
            Windows::KeyF24 => Ok(Self::KeyF24),
            Windows::KeypadComma => Ok(Self::KeyKpcomma),
            Windows::KeyInternational1 => Ok(Self::KeyRo),
            Windows::KeyInternational2 => Ok(Self::KeyKatakanahiragana),
            Windows::KeyInternational3 => Ok(Self::KeyYen),
            Windows::KeyInternational4 => Ok(Self::KeyHenkan),
            Windows::KeyInternational5 => Ok(Self::KeyMuhenkan),
            Windows::KeyLANG1 => Ok(Self::KeyHanguel),
            Windows::KeyLANG2 => Ok(Self::KeyHanja),
            Windows::KeyLANG3 => Ok(Self::KeyKatakana),
            Windows::KeyLANG4 => Ok(Self::KeyHiragana),
            Windows::KeyLeftCtrl => Ok(Self::KeyLeftCtrl),
            Windows::KeyLeftShift => Ok(Self::KeyLeftShift),
            Windows::KeyLeftAlt => Ok(Self::KeyLeftAlt),
            Windows::KeyLeftGUI => Ok(Self::KeyLeftMeta),
            Windows::KeyRightCtrl => Ok(Self::KeyRightCtrl),
            Windows::KeyRightShift => Ok(Self::KeyRightShift),
            Windows::KeyFakeRightShift => Ok(Self::KeyRightShift),
            Windows::KeyRightAlt => Ok(Self::KeyRightalt),
            Windows::KeyRightGUI => Ok(Self::KeyRightmeta),
            Windows::KeyScanNextTrack => Ok(Self::KeyNextsong),
            Windows::KeyScanPreviousTrack => Ok(Self::KeyPrevioussong),
            Windows::KeyStop => Ok(Self::KeyStopcd),
            Windows::KeyPlayPause => Ok(Self::KeyPlaypause),
            Windows::KeyMute => Ok(Self::KeyMute),
            Windows::KeyVolumeUp => Ok(Self::KeyVolumeUp),
            Windows::KeyVolumeDown => Ok(Self::KeyVolumeDown),
            Windows::ALConsumerControlConfiguration => Err(()),
            Windows::ALEmailReader => Ok(Self::KeyMail),
            Windows::ALCalculator => Ok(Self::KeyCalc),
            Windows::ALLocalMachineBrowser => Ok(Self::KeyFile),
            Windows::ACSearch => Ok(Self::KeyWww),
            Windows::ACHome => Ok(Self::KeyHomepage),
            Windows::ACBack => Ok(Self::KeyBack),
            Windows::ACForward => Ok(Self::KeyForward),
            Windows::ACStop => Ok(Self::KeyStop),
            Windows::ACRefresh => Ok(Self::KeyRefresh),
            Windows::ACBookmarks => Ok(Self::KeyBookmarks),
        }
    }
}