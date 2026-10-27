use std::sync::LazyLock;

use indexmap::IndexMap;

use crate::bigint_hosts::{lookup_bigint_host, BIGINT_HOST_TABLE};
use crate::collection_hosts::{lookup_collection_host, COLLECTION_HOST_TABLE};
use crate::numeric_hosts::{lookup_numeric_host, NUMERIC_HOST_TABLE};
use crate::result_hosts::{lookup_result_host, RESULT_HOST_TABLE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostFnId {
    FileRead,
    FileReadLines,
    FileWrite,
    FileAppend,
    FileExists,
    FileDelete,
    FileCopy,
    FileMove,
    DirCreate,
    DirList,
    DirExists,
    HttpGet,
    HttpPost,
    HttpResponseJson,
    RegexMatch,
    ReadLine,
    HasherNew,
    HasherCombine,
    HasherFinalize,
    ObjectIdentifier,
    JsonEncode,
    JsonDecode,
    TaskDetach,
    TaskJoin,
    TaskCancel,
    TaskSleep,
    MainActorRun,
    AwaitTask,
    TaskGroupCreate,
    TaskGroupAdd,
    TaskGroupWait,
    HttpRequest,
    HttpResponseStatus,
    HttpResponseText,
    HttpResponseOk,
    HttpResponseHeader,
    HttpResponseHeaders,
    HttpServe,
    HttpMakeResponse,
    UrlParse,
    UrlEncodeQuery,
    UrlBuild,
    EnvGet,
    EnvSet,
    PathJoin,
    PathNormalize,
    PathExists,
    PathExtension,
    TimeNowMs,
    TimeSleepMs,
    InstantNowMs,
    ChannelCreate,
    ChannelSend,
    ChannelTrySend,
    ChannelTryRecv,
    ChannelSelect,
    ChannelRecv,
    ChannelRecvAsync,
    AsyncIterFrom,
    AsyncIterNext,
    DateTimeParse,
    DateTimeFormat,
    DateTimeParseIso8601,
    DateTimeFormatWithTimezone,
    DateTimeTimezoneOffset,
    DateTimeNow,
    OrderedSetNew,
    OrderedSetInsert,
    OrderedSetContains,
    ChannelSelectTimeout,
    ChannelSelectDefault,
    ChannelSelectMixedAwait,
    HttpRequestAsync,
    WsConnect,
    WsConnectAsync,
    WsSendAsync,
    WsSend,
    WsSendProd,
    WasmSabNew,
    WasmAtomicAddI32,
    TcpConnect,
    TcpRead,
    TcpWrite,
    TcpStream,
    TcpStreamTls,
    TcpStreamHandle,
    NetBackpressure,
    NetRingBuffer,
    TcpHandleRelease,
    NetFlowControl,
    TcpHandleRefcount,
    NetSocketWindow,
    TcpSockbuf,
    NetSetsockopt,
    VmNativeGcMark,
    VmNativeGcCount,
    VmNativeGcCollect,
    RandomNextInt64,
    RandomNextUInt64,
    RandomFillBytes,
    CryptoSha256,
    CryptoSha512,
    CryptoHmacSha256,
    CryptoAesGcmEncrypt,
    SimdFromArray,
    SimdReduceAdd,
    SimdLoadFromArray,
    SimdStoreToArray,
    StringCharAt,
    ArraySetAt,
    LeadingZeros128,
    SimdShuffle,
    FileReadBytes,
    FileWriteBytes,
    FileOpen,
    FileReadChunk,
    FileWriteChunk,
    FileClose,
    /// ADR-234 / M2-S3-02b: never-throw close for withOpen/deinit.
    FileCloseQuiet,
    /// ADR-234 / M2-S3-02b: open→closure(Int)→always close.
    FileWithOpen,
    /// ADR-234 / M2-S3-02b: open→close→Err host inject for withOpen throw-path nail.
    FileWithOpenFail,
    FileSeek,
    FileMmapRead,
    KeyPathGet,
    AsOptional,
    TypeIs,
    TypeCast,
    StrInstSplit,
    StrInstTrim,
    StrInstHasPrefix,
    StrInstLowercased,
    StrInstUppercased,
    StrInstMatches,
    StrInstLength,
    StrInstSubstring,
    /// ADR-226 / M2-S2-01: Unicode scalar view count.
    StrUnicodeScalarCount,
    /// ADR-226 / M2-S2-01: Unicode scalar view as Array<Character>.
    StrUnicodeScalars,
    /// ADR-226 / M2-S2-01: UTF-8 byte count.
    StrUtf8Count,
    /// ADR-226 / M2-S2-01: UTF-8 bytes as Array<Int> (0–255).
    StrUtf8Bytes,
    /// ADR-226 / M2-S2-01: Character.isSingleScalar (true scalar count == 1).
    CharacterIsSingleScalar,
    /// ADR-228 / M2-S2-03: Unicode NFC normalization.
    StrNormalizedNFC,
    /// ADR-228 / M2-S2-03: Unicode NFD normalization.
    StrNormalizedNFD,
    /// ADR-228 / M2-S2-03: Unicode default/full case folding.
    StrCaseFolded,
    FmtPrint,
    FmtPrintLine,
    FmtFormat,
    FmtDebug,
    Base64Encode,
    Base64Decode,
    HexEncode,
    HexDecode,
    UrlEncode,
    UrlDecode,
    MathFloatSin,
    MathFloatCos,
    MathFloatTan,
    MathFloatSqrt,
    MathFloatPow,
    MathFloatFloor,
    MathFloatCeil,
    MathFloatRound,
    MathFloatLog,
    MathFloatAsin,
    MathFloatAcos,
    MathFloatAtan,
    MathFloatAtan2,
    MathFloatExp,
    MathFloatExp2,
    MathFloatLog2,
    MathFloatLog10,
    MathFloatHypot,
    MathFloatCbrt,
    MathFloatTrunc,
    MathFloatAbs,
    MathFloatSinh,
    MathFloatCosh,
    MathFloatTanh,
    MathFloatFmod,
    MathFloatCopysign,
    MathFloatIsNan,
    MathFloatIsInfinite,
    MathFloatIsFinite,
    MathFloatExpm1,
    MathFloatLog1p,
    MathFloatAsinh,
    MathFloatAcosh,
    MathFloatAtanh,
    MathFloatRemainder,
    MathFloatNextUp,
    MathFloatNextDown,
    MathFloatMulAdd,
    MathFloatErf,
    MathFloatErfc,
    MathFloatTgamma,
    MathFloatLgamma,
    MathFloat32Sqrt,
    MathFloat32Sin,
    MathFloat32Cos,
    MathFloat32Tan,
    MathFloat32Exp,
    MathFloat32Log,
    MathFloat32Abs,
    MathFloat32Floor,
    MathFloat32Ceil,
    MathFloat32Round,
    MathFloat32Trunc,
    MathFloat32Pow,
    MathFloat32Asin,
    MathFloat32Acos,
    MathFloat32Atan,
    MathFloat32Hypot,
    MathFloat32Atan2,
    MathFloat32Log2,
    MathFloat32Log10,
    MathFloat32Exp2,
    MathFloat32Cbrt,
    MathFloat32Sinh,
    MathFloat32Cosh,
    MathFloat32Tanh,
    MathFloat32Fmod,
    MathFloat32Copysign,
    MathFloat32Expm1,
    MathFloat32Log1p,
    MathFloat32Asinh,
    MathFloat32Acosh,
    MathFloat32Atanh,
    MathFloat32Erf,
    MathFloat32Erfc,
    MathFloat32Tgamma,
    MathFloat32Lgamma,
    MathFloat32Remainder,
    MathFloat32IsNan,
    MathFloat32IsInfinite,
    MathFloat32IsFinite,
    MathFloat32NextUp,
    MathFloat32NextDown,
    MathFloat32MulAdd,
    ProcessArgs,
    ProcessEnv,
    ProcessExec,
    ProcessExitCode,
    FileStat,
    FileSymlink,
    FileReadLink,
    FileChmod,
    UdpBind,
    UdpSendTo,
    UdpRecvFrom,
    DnsResolve,
    DnsResolveIp,
    CryptoEd25519Sign,
    CryptoEd25519Verify,
    CryptoPbkdf2,
    MutexCreate,
    MutexLock,
    MutexUnlock,
    WsRecv,
    WsClose,
    ArraySortBy,
    ArrayBinarySearch,
    ArrayChunked,
    ArrayZip,
    ArrayFlatMap,
    LogDebug,
    LogInfo,
    LogWarn,
    LogError,
    TestingAssert,
    TestingAssertEq,
    ProcessExecOutput,
    ProcessSpawn,
    ProcessWait,
    FileWatch,
    StringBuilderNew,
    StringBuilderAppend,
    StringBuilderToString,
    BytesFromArray,
    BytesSlice,
    BytesConcat,
    ChannelClose,
    TestingTest,
    TestingExpectEqual,
    RegexCompile,
    RegexFindAll,
    RegexReplaceAll,
    EncodingGzipCompress,
    EncodingGzipDecompress,
    ReflectType,
    ReflectSpecialization,
    ValueDescription,
    ValueDebugDescription,
    ValueCompare,
    EncodingZstdCompress,
    EncodingZstdDecompress,
    CodableEncode,
    CodableDecode,
    TestingRunAll,
    RuntimeCast,
    CodableEncodeL4,
    CodableDecodeL4,
    JsonEncoderEncode,
    JsonDecoderDecode,
    JsonStreamOpenWriter,
    JsonStreamWrite,
    JsonStreamFinishWriter,
    JsonStreamOpenReader,
    JsonStreamFeed,
    JsonStreamDecodeNext,
    JsonStreamCloseReader,
    GrpcUnary,
    GrpcStreamClient,
    GrpcStreamServer,
    KvoObserve,
    SqlExec,
    SqlQuery,
    XmlParse,
    SqlPrepare,
    SqlOpen,
    SqlClose,
    SqlExecOn,
    SqlQueryOn,
    SqlPrepareOn,
    SqlBindStepOn,
    SqlBindPreparedOn,
    SqlBeginTransactionOn,
    SqlCommitTransactionOn,
    SqlRollbackTransactionOn,
    OrmSave,
    OrmSaveOn,
    OrmFindWhereOn,
    OrmUpdateWhereOn,
    OrmDeleteWhereOn,
    OrmFind,
    CodableSchemaRegister,
    XmlXPath,
    GuiWindowCreate,
    GuiEventPump,
    GuiCanvasDraw,
    SqlMigrate,
    SqlBindStep,
    OrmFindWhere,
    OrmUpdateWhere,
    OrmDeleteWhere,
    GuiWinitWindowCreate,
    GuiWinitEventPump,
    GuiWinitCanvasDraw,
    GuiButtonCreate,
    GuiLabelCreate,
    GuiOnClick,
    GuiLayoutCreate,
    GuiLayoutConfigure,
    GuiTextFieldCreate,
    GuiRunLoopOnce,
    GuiPixelRead,
    GuiClickInject,
    GuiKeyInject,
    GuiWidgetGetText,
    GuiWidgetSetText,
    SqlBeginTransaction,
    SqlCommitTransaction,
    CodableL14RegisterMigration,
    CodableEncodeL14,
    CodableDecodeL14,
    CodableL14RegisterRename,
    SqlRollback,
    SqlBindMulti,
    GuiTouchEvent,
    GuiMobileWindowCreate,
    GuiMobileEventPump,
    GuiMobileCanvasDraw,
    GuiMobileRunLoopOnce,
    GuiMobilePixelRead,
    GcChurnHeap,
    ResultBuilderBuildBlock,
    ResultBuilderBuildEither,
    ResultBuilderBuildArray,
    ResultBuilderBuildOptional,
    WeakFrom,
    WeakLoad,
    WeakAlive,
    SimdAnd,
    SimdOr,
    SimdXor,
    /// Phase 109: bitwise not / lane shift.
    SimdNot,
    SimdShl,
    /// Phase 110: lane-wise arithmetic right shift.
    SimdShr,
    /// Phase 111: lane-wise signed min/max (`i32x4`).
    SimdMin,
    SimdMax,
    /// Phase 111: lane-wise `f32x4` add guest pilot.
    SimdF32Add,
    /// Phase 112: lane-wise `f32x4` sub guest.
    SimdF32Sub,
    /// Phase 112: lane-wise `f32x4` mul guest.
    SimdF32Mul,
    /// Phase 113: lane-wise `f32x4` div guest.
    SimdF32Div,
    /// Phase 113: lane-wise `f64x2` add guest pilot.
    SimdF64Add,
    /// Phase 114: lane-wise `f64x2` sub guest pilot.
    SimdF64Sub,
    /// Phase 115: lane-wise `f64x2` mul guest pilot.
    SimdF64Mul,
    /// Phase 115: lane-wise `f64x2` div guest pilot.
    SimdF64Div,
    /// Phase 117: lane-wise `i16x8` add guest pilot.
    SimdI16Add,
    /// Phase 118: lane-wise `i16x8` sub/mul guest pilot.
    SimdI16Sub,
    SimdI16Mul,
    /// Phase 118: lane-wise `i8x16` add/sub guest pilot.
    SimdI8Add,
    SimdI8Sub,
    /// Phase 119: lane-wise `i8x16` mul guest pilot (SIMD16 matrix expand).
    SimdI8Mul,
    /// Phase 120: lane-wise `i8x16` and guest pilot (SIMD16 logic/shift tail).
    SimdI8And,
    /// Phase 121: lane-wise `i8x16` shl guest pilot (SIMD16 shift lane MVP).
    SimdI8Shl,
    /// Phase 121: lane-wise `i8x16` shr guest pilot (SIMD16 shift lane MVP).
    SimdI8Shr,
    /// Phase 122: lane-wise `i8x16` or guest pilot (SIMD16 logic-or-xor MVP).
    SimdI8Or,
    /// Phase 122: lane-wise `i8x16` xor guest pilot (SIMD16 logic-or-xor MVP).
    SimdI8Xor,
    /// Phase 123: lane-wise `i8x16` not guest pilot (SIMD16 logic-not MVP).
    SimdI8Not,
    /// Phase 124: lane-wise `i8x16` min guest pilot (SIMD16 lane-min-max MVP).
    SimdI8Min,
    /// Phase 124: lane-wise `i8x16` max guest pilot (SIMD16 lane-min-max MVP).
    SimdI8Max,
    /// Phase 130: lane-wise `i8x16` unsigned min guest pilot (SIMD16 unsigned-min-max MVP).
    SimdI8UMin,
    /// Phase 130: lane-wise `i8x16` unsigned max guest pilot (SIMD16 unsigned-min-max MVP).
    SimdI8UMax,
    /// Phase 131: lane-wise `i8x16` abs guest pilot (SIMD16 abs MVP).
    SimdI8Abs,
    /// Phase 132: lane-wise `i8x16` neg guest pilot (SIMD16 neg MVP).
    SimdI8Neg,
    /// Phase 125: `i8x16` extract lane guest pilot (SIMD16 lane-extract-replace MVP).
    SimdI8Extract,
    /// Phase 125: `i8x16` replace lane guest pilot (SIMD16 lane-extract-replace MVP).
    SimdI8Replace,
    /// Phase 126: lane-wise `i8x16` eq guest pilot (SIMD16 compare-lane MVP).
    SimdI8Eq,
    /// Phase 133: lane-wise `i8x16` ne guest pilot (SIMD16 ne MVP).
    SimdI8Ne,
    /// Phase 134: lane-wise `i8x16` le guest pilot (SIMD16 le MVP).
    SimdI8Le,
    /// Phase 135: lane-wise `i8x16` ge guest pilot (SIMD16 ge MVP).
    SimdI8Ge,
    /// Phase 136: lane-wise `i8x16` avgr_u guest pilot (SIMD16 avgr_u MVP).
    SimdI8AvgrU,
    /// Phase 137: lane-wise `i8x16` popcnt guest pilot (SIMD16 popcnt MVP).
    SimdI8Popcnt,
    /// Phase 138: MSB bitmask (`i8x16.bitmask` guest pilot).
    SimdI8Bitmask,
    /// Phase 139: all lanes non-zero (`i8x16.all_true` guest pilot).
    SimdI8AllTrue,
    /// Phase 139: any lane non-zero (`v128.any_true` guest pilot).
    SimdI8AnyTrue,
    /// Phase 126: lane-wise `i8x16` lt guest pilot (SIMD16 compare-lane MVP).
    SimdI8Lt,
    /// Phase 126: lane-wise `i8x16` gt guest pilot (SIMD16 compare-lane MVP).
    SimdI8Gt,
    /// Phase 127: lane-wise `i8x16` signed saturating add guest pilot.
    SimdI8AddSat,
    /// Phase 127: lane-wise `i8x16` signed saturating sub guest pilot.
    SimdI8SubSat,
    /// Phase 128: fixed-mode `i8x16` shuffle MVP (mode 0=reverse, 1=swap pairs).
    SimdI8Shuffle,
    /// Phase 117: lane-wise `i64x2` add guest pilot.
    SimdI64Add,
    /// Phase 108: build Object-tagged select arm (`__syx_select_tag(dir, ch, payload)`).
    SelectTag,
    /// Phase 120: Service DI singleton container hosts.
    ServiceRegister,
    /// Phase 126: register protocol conformance witness (protocol → impl + qualifier).
    ServiceRegisterWitness,
    ServiceAddDep,
    ServiceSeal,
    ServiceResolve,
    ServicePut,
    ServiceReset,
    /// Phase 122: explicit container teardown with `@PreDestroy` hooks.
    ServiceDestroy,
    /// Phase 120: apply compile-time queued `@Service` registrations then seal.
    ServiceBoot,
    /// Phase 123: begin/end request frame for `@Scope("request")` services.
    ServiceBeginRequest,
    ServiceEndRequest,
    /// Phase 123: user-facing dynamic resolve (ceremony sugar wrapper).
    ServiceResolveDynamic,
    /// Phase 124: begin+invoke handler+end request frame (`Di.withRequest`).
    ServiceRunInRequest,
    /// Phase 124: set active `@Profile` name.
    ServiceSetProfile,
    /// Phase 128: create/reuse a `@Lazy` cycle-break proxy handle.
    ServiceLazyProxy,
    /// Phase 124: SynUI host bridge (path dep + in-memory View tree).
    SynuiAppCreate,
    SynuiLabel,
    SynuiButton,
    SynuiColumn,
    SynuiPump,
    SynuiFingerprint,
    /// Phase 125: reactive Signal hosts.
    SynuiSignalNew,
    SynuiSignalGet,
    SynuiSignalSet,
    /// Phase 125: row / textField widget expand.
    SynuiRow,
    SynuiTextField,
    /// Phase 125: named-handler onClick (Di.withRequest pattern).
    SynuiButtonOnClick,
    SynuiButtonClick,
    /// Phase 125: structural render fingerprint (CI-safe, no HeadlessRenderer).
    SynuiRenderFingerprint,
    /// Phase 125: optional desktop run gate (`SYX_SYNUI_RUN=1`).
    SynuiRun,
    /// Phase 126: checkbox widget host.
    SynuiCheckbox,
    /// Phase 126: slider widget host.
    SynuiSlider,
    /// Phase 127: bool signal hosts for widget bind.
    SynuiSignalBoolNew,
    SynuiSignalBoolGet,
    SynuiSignalBoolSet,
    /// Phase 127: numeric signal hosts for widget bind.
    SynuiSignalNumberNew,
    SynuiSignalNumberGet,
    SynuiSignalNumberSet,
    /// Phase 127: Signal→widget bind hosts.
    SynuiBindTextField,
    SynuiBindCheckbox,
    SynuiBindSlider,
    /// Phase 127: switch + radio widgets.
    SynuiSwitch,
    SynuiRadio,
    /// Phase 128: toggle/radio Bool Signal bind.
    SynuiBindSwitch,
    SynuiBindRadio,
    /// Phase 128: progress + picker widgets.
    SynuiProgress,
    SynuiPicker,
    /// Phase 129: textArea + list widgets.
    SynuiTextArea,
    SynuiList,
    /// Phase 129: progress Number Signal bind.
    SynuiBindProgress,
    /// Phase 130: image + scrollView widgets.
    SynuiImage,
    SynuiScrollView,
    /// Phase 130: picker Number/Int Signal bind (selected index).
    SynuiBindPicker,
    /// Phase 131: textArea String Signal bind + list Number selected-index bind.
    SynuiBindTextArea,
    SynuiBindList,
    /// Phase 131: spacer widget (size/weight MVP).
    SynuiSpacer,
    /// Phase 132: image String Signal bind.
    SynuiBindImage,
    /// Phase 132: divider widget (orientation/thickness MVP).
    SynuiDivider,
    /// Phase 133 Track B: stack widget.
    SynuiStack,
    /// Phase 133 Track B: scrollView Signal bind.
    SynuiBindScrollView,
    /// Phase 134 Track B: tabView widget.
    SynuiTabView,
    /// Phase 134 Track B: tabView Signal bind.
    SynuiBindTabView,
    /// Phase 135 Track B: navigation widget.
    SynuiNavigation,
    /// Phase 135 Track B: navigation Signal bind.
    SynuiBindNavigation,
    /// Phase 133 Track E: Os runtime queries.
    OsCpuCount,
    OsMemoryTotal,
    OsMemoryAvailable,
    OsUserName,
    OsHomeDir,
    /// Phase 136 Track D: Os hostname / uptime.
    OsHostname,
    OsUptimeMs,
    /// Phase 137 Track D: Os disk / load.
    OsDiskTotal,
    OsDiskAvailable,
    OsLoadAverage,
    /// Phase 138 Track D: Os primary IPv4.
    OsPrimaryIpv4,
    /// Phase 133 Track E: Task worker pool config.
    TaskSetWorkerCount,
    TaskWorkerCount,
    /// Phase 134 Track E: bounded Task queue capacity + Abort reject.
    TaskSetQueueCapacity,
    TaskQueueCapacity,
    /// Phase 135 Track E: Task reject policy (Abort / CallerRuns).
    TaskSetRejectPolicy,
    TaskRejectPolicy,
    /// Phase 136 Track E: Task pool core / max / keepAlive.
    TaskSetCorePoolSize,
    TaskCorePoolSize,
    TaskSetMaximumPoolSize,
    TaskMaximumPoolSize,
    TaskSetKeepAliveMs,
    TaskKeepAliveMs,
    TaskLiveWorkerCount,
    /// Phase 133 Track E: Actor max + mailbox capacity.
    ActorSetMaxActors,
    ActorMaxActors,
    ActorSetMailboxCapacity,
    ActorMailboxCapacity,
    /// Phase 138 Track E: Actor shared pool opt-in.
    ActorSetSharedPoolEnabled,
    ActorSharedPoolEnabled,
    ActorSetSharedPoolSize,
    ActorSharedPoolSize,
    /// Phase 133 Track E: Channel length/capacity observation.
    ChannelLength,
    ChannelCapacity,
    /// ADR-238 / M2-S4-03b: Http.serve graceful shutdown (append-only — do not reorder).
    HttpShutdown,
    /// ADR-239 / M2-S4-04: client response body stream (append-only — do not reorder).
    HttpBodyStreamOpen,
    HttpBodyStreamOpenOpts,
    HttpBodyStreamReadChunk,
    HttpBodyStreamClose,
    /// ADR-240 / M2-S3-03: FileStream + FileWatcher (append-only — do not reorder).
    FileStreamOpen,
    FileStreamReadChunk,
    FileStreamClose,
    FileWatcherOpen,
    FileWatcherPoll,
    FileWatcherClose,
    /// ADR-242 / M2-S6-01: ProcessBuilder + Child (append-only — do not reorder).
    ProcessBuilderCreate,
    ProcessBuilderCwd,
    ProcessBuilderEnv,
    ProcessBuilderSpawn,
    ChildWait,
    ChildKill,
    ChildWaitTimeout,
    /// ADR-243 / M2-S6-02b: ChildPipe stdio (append-only — do not reorder).
    ChildStdout,
    ChildStderr,
    ChildStdin,
    ChildPipeReadChunk,
    ChildPipeWrite,
    ChildPipeClose,
    /// ADR-244 / M2-S6-03: Child.signal (append-only — do not reorder).
    ChildSignal,
    /// ADR-245 / M2-S8-01c: Sql connection pool (append-only — do not reorder).
    SqlPoolCreate,
    SqlPoolAcquire,
    SqlPoolRelease,
    SqlPoolClose,
    /// ADR-246 / M2-S8-02b: SqlRow accessors (append-only — do not reorder).
    SqlRowGetString,
    SqlRowGetInt,
    SqlRowGetFloat,
    SqlRowGetBlob,
    SqlRowIsNull,
    SqlRowColumnNames,
    /// ADR-248 / M2-S1-03: Deque Int MVP (append-only — do not reorder).
    DequeNew,
    DequePushBack,
    DequePushFront,
    DequePopBack,
    DequePopFront,
    DequeCount,
    DequeIsEmpty,
    /// ADR-255 / M2 W111: TcpListener bind/close (append-only — do not reorder; ≠ accept / ≠ TLS).
    TcpListen,
    TcpListenerClose,
    /// ADR-260 / M3 W1: TaskGroup.cancel_all product surface (append-only — do not reorder).
    TaskGroupCancel,
    /// ADR-261 / M3 W2: TaskRegistry unreaped live_count (append-only — do not reorder).
    TaskLiveCount,
    /// ADR-262 / M3 W3: task-scoped deadline ms from now (append-only — do not reorder).
    TaskSetDeadlineMs,
    /// ADR-263 / M3 W4: task-local key/value (append-only — do not reorder).
    TaskLocalSet,
    TaskLocalGet,
    /// ADR-264 / M3 W5: actor reentrancy policy fingerprint (append-only — do not reorder).
    ActorReentrancyPolicy,
    /// ADR-275 / M3 W16: blocking pool sleep + isolation policy fingerprint (append-only — do not reorder).
    BlockingSleep,
    IsolationPolicy,
    /// ADR-276 / M3 W17: tunable blocking pool size (append-only — do not reorder).
    BlockingPoolSetSize,
    BlockingPoolSize,
    /// ADR-278 / M3 W19: File async adapter + I/O adapter policy (append-only — do not reorder).
    FileReadAsync,
    IoAdapterPolicy,
    /// ADR-279 / M3 W20: Tcp read async adapter on blocking_pool (append-only — do not reorder).
    TcpReadAsync,
    /// ADR-281 / M3 W22: Mutex RAII quiet unlock + sync policy fingerprint (append-only — do not reorder).
    MutexUnlockQuiet,
    SyncPolicy,
    /// ADR-282 / M3 W23: blocking mutex lock wait (append-only — do not reorder).
    MutexLockWait,
    /// ADR-284 / M3 W25: Semaphore MVP + policy fingerprint (append-only — do not reorder).
    SemaphoreCreate,
    SemaphoreWait,
    SemaphoreTryWait,
    SemaphoreSignal,
    SemaphorePolicy,
    /// ADR-285 / M3 W26: never-throw quiet signal for SemaphoreGuard.deinit (append-only).
    SemaphoreSignalQuiet,
    /// ADR-292 / M3 W33: structured logging thin — min-level filter + infoFields (append-only).
    LogSetMinLevel,
    LogMinLevel,
    LogPolicy,
    LogInfoFields,
    LogTakeLast,
    /// ADR-293 / M3 W34: all-level *Fields deepen (append-only).
    LogDebugFields,
    LogWarnFields,
    LogErrorFields,
    /// ADR-295 / M3 W36: Trace/Span thin start (append-only — do not reorder).
    TraceStart,
    TraceEnd,
    TraceCurrent,
    TraceContextLine,
    TracePolicy,
    /// ADR-298 / M3 W39: Metrics Counter thin start (append-only — do not reorder).
    MetricsCounterInc,
    MetricsCounterGet,
    MetricsCounterReset,
    MetricsSnapshot,
    MetricsPolicy,
    /// ADR-299 / M3 W40: Gauge + labeled counter deepen (append-only — do not reorder).
    MetricsGaugeSet,
    MetricsGaugeGet,
    MetricsCounterIncLabels,
    MetricsCounterGetLabels,
    /// ADR-301 / M3 W42: Profiler section wall-clock thin start (append-only — do not reorder).
    ProfilerStart,
    ProfilerStop,
    ProfilerElapsed,
    ProfilerReset,
    ProfilerSnapshot,
    ProfilerPolicy,
    /// ADR-302 / M3 W43: Process CPU section deepen (append-only — do not reorder).
    ProfilerCpuStart,
    ProfilerCpuStop,
    ProfilerCpuElapsed,
    /// ADR-727 / Wave A W383: TcpListener.accept llvm/native first-slice (append-only — do not reorder; ≠ accept GA / ≠ TLS / ≠ wasm).
    TcpListenerAccept,
    /// ADR-737 / Wave A W393: TLS server HostFn llvm first-slice (append-only — do not reorder; ≠ TLS GA / ≠ wasm / ≠ status=implemented).
    TlsServe,
    /// Array.append — grow + HeapSlot write-back (sgate / collection mutation).
    ArrayAppend,
}

impl HostFnId {
    pub fn name(self) -> &'static str {
        match self {
            Self::FileRead => "__syx_file_read",
            Self::FileReadLines => "__syx_file_read_lines",
            Self::FileWrite => "__syx_file_write",
            Self::FileAppend => "__syx_file_append",
            Self::FileExists => "__syx_file_exists",
            Self::FileDelete => "__syx_file_delete",
            Self::FileCopy => "__syx_file_copy",
            Self::FileMove => "__syx_file_move",
            Self::DirCreate => "__syx_dir_create",
            Self::DirList => "__syx_dir_list",
            Self::DirExists => "__syx_dir_exists",
            Self::HttpGet => "__syx_http_get",
            Self::HttpPost => "__syx_http_post",
            Self::HttpResponseJson => "__syx_http_response_json",
            Self::RegexMatch => "__syx_regex_match",
            Self::ReadLine => "__syx_read_line",
            Self::HasherNew => "__syx_hasher_new",
            Self::HasherCombine => "__syx_hasher_combine",
            Self::HasherFinalize => "__syx_hasher_finalize",
            Self::ObjectIdentifier => "__syx_object_identifier",
            Self::JsonEncode => "__syx_json_encode",
            Self::JsonDecode => "__syx_json_decode",
            Self::TaskDetach => "__syx_task_detach",
            Self::TaskJoin => "__syx_task_join",
            Self::TaskCancel => "__syx_task_cancel",
            Self::TaskSleep => "__syx_task_sleep",
            Self::MainActorRun => "__syx_main_actor_run",
            Self::AwaitTask => "__syx_await_task",
            Self::TaskGroupCreate => "__syx_task_group_create",
            Self::TaskGroupAdd => "__syx_task_group_add",
            Self::TaskGroupWait => "__syx_task_group_wait",
            Self::HttpRequest => "__syx_http_request",
            Self::HttpResponseStatus => "__syx_http_response_status",
            Self::HttpResponseText => "__syx_http_response_text",
            Self::HttpResponseOk => "__syx_http_response_ok",
            Self::HttpResponseHeader => "__syx_http_response_header",
            Self::HttpResponseHeaders => "__syx_http_response_headers",
            Self::HttpServe => "__syx_http_serve",
            Self::HttpMakeResponse => "__syx_http_make_response",
            Self::UrlParse => "__syx_url_parse",
            Self::UrlEncodeQuery => "__syx_url_encode_query",
            Self::UrlBuild => "__syx_url_build",
            Self::EnvGet => "__syx_env_get",
            Self::EnvSet => "__syx_env_set",
            Self::PathJoin => "__syx_path_join",
            Self::PathNormalize => "__syx_path_normalize",
            Self::PathExists => "__syx_path_exists",
            Self::PathExtension => "__syx_path_extension",
            Self::TimeNowMs => "__syx_time_now_ms",
            Self::TimeSleepMs => "__syx_time_sleep_ms",
            Self::InstantNowMs => "__syx_instant_now_ms",
            Self::ChannelCreate => "__syx_channel_create",
            Self::ChannelSend => "__syx_channel_send",
            Self::ChannelTrySend => "__syx_channel_try_send",
            Self::ChannelTryRecv => "__syx_channel_try_recv",
            Self::ChannelSelect => "__syx_channel_select",
            Self::ChannelRecv => "__syx_channel_recv",
            Self::ChannelRecvAsync => "__syx_channel_recv_async",
            Self::AsyncIterFrom => "__syx_async_iter_from",
            Self::AsyncIterNext => "__syx_async_iter_next",
            Self::DateTimeParse => "__syx_datetime_parse",
            Self::DateTimeFormat => "__syx_datetime_format",
            Self::DateTimeParseIso8601 => "__syx_datetime_parse_iso8601",
            Self::DateTimeFormatWithTimezone => "__syx_datetime_format_with_timezone",
            Self::DateTimeTimezoneOffset => "__syx_datetime_timezone_offset",
            Self::DateTimeNow => "__syx_datetime_now",
            Self::OrderedSetNew => "__syx_ordered_set_new",
            Self::OrderedSetInsert => "__syx_ordered_set_insert",
            Self::OrderedSetContains => "__syx_ordered_set_contains",
            Self::ChannelSelectTimeout => "__syx_channel_select_timeout",
            Self::ChannelSelectDefault => "__syx_channel_select_default",
            Self::ChannelSelectMixedAwait => "__syx_channel_select_mixed_await",
            Self::HttpRequestAsync => "__syx_http_request_async",
            Self::WsConnect => "__syx_ws_connect",
            Self::WsConnectAsync => "__syx_ws_connect_async",
            Self::WsSendAsync => "__syx_ws_send_async",
            Self::WsSend => "__syx_ws_send",
            Self::WsSendProd => "__syx_ws_send_prod",
            Self::WasmSabNew => "__syx_wasm_sab_new",
            Self::WasmAtomicAddI32 => "__syx_wasm_atomic_add_i32",
            Self::TcpConnect => "__syx_tcp_connect",
            Self::TcpRead => "__syx_tcp_read",
            Self::TcpWrite => "__syx_tcp_write",
            Self::TcpStream => "__syx_tcp_stream",
            Self::TcpStreamTls => "__syx_tcp_stream_tls",
            Self::TcpStreamHandle => "__syx_tcp_stream_handle",
            Self::NetBackpressure => "__syx_net_backpressure",
            Self::NetRingBuffer => "__syx_net_ring_buffer",
            Self::TcpHandleRelease => "__syx_tcp_handle_release",
            Self::NetFlowControl => "__syx_net_flow_control",
            Self::TcpHandleRefcount => "__syx_tcp_handle_refcount",
            Self::NetSocketWindow => "__syx_net_socket_window",
            Self::TcpSockbuf => "__syx_tcp_sockbuf",
            Self::NetSetsockopt => "__syx_net_setsockopt",
            Self::VmNativeGcMark => "__syx_vm_native_gc_mark",
            Self::VmNativeGcCount => "__syx_vm_native_gc_count",
            Self::VmNativeGcCollect => "__syx_vm_native_gc_collect",
            Self::RandomNextInt64 => "__syx_random_next_int64",
            Self::RandomNextUInt64 => "__syx_random_next_uint64",
            Self::RandomFillBytes => "__syx_random_fill_bytes",
            Self::CryptoSha256 => "__syx_crypto_sha256",
            Self::CryptoSha512 => "__syx_crypto_sha512",
            Self::CryptoHmacSha256 => "__syx_crypto_hmac_sha256",
            Self::CryptoAesGcmEncrypt => "__syx_crypto_aes_gcm_encrypt",
            Self::SimdFromArray => "__syx_simd_from_array",
            Self::SimdReduceAdd => "__syx_simd_reduce_add",
            Self::SimdLoadFromArray => "__syx_simd_load_from_array",
            Self::SimdStoreToArray => "__syx_simd_store_to_array",
            Self::StringCharAt => "__syx_string_char_at",
            Self::ArraySetAt => "__syx_array_set_at",
            Self::ArrayAppend => "__syx_array_append",
            Self::LeadingZeros128 => "__syx_leading_zeros128",
            Self::SimdShuffle => "__syx_simd_shuffle",
            Self::SimdAnd => "__syx_simd_and",
            Self::SimdOr => "__syx_simd_or",
            Self::SimdXor => "__syx_simd_xor",
            Self::SimdNot => "__syx_simd_not",
            Self::SimdShl => "__syx_simd_shl",
            Self::SimdShr => "__syx_simd_shr",
            Self::SimdMin => "__syx_simd_min",
            Self::SimdMax => "__syx_simd_max",
            Self::SimdF32Add => "__syx_simd_f32_add",
            Self::SimdF32Sub => "__syx_simd_f32_sub",
            Self::SimdF32Mul => "__syx_simd_f32_mul",
            Self::SimdF32Div => "__syx_simd_f32_div",
            Self::SimdF64Add => "__syx_simd_f64_add",
            Self::SimdF64Sub => "__syx_simd_f64_sub",
            Self::SimdF64Mul => "__syx_simd_f64_mul",
            Self::SimdF64Div => "__syx_simd_f64_div",
            Self::SimdI16Add => "__syx_simd_i16_add",
            Self::SimdI16Sub => "__syx_simd_i16_sub",
            Self::SimdI16Mul => "__syx_simd_i16_mul",
            Self::SimdI8Add => "__syx_simd_i8_add",
            Self::SimdI8Sub => "__syx_simd_i8_sub",
            Self::SimdI8Mul => "__syx_simd_i8_mul",
            Self::SimdI8And => "__syx_simd_i8_and",
            Self::SimdI8Or => "__syx_simd_i8_or",
            Self::SimdI8Xor => "__syx_simd_i8_xor",
            Self::SimdI8Not => "__syx_simd_i8_not",
            Self::SimdI8Min => "__syx_simd_i8_min",
            Self::SimdI8Max => "__syx_simd_i8_max",
            Self::SimdI8UMin => "__syx_simd_i8_umin",
            Self::SimdI8UMax => "__syx_simd_i8_umax",
            Self::SimdI8Abs => "__syx_simd_i8_abs",
            Self::SimdI8Neg => "__syx_simd_i8_neg",
            Self::SimdI8Extract => "__syx_simd_i8_extract",
            Self::SimdI8Replace => "__syx_simd_i8_replace",
            Self::SimdI8Eq => "__syx_simd_i8_eq",
            Self::SimdI8Ne => "__syx_simd_i8_ne",
            Self::SimdI8Le => "__syx_simd_i8_le",
            Self::SimdI8Ge => "__syx_simd_i8_ge",
            Self::SimdI8AvgrU => "__syx_simd_i8_avgr_u",
            Self::SimdI8Popcnt => "__syx_simd_i8_popcnt",
            Self::SimdI8Bitmask => "__syx_simd_i8_bitmask",
            Self::SimdI8AllTrue => "__syx_simd_i8_all_true",
            Self::SimdI8AnyTrue => "__syx_simd_i8_any_true",
            Self::SimdI8Lt => "__syx_simd_i8_lt",
            Self::SimdI8Gt => "__syx_simd_i8_gt",
            Self::SimdI8AddSat => "__syx_simd_i8_add_sat",
            Self::SimdI8SubSat => "__syx_simd_i8_sub_sat",
            Self::SimdI8Shuffle => "__syx_simd_i8_shuffle",
            Self::SimdI8Shl => "__syx_simd_i8_shl",
            Self::SimdI8Shr => "__syx_simd_i8_shr",
            Self::SimdI64Add => "__syx_simd_i64_add",
            Self::SelectTag => "__syx_select_tag",
            Self::ServiceRegister => "__syx_service_register",
            Self::ServiceRegisterWitness => "__syx_service_register_witness",
            Self::ServiceAddDep => "__syx_service_add_dep",
            Self::ServiceSeal => "__syx_service_seal",
            Self::ServiceResolve => "__syx_service_resolve",
            Self::ServicePut => "__syx_service_put",
            Self::ServiceReset => "__syx_service_reset",
            Self::ServiceDestroy => "__syx_service_destroy",
            Self::ServiceBoot => "__syx_service_boot",
            Self::ServiceBeginRequest => "__syx_service_begin_request",
            Self::ServiceEndRequest => "__syx_service_end_request",
            Self::ServiceResolveDynamic => "__syx_service_resolve_dynamic",
            Self::ServiceRunInRequest => "__syx_service_run_in_request",
            Self::ServiceSetProfile => "__syx_service_set_profile",
            Self::ServiceLazyProxy => "__syx_service_lazy_proxy",
            Self::SynuiAppCreate => "__syx_synui_app_create",
            Self::SynuiLabel => "__syx_synui_label",
            Self::SynuiButton => "__syx_synui_button",
            Self::SynuiColumn => "__syx_synui_column",
            Self::SynuiPump => "__syx_synui_pump",
            Self::SynuiFingerprint => "__syx_synui_fingerprint",
            Self::SynuiSignalNew => "__syx_synui_signal_new",
            Self::SynuiSignalGet => "__syx_synui_signal_get",
            Self::SynuiSignalSet => "__syx_synui_signal_set",
            Self::SynuiRow => "__syx_synui_row",
            Self::SynuiTextField => "__syx_synui_text_field",
            Self::SynuiButtonOnClick => "__syx_synui_button_on_click",
            Self::SynuiButtonClick => "__syx_synui_button_click",
            Self::SynuiRenderFingerprint => "__syx_synui_render_fingerprint",
            Self::SynuiRun => "__syx_synui_run",
            Self::SynuiCheckbox => "__syx_synui_checkbox",
            Self::SynuiSlider => "__syx_synui_slider",
            Self::SynuiSignalBoolNew => "__syx_synui_signal_bool_new",
            Self::SynuiSignalBoolGet => "__syx_synui_signal_bool_get",
            Self::SynuiSignalBoolSet => "__syx_synui_signal_bool_set",
            Self::SynuiSignalNumberNew => "__syx_synui_signal_number_new",
            Self::SynuiSignalNumberGet => "__syx_synui_signal_number_get",
            Self::SynuiSignalNumberSet => "__syx_synui_signal_number_set",
            Self::SynuiBindTextField => "__syx_synui_bind_text_field",
            Self::SynuiBindCheckbox => "__syx_synui_bind_checkbox",
            Self::SynuiBindSlider => "__syx_synui_bind_slider",
            Self::SynuiSwitch => "__syx_synui_switch",
            Self::SynuiRadio => "__syx_synui_radio",
            Self::SynuiBindSwitch => "__syx_synui_bind_switch",
            Self::SynuiBindRadio => "__syx_synui_bind_radio",
            Self::SynuiProgress => "__syx_synui_progress",
            Self::SynuiPicker => "__syx_synui_picker",
            Self::SynuiTextArea => "__syx_synui_text_area",
            Self::SynuiList => "__syx_synui_list",
            Self::SynuiBindProgress => "__syx_synui_bind_progress",
            Self::SynuiImage => "__syx_synui_image",
            Self::SynuiScrollView => "__syx_synui_scroll_view",
            Self::SynuiBindPicker => "__syx_synui_bind_picker",
            Self::SynuiBindTextArea => "__syx_synui_bind_text_area",
            Self::SynuiBindList => "__syx_synui_bind_list",
            Self::SynuiSpacer => "__syx_synui_spacer",
            Self::SynuiBindImage => "__syx_synui_bind_image",
            Self::SynuiDivider => "__syx_synui_divider",
            Self::SynuiStack => "__syx_synui_stack",
            Self::SynuiBindScrollView => "__syx_synui_bind_scroll_view",
            Self::SynuiTabView => "__syx_synui_tab_view",
            Self::SynuiBindTabView => "__syx_synui_bind_tab_view",
            Self::SynuiNavigation => "__syx_synui_navigation",
            Self::SynuiBindNavigation => "__syx_synui_bind_navigation",
            Self::OsCpuCount => "__syx_os_cpu_count",
            Self::OsMemoryTotal => "__syx_os_memory_total",
            Self::OsMemoryAvailable => "__syx_os_memory_available",
            Self::OsUserName => "__syx_os_user_name",
            Self::OsHomeDir => "__syx_os_home_dir",
            Self::OsHostname => "__syx_os_hostname",
            Self::OsUptimeMs => "__syx_os_uptime_ms",
            Self::OsDiskTotal => "__syx_os_disk_total",
            Self::OsDiskAvailable => "__syx_os_disk_available",
            Self::OsLoadAverage => "__syx_os_load_average",
            Self::OsPrimaryIpv4 => "__syx_os_primary_ipv4",
            Self::TaskSetWorkerCount => "__syx_task_set_worker_count",
            Self::TaskWorkerCount => "__syx_task_worker_count",
            Self::TaskSetQueueCapacity => "__syx_task_set_queue_capacity",
            Self::TaskQueueCapacity => "__syx_task_queue_capacity",
            Self::TaskSetRejectPolicy => "__syx_task_set_reject_policy",
            Self::TaskRejectPolicy => "__syx_task_reject_policy",
            Self::TaskSetCorePoolSize => "__syx_task_set_core_pool_size",
            Self::TaskCorePoolSize => "__syx_task_core_pool_size",
            Self::TaskSetMaximumPoolSize => "__syx_task_set_maximum_pool_size",
            Self::TaskMaximumPoolSize => "__syx_task_maximum_pool_size",
            Self::TaskSetKeepAliveMs => "__syx_task_set_keep_alive_ms",
            Self::TaskKeepAliveMs => "__syx_task_keep_alive_ms",
            Self::TaskLiveWorkerCount => "__syx_task_live_worker_count",
            Self::ActorSetMaxActors => "__syx_actor_set_max_actors",
            Self::ActorMaxActors => "__syx_actor_max_actors",
            Self::ActorSetMailboxCapacity => "__syx_actor_set_mailbox_capacity",
            Self::ActorMailboxCapacity => "__syx_actor_mailbox_capacity",
            Self::ActorSetSharedPoolEnabled => "__syx_actor_set_shared_pool_enabled",
            Self::ActorSharedPoolEnabled => "__syx_actor_shared_pool_enabled",
            Self::ActorSetSharedPoolSize => "__syx_actor_set_shared_pool_size",
            Self::ActorSharedPoolSize => "__syx_actor_shared_pool_size",
            Self::ChannelLength => "__syx_channel_length",
            Self::ChannelCapacity => "__syx_channel_capacity",
            Self::HttpShutdown => "__syx_http_serve_shutdown",
            Self::HttpBodyStreamOpen => "__syx_http_body_stream_open",
            Self::HttpBodyStreamOpenOpts => "__syx_http_body_stream_open_opts",
            Self::HttpBodyStreamReadChunk => "__syx_http_body_stream_read_chunk",
            Self::HttpBodyStreamClose => "__syx_http_body_stream_close",
            Self::FileStreamOpen => "__syx_file_stream_open",
            Self::FileStreamReadChunk => "__syx_file_stream_read_chunk",
            Self::FileStreamClose => "__syx_file_stream_close",
            Self::FileWatcherOpen => "__syx_file_watcher_open",
            Self::FileWatcherPoll => "__syx_file_watcher_poll",
            Self::FileWatcherClose => "__syx_file_watcher_close",
            Self::ProcessBuilderCreate => "__syx_process_builder_create",
            Self::ProcessBuilderCwd => "__syx_process_builder_cwd",
            Self::ProcessBuilderEnv => "__syx_process_builder_env",
            Self::ProcessBuilderSpawn => "__syx_process_builder_spawn",
            Self::ChildWait => "__syx_child_wait",
            Self::ChildKill => "__syx_child_kill",
            Self::ChildWaitTimeout => "__syx_child_wait_timeout",
            Self::ChildStdout => "__syx_child_stdout",
            Self::ChildStderr => "__syx_child_stderr",
            Self::ChildStdin => "__syx_child_stdin",
            Self::ChildPipeReadChunk => "__syx_child_pipe_read_chunk",
            Self::ChildPipeWrite => "__syx_child_pipe_write",
            Self::ChildPipeClose => "__syx_child_pipe_close",
            Self::ChildSignal => "__syx_child_signal",
            Self::SqlPoolCreate => "__syx_sql_pool_create",
            Self::SqlPoolAcquire => "__syx_sql_pool_acquire",
            Self::SqlPoolRelease => "__syx_sql_pool_release",
            Self::SqlPoolClose => "__syx_sql_pool_close",
            Self::SqlRowGetString => "__syx_sql_row_get_string",
            Self::SqlRowGetInt => "__syx_sql_row_get_int",
            Self::SqlRowGetFloat => "__syx_sql_row_get_float",
            Self::SqlRowGetBlob => "__syx_sql_row_get_blob",
            Self::SqlRowIsNull => "__syx_sql_row_is_null",
            Self::SqlRowColumnNames => "__syx_sql_row_column_names",
            Self::DequeNew => "__syx_deque_new",
            Self::DequePushBack => "__syx_deque_push_back",
            Self::DequePushFront => "__syx_deque_push_front",
            Self::DequePopBack => "__syx_deque_pop_back",
            Self::DequePopFront => "__syx_deque_pop_front",
            Self::DequeCount => "__syx_deque_count",
            Self::DequeIsEmpty => "__syx_deque_is_empty",
            Self::TcpListen => "__syx_tcp_listen",
            Self::TcpListenerClose => "__syx_tcp_listener_close",
            Self::TcpListenerAccept => "__syx_tcp_listener_accept",
            Self::TlsServe => "__syx_tls_serve",
            Self::TaskGroupCancel => "__syx_task_group_cancel",
            Self::TaskLiveCount => "__syx_task_live_count",
            Self::TaskSetDeadlineMs => "__syx_task_set_deadline_ms",
            Self::TaskLocalSet => "__syx_task_local_set",
            Self::TaskLocalGet => "__syx_task_local_get",
            Self::ActorReentrancyPolicy => "__syx_actor_reentrancy_policy",
            Self::BlockingSleep => "__syx_blocking_sleep",
            Self::IsolationPolicy => "__syx_isolation_policy",
            Self::BlockingPoolSetSize => "__syx_blocking_pool_set_size",
            Self::BlockingPoolSize => "__syx_blocking_pool_size",
            Self::FileReadAsync => "__syx_file_read_async",
            Self::IoAdapterPolicy => "__syx_io_adapter_policy",
            Self::TcpReadAsync => "__syx_tcp_read_async",
            Self::MutexUnlockQuiet => "__syx_mutex_unlock_quiet",
            Self::SyncPolicy => "__syx_sync_policy",
            Self::MutexLockWait => "__syx_mutex_lock_wait",
            Self::SemaphoreCreate => "__syx_semaphore_create",
            Self::SemaphoreWait => "__syx_semaphore_wait",
            Self::SemaphoreTryWait => "__syx_semaphore_try_wait",
            Self::SemaphoreSignal => "__syx_semaphore_signal",
            Self::SemaphorePolicy => "__syx_semaphore_policy",
            Self::SemaphoreSignalQuiet => "__syx_semaphore_signal_quiet",
            Self::LogSetMinLevel => "__syx_log_set_min_level",
            Self::LogMinLevel => "__syx_log_min_level",
            Self::LogPolicy => "__syx_log_policy",
            Self::LogInfoFields => "__syx_log_info_fields",
            Self::LogTakeLast => "__syx_log_take_last",
            Self::LogDebugFields => "__syx_log_debug_fields",
            Self::LogWarnFields => "__syx_log_warn_fields",
            Self::LogErrorFields => "__syx_log_error_fields",
            Self::TraceStart => "__syx_trace_start",
            Self::TraceEnd => "__syx_trace_end",
            Self::TraceCurrent => "__syx_trace_current",
            Self::TraceContextLine => "__syx_trace_context_line",
            Self::TracePolicy => "__syx_trace_policy",
            Self::MetricsCounterInc => "__syx_metrics_counter_inc",
            Self::MetricsCounterGet => "__syx_metrics_counter_get",
            Self::MetricsCounterReset => "__syx_metrics_counter_reset",
            Self::MetricsSnapshot => "__syx_metrics_snapshot",
            Self::MetricsPolicy => "__syx_metrics_policy",
            Self::MetricsGaugeSet => "__syx_metrics_gauge_set",
            Self::MetricsGaugeGet => "__syx_metrics_gauge_get",
            Self::MetricsCounterIncLabels => "__syx_metrics_counter_inc_labels",
            Self::MetricsCounterGetLabels => "__syx_metrics_counter_get_labels",
            Self::ProfilerStart => "__syx_profiler_start",
            Self::ProfilerStop => "__syx_profiler_stop",
            Self::ProfilerElapsed => "__syx_profiler_elapsed",
            Self::ProfilerReset => "__syx_profiler_reset",
            Self::ProfilerSnapshot => "__syx_profiler_snapshot",
            Self::ProfilerPolicy => "__syx_profiler_policy",
            Self::ProfilerCpuStart => "__syx_profiler_cpu_start",
            Self::ProfilerCpuStop => "__syx_profiler_cpu_stop",
            Self::ProfilerCpuElapsed => "__syx_profiler_cpu_elapsed",
            Self::FileReadBytes => "__syx_file_read_bytes",
            Self::FileWriteBytes => "__syx_file_write_bytes",
            Self::FileOpen => "__syx_file_open",
            Self::FileReadChunk => "__syx_file_read_chunk",
            Self::FileWriteChunk => "__syx_file_write_chunk",
            Self::FileClose => "__syx_file_close",
            Self::FileCloseQuiet => "__syx_file_close_quiet",
            Self::FileWithOpen => "__syx_file_with_open",
            Self::FileWithOpenFail => "__syx_file_with_open_fail",
            Self::FileSeek => "__syx_file_seek",
            Self::FileMmapRead => "__syx_file_read_all_bytes",
            Self::KeyPathGet => "__syx_keypath_get",
            Self::AsOptional => "__syx_as_optional",
            Self::TypeIs => "__syx_type_is",
            Self::TypeCast => "__syx_type_cast",
            Self::StrInstSplit => "__syx_str_split",
            Self::StrInstTrim => "__syx_str_trim",
            Self::StrInstHasPrefix => "__syx_str_has_prefix",
            Self::StrInstLowercased => "__syx_str_lowercased",
            Self::StrInstUppercased => "__syx_str_uppercased",
            Self::StrInstMatches => "__syx_str_matches",
            Self::StrInstLength => "__syx_str_length",
            Self::StrInstSubstring => "__syx_str_substring",
            Self::StrUnicodeScalarCount => "__syx_str_unicode_scalar_count",
            Self::StrUnicodeScalars => "__syx_str_unicode_scalars",
            Self::StrUtf8Count => "__syx_str_utf8_count",
            Self::StrUtf8Bytes => "__syx_str_utf8_bytes",
            Self::CharacterIsSingleScalar => "__syx_character_is_single_scalar",
            Self::StrNormalizedNFC => "__syx_str_normalized_nfc",
            Self::StrNormalizedNFD => "__syx_str_normalized_nfd",
            Self::StrCaseFolded => "__syx_str_case_folded",
            Self::FmtPrint => "__syx_fmt_print",
            Self::FmtPrintLine => "__syx_fmt_print_line",
            Self::FmtFormat => "__syx_fmt_format",
            Self::FmtDebug => "__syx_fmt_debug",
            Self::Base64Encode => "__syx_base64_encode",
            Self::Base64Decode => "__syx_base64_decode",
            Self::HexEncode => "__syx_hex_encode",
            Self::HexDecode => "__syx_hex_decode",
            Self::UrlEncode => "__syx_url_encode",
            Self::UrlDecode => "__syx_url_decode",
            Self::MathFloatSin => "__syx_f64_sin",
            Self::MathFloatCos => "__syx_f64_cos",
            Self::MathFloatTan => "__syx_f64_tan",
            Self::MathFloatSqrt => "__syx_f64_sqrt",
            Self::MathFloatPow => "__syx_f64_pow",
            Self::MathFloatFloor => "__syx_f64_floor",
            Self::MathFloatCeil => "__syx_f64_ceil",
            Self::MathFloatRound => "__syx_f64_round",
            Self::MathFloatLog => "__syx_f64_log",
            Self::MathFloatAsin => "__syx_f64_asin",
            Self::MathFloatAcos => "__syx_f64_acos",
            Self::MathFloatAtan => "__syx_f64_atan",
            Self::MathFloatAtan2 => "__syx_f64_atan2",
            Self::MathFloatExp => "__syx_f64_exp",
            Self::MathFloatExp2 => "__syx_f64_exp2",
            Self::MathFloatLog2 => "__syx_f64_log2",
            Self::MathFloatLog10 => "__syx_f64_log10",
            Self::MathFloatHypot => "__syx_f64_hypot",
            Self::MathFloatCbrt => "__syx_f64_cbrt",
            Self::MathFloatTrunc => "__syx_f64_trunc",
            Self::MathFloatAbs => "__syx_f64_abs",
            Self::MathFloatSinh => "__syx_f64_sinh",
            Self::MathFloatCosh => "__syx_f64_cosh",
            Self::MathFloatTanh => "__syx_f64_tanh",
            Self::MathFloatFmod => "__syx_f64_fmod",
            Self::MathFloatCopysign => "__syx_f64_copysign",
            Self::MathFloatIsNan => "__syx_f64_isnan",
            Self::MathFloatIsInfinite => "__syx_f64_isinf",
            Self::MathFloatIsFinite => "__syx_f64_isfinite",
            Self::MathFloatExpm1 => "__syx_f64_expm1",
            Self::MathFloatLog1p => "__syx_f64_log1p",
            Self::MathFloatAsinh => "__syx_f64_asinh",
            Self::MathFloatAcosh => "__syx_f64_acosh",
            Self::MathFloatAtanh => "__syx_f64_atanh",
            Self::MathFloatRemainder => "__syx_f64_remainder",
            Self::MathFloatNextUp => "__syx_f64_next_up",
            Self::MathFloatNextDown => "__syx_f64_next_down",
            Self::MathFloatMulAdd => "__syx_f64_mul_add",
            Self::MathFloatErf => "__syx_f64_erf",
            Self::MathFloatErfc => "__syx_f64_erfc",
            Self::MathFloatTgamma => "__syx_f64_tgamma",
            Self::MathFloatLgamma => "__syx_f64_lgamma",
            Self::MathFloat32Sqrt => "__syx_f32_sqrt",
            Self::MathFloat32Sin => "__syx_f32_sin",
            Self::MathFloat32Cos => "__syx_f32_cos",
            Self::MathFloat32Tan => "__syx_f32_tan",
            Self::MathFloat32Exp => "__syx_f32_exp",
            Self::MathFloat32Log => "__syx_f32_log",
            Self::MathFloat32Abs => "__syx_f32_abs",
            Self::MathFloat32Floor => "__syx_f32_floor",
            Self::MathFloat32Ceil => "__syx_f32_ceil",
            Self::MathFloat32Round => "__syx_f32_round",
            Self::MathFloat32Trunc => "__syx_f32_trunc",
            Self::MathFloat32Pow => "__syx_f32_pow",
            Self::MathFloat32Asin => "__syx_f32_asin",
            Self::MathFloat32Acos => "__syx_f32_acos",
            Self::MathFloat32Atan => "__syx_f32_atan",
            Self::MathFloat32Hypot => "__syx_f32_hypot",
            Self::MathFloat32Atan2 => "__syx_f32_atan2",
            Self::MathFloat32Log2 => "__syx_f32_log2",
            Self::MathFloat32Log10 => "__syx_f32_log10",
            Self::MathFloat32Exp2 => "__syx_f32_exp2",
            Self::MathFloat32Cbrt => "__syx_f32_cbrt",
            Self::MathFloat32Sinh => "__syx_f32_sinh",
            Self::MathFloat32Cosh => "__syx_f32_cosh",
            Self::MathFloat32Tanh => "__syx_f32_tanh",
            Self::MathFloat32Fmod => "__syx_f32_fmod",
            Self::MathFloat32Copysign => "__syx_f32_copysign",
            Self::MathFloat32Expm1 => "__syx_f32_expm1",
            Self::MathFloat32Log1p => "__syx_f32_log1p",
            Self::MathFloat32Asinh => "__syx_f32_asinh",
            Self::MathFloat32Acosh => "__syx_f32_acosh",
            Self::MathFloat32Atanh => "__syx_f32_atanh",
            Self::MathFloat32Erf => "__syx_f32_erf",
            Self::MathFloat32Erfc => "__syx_f32_erfc",
            Self::MathFloat32Tgamma => "__syx_f32_tgamma",
            Self::MathFloat32Lgamma => "__syx_f32_lgamma",
            Self::MathFloat32Remainder => "__syx_f32_remainder",
            Self::MathFloat32IsNan => "__syx_f32_isnan",
            Self::MathFloat32IsInfinite => "__syx_f32_isinf",
            Self::MathFloat32IsFinite => "__syx_f32_isfinite",
            Self::MathFloat32NextUp => "__syx_f32_next_up",
            Self::MathFloat32NextDown => "__syx_f32_next_down",
            Self::MathFloat32MulAdd => "__syx_f32_mul_add",
            Self::ProcessArgs => "__syx_process_args",
            Self::ProcessEnv => "__syx_process_env",
            Self::ProcessExec => "__syx_process_exec",
            Self::ProcessExitCode => "__syx_process_exit_code",
            Self::FileStat => "__syx_file_stat",
            Self::FileSymlink => "__syx_file_symlink",
            Self::FileReadLink => "__syx_file_read_link",
            Self::FileChmod => "__syx_file_chmod",
            Self::UdpBind => "__syx_udp_bind",
            Self::UdpSendTo => "__syx_udp_send_to",
            Self::UdpRecvFrom => "__syx_udp_recv_from",
            Self::DnsResolve => "__syx_dns_resolve",
            Self::DnsResolveIp => "__syx_dns_resolve_ip",
            Self::CryptoEd25519Sign => "__syx_crypto_ed25519_sign",
            Self::CryptoEd25519Verify => "__syx_crypto_ed25519_verify",
            Self::CryptoPbkdf2 => "__syx_crypto_pbkdf2",
            Self::MutexCreate => "__syx_mutex_create",
            Self::MutexLock => "__syx_mutex_lock",
            Self::MutexUnlock => "__syx_mutex_unlock",
            Self::WsRecv => "__syx_ws_recv",
            Self::WsClose => "__syx_ws_close",
            Self::ArraySortBy => "__syx_array_sort_by",
            Self::ArrayBinarySearch => "__syx_array_binary_search",
            Self::ArrayChunked => "__syx_array_chunked",
            Self::ArrayZip => "__syx_array_zip",
            Self::ArrayFlatMap => "__syx_array_flat_map",
            Self::LogDebug => "__syx_log_debug",
            Self::LogInfo => "__syx_log_info",
            Self::LogWarn => "__syx_log_warn",
            Self::LogError => "__syx_log_error",
            Self::TestingAssert => "__syx_testing_assert",
            Self::TestingAssertEq => "__syx_testing_assert_eq",
            Self::ProcessExecOutput => "__syx_process_exec_output",
            Self::ProcessSpawn => "__syx_process_spawn",
            Self::ProcessWait => "__syx_process_wait",
            Self::FileWatch => "__syx_file_watch",
            Self::StringBuilderNew => "__syx_string_builder_new",
            Self::StringBuilderAppend => "__syx_string_builder_append",
            Self::StringBuilderToString => "__syx_string_builder_to_string",
            Self::BytesFromArray => "__syx_bytes_from_array",
            Self::BytesSlice => "__syx_bytes_slice",
            Self::BytesConcat => "__syx_bytes_concat",
            Self::ChannelClose => "__syx_channel_close",
            Self::TestingTest => "__syx_testing_test",
            Self::TestingExpectEqual => "__syx_testing_expect_equal",
            Self::RegexCompile => "__syx_regex_compile",
            Self::RegexFindAll => "__syx_regex_find_all",
            Self::RegexReplaceAll => "__syx_regex_replace_all",
            Self::EncodingGzipCompress => "__syx_encoding_gzip_compress",
            Self::EncodingGzipDecompress => "__syx_encoding_gzip_decompress",
            Self::ReflectType => "__syx_reflect_type",
            Self::ReflectSpecialization => "__syx_reflect_specialization",
            Self::ValueDescription => "__syx_value_description",
            Self::ValueDebugDescription => "__syx_value_debug_description",
            Self::ValueCompare => "__syx_value_compare",
            Self::EncodingZstdCompress => "__syx_encoding_zstd_compress",
            Self::EncodingZstdDecompress => "__syx_encoding_zstd_decompress",
            Self::CodableEncode => "__syx_codable_encode",
            Self::CodableDecode => "__syx_codable_decode",
            Self::TestingRunAll => "__syx_testing_run_all",
            Self::RuntimeCast => "__syx_runtime_cast",
            Self::CodableEncodeL4 => "__syx_codable_encode_l4",
            Self::CodableDecodeL4 => "__syx_codable_decode_l4",
            Self::JsonEncoderEncode => "__syx_json_encoder_encode",
            Self::JsonDecoderDecode => "__syx_json_decoder_decode",
            Self::JsonStreamOpenWriter => "__syx_json_stream_open_writer",
            Self::JsonStreamWrite => "__syx_json_stream_write",
            Self::JsonStreamFinishWriter => "__syx_json_stream_finish_writer",
            Self::JsonStreamOpenReader => "__syx_json_stream_open_reader",
            Self::JsonStreamFeed => "__syx_json_stream_feed",
            Self::JsonStreamDecodeNext => "__syx_json_stream_decode_next",
            Self::JsonStreamCloseReader => "__syx_json_stream_close_reader",
            Self::GrpcUnary => "__syx_grpc_unary",
            Self::GrpcStreamClient => "__syx_grpc_stream_client",
            Self::GrpcStreamServer => "__syx_grpc_stream_server",
            Self::KvoObserve => "__syx_kvo_observe",
            Self::SqlExec => "__syx_sql_exec",
            Self::SqlQuery => "__syx_sql_query",
            Self::XmlParse => "__syx_xml_parse",
            Self::SqlPrepare => "__syx_sql_prepare",
            Self::SqlOpen => "__syx_sql_open",
            Self::SqlClose => "__syx_sql_close",
            Self::SqlExecOn => "__syx_sql_exec_on",
            Self::SqlQueryOn => "__syx_sql_query_on",
            Self::SqlPrepareOn => "__syx_sql_prepare_on",
            Self::SqlBindStepOn => "__syx_sql_bind_step_on",
            Self::SqlBindPreparedOn => "__syx_sql_bind_prepared_on",
            Self::SqlBeginTransactionOn => "__syx_sql_begin_transaction_on",
            Self::SqlCommitTransactionOn => "__syx_sql_commit_transaction_on",
            Self::SqlRollbackTransactionOn => "__syx_sql_rollback_transaction_on",
            Self::OrmSave => "__syx_orm_save",
            Self::OrmSaveOn => "__syx_orm_save_on",
            Self::OrmFindWhereOn => "__syx_orm_find_where_on",
            Self::OrmUpdateWhereOn => "__syx_orm_update_where_on",
            Self::OrmDeleteWhereOn => "__syx_orm_delete_where_on",
            Self::OrmFind => "__syx_orm_find",
            Self::CodableSchemaRegister => "__syx_codable_schema_register",
            Self::XmlXPath => "__syx_xml_xpath",
            Self::GuiWindowCreate => "__syx_gui_window_create",
            Self::GuiEventPump => "__syx_gui_event_pump",
            Self::GuiCanvasDraw => "__syx_gui_canvas_draw",
            Self::SqlMigrate => "__syx_sql_migrate",
            Self::SqlBindStep => "__syx_sql_bind_step",
            Self::OrmFindWhere => "__syx_orm_find_where",
            Self::OrmUpdateWhere => "__syx_orm_update_where",
            Self::OrmDeleteWhere => "__syx_orm_delete_where",
            Self::GuiWinitWindowCreate => "__syx_gui_winit_window_create",
            Self::GuiWinitEventPump => "__syx_gui_winit_event_pump",
            Self::GuiWinitCanvasDraw => "__syx_gui_winit_canvas_draw",
            Self::GuiButtonCreate => "__syx_gui_button_create",
            Self::GuiLabelCreate => "__syx_gui_label_create",
            Self::GuiOnClick => "__syx_gui_on_click",
            Self::GuiLayoutCreate => "__syx_gui_layout_create",
            Self::GuiLayoutConfigure => "__syx_gui_layout_configure",
            Self::GuiTextFieldCreate => "__syx_gui_text_field_create",
            Self::GuiRunLoopOnce => "__syx_gui_run_loop_once",
            Self::GuiPixelRead => "__syx_gui_pixel_read",
            Self::GuiClickInject => "__syx_gui_click_inject",
            Self::GuiKeyInject => "__syx_gui_key_inject",
            Self::GuiWidgetGetText => "__syx_gui_widget_get_text",
            Self::GuiWidgetSetText => "__syx_gui_widget_set_text",
            Self::SqlBeginTransaction => "__syx_sql_begin_transaction",
            Self::SqlCommitTransaction => "__syx_sql_commit_transaction",
            Self::CodableL14RegisterMigration => "__syx_codable_l14_register_migration",
            Self::CodableEncodeL14 => "__syx_codable_encode_l14",
            Self::CodableDecodeL14 => "__syx_codable_decode_l14",
            Self::CodableL14RegisterRename => "__syx_codable_l14_register_rename",
            Self::SqlRollback => "__syx_sql_rollback_transaction",
            Self::SqlBindMulti => "__syx_sql_bind_multi",
            Self::GuiTouchEvent => "__syx_gui_touch_event",
            Self::GuiMobileWindowCreate => "__syx_gui_mobile_window_create",
            Self::GuiMobileEventPump => "__syx_gui_mobile_event_pump",
            Self::GuiMobileCanvasDraw => "__syx_gui_mobile_canvas_draw",
            Self::GuiMobileRunLoopOnce => "__syx_gui_mobile_run_loop_once",
            Self::GuiMobilePixelRead => "__syx_gui_mobile_pixel_read",
            Self::GcChurnHeap => "__syx_gc_churn_heap",
            Self::ResultBuilderBuildBlock => "__syx_result_builder_buildBlock",
            Self::ResultBuilderBuildEither => "__syx_result_builder_buildEither",
            Self::ResultBuilderBuildArray => "__syx_result_builder_buildArray",
            Self::ResultBuilderBuildOptional => "__syx_result_builder_buildOptional",
            Self::WeakFrom => "__syx_weak_from",
            Self::WeakLoad => "__syx_weak_load",
            Self::WeakAlive => "__syx_weak_alive",
        }
    }

    pub fn all() -> &'static [HostFnId] {
        &[
            Self::FileRead,
            Self::FileReadLines,
            Self::FileWrite,
            Self::FileAppend,
            Self::FileExists,
            Self::FileDelete,
            Self::FileCopy,
            Self::FileMove,
            Self::DirCreate,
            Self::DirList,
            Self::DirExists,
            Self::HttpGet,
            Self::HttpPost,
            Self::HttpResponseJson,
            Self::RegexMatch,
            Self::ReadLine,
            Self::HasherNew,
            Self::HasherCombine,
            Self::HasherFinalize,
            Self::ObjectIdentifier,
            Self::JsonEncode,
            Self::JsonDecode,
            Self::TaskDetach,
            Self::TaskJoin,
            Self::TaskCancel,
            Self::TaskSleep,
            Self::MainActorRun,
            Self::AwaitTask,
            Self::TaskGroupCreate,
            Self::TaskGroupAdd,
            Self::TaskGroupWait,
            Self::HttpRequest,
            Self::HttpResponseStatus,
            Self::HttpResponseText,
            Self::HttpResponseOk,
            Self::HttpResponseHeader,
            Self::HttpResponseHeaders,
            Self::HttpServe,
            Self::HttpMakeResponse,
            Self::UrlParse,
            Self::UrlEncodeQuery,
            Self::UrlBuild,
            Self::EnvGet,
            Self::EnvSet,
            Self::PathJoin,
            Self::PathNormalize,
            Self::PathExists,
            Self::PathExtension,
            Self::TimeNowMs,
            Self::TimeSleepMs,
            Self::InstantNowMs,
            Self::ChannelCreate,
            Self::ChannelSend,
            Self::ChannelTrySend,
            Self::ChannelTryRecv,
            Self::ChannelSelect,
            Self::ChannelRecv,
            Self::ChannelRecvAsync,
            Self::AsyncIterFrom,
            Self::AsyncIterNext,
            Self::DateTimeParse,
            Self::DateTimeFormat,
            Self::DateTimeParseIso8601,
            Self::DateTimeFormatWithTimezone,
            Self::DateTimeNow,
            Self::DateTimeTimezoneOffset,
            Self::OrderedSetNew,
            Self::OrderedSetInsert,
            Self::OrderedSetContains,
            Self::ChannelSelectTimeout,
            Self::ChannelSelectDefault,
            Self::ChannelSelectMixedAwait,
            Self::HttpRequestAsync,
            Self::WsConnect,
            Self::WsConnectAsync,
            Self::WsSendAsync,
            Self::WsSend,
            Self::WsSendProd,
            Self::WasmSabNew,
            Self::WasmAtomicAddI32,
            Self::TcpConnect,
            Self::TcpRead,
            Self::TcpWrite,
            Self::TcpStream,
            Self::TcpStreamTls,
            Self::TcpStreamHandle,
            Self::NetBackpressure,
            Self::NetRingBuffer,
            Self::TcpHandleRelease,
            Self::NetFlowControl,
            Self::TcpHandleRefcount,
            Self::NetSocketWindow,
            Self::TcpSockbuf,
            Self::NetSetsockopt,
            Self::VmNativeGcMark,
            Self::VmNativeGcCount,
            Self::VmNativeGcCollect,
            Self::RandomNextInt64,
            Self::RandomNextUInt64,
            Self::RandomFillBytes,
            Self::CryptoSha256,
            Self::CryptoSha512,
            Self::CryptoHmacSha256,
            Self::CryptoAesGcmEncrypt,
            Self::SimdFromArray,
            Self::SimdReduceAdd,
            Self::SimdLoadFromArray,
            Self::SimdStoreToArray,
            Self::StringCharAt,
            Self::ArraySetAt,
            Self::LeadingZeros128,
            Self::SimdShuffle,
            Self::FileReadBytes,
            Self::FileWriteBytes,
            Self::FileOpen,
            Self::FileReadChunk,
            Self::FileWriteChunk,
            Self::FileClose,
            Self::FileCloseQuiet,
            Self::FileWithOpen,
            Self::FileWithOpenFail,
            Self::FileSeek,
            Self::FileMmapRead,
            Self::KeyPathGet,
            Self::AsOptional,
            Self::TypeIs,
            Self::TypeCast,
            Self::StrInstSplit,
            Self::StrInstTrim,
            Self::StrInstHasPrefix,
            Self::StrInstLowercased,
            Self::StrInstUppercased,
            Self::StrInstMatches,
            Self::StrInstLength,
            Self::StrInstSubstring,
            Self::StrUnicodeScalarCount,
            Self::StrUnicodeScalars,
            Self::StrUtf8Count,
            Self::StrUtf8Bytes,
            Self::CharacterIsSingleScalar,
            Self::StrNormalizedNFC,
            Self::StrNormalizedNFD,
            Self::StrCaseFolded,
            Self::FmtPrint,
            Self::FmtPrintLine,
            Self::FmtFormat,
            Self::FmtDebug,
            Self::Base64Encode,
            Self::Base64Decode,
            Self::HexEncode,
            Self::HexDecode,
            Self::UrlEncode,
            Self::UrlDecode,
            Self::MathFloatSin,
            Self::MathFloatCos,
            Self::MathFloatTan,
            Self::MathFloatSqrt,
            Self::MathFloatPow,
            Self::MathFloatFloor,
            Self::MathFloatCeil,
            Self::MathFloatRound,
            Self::MathFloatLog,
            Self::MathFloatAsin,
            Self::MathFloatAcos,
            Self::MathFloatAtan,
            Self::MathFloatAtan2,
            Self::MathFloatExp,
            Self::MathFloatExp2,
            Self::MathFloatLog2,
            Self::MathFloatLog10,
            Self::MathFloatHypot,
            Self::MathFloatCbrt,
            Self::MathFloatTrunc,
            Self::MathFloatAbs,
            Self::MathFloatSinh,
            Self::MathFloatCosh,
            Self::MathFloatTanh,
            Self::MathFloatFmod,
            Self::MathFloatCopysign,
            Self::MathFloatIsNan,
            Self::MathFloatIsInfinite,
            Self::MathFloatIsFinite,
            Self::MathFloatExpm1,
            Self::MathFloatLog1p,
            Self::MathFloatAsinh,
            Self::MathFloatAcosh,
            Self::MathFloatAtanh,
            Self::MathFloatRemainder,
            Self::MathFloatNextUp,
            Self::MathFloatNextDown,
            Self::MathFloatMulAdd,
            Self::MathFloatErf,
            Self::MathFloatErfc,
            Self::MathFloatTgamma,
            Self::MathFloatLgamma,
            Self::MathFloat32Sqrt,
            Self::MathFloat32Sin,
            Self::MathFloat32Cos,
            Self::MathFloat32Tan,
            Self::MathFloat32Exp,
            Self::MathFloat32Log,
            Self::MathFloat32Abs,
            Self::MathFloat32Floor,
            Self::MathFloat32Ceil,
            Self::MathFloat32Round,
            Self::MathFloat32Trunc,
            Self::MathFloat32Pow,
            Self::MathFloat32Asin,
            Self::MathFloat32Acos,
            Self::MathFloat32Atan,
            Self::MathFloat32Hypot,
            Self::MathFloat32Atan2,
            Self::MathFloat32Log2,
            Self::MathFloat32Log10,
            Self::MathFloat32Exp2,
            Self::MathFloat32Cbrt,
            Self::MathFloat32Sinh,
            Self::MathFloat32Cosh,
            Self::MathFloat32Tanh,
            Self::MathFloat32Fmod,
            Self::MathFloat32Copysign,
            Self::MathFloat32Expm1,
            Self::MathFloat32Log1p,
            Self::MathFloat32Asinh,
            Self::MathFloat32Acosh,
            Self::MathFloat32Atanh,
            Self::MathFloat32Erf,
            Self::MathFloat32Erfc,
            Self::MathFloat32Tgamma,
            Self::MathFloat32Lgamma,
            Self::MathFloat32Remainder,
            Self::MathFloat32IsNan,
            Self::MathFloat32IsInfinite,
            Self::MathFloat32IsFinite,
            Self::MathFloat32NextUp,
            Self::MathFloat32NextDown,
            Self::MathFloat32MulAdd,
            Self::ProcessArgs,
            Self::ProcessEnv,
            Self::ProcessExec,
            Self::ProcessExitCode,
            Self::FileStat,
            Self::FileSymlink,
            Self::FileReadLink,
            Self::FileChmod,
            Self::UdpBind,
            Self::UdpSendTo,
            Self::UdpRecvFrom,
            Self::DnsResolve,
            Self::DnsResolveIp,
            Self::CryptoEd25519Sign,
            Self::CryptoEd25519Verify,
            Self::CryptoPbkdf2,
            Self::MutexCreate,
            Self::MutexLock,
            Self::MutexUnlock,
            Self::WsRecv,
            Self::WsClose,
            Self::ArraySortBy,
            Self::ArrayBinarySearch,
            Self::ArrayChunked,
            Self::ArrayZip,
            Self::ArrayFlatMap,
            Self::LogDebug,
            Self::LogInfo,
            Self::LogWarn,
            Self::LogError,
            Self::TestingAssert,
            Self::TestingAssertEq,
            Self::ProcessExecOutput,
            Self::ProcessSpawn,
            Self::ProcessWait,
            Self::FileWatch,
            Self::StringBuilderNew,
            Self::StringBuilderAppend,
            Self::StringBuilderToString,
            Self::BytesFromArray,
            Self::BytesSlice,
            Self::BytesConcat,
            Self::ChannelClose,
            Self::TestingTest,
            Self::TestingExpectEqual,
            Self::RegexCompile,
            Self::RegexFindAll,
            Self::RegexReplaceAll,
            Self::EncodingGzipCompress,
            Self::EncodingGzipDecompress,
            Self::ReflectType,
            Self::ReflectSpecialization,
            Self::ValueDescription,
            Self::ValueDebugDescription,
            Self::ValueCompare,
            Self::EncodingZstdCompress,
            Self::EncodingZstdDecompress,
            Self::CodableEncode,
            Self::CodableDecode,
            Self::TestingRunAll,
            Self::RuntimeCast,
            Self::CodableEncodeL4,
            Self::CodableDecodeL4,
            Self::JsonEncoderEncode,
            Self::JsonDecoderDecode,
            Self::JsonStreamOpenWriter,
            Self::JsonStreamWrite,
            Self::JsonStreamFinishWriter,
            Self::JsonStreamOpenReader,
            Self::JsonStreamFeed,
            Self::JsonStreamDecodeNext,
            Self::JsonStreamCloseReader,
            Self::GrpcUnary,
            Self::GrpcStreamClient,
            Self::GrpcStreamServer,
            Self::KvoObserve,
            Self::SqlExec,
            Self::SqlQuery,
            Self::XmlParse,
            Self::SqlPrepare,
            Self::SqlOpen,
            Self::SqlClose,
            Self::SqlExecOn,
            Self::SqlQueryOn,
            Self::SqlPrepareOn,
            Self::SqlBindStepOn,
            Self::SqlBindPreparedOn,
            Self::SqlBeginTransactionOn,
            Self::SqlCommitTransactionOn,
            Self::SqlRollbackTransactionOn,
            Self::OrmSave,
            Self::OrmSaveOn,
            Self::OrmFindWhereOn,
            Self::OrmUpdateWhereOn,
            Self::OrmDeleteWhereOn,
            Self::OrmFind,
            Self::CodableSchemaRegister,
            Self::XmlXPath,
            Self::GuiWindowCreate,
            Self::GuiEventPump,
            Self::GuiCanvasDraw,
            Self::SqlMigrate,
            Self::SqlBindStep,
            Self::OrmFindWhere,
            Self::OrmUpdateWhere,
            Self::OrmDeleteWhere,
            Self::GuiWinitWindowCreate,
            Self::GuiWinitEventPump,
            Self::GuiWinitCanvasDraw,
            Self::GuiButtonCreate,
            Self::GuiLabelCreate,
            Self::GuiOnClick,
            Self::GuiLayoutCreate,
            Self::GuiLayoutConfigure,
            Self::GuiTextFieldCreate,
            Self::GuiRunLoopOnce,
            Self::GuiPixelRead,
            Self::GuiClickInject,
            Self::GuiKeyInject,
            Self::GuiWidgetGetText,
            Self::GuiWidgetSetText,
            Self::SqlBeginTransaction,
            Self::SqlCommitTransaction,
            Self::CodableL14RegisterMigration,
            Self::CodableEncodeL14,
            Self::CodableDecodeL14,
            Self::CodableL14RegisterRename,
            Self::SqlRollback,
            Self::SqlBindMulti,
            Self::GuiTouchEvent,
            Self::GuiMobileWindowCreate,
            Self::GuiMobileEventPump,
            Self::GuiMobileCanvasDraw,
            Self::GuiMobileRunLoopOnce,
            Self::GuiMobilePixelRead,
            Self::GcChurnHeap,
            Self::ResultBuilderBuildBlock,
            Self::ResultBuilderBuildEither,
            Self::ResultBuilderBuildArray,
            Self::ResultBuilderBuildOptional,
            Self::WeakFrom,
            Self::WeakLoad,
            Self::WeakAlive,
            Self::SimdAnd,
            Self::SimdOr,
            Self::SimdXor,
            Self::SimdNot,
            Self::SimdShl,
            Self::SimdShr,
            Self::SimdMin,
            Self::SimdMax,
            Self::SimdF32Add,
            Self::SimdF32Sub,
            Self::SimdF32Mul,
            Self::SimdF32Div,
            Self::SimdF64Add,
            Self::SimdF64Sub,
            Self::SimdF64Mul,
            Self::SimdF64Div,
            Self::SimdI16Add,
            Self::SimdI16Sub,
            Self::SimdI16Mul,
            Self::SimdI8Add,
            Self::SimdI8Sub,
            Self::SimdI8Mul,
            Self::SimdI8And,
            Self::SimdI8Or,
            Self::SimdI8Xor,
            Self::SimdI8Not,
            Self::SimdI8Min,
            Self::SimdI8Max,
            Self::SimdI8UMin,
            Self::SimdI8UMax,
            Self::SimdI8Abs,
            Self::SimdI8Neg,
            Self::SimdI8Extract,
            Self::SimdI8Replace,
            Self::SimdI8Eq,
            Self::SimdI8Ne,
            Self::SimdI8Le,
            Self::SimdI8Ge,
            Self::SimdI8AvgrU,
            Self::SimdI8Popcnt,
            Self::SimdI8Bitmask,
            Self::SimdI8AllTrue,
            Self::SimdI8AnyTrue,
            Self::SimdI8Lt,
            Self::SimdI8Gt,
            Self::SimdI8AddSat,
            Self::SimdI8SubSat,
            Self::SimdI8Shuffle,
            Self::SimdI8Shl,
            Self::SimdI8Shr,
            Self::SimdI64Add,
            Self::SelectTag,
            Self::ServiceRegister,
            Self::ServiceRegisterWitness,
            Self::ServiceAddDep,
            Self::ServiceSeal,
            Self::ServiceResolve,
            Self::ServicePut,
            Self::ServiceReset,
            Self::ServiceDestroy,
            Self::ServiceBoot,
            Self::ServiceBeginRequest,
            Self::ServiceEndRequest,
            Self::ServiceResolveDynamic,
            Self::ServiceRunInRequest,
            Self::ServiceSetProfile,
            Self::ServiceLazyProxy,
            Self::SynuiAppCreate,
            Self::SynuiLabel,
            Self::SynuiButton,
            Self::SynuiColumn,
            Self::SynuiPump,
            Self::SynuiFingerprint,
            Self::SynuiSignalNew,
            Self::SynuiSignalGet,
            Self::SynuiSignalSet,
            Self::SynuiRow,
            Self::SynuiTextField,
            Self::SynuiButtonOnClick,
            Self::SynuiButtonClick,
            Self::SynuiRenderFingerprint,
            Self::SynuiRun,
            Self::SynuiCheckbox,
            Self::SynuiSlider,
            Self::SynuiSignalBoolNew,
            Self::SynuiSignalBoolGet,
            Self::SynuiSignalBoolSet,
            Self::SynuiSignalNumberNew,
            Self::SynuiSignalNumberGet,
            Self::SynuiSignalNumberSet,
            Self::SynuiBindTextField,
            Self::SynuiBindCheckbox,
            Self::SynuiBindSlider,
            Self::SynuiSwitch,
            Self::SynuiRadio,
            Self::SynuiBindSwitch,
            Self::SynuiBindRadio,
            Self::SynuiProgress,
            Self::SynuiPicker,
            Self::SynuiTextArea,
            Self::SynuiList,
            Self::SynuiBindProgress,
            Self::SynuiImage,
            Self::SynuiScrollView,
            Self::SynuiBindPicker,
            Self::SynuiBindTextArea,
            Self::SynuiBindList,
            Self::SynuiSpacer,
            Self::SynuiBindImage,
            Self::SynuiDivider,
            Self::SynuiStack,
            Self::SynuiBindScrollView,
            Self::SynuiTabView,
            Self::SynuiBindTabView,
            Self::SynuiNavigation,
            Self::SynuiBindNavigation,
            Self::OsCpuCount,
            Self::OsMemoryTotal,
            Self::OsMemoryAvailable,
            Self::OsUserName,
            Self::OsHomeDir,
            Self::OsHostname,
            Self::OsUptimeMs,
            Self::OsDiskTotal,
            Self::OsDiskAvailable,
            Self::OsLoadAverage,
            Self::OsPrimaryIpv4,
            Self::TaskSetWorkerCount,
            Self::TaskWorkerCount,
            Self::TaskSetQueueCapacity,
            Self::TaskQueueCapacity,
            Self::TaskSetRejectPolicy,
            Self::TaskRejectPolicy,
            Self::TaskSetCorePoolSize,
            Self::TaskCorePoolSize,
            Self::TaskSetMaximumPoolSize,
            Self::TaskMaximumPoolSize,
            Self::TaskSetKeepAliveMs,
            Self::TaskKeepAliveMs,
            Self::TaskLiveWorkerCount,
            Self::ActorSetMaxActors,
            Self::ActorMaxActors,
            Self::ActorSetMailboxCapacity,
            Self::ActorMailboxCapacity,
            Self::ActorSetSharedPoolEnabled,
            Self::ActorSharedPoolEnabled,
            Self::ActorSetSharedPoolSize,
            Self::ActorSharedPoolSize,
            Self::ChannelLength,
            Self::ChannelCapacity,
            Self::HttpShutdown,
            Self::HttpBodyStreamOpen,
            Self::HttpBodyStreamOpenOpts,
            Self::HttpBodyStreamReadChunk,
            Self::HttpBodyStreamClose,
            Self::FileStreamOpen,
            Self::FileStreamReadChunk,
            Self::FileStreamClose,
            Self::FileWatcherOpen,
            Self::FileWatcherPoll,
            Self::FileWatcherClose,
            Self::ProcessBuilderCreate,
            Self::ProcessBuilderCwd,
            Self::ProcessBuilderEnv,
            Self::ProcessBuilderSpawn,
            Self::ChildWait,
            Self::ChildKill,
            Self::ChildWaitTimeout,
            Self::ChildStdout,
            Self::ChildStderr,
            Self::ChildStdin,
            Self::ChildPipeReadChunk,
            Self::ChildPipeWrite,
            Self::ChildPipeClose,
            Self::ChildSignal,
            Self::SqlPoolCreate,
            Self::SqlPoolAcquire,
            Self::SqlPoolRelease,
            Self::SqlPoolClose,
            Self::SqlRowGetString,
            Self::SqlRowGetInt,
            Self::SqlRowGetFloat,
            Self::SqlRowGetBlob,
            Self::SqlRowIsNull,
            Self::SqlRowColumnNames,
            Self::DequeNew,
            Self::DequePushBack,
            Self::DequePushFront,
            Self::DequePopBack,
            Self::DequePopFront,
            Self::DequeCount,
            Self::DequeIsEmpty,
            Self::TcpListen,
            Self::TcpListenerClose,
            Self::TaskGroupCancel,
            Self::TaskLiveCount,
            Self::TaskSetDeadlineMs,
            Self::TaskLocalSet,
            Self::TaskLocalGet,
            Self::ActorReentrancyPolicy,
            Self::BlockingSleep,
            Self::IsolationPolicy,
            Self::BlockingPoolSetSize,
            Self::BlockingPoolSize,
            Self::FileReadAsync,
            Self::IoAdapterPolicy,
            Self::TcpReadAsync,
            Self::MutexUnlockQuiet,
            Self::SyncPolicy,
            Self::MutexLockWait,
            Self::SemaphoreCreate,
            Self::SemaphoreWait,
            Self::SemaphoreTryWait,
            Self::SemaphoreSignal,
            Self::SemaphorePolicy,
            Self::SemaphoreSignalQuiet,
            Self::LogSetMinLevel,
            Self::LogMinLevel,
            Self::LogPolicy,
            Self::LogInfoFields,
            Self::LogTakeLast,
            Self::LogDebugFields,
            Self::LogWarnFields,
            Self::LogErrorFields,
            Self::TraceStart,
            Self::TraceEnd,
            Self::TraceCurrent,
            Self::TraceContextLine,
            Self::TracePolicy,
            Self::MetricsCounterInc,
            Self::MetricsCounterGet,
            Self::MetricsCounterReset,
            Self::MetricsSnapshot,
            Self::MetricsPolicy,
            Self::MetricsGaugeSet,
            Self::MetricsGaugeGet,
            Self::MetricsCounterIncLabels,
            Self::MetricsCounterGetLabels,
            Self::ProfilerStart,
            Self::ProfilerStop,
            Self::ProfilerElapsed,
            Self::ProfilerReset,
            Self::ProfilerSnapshot,
            Self::ProfilerPolicy,
            Self::ProfilerCpuStart,
            Self::ProfilerCpuStop,
            Self::ProfilerCpuElapsed,
            Self::TcpListenerAccept,
            Self::TlsServe,
            Self::ArrayAppend,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MethodId {
    StrSplit,
    StrTrim,
    StrHasPrefix,
    StrLowercased,
    StrUppercased,
    StrReplacingOccurrences,
    StrCount,
    StrSubstring,
    StrMatches,
    ArrLen,
    ArrMap,
    ArrFilter,
    ArrReduce,
    ArrSorted,
    ArrContains,
    ArrReverse,
    ArrJoined,
    HttpResponseJson,
    SetInsert,
    SetRemove,
    SetContains,
    SetCount,
    SetUnion,
    SetIntersection,
    DictContainsKey,
    DictRemove,
    DictCount,
    DictKeys,
    DictValues,
    HttpResponseStatus,
    HttpResponseText,
    HttpResponseBody,
    HttpResponseOk,
    HttpResponseHeader,
    HttpResponseHeaders,
    /// ADR-246 / M2-S8-02b: SqlRow instance accessors (append-only — do not reorder).
    SqlRowGetString,
    SqlRowGetInt,
    SqlRowGetFloat,
    SqlRowGetBlob,
    SqlRowIsNull,
    SqlRowColumnNames,
    /// Append-only: Array.flatMap → CallHost `__syx_array_flat_map`.
    ArrFlatMap,
}

impl MethodId {
    pub fn index(self) -> u32 {
        self as u32
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TypeMethod {
    pub type_name: &'static str,
    pub method: &'static str,
    pub host: Option<HostFnId>,
    pub instance: Option<MethodId>,
}

pub static TYPE_METHODS: LazyLock<Vec<TypeMethod>> = LazyLock::new(|| {
    vec![
        TypeMethod {
            type_name: "File",
            method: "read",
            host: Some(HostFnId::FileRead),
            instance: None,
        },
        TypeMethod {
            type_name: "File",
            method: "readLines",
            host: Some(HostFnId::FileReadLines),
            instance: None,
        },
        TypeMethod {
            type_name: "File",
            method: "write",
            host: Some(HostFnId::FileWrite),
            instance: None,
        },
        TypeMethod {
            type_name: "File",
            method: "append",
            host: Some(HostFnId::FileAppend),
            instance: None,
        },
        TypeMethod {
            type_name: "File",
            method: "exists",
            host: Some(HostFnId::FileExists),
            instance: None,
        },
        TypeMethod {
            type_name: "File",
            method: "delete",
            host: Some(HostFnId::FileDelete),
            instance: None,
        },
        TypeMethod {
            type_name: "File",
            method: "copy",
            host: Some(HostFnId::FileCopy),
            instance: None,
        },
        TypeMethod {
            type_name: "File",
            method: "move",
            host: Some(HostFnId::FileMove),
            instance: None,
        },
        TypeMethod {
            type_name: "File",
            method: "readBytes",
            host: Some(HostFnId::FileReadBytes),
            instance: None,
        },
        TypeMethod {
            type_name: "File",
            method: "writeBytes",
            host: Some(HostFnId::FileWriteBytes),
            instance: None,
        },
        // M2 W3 / ADR-147: open/close must Call the SYX facade (IOError throws), not CallHost.
        // M2 W5 / ADR-149: read/write likewise — hosts remain `__syx_file_*` for facade/raw use.
        // M2 W8 / ADR-152: seek likewise.
        // M2 W9 / ADR-153: readAllBytes likewise (defensive host: None).
        TypeMethod {
            type_name: "FileHandle",
            method: "open",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "FileHandle",
            method: "read",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "FileHandle",
            method: "write",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "FileHandle",
            method: "close",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "FileHandle",
            method: "seek",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "FileHandle",
            method: "readAllBytes",
            host: None,
            instance: None,
        },
        // M2 W90 / ADR-234: withOpen / openResource stay facade (resource contract).
        TypeMethod {
            type_name: "FileHandle",
            method: "openResource",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "FileHandle",
            method: "withOpen",
            host: None,
            instance: None,
        },
        // M2 W4/W10/W11/W12 / ADR-148/154/155/156: Sql facade throws (not CallHost bypass).
        TypeMethod {
            type_name: "Sql",
            method: "exec",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Sql",
            method: "query",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Sql",
            method: "prepare",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Sql",
            method: "bindStep",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Sql",
            method: "beginTransaction",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Sql",
            method: "commitTransaction",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Sql",
            method: "rollbackTransaction",
            host: None,
            instance: None,
        },
        // M2 W16 / ADR-160: multi-Connection open/close/execOn/queryOn (not CallHost bypass).
        TypeMethod {
            type_name: "Sql",
            method: "open",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Sql",
            method: "close",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Sql",
            method: "execOn",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Sql",
            method: "queryOn",
            host: None,
            instance: None,
        },
        // M2 W18 / ADR-162: prepareOn (not CallHost bypass).
        TypeMethod {
            type_name: "Sql",
            method: "prepareOn",
            host: None,
            instance: None,
        },
        // M2 W19 / ADR-163: bindStepOn (not CallHost bypass).
        TypeMethod {
            type_name: "Sql",
            method: "bindStepOn",
            host: None,
            instance: None,
        },
        // M2 W20 / ADR-164: bindPreparedOn (not CallHost bypass).
        TypeMethod {
            type_name: "Sql",
            method: "bindPreparedOn",
            host: None,
            instance: None,
        },
        // M2 W21 / ADR-165: txnOn trio (not CallHost bypass).
        TypeMethod {
            type_name: "Sql",
            method: "beginTransactionOn",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Sql",
            method: "commitTransactionOn",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Sql",
            method: "rollbackTransactionOn",
            host: None,
            instance: None,
        },
        // M2 W101 / ADR-245: connection pool MVP.
        TypeMethod {
            type_name: "Sql",
            method: "poolCreate",
            host: Some(HostFnId::SqlPoolCreate),
            instance: None,
        },
        // Wave A W179 / ADR-523: poolAcquire must Call facade (SqlError throws), not CallHost.
        TypeMethod {
            type_name: "Sql",
            method: "poolAcquire",
            host: None,
            instance: None,
        },
        // Wave A W178 / ADR-522: poolRelease must Call facade (SqlError throws), not CallHost.
        TypeMethod {
            type_name: "Sql",
            method: "poolRelease",
            host: None,
            instance: None,
        },
        // Wave A W177 / ADR-521: poolClose must Call facade (SqlError throws), not CallHost.
        TypeMethod {
            type_name: "Sql",
            method: "poolClose",
            host: None,
            instance: None,
        },
        // M2 W102 / ADR-246: SqlRow typed accessors.
        TypeMethod {
            type_name: "SqlRow",
            method: "getString",
            host: Some(HostFnId::SqlRowGetString),
            instance: Some(MethodId::SqlRowGetString),
        },
        TypeMethod {
            type_name: "SqlRow",
            method: "getInt",
            host: Some(HostFnId::SqlRowGetInt),
            instance: Some(MethodId::SqlRowGetInt),
        },
        TypeMethod {
            type_name: "SqlRow",
            method: "getFloat",
            host: Some(HostFnId::SqlRowGetFloat),
            instance: Some(MethodId::SqlRowGetFloat),
        },
        TypeMethod {
            type_name: "SqlRow",
            method: "getBlob",
            host: Some(HostFnId::SqlRowGetBlob),
            instance: Some(MethodId::SqlRowGetBlob),
        },
        TypeMethod {
            type_name: "SqlRow",
            method: "isNull",
            host: Some(HostFnId::SqlRowIsNull),
            instance: Some(MethodId::SqlRowIsNull),
        },
        TypeMethod {
            type_name: "SqlRow",
            method: "columnNames",
            host: Some(HostFnId::SqlRowColumnNames),
            instance: Some(MethodId::SqlRowColumnNames),
        },
        // M2 W104 / ADR-248: Deque Int MVP.
        TypeMethod {
            type_name: "Deque",
            method: "new",
            host: Some(HostFnId::DequeNew),
            instance: None,
        },
        TypeMethod {
            type_name: "Deque",
            method: "pushBack",
            host: Some(HostFnId::DequePushBack),
            instance: None,
        },
        TypeMethod {
            type_name: "Deque",
            method: "pushFront",
            host: Some(HostFnId::DequePushFront),
            instance: None,
        },
        TypeMethod {
            type_name: "Deque",
            method: "popBack",
            host: Some(HostFnId::DequePopBack),
            instance: None,
        },
        TypeMethod {
            type_name: "Deque",
            method: "popFront",
            host: Some(HostFnId::DequePopFront),
            instance: None,
        },
        TypeMethod {
            type_name: "Deque",
            method: "count",
            host: Some(HostFnId::DequeCount),
            instance: None,
        },
        TypeMethod {
            type_name: "Deque",
            method: "isEmpty",
            host: Some(HostFnId::DequeIsEmpty),
            instance: None,
        },
        TypeMethod {
            type_name: "TcpListener",
            method: "bind",
            host: Some(HostFnId::TcpListen),
            instance: None,
        },
        // Wave A W175 / ADR-519: close must Call facade (NetworkError throws), not CallHost.
        TypeMethod {
            type_name: "TcpListener",
            method: "close",
            host: None,
            instance: None,
        },
        // M2 W14 / ADR-158: Os hostname/diskTotal facade throws (not CallHost bypass).
        // M2 W15 / ADR-159: Os diskAvailable/primaryIpv4 facade throws.
        // M2 W17 / ADR-161: Os userName/homeDir facade throws.
        // M2 W26 / ADR-170: Os memoryTotal facade throws.
        // M2 W27 / ADR-171: Os memoryAvailable facade throws.
        // M2 W28 / ADR-172: Os uptimeMs facade throws.
        // M2 W29 / ADR-173: Os loadAverage facade throws.
        TypeMethod {
            type_name: "Os",
            method: "hostname",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Os",
            method: "diskTotal",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Os",
            method: "diskAvailable",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Os",
            method: "primaryIpv4",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Os",
            method: "userName",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Os",
            method: "homeDir",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Os",
            method: "memoryTotal",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Os",
            method: "memoryAvailable",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Os",
            method: "uptimeMs",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Os",
            method: "loadAverage",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Directory",
            method: "create",
            host: Some(HostFnId::DirCreate),
            instance: None,
        },
        TypeMethod {
            type_name: "Directory",
            method: "list",
            host: Some(HostFnId::DirList),
            instance: None,
        },
        TypeMethod {
            type_name: "Directory",
            method: "exists",
            host: Some(HostFnId::DirExists),
            instance: None,
        },
        TypeMethod {
            type_name: "Http",
            method: "get",
            host: Some(HostFnId::HttpGet),
            instance: None,
        },
        TypeMethod {
            type_name: "Http",
            method: "post",
            host: Some(HostFnId::HttpPost),
            instance: None,
        },
        TypeMethod {
            type_name: "Http",
            method: "fetch",
            host: Some(HostFnId::HttpRequest),
            instance: None,
        },
        TypeMethod {
            type_name: "Http",
            method: "request",
            host: Some(HostFnId::HttpRequest),
            instance: None,
        },
        TypeMethod {
            type_name: "Http",
            method: "serve",
            host: Some(HostFnId::HttpServe),
            instance: None,
        },
        TypeMethod {
            type_name: "Http",
            method: "shutdown",
            host: Some(HostFnId::HttpShutdown),
            instance: None,
        },
        TypeMethod {
            type_name: "Http",
            method: "response",
            host: Some(HostFnId::HttpMakeResponse),
            instance: None,
        },
        TypeMethod {
            type_name: "HttpBodyStream",
            method: "open",
            host: Some(HostFnId::HttpBodyStreamOpen),
            instance: None,
        },
        TypeMethod {
            type_name: "HttpBodyStream",
            method: "openWithOptions",
            host: Some(HostFnId::HttpBodyStreamOpenOpts),
            instance: None,
        },
        TypeMethod {
            type_name: "HttpBodyStream",
            method: "readChunk",
            host: Some(HostFnId::HttpBodyStreamReadChunk),
            instance: None,
        },
        // Wave A W174 / ADR-518: close must Call facade (IOError throws), not CallHost.
        TypeMethod {
            type_name: "HttpBodyStream",
            method: "close",
            host: None,
            instance: None,
        },
        // M2 W96 / ADR-240: FileStream must Call the SYX facade (IOError throws), not CallHost.
        // Wave A W136 / ADR-480: parity with FileHandle ADR-147 (facade Display product nails).
        TypeMethod {
            type_name: "FileStream",
            method: "open",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "FileStream",
            method: "readChunk",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "FileStream",
            method: "close",
            host: None,
            instance: None,
        },
        // Wave A W138 / ADR-482: parity with FileStream ADR-480 / FileHandle ADR-147 (facade Display product nails).
        TypeMethod {
            type_name: "FileWatcher",
            method: "open",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "FileWatcher",
            method: "poll",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "FileWatcher",
            method: "close",
            host: None,
            instance: None,
        },
        // Wave A W191 / ADR-535: ProcessBuilder.create must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "ProcessBuilder",
            method: "create",
            host: None,
            instance: None,
        },
        // Wave A W191 / ADR-535: ProcessBuilder.cwd must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "ProcessBuilder",
            method: "cwd",
            host: None,
            instance: None,
        },
        // Wave A W190 / ADR-534: ProcessBuilder.env must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "ProcessBuilder",
            method: "env",
            host: None,
            instance: None,
        },
        // Wave A W189 / ADR-533: ProcessBuilder.spawn must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "ProcessBuilder",
            method: "spawn",
            host: None,
            instance: None,
        },
        // Wave A W181 / ADR-525: Child.wait must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "Child",
            method: "wait",
            host: None,
            instance: None,
        },
        // Wave A W180 / ADR-524: Child.kill must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "Child",
            method: "kill",
            host: None,
            instance: None,
        },
        // Wave A W183 / ADR-527: Child.waitTimeout must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "Child",
            method: "waitTimeout",
            host: None,
            instance: None,
        },
        // Wave A W184 / ADR-528: Child.stdout must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "Child",
            method: "stdout",
            host: None,
            instance: None,
        },
        // Wave A W185 / ADR-529: Child.stderr must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "Child",
            method: "stderr",
            host: None,
            instance: None,
        },
        // Wave A W186 / ADR-530: Child.stdin must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "Child",
            method: "stdin",
            host: None,
            instance: None,
        },
        // Wave A W187 / ADR-531: ChildPipe.readChunk must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "ChildPipe",
            method: "readChunk",
            host: None,
            instance: None,
        },
        // Wave A W188 / ADR-532: ChildPipe.write must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "ChildPipe",
            method: "write",
            host: None,
            instance: None,
        },
        // Wave A W176 / ADR-520: close must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "ChildPipe",
            method: "close",
            host: None,
            instance: None,
        },
        // Wave A W182 / ADR-526: Child.signal must Call facade (ProcessError throws), not CallHost.
        TypeMethod {
            type_name: "Child",
            method: "signal",
            host: None,
            instance: None,
        },
        TypeMethod {
            type_name: "Url",
            method: "parse",
            host: Some(HostFnId::UrlParse),
            instance: None,
        },
        TypeMethod {
            type_name: "Url",
            method: "encodeQuery",
            host: Some(HostFnId::UrlEncodeQuery),
            instance: None,
        },
        TypeMethod {
            type_name: "Url",
            method: "build",
            host: Some(HostFnId::UrlBuild),
            instance: None,
        },
        TypeMethod {
            type_name: "Env",
            method: "get",
            host: Some(HostFnId::EnvGet),
            instance: None,
        },
        TypeMethod {
            type_name: "Env",
            method: "set",
            host: Some(HostFnId::EnvSet),
            instance: None,
        },
        TypeMethod {
            type_name: "Path",
            method: "join",
            host: Some(HostFnId::PathJoin),
            instance: None,
        },
        TypeMethod {
            type_name: "Path",
            method: "normalize",
            host: Some(HostFnId::PathNormalize),
            instance: None,
        },
        TypeMethod {
            type_name: "Path",
            method: "exists",
            host: Some(HostFnId::PathExists),
            instance: None,
        },
        TypeMethod {
            type_name: "Path",
            method: "fileExtension",
            host: Some(HostFnId::PathExtension),
            instance: None,
        },
        TypeMethod {
            type_name: "Time",
            method: "nowMs",
            host: Some(HostFnId::TimeNowMs),
            instance: None,
        },
        TypeMethod {
            type_name: "Time",
            method: "sleepMs",
            host: Some(HostFnId::TimeSleepMs),
            instance: None,
        },
        TypeMethod {
            type_name: "Instant",
            method: "now",
            host: Some(HostFnId::InstantNowMs),
            instance: None,
        },
        TypeMethod {
            type_name: "Hasher",
            method: "combine",
            host: Some(HostFnId::HasherCombine),
            instance: None,
        },
        TypeMethod {
            type_name: "Hasher",
            method: "finalize",
            host: Some(HostFnId::HasherFinalize),
            instance: None,
        },
        TypeMethod {
            type_name: "Hasher",
            method: "new",
            host: Some(HostFnId::HasherNew),
            instance: None,
        },
        TypeMethod {
            type_name: "Random",
            method: "nextInt",
            host: Some(HostFnId::RandomNextInt64),
            instance: None,
        },
        TypeMethod {
            type_name: "Random",
            method: "nextUInt64",
            host: Some(HostFnId::RandomNextUInt64),
            instance: None,
        },
        TypeMethod {
            type_name: "Random",
            method: "nextBytes",
            host: Some(HostFnId::RandomFillBytes),
            instance: None,
        },
        TypeMethod {
            type_name: "Crypto",
            method: "sha256",
            host: Some(HostFnId::CryptoSha256),
            instance: None,
        },
        TypeMethod {
            type_name: "Crypto",
            method: "sha512",
            host: Some(HostFnId::CryptoSha512),
            instance: None,
        },
        TypeMethod {
            type_name: "Crypto",
            method: "hmacSha256",
            host: Some(HostFnId::CryptoHmacSha256),
            instance: None,
        },
        TypeMethod {
            type_name: "String",
            method: "count",
            host: Some(HostFnId::StrInstLength),
            instance: Some(MethodId::StrCount),
        },
        TypeMethod {
            type_name: "String",
            method: "unicodeScalarCount",
            host: Some(HostFnId::StrUnicodeScalarCount),
            instance: None,
        },
        TypeMethod {
            type_name: "String",
            method: "unicodeScalars",
            host: Some(HostFnId::StrUnicodeScalars),
            instance: None,
        },
        TypeMethod {
            type_name: "String",
            method: "utf8Count",
            host: Some(HostFnId::StrUtf8Count),
            instance: None,
        },
        TypeMethod {
            type_name: "String",
            method: "utf8Bytes",
            host: Some(HostFnId::StrUtf8Bytes),
            instance: None,
        },
        TypeMethod {
            type_name: "Character",
            method: "isSingleScalar",
            host: Some(HostFnId::CharacterIsSingleScalar),
            instance: None,
        },
        TypeMethod {
            type_name: "String",
            method: "normalizedNFC",
            host: Some(HostFnId::StrNormalizedNFC),
            instance: None,
        },
        TypeMethod {
            type_name: "String",
            method: "normalizedNFD",
            host: Some(HostFnId::StrNormalizedNFD),
            instance: None,
        },
        TypeMethod {
            type_name: "String",
            method: "caseFolded",
            host: Some(HostFnId::StrCaseFolded),
            instance: None,
        },
        TypeMethod {
            type_name: "String",
            method: "split",
            host: Some(HostFnId::StrInstSplit),
            instance: Some(MethodId::StrSplit),
        },
        TypeMethod {
            type_name: "Crypto",
            method: "aesGcmEncrypt",
            host: Some(HostFnId::CryptoAesGcmEncrypt),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD4",
            method: "reduceAdd",
            host: Some(HostFnId::SimdReduceAdd),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD4",
            method: "min",
            host: Some(HostFnId::SimdMin),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD4",
            method: "max",
            host: Some(HostFnId::SimdMax),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD4",
            method: "addF32",
            host: Some(HostFnId::SimdF32Add),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD4",
            method: "subF32",
            host: Some(HostFnId::SimdF32Sub),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD4",
            method: "mulF32",
            host: Some(HostFnId::SimdF32Mul),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD4",
            method: "divF32",
            host: Some(HostFnId::SimdF32Div),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD4",
            method: "loadFromArray",
            host: Some(HostFnId::SimdLoadFromArray),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD4",
            method: "storeToArray",
            host: Some(HostFnId::SimdStoreToArray),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD2",
            method: "addF64",
            host: Some(HostFnId::SimdF64Add),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD2",
            method: "subF64",
            host: Some(HostFnId::SimdF64Sub),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD2",
            method: "mulF64",
            host: Some(HostFnId::SimdF64Mul),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD2",
            method: "divF64",
            host: Some(HostFnId::SimdF64Div),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD2",
            method: "addI64",
            host: Some(HostFnId::SimdI64Add),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD8",
            method: "add",
            host: Some(HostFnId::SimdI16Add),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD8",
            method: "mul",
            host: Some(HostFnId::SimdI16Mul),
            instance: None,
        },
        // fromArray stays on the stdlib body (`__syx_simd_from_array(items, 8)`);
        // a host TypeMethod would only pass the array and drop the lanes arg.
        TypeMethod {
            type_name: "SIMD8",
            method: "loadFromArray",
            host: Some(HostFnId::SimdLoadFromArray),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD8",
            method: "storeToArray",
            host: Some(HostFnId::SimdStoreToArray),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD8",
            method: "reduceAdd",
            host: Some(HostFnId::SimdReduceAdd),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "add",
            host: Some(HostFnId::SimdI8Add),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "sub",
            host: Some(HostFnId::SimdI8Sub),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "mul",
            host: Some(HostFnId::SimdI8Mul),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "and",
            host: Some(HostFnId::SimdI8And),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "or",
            host: Some(HostFnId::SimdI8Or),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "xor",
            host: Some(HostFnId::SimdI8Xor),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "not",
            host: Some(HostFnId::SimdI8Not),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "min",
            host: Some(HostFnId::SimdI8Min),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "max",
            host: Some(HostFnId::SimdI8Max),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "umin",
            host: Some(HostFnId::SimdI8UMin),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "umax",
            host: Some(HostFnId::SimdI8UMax),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "abs",
            host: Some(HostFnId::SimdI8Abs),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "neg",
            host: Some(HostFnId::SimdI8Neg),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "extract",
            host: Some(HostFnId::SimdI8Extract),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "replace",
            host: Some(HostFnId::SimdI8Replace),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "eq",
            host: Some(HostFnId::SimdI8Eq),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "ne",
            host: Some(HostFnId::SimdI8Ne),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "le",
            host: Some(HostFnId::SimdI8Le),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "ge",
            host: Some(HostFnId::SimdI8Ge),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "avgrU",
            host: Some(HostFnId::SimdI8AvgrU),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "popcnt",
            host: Some(HostFnId::SimdI8Popcnt),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "bitmask",
            host: Some(HostFnId::SimdI8Bitmask),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "allTrue",
            host: Some(HostFnId::SimdI8AllTrue),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "anyTrue",
            host: Some(HostFnId::SimdI8AnyTrue),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "lt",
            host: Some(HostFnId::SimdI8Lt),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "gt",
            host: Some(HostFnId::SimdI8Gt),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "addSat",
            host: Some(HostFnId::SimdI8AddSat),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "subSat",
            host: Some(HostFnId::SimdI8SubSat),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "shuffle",
            host: Some(HostFnId::SimdI8Shuffle),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "shl",
            host: Some(HostFnId::SimdI8Shl),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "shr",
            host: Some(HostFnId::SimdI8Shr),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "reduceAdd",
            host: Some(HostFnId::SimdReduceAdd),
            instance: None,
        },
        // Phase 129: SIMD16 i8x16 load/store MVP (lanes from type; body keeps lanes=16 for free calls).
        TypeMethod {
            type_name: "SIMD16",
            method: "loadFromArray",
            host: Some(HostFnId::SimdLoadFromArray),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD16",
            method: "storeToArray",
            host: Some(HostFnId::SimdStoreToArray),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD2",
            method: "loadFromArray",
            host: Some(HostFnId::SimdLoadFromArray),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD2",
            method: "storeToArray",
            host: Some(HostFnId::SimdStoreToArray),
            instance: None,
        },
        TypeMethod {
            type_name: "SIMD2",
            method: "reduceAdd",
            host: Some(HostFnId::SimdReduceAdd),
            instance: None,
        },
        TypeMethod {
            type_name: "Fmt",
            method: "print",
            host: Some(HostFnId::FmtPrint),
            instance: None,
        },
        TypeMethod {
            type_name: "Fmt",
            method: "printLine",
            host: Some(HostFnId::FmtPrintLine),
            instance: None,
        },
        TypeMethod {
            type_name: "Fmt",
            method: "format",
            host: Some(HostFnId::FmtFormat),
            instance: None,
        },
        TypeMethod {
            type_name: "Fmt",
            method: "debug",
            host: Some(HostFnId::FmtDebug),
            instance: None,
        },
        TypeMethod {
            type_name: "Encoding",
            method: "base64Encode",
            host: Some(HostFnId::Base64Encode),
            instance: None,
        },
        TypeMethod {
            type_name: "Encoding",
            method: "base64Decode",
            host: Some(HostFnId::Base64Decode),
            instance: None,
        },
        TypeMethod {
            type_name: "Encoding",
            method: "hexEncode",
            host: Some(HostFnId::HexEncode),
            instance: None,
        },
        TypeMethod {
            type_name: "Encoding",
            method: "hexDecode",
            host: Some(HostFnId::HexDecode),
            instance: None,
        },
        TypeMethod {
            type_name: "Encoding",
            method: "urlEncode",
            host: Some(HostFnId::UrlEncode),
            instance: None,
        },
        TypeMethod {
            type_name: "Encoding",
            method: "urlDecode",
            host: Some(HostFnId::UrlDecode),
            instance: None,
        },
        TypeMethod {
            type_name: "DateTime",
            method: "now",
            host: Some(HostFnId::DateTimeNow),
            instance: None,
        },
        TypeMethod {
            type_name: "DateTime",
            method: "parse",
            host: Some(HostFnId::DateTimeParse),
            instance: None,
        },
        TypeMethod {
            type_name: "DateTime",
            method: "format",
            host: Some(HostFnId::DateTimeFormat),
            instance: None,
        },
        TypeMethod {
            type_name: "DateTime",
            method: "parseIso8601",
            host: Some(HostFnId::DateTimeParseIso8601),
            instance: None,
        },
        TypeMethod {
            type_name: "DateTime",
            method: "formatWithTimezone",
            host: Some(HostFnId::DateTimeFormatWithTimezone),
            instance: None,
        },
        TypeMethod {
            type_name: "DateTime",
            method: "timezoneOffset",
            host: Some(HostFnId::DateTimeTimezoneOffset),
            instance: None,
        },
        // Channel product API lives in packages/sync (class Channel + init/create).
        // Do not bind Channel.* TypeMethods to hosts here — that bypasses the Sy facade
        // (e.g. Channel.create would return a raw host object instead of Channel).
        TypeMethod {
            type_name: "Process",
            method: "args",
            host: Some(HostFnId::ProcessArgs),
            instance: None,
        },
        TypeMethod {
            type_name: "Process",
            method: "env",
            host: Some(HostFnId::ProcessEnv),
            instance: None,
        },
        TypeMethod {
            type_name: "Process",
            method: "exec",
            host: Some(HostFnId::ProcessExec),
            instance: None,
        },
        TypeMethod {
            type_name: "Process",
            method: "exitCode",
            host: Some(HostFnId::ProcessExitCode),
            instance: None,
        },
        TypeMethod {
            type_name: "Testing",
            method: "assert",
            host: Some(HostFnId::TestingAssert),
            instance: None,
        },
        TypeMethod {
            type_name: "Testing",
            method: "assertEq",
            host: Some(HostFnId::TestingAssertEq),
            instance: None,
        },
        TypeMethod {
            type_name: "Log",
            method: "debug",
            host: Some(HostFnId::LogDebug),
            instance: None,
        },
        TypeMethod {
            type_name: "Log",
            method: "info",
            host: Some(HostFnId::LogInfo),
            instance: None,
        },
        TypeMethod {
            type_name: "Log",
            method: "warn",
            host: Some(HostFnId::LogWarn),
            instance: None,
        },
        TypeMethod {
            type_name: "Log",
            method: "error",
            host: Some(HostFnId::LogError),
            instance: None,
        },
        TypeMethod {
            type_name: "Log",
            method: "setMinLevel",
            host: Some(HostFnId::LogSetMinLevel),
            instance: None,
        },
        TypeMethod {
            type_name: "Log",
            method: "minLevel",
            host: Some(HostFnId::LogMinLevel),
            instance: None,
        },
        TypeMethod {
            type_name: "Log",
            method: "policy",
            host: Some(HostFnId::LogPolicy),
            instance: None,
        },
        TypeMethod {
            type_name: "Log",
            method: "infoFields",
            host: Some(HostFnId::LogInfoFields),
            instance: None,
        },
        TypeMethod {
            type_name: "Log",
            method: "takeLast",
            host: Some(HostFnId::LogTakeLast),
            instance: None,
        },
        TypeMethod {
            type_name: "Log",
            method: "debugFields",
            host: Some(HostFnId::LogDebugFields),
            instance: None,
        },
        TypeMethod {
            type_name: "Log",
            method: "warnFields",
            host: Some(HostFnId::LogWarnFields),
            instance: None,
        },
        TypeMethod {
            type_name: "Log",
            method: "errorFields",
            host: Some(HostFnId::LogErrorFields),
            instance: None,
        },
        TypeMethod {
            type_name: "Trace",
            method: "start",
            host: Some(HostFnId::TraceStart),
            instance: None,
        },
        TypeMethod {
            type_name: "Trace",
            method: "end",
            host: Some(HostFnId::TraceEnd),
            instance: None,
        },
        TypeMethod {
            type_name: "Trace",
            method: "current",
            host: Some(HostFnId::TraceCurrent),
            instance: None,
        },
        TypeMethod {
            type_name: "Trace",
            method: "contextLine",
            host: Some(HostFnId::TraceContextLine),
            instance: None,
        },
        TypeMethod {
            type_name: "Trace",
            method: "policy",
            host: Some(HostFnId::TracePolicy),
            instance: None,
        },
        TypeMethod {
            type_name: "Metrics",
            method: "inc",
            host: Some(HostFnId::MetricsCounterInc),
            instance: None,
        },
        TypeMethod {
            type_name: "Metrics",
            method: "get",
            host: Some(HostFnId::MetricsCounterGet),
            instance: None,
        },
        TypeMethod {
            type_name: "Metrics",
            method: "reset",
            host: Some(HostFnId::MetricsCounterReset),
            instance: None,
        },
        TypeMethod {
            type_name: "Metrics",
            method: "snapshot",
            host: Some(HostFnId::MetricsSnapshot),
            instance: None,
        },
        TypeMethod {
            type_name: "Metrics",
            method: "policy",
            host: Some(HostFnId::MetricsPolicy),
            instance: None,
        },
        TypeMethod {
            type_name: "Metrics",
            method: "gaugeSet",
            host: Some(HostFnId::MetricsGaugeSet),
            instance: None,
        },
        TypeMethod {
            type_name: "Metrics",
            method: "gaugeGet",
            host: Some(HostFnId::MetricsGaugeGet),
            instance: None,
        },
        TypeMethod {
            type_name: "Metrics",
            method: "incLabels",
            host: Some(HostFnId::MetricsCounterIncLabels),
            instance: None,
        },
        TypeMethod {
            type_name: "Metrics",
            method: "getLabels",
            host: Some(HostFnId::MetricsCounterGetLabels),
            instance: None,
        },
        TypeMethod {
            type_name: "Profiler",
            method: "start",
            host: Some(HostFnId::ProfilerStart),
            instance: None,
        },
        TypeMethod {
            type_name: "Profiler",
            method: "stop",
            host: Some(HostFnId::ProfilerStop),
            instance: None,
        },
        TypeMethod {
            type_name: "Profiler",
            method: "elapsed",
            host: Some(HostFnId::ProfilerElapsed),
            instance: None,
        },
        TypeMethod {
            type_name: "Profiler",
            method: "reset",
            host: Some(HostFnId::ProfilerReset),
            instance: None,
        },
        TypeMethod {
            type_name: "Profiler",
            method: "snapshot",
            host: Some(HostFnId::ProfilerSnapshot),
            instance: None,
        },
        TypeMethod {
            type_name: "Profiler",
            method: "policy",
            host: Some(HostFnId::ProfilerPolicy),
            instance: None,
        },
        TypeMethod {
            type_name: "Profiler",
            method: "cpuStart",
            host: Some(HostFnId::ProfilerCpuStart),
            instance: None,
        },
        TypeMethod {
            type_name: "Profiler",
            method: "cpuStop",
            host: Some(HostFnId::ProfilerCpuStop),
            instance: None,
        },
        TypeMethod {
            type_name: "Profiler",
            method: "cpuElapsed",
            host: Some(HostFnId::ProfilerCpuElapsed),
            instance: None,
        },
        TypeMethod {
            type_name: "TcpSocket",
            method: "connect",
            host: Some(HostFnId::TcpConnect),
            instance: None,
        },
        TypeMethod {
            type_name: "TcpSocket",
            method: "read",
            host: Some(HostFnId::TcpRead),
            instance: None,
        },
        TypeMethod {
            type_name: "TcpSocket",
            method: "write",
            host: Some(HostFnId::TcpWrite),
            instance: None,
        },
        TypeMethod {
            type_name: "File",
            method: "stat",
            host: Some(HostFnId::FileStat),
            instance: None,
        },
        TypeMethod {
            type_name: "File",
            method: "symlink",
            host: Some(HostFnId::FileSymlink),
            instance: None,
        },
        TypeMethod {
            type_name: "File",
            method: "readLink",
            host: Some(HostFnId::FileReadLink),
            instance: None,
        },
        TypeMethod {
            type_name: "File",
            method: "chmod",
            host: Some(HostFnId::FileChmod),
            instance: None,
        },
    ]
});

pub static SET_METHODS: LazyLock<IndexMap<&'static str, MethodId>> = LazyLock::new(|| {
    IndexMap::from([
        ("insert", MethodId::SetInsert),
        ("remove", MethodId::SetRemove),
        ("contains", MethodId::SetContains),
        ("count", MethodId::SetCount),
        ("union", MethodId::SetUnion),
        ("intersection", MethodId::SetIntersection),
    ])
});

pub static DICT_METHODS: LazyLock<IndexMap<&'static str, MethodId>> = LazyLock::new(|| {
    IndexMap::from([
        ("containsKey", MethodId::DictContainsKey),
        ("remove", MethodId::DictRemove),
        ("count", MethodId::DictCount),
        ("keys", MethodId::DictKeys),
        ("values", MethodId::DictValues),
    ])
});

pub fn lookup_set_method(method: &str) -> Option<MethodId> {
    SET_METHODS.get(method).copied()
}

pub fn lookup_dict_method(method: &str) -> Option<MethodId> {
    DICT_METHODS.get(method).copied()
}

pub static STRING_METHODS: LazyLock<IndexMap<&'static str, MethodId>> = LazyLock::new(|| {
    IndexMap::from([
        ("split", MethodId::StrSplit),
        ("trim", MethodId::StrTrim),
        ("hasPrefix", MethodId::StrHasPrefix),
        ("lowercased", MethodId::StrLowercased),
        ("uppercased", MethodId::StrUppercased),
        ("replacingOccurrences", MethodId::StrReplacingOccurrences),
        ("count", MethodId::StrCount),
        ("substring", MethodId::StrSubstring),
        ("matches", MethodId::StrMatches),
    ])
});

pub static ARRAY_METHODS: LazyLock<IndexMap<&'static str, MethodId>> = LazyLock::new(|| {
    IndexMap::from([
        ("count", MethodId::ArrLen),
        ("map", MethodId::ArrMap),
        ("filter", MethodId::ArrFilter),
        ("reduce", MethodId::ArrReduce),
        ("sorted", MethodId::ArrSorted),
        ("contains", MethodId::ArrContains),
        ("reverse", MethodId::ArrReverse),
        ("joined", MethodId::ArrJoined),
        ("flatMap", MethodId::ArrFlatMap),
    ])
});

pub static HTTP_RESPONSE_METHODS: LazyLock<IndexMap<&'static str, MethodId>> =
    LazyLock::new(|| {
        IndexMap::from([
            ("json", MethodId::HttpResponseJson),
            ("status", MethodId::HttpResponseStatus),
            ("text", MethodId::HttpResponseText),
            ("body", MethodId::HttpResponseBody),
            ("ok", MethodId::HttpResponseOk),
            ("header", MethodId::HttpResponseHeader),
            ("headers", MethodId::HttpResponseHeaders),
        ])
    });

pub fn lookup_http_response_method(method: &str) -> Option<MethodId> {
    HTTP_RESPONSE_METHODS.get(method).copied()
}

pub static SQL_ROW_METHODS: LazyLock<IndexMap<&'static str, MethodId>> = LazyLock::new(|| {
    IndexMap::from([
        ("getString", MethodId::SqlRowGetString),
        ("getInt", MethodId::SqlRowGetInt),
        ("getFloat", MethodId::SqlRowGetFloat),
        ("getBlob", MethodId::SqlRowGetBlob),
        ("isNull", MethodId::SqlRowIsNull),
        ("columnNames", MethodId::SqlRowColumnNames),
    ])
});

pub fn lookup_sql_row_method(method: &str) -> Option<MethodId> {
    SQL_ROW_METHODS.get(method).copied()
}

pub fn lookup_type_method(type_name: &str, method: &str) -> Option<TypeMethod> {
    TYPE_METHODS
        .iter()
        .find(|m| m.type_name == type_name && m.method == method)
        .copied()
}

/// W912–W917: Float64/Double Math instance methods (unary + binary + ternary) → MathFloat* hosts.
pub fn lookup_float64_math_host(method: &str) -> Option<HostFnId> {
    match method {
        "sqrt" => Some(HostFnId::MathFloatSqrt),
        "sin" => Some(HostFnId::MathFloatSin),
        "cos" => Some(HostFnId::MathFloatCos),
        "tan" => Some(HostFnId::MathFloatTan),
        "exp" => Some(HostFnId::MathFloatExp),
        "log" => Some(HostFnId::MathFloatLog),
        "floor" => Some(HostFnId::MathFloatFloor),
        "ceil" => Some(HostFnId::MathFloatCeil),
        "abs" => Some(HostFnId::MathFloatAbs),
        "trunc" => Some(HostFnId::MathFloatTrunc),
        "round" => Some(HostFnId::MathFloatRound),
        "asin" => Some(HostFnId::MathFloatAsin),
        "acos" => Some(HostFnId::MathFloatAcos),
        "atan" => Some(HostFnId::MathFloatAtan),
        "exp2" => Some(HostFnId::MathFloatExp2),
        "cbrt" => Some(HostFnId::MathFloatCbrt),
        "log2" => Some(HostFnId::MathFloatLog2),
        "log10" => Some(HostFnId::MathFloatLog10),
        "pow" => Some(HostFnId::MathFloatPow),
        "atan2" => Some(HostFnId::MathFloatAtan2),
        "hypot" => Some(HostFnId::MathFloatHypot),
        "sinh" => Some(HostFnId::MathFloatSinh),
        "cosh" => Some(HostFnId::MathFloatCosh),
        "tanh" => Some(HostFnId::MathFloatTanh),
        "expm1" => Some(HostFnId::MathFloatExpm1),
        "log1p" => Some(HostFnId::MathFloatLog1p),
        "asinh" => Some(HostFnId::MathFloatAsinh),
        "acosh" => Some(HostFnId::MathFloatAcosh),
        "atanh" => Some(HostFnId::MathFloatAtanh),
        "remainder" => Some(HostFnId::MathFloatRemainder),
        "nextUp" => Some(HostFnId::MathFloatNextUp),
        "nextDown" => Some(HostFnId::MathFloatNextDown),
        "fmod" => Some(HostFnId::MathFloatFmod),
        "copysign" => Some(HostFnId::MathFloatCopysign),
        "mulAdd" => Some(HostFnId::MathFloatMulAdd),
        "erf" => Some(HostFnId::MathFloatErf),
        "erfc" => Some(HostFnId::MathFloatErfc),
        "tgamma" => Some(HostFnId::MathFloatTgamma),
        "lgamma" => Some(HostFnId::MathFloatLgamma),
        "isNaN" => Some(HostFnId::MathFloatIsNan),
        "isInfinite" => Some(HostFnId::MathFloatIsInfinite),
        "isFinite" => Some(HostFnId::MathFloatIsFinite),
        _ => None,
    }
}

/// W917–W922: Float32 Math instance methods → MathFloat32* hosts.
pub fn lookup_float32_math_host(method: &str) -> Option<HostFnId> {
    match method {
        "sqrt" => Some(HostFnId::MathFloat32Sqrt),
        "sin" => Some(HostFnId::MathFloat32Sin),
        "cos" => Some(HostFnId::MathFloat32Cos),
        "tan" => Some(HostFnId::MathFloat32Tan),
        "exp" => Some(HostFnId::MathFloat32Exp),
        "log" => Some(HostFnId::MathFloat32Log),
        "floor" => Some(HostFnId::MathFloat32Floor),
        "ceil" => Some(HostFnId::MathFloat32Ceil),
        "abs" => Some(HostFnId::MathFloat32Abs),
        "trunc" => Some(HostFnId::MathFloat32Trunc),
        "round" => Some(HostFnId::MathFloat32Round),
        "asin" => Some(HostFnId::MathFloat32Asin),
        "acos" => Some(HostFnId::MathFloat32Acos),
        "atan" => Some(HostFnId::MathFloat32Atan),
        "log2" => Some(HostFnId::MathFloat32Log2),
        "log10" => Some(HostFnId::MathFloat32Log10),
        "pow" => Some(HostFnId::MathFloat32Pow),
        "atan2" => Some(HostFnId::MathFloat32Atan2),
        "hypot" => Some(HostFnId::MathFloat32Hypot),
        "sinh" => Some(HostFnId::MathFloat32Sinh),
        "cosh" => Some(HostFnId::MathFloat32Cosh),
        "tanh" => Some(HostFnId::MathFloat32Tanh),
        "expm1" => Some(HostFnId::MathFloat32Expm1),
        "log1p" => Some(HostFnId::MathFloat32Log1p),
        "asinh" => Some(HostFnId::MathFloat32Asinh),
        "acosh" => Some(HostFnId::MathFloat32Acosh),
        "atanh" => Some(HostFnId::MathFloat32Atanh),
        "remainder" => Some(HostFnId::MathFloat32Remainder),
        "nextUp" => Some(HostFnId::MathFloat32NextUp),
        "nextDown" => Some(HostFnId::MathFloat32NextDown),
        "fmod" => Some(HostFnId::MathFloat32Fmod),
        "copysign" => Some(HostFnId::MathFloat32Copysign),
        "mulAdd" => Some(HostFnId::MathFloat32MulAdd),
        "erf" => Some(HostFnId::MathFloat32Erf),
        "erfc" => Some(HostFnId::MathFloat32Erfc),
        "tgamma" => Some(HostFnId::MathFloat32Tgamma),
        "lgamma" => Some(HostFnId::MathFloat32Lgamma),
        "isNaN" => Some(HostFnId::MathFloat32IsNan),
        "exp2" => Some(HostFnId::MathFloat32Exp2),
        "cbrt" => Some(HostFnId::MathFloat32Cbrt),
        "isInfinite" => Some(HostFnId::MathFloat32IsInfinite),
        "isFinite" => Some(HostFnId::MathFloat32IsFinite),
        _ => None,
    }
}

pub fn lookup_string_method(method: &str) -> Option<MethodId> {
    STRING_METHODS.get(method).copied()
}

pub fn lookup_array_method(method: &str) -> Option<MethodId> {
    ARRAY_METHODS.get(method).copied()
}

pub fn host_fn_id_by_name(name: &str) -> Option<HostFnId> {
    if name == "__syx_async_let_detach" {
        return Some(HostFnId::TaskDetach);
    }
    // M0-B07: legacy alias — was misnamed mmap; same `read_to_end` host.
    if name == "__syx_file_mmap_read" {
        return Some(HostFnId::FileMmapRead);
    }
    if lookup_numeric_host(name).is_some()
        || lookup_collection_host(name).is_some()
        || lookup_bigint_host(name).is_some()
        || lookup_result_host(name).is_some()
    {
        return None;
    }
    HostFnId::all().iter().copied().find(|id| id.name() == name)
}

fn numeric_host_base_index() -> u32 {
    HostFnId::all().len() as u32
}

fn collection_host_base_index() -> u32 {
    numeric_host_base_index() + NUMERIC_HOST_TABLE.len() as u32
}

fn bigint_host_base_index() -> u32 {
    collection_host_base_index() + COLLECTION_HOST_TABLE.len() as u32
}

fn result_host_base_index() -> u32 {
    bigint_host_base_index() + BIGINT_HOST_TABLE.len() as u32
}

/// First index of append-only dynamic plugin Host segment (ADR-071 §1.3).
pub fn plugin_host_base_index() -> u32 {
    result_host_base_index() + RESULT_HOST_TABLE.len() as u32
}

static COMPILE_PLUGIN_HOSTS: LazyLock<std::sync::Mutex<Vec<(&'static str, &'static str)>>> =
    LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

/// Register compile-time plugin Host names (name, return_surface).
pub fn register_compile_plugin_hosts(entries: Vec<(&'static str, &'static str)>) {
    if let Ok(mut g) = COMPILE_PLUGIN_HOSTS.lock() {
        g.extend(entries);
    }
}

fn compile_plugin_hosts() -> Vec<(&'static str, &'static str)> {
    COMPILE_PLUGIN_HOSTS
        .lock()
        .map(|g| g.clone())
        .unwrap_or_default()
}

pub fn plugin_host_return_type(name: &str) -> Option<&'static str> {
    compile_plugin_hosts()
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, ret)| *ret)
}

pub fn host_fn_index_by_name(name: &str) -> Option<u32> {
    if let Some(id) = host_fn_id_by_name(name) {
        return Some(host_fn_index(id));
    }
    if let Some(pos) = lookup_numeric_host(name).and_then(|e| {
        NUMERIC_HOST_TABLE
            .iter()
            .position(|x| x.name == e.name)
    }) {
        return Some(numeric_host_base_index() + pos as u32);
    }
    if let Some(pos) = lookup_collection_host(name).and_then(|e| {
        COLLECTION_HOST_TABLE
            .iter()
            .position(|x| x.name == e.name)
    }) {
        return Some(collection_host_base_index() + pos as u32);
    }
    if let Some(pos) = lookup_bigint_host(name).and_then(|e| {
        BIGINT_HOST_TABLE.iter().position(|x| x.name == e.name)
    }) {
        return Some(bigint_host_base_index() + pos as u32);
    }
    if let Some(pos) = lookup_result_host(name).and_then(|e| {
        RESULT_HOST_TABLE.iter().position(|x| x.name == e.name)
    }) {
        return Some(result_host_base_index() + pos as u32);
    }
    for (i, (host_name, _)) in compile_plugin_hosts().iter().enumerate() {
        if *host_name == name {
            return Some(plugin_host_base_index() + i as u32);
        }
    }
    None
}

pub fn host_fn_index(id: HostFnId) -> u32 {
    HostFnId::all()
        .iter()
        .position(|h| *h == id)
        .unwrap_or(0) as u32
}

pub fn host_fn_name_for_index(idx: u32) -> Option<&'static str> {
    let base = numeric_host_base_index();
    if idx < base {
        return HostFnId::all().get(idx as usize).map(|id| id.name());
    }
    let idx = idx - base;
    if idx < NUMERIC_HOST_TABLE.len() as u32 {
        return NUMERIC_HOST_TABLE.get(idx as usize).map(|e| e.name);
    }
    let idx = idx - NUMERIC_HOST_TABLE.len() as u32;
    if idx < COLLECTION_HOST_TABLE.len() as u32 {
        return COLLECTION_HOST_TABLE.get(idx as usize).map(|e| e.name);
    }
    let idx = idx - COLLECTION_HOST_TABLE.len() as u32;
    if idx < BIGINT_HOST_TABLE.len() as u32 {
        return BIGINT_HOST_TABLE.get(idx as usize).map(|e| e.name);
    }
    let idx = idx - BIGINT_HOST_TABLE.len() as u32;
    if idx < RESULT_HOST_TABLE.len() as u32 {
        return RESULT_HOST_TABLE.get(idx as usize).map(|e| e.name);
    }
    let idx = idx - RESULT_HOST_TABLE.len() as u32;
    let off = idx;
    compile_plugin_hosts()
        .get(off as usize)
        .map(|(name, _)| *name)
}

pub fn method_return_type(method: MethodId) -> &'static str {
    match method {
        MethodId::StrSplit | MethodId::StrMatches => "Array",
        MethodId::StrTrim
        | MethodId::StrLowercased
        | MethodId::StrUppercased
        | MethodId::StrReplacingOccurrences
        | MethodId::StrSubstring => "String",
        MethodId::StrHasPrefix => "Bool",
        MethodId::StrCount | MethodId::ArrLen => "Int",
        MethodId::ArrMap
        | MethodId::ArrFilter
        | MethodId::ArrSorted
        | MethodId::ArrReverse
        | MethodId::ArrFlatMap => "Array",
        MethodId::ArrContains => "Bool",
        MethodId::ArrJoined => "String",
        MethodId::ArrReduce => "Unknown",
        MethodId::HttpResponseJson => "Object",
        MethodId::HttpResponseStatus => "Int",
        MethodId::HttpResponseText | MethodId::HttpResponseBody => "String",
        MethodId::HttpResponseOk => "Bool",
        MethodId::HttpResponseHeader => "String",
        MethodId::HttpResponseHeaders => "Object",
        MethodId::SetInsert | MethodId::SetRemove | MethodId::SetContains => "Bool",
        MethodId::SetCount | MethodId::DictCount => "Int",
        MethodId::SetUnion | MethodId::SetIntersection => "Set",
        MethodId::DictContainsKey | MethodId::DictRemove => "Bool",
        MethodId::DictKeys | MethodId::DictValues => "Array",
        MethodId::SqlRowGetString => "String",
        MethodId::SqlRowGetInt => "Int",
        MethodId::SqlRowGetFloat => "Float64",
        MethodId::SqlRowGetBlob => "Array",
        MethodId::SqlRowIsNull => "Bool",
        MethodId::SqlRowColumnNames => "Array",
    }
}

/// True for members accessed without call parentheses (`arr.count`, not `arr.map()`).
pub fn is_instance_property_member(name: &str, method: MethodId) -> bool {
    matches!(
        (name, method),
        (
            "count",
            MethodId::ArrLen | MethodId::DictCount | MethodId::SetCount | MethodId::StrCount
        ) | (
            "status" | "text" | "body" | "ok" | "headers",
            MethodId::HttpResponseStatus
                | MethodId::HttpResponseText
                | MethodId::HttpResponseBody
                | MethodId::HttpResponseOk
                | MethodId::HttpResponseHeaders
        )
    )
}

/// Parameter list for IDE completion detail (instance / collection methods).
pub fn method_param_label(method: MethodId) -> &'static str {
    match method {
        MethodId::StrSplit => "sep: String",
        MethodId::StrTrim | MethodId::StrLowercased | MethodId::StrUppercased => "",
        MethodId::StrHasPrefix => "prefix: String",
        MethodId::StrReplacingOccurrences => "of: String, with: String",
        MethodId::StrSubstring => "start: Int, end: Int",
        MethodId::StrMatches => "pattern: String",
        MethodId::StrCount => "",
        MethodId::ArrLen => "",
        MethodId::ArrMap | MethodId::ArrFilter | MethodId::ArrFlatMap => "transform: Function",
        MethodId::ArrSorted | MethodId::ArrReverse => "",
        MethodId::ArrReduce => "initial: Int, combine: Function",
        MethodId::ArrContains => "item: Object",
        MethodId::ArrJoined => "sep: String",
        MethodId::HttpResponseJson | MethodId::HttpResponseStatus | MethodId::HttpResponseText
        | MethodId::HttpResponseBody | MethodId::HttpResponseOk | MethodId::HttpResponseHeaders => "",
        MethodId::HttpResponseHeader => "name: String",
        MethodId::SetInsert => "item: Object",
        MethodId::SetRemove | MethodId::SetContains => "item: Object",
        MethodId::SetCount => "",
        MethodId::SetUnion | MethodId::SetIntersection => "other: Set",
        MethodId::DictContainsKey | MethodId::DictRemove => "key: Object",
        MethodId::DictCount => "",
        MethodId::DictKeys | MethodId::DictValues => "",
        MethodId::SqlRowGetString
        | MethodId::SqlRowGetInt
        | MethodId::SqlRowGetFloat
        | MethodId::SqlRowGetBlob
        | MethodId::SqlRowIsNull => "name: String",
        MethodId::SqlRowColumnNames => "",
    }
}

pub fn host_fn_return_type(id: HostFnId) -> &'static str {
    match id {
        HostFnId::FileRead => "String",
        HostFnId::FileReadBytes | HostFnId::FileReadChunk | HostFnId::FileMmapRead => "Array",
        HostFnId::FileReadLines | HostFnId::DirList | HostFnId::RegexMatch => "Array",
        HostFnId::FileWrite
        | HostFnId::FileAppend
        | HostFnId::FileWriteBytes
        | HostFnId::FileExists
        | HostFnId::FileDelete
        | HostFnId::FileCopy
        | HostFnId::FileMove
        | HostFnId::FileClose
        | HostFnId::FileCloseQuiet
        | HostFnId::FileSeek
        | HostFnId::DirCreate
        | HostFnId::DirExists => "Bool",
        HostFnId::FileOpen => "Int",
        HostFnId::FileWriteChunk => "Int",
        HostFnId::FileWithOpen => "Int",
        HostFnId::FileWithOpenFail => "Bool",
        HostFnId::KeyPathGet => "String",
        HostFnId::AsOptional => "Object",
        HostFnId::TypeIs => "Bool",
        HostFnId::TypeCast => "Object",
        HostFnId::StrInstSplit | HostFnId::StrInstMatches => "Array",
        HostFnId::StrInstTrim
        | HostFnId::StrInstLowercased
        | HostFnId::StrInstUppercased => "String",
        HostFnId::StrInstHasPrefix => "Bool",
        HostFnId::StrInstLength => "Int",
        HostFnId::StrInstSubstring => "String",
        HostFnId::StrUnicodeScalarCount | HostFnId::StrUtf8Count => "Int",
        HostFnId::StrUnicodeScalars | HostFnId::StrUtf8Bytes => "Array",
        HostFnId::CharacterIsSingleScalar => "Bool",
        HostFnId::StrNormalizedNFC
        | HostFnId::StrNormalizedNFD
        | HostFnId::StrCaseFolded => "String",
        HostFnId::HttpGet | HostFnId::HttpPost => "HTTPResponse",
        HostFnId::HttpResponseJson => "Object",
        HostFnId::ReadLine => "String",
        HostFnId::HasherNew => "Hasher",
        HostFnId::HasherCombine => "Hasher",
        HostFnId::HasherFinalize => "Int",
        HostFnId::ObjectIdentifier => "ObjectIdentifier",
        HostFnId::JsonEncode => "String",
        HostFnId::JsonDecode => "Object",
        HostFnId::TaskDetach => "Object",
        HostFnId::TaskJoin => "Object",
        HostFnId::TaskCancel => "Bool",
        HostFnId::TaskSleep => "Nil",
        HostFnId::MainActorRun => "Object",
        HostFnId::AwaitTask => "Object",
        HostFnId::TaskGroupCreate => "Object",
        HostFnId::TaskGroupAdd => "Bool",
        HostFnId::TaskGroupWait => "Nil",
        HostFnId::HttpRequest => "HTTPResponse",
        HostFnId::HttpResponseStatus => "Int",
        HostFnId::HttpResponseText => "String",
        HostFnId::HttpResponseOk => "Bool",
        HostFnId::HttpResponseHeader => "String",
        HostFnId::HttpResponseHeaders => "Object",
        HostFnId::HttpServe => "Nil",
        HostFnId::HttpShutdown => "Nil",
        HostFnId::HttpBodyStreamOpen => "Object",
        HostFnId::HttpBodyStreamOpenOpts => "Object",
        HostFnId::HttpBodyStreamReadChunk => "String",
        HostFnId::HttpBodyStreamClose => "Nil",
        HostFnId::FileStreamOpen => "Int",
        HostFnId::FileStreamReadChunk => "Array",
        HostFnId::FileStreamClose => "Nil",
        HostFnId::FileWatcherOpen => "Int",
        HostFnId::FileWatcherPoll => "Array",
        HostFnId::FileWatcherClose => "Nil",
        HostFnId::ProcessBuilderCreate => "Int",
        HostFnId::ProcessBuilderCwd => "Nil",
        HostFnId::ProcessBuilderEnv => "Nil",
        HostFnId::ProcessBuilderSpawn => "Int",
        HostFnId::ChildWait => "Int",
        HostFnId::ChildKill => "Nil",
        HostFnId::ChildWaitTimeout => "Int",
        HostFnId::ChildStdout => "Int",
        HostFnId::ChildStderr => "Int",
        HostFnId::ChildStdin => "Int",
        HostFnId::ChildPipeReadChunk => "Array",
        HostFnId::ChildPipeWrite => "Int",
        HostFnId::ChildPipeClose => "Nil",
        HostFnId::ChildSignal => "Nil",
        HostFnId::SqlPoolCreate => "Int",
        HostFnId::SqlPoolAcquire => "Int",
        HostFnId::SqlPoolRelease => "Nil",
        HostFnId::SqlPoolClose => "Nil",
        HostFnId::SqlRowGetString => "String",
        HostFnId::SqlRowGetInt => "Int",
        HostFnId::SqlRowGetFloat => "Float64",
        HostFnId::SqlRowGetBlob => "Array",
        HostFnId::SqlRowIsNull => "Bool",
        HostFnId::SqlRowColumnNames => "Array",
        HostFnId::DequeNew => "Deque",
        HostFnId::DequePushBack => "Nil",
        HostFnId::DequePushFront => "Nil",
        HostFnId::DequePopBack => "Unknown",
        HostFnId::DequePopFront => "Unknown",
        HostFnId::DequeCount => "Int",
        HostFnId::DequeIsEmpty => "Bool",
        HostFnId::TcpListen => "Int",
        HostFnId::TcpListenerClose => "Nil",
        HostFnId::TcpListenerAccept => "Int",
        HostFnId::TlsServe => "Int",
        HostFnId::TaskGroupCancel => "Nil",
        HostFnId::TaskLiveCount => "Int",
        HostFnId::TaskSetDeadlineMs => "Nil",
        HostFnId::TaskLocalSet => "Nil",
        HostFnId::TaskLocalGet => "Object",
        HostFnId::ActorReentrancyPolicy => "String",
        HostFnId::BlockingSleep => "Nil",
        HostFnId::IsolationPolicy => "String",
        HostFnId::BlockingPoolSetSize => "Int",
        HostFnId::BlockingPoolSize => "Int",
        HostFnId::FileReadAsync => "Object",
        HostFnId::IoAdapterPolicy => "String",
        HostFnId::TcpReadAsync => "Object",
        HostFnId::MutexUnlockQuiet => "Bool",
        HostFnId::SyncPolicy => "String",
        HostFnId::MutexLockWait => "Bool",
        HostFnId::SemaphoreCreate => "Int",
        HostFnId::SemaphoreWait => "Bool",
        HostFnId::SemaphoreTryWait => "Bool",
        HostFnId::SemaphoreSignal => "Bool",
        HostFnId::SemaphorePolicy => "String",
        HostFnId::SemaphoreSignalQuiet => "Bool",
        HostFnId::LogSetMinLevel => "Bool",
        HostFnId::LogMinLevel => "String",
        HostFnId::LogPolicy => "String",
        HostFnId::LogInfoFields => "String",
        HostFnId::LogTakeLast => "String",
        HostFnId::LogDebugFields => "String",
        HostFnId::LogWarnFields => "String",
        HostFnId::LogErrorFields => "String",
        HostFnId::TraceStart => "String",
        HostFnId::TraceEnd => "Bool",
        HostFnId::TraceCurrent => "String",
        HostFnId::TraceContextLine => "String",
        HostFnId::TracePolicy => "String",
        HostFnId::MetricsCounterInc => "Int",
        HostFnId::MetricsCounterGet => "Int",
        HostFnId::MetricsCounterReset => "Bool",
        HostFnId::MetricsSnapshot => "String",
        HostFnId::MetricsPolicy => "String",
        HostFnId::MetricsGaugeSet => "Int",
        HostFnId::MetricsGaugeGet => "Int",
        HostFnId::MetricsCounterIncLabels => "Int",
        HostFnId::MetricsCounterGetLabels => "Int",
        HostFnId::ProfilerStart => "Bool",
        HostFnId::ProfilerStop => "Int",
        HostFnId::ProfilerElapsed => "Int",
        HostFnId::ProfilerReset => "Bool",
        HostFnId::ProfilerSnapshot => "String",
        HostFnId::ProfilerPolicy => "String",
        HostFnId::ProfilerCpuStart => "Bool",
        HostFnId::ProfilerCpuStop => "Int",
        HostFnId::ProfilerCpuElapsed => "Int",
        // ADR-236 / M2-S4-01: named types (not糊墙 Object).
        HostFnId::HttpMakeResponse => "HTTPResponse",
        HostFnId::UrlParse => "Url",
        HostFnId::UrlEncodeQuery => "String",
        HostFnId::UrlBuild => "String",
        HostFnId::EnvGet => "String",
        HostFnId::EnvSet => "Bool",
        HostFnId::PathJoin => "String",
        HostFnId::PathNormalize => "String",
        HostFnId::PathExists => "Bool",
        HostFnId::PathExtension => "String",
        HostFnId::TimeNowMs => "Int",
        HostFnId::TimeSleepMs => "Nil",
        HostFnId::InstantNowMs => "Int",
        HostFnId::ChannelCreate => "Object",
        HostFnId::ChannelSend => "Bool",
        HostFnId::ChannelTrySend => "Bool",
        HostFnId::ChannelTryRecv => "Unknown",
        HostFnId::ChannelSelect => "Unknown",
        HostFnId::ChannelRecv => "Unknown",
        HostFnId::ChannelRecvAsync => "Object",
        HostFnId::AsyncIterFrom => "Object",
        HostFnId::AsyncIterNext => "Object",
        HostFnId::DateTimeParse => "Object",
        HostFnId::DateTimeFormat => "String",
        HostFnId::DateTimeParseIso8601 => "Object",
        HostFnId::DateTimeFormatWithTimezone => "String",
        HostFnId::DateTimeTimezoneOffset => "Int",
        HostFnId::DateTimeNow => "Object",
        HostFnId::OrderedSetNew => "Object",
        HostFnId::OrderedSetInsert => "Object",
        HostFnId::OrderedSetContains => "Bool",
        HostFnId::ChannelSelectTimeout => "Unknown",
        HostFnId::ChannelSelectDefault => "Unknown",
        HostFnId::ChannelSelectMixedAwait => "Unknown",
        HostFnId::HttpRequestAsync => "HTTPResponse",
        HostFnId::WsConnect => "Object",
        HostFnId::WsConnectAsync => "Object",
        HostFnId::WsSendAsync => "String",
        HostFnId::WsSend => "Bool",
        HostFnId::WsSendProd => "Bool",
        HostFnId::WasmSabNew => "Object",
        HostFnId::WasmAtomicAddI32 => "Int",
        HostFnId::TcpConnect => "Int",
        HostFnId::TcpRead => "String",
        HostFnId::TcpWrite => "String",
        HostFnId::TcpStream => "Int",
        HostFnId::TcpStreamTls => "Int",
        HostFnId::TcpStreamHandle => "Int",
        HostFnId::NetBackpressure => "Bool",
        HostFnId::NetRingBuffer => "String",
        HostFnId::TcpHandleRelease => "Bool",
        HostFnId::NetFlowControl => "String",
        HostFnId::TcpHandleRefcount => "Int",
        HostFnId::NetSocketWindow => "String",
        HostFnId::TcpSockbuf => "Int",
        HostFnId::NetSetsockopt => "String",
        HostFnId::VmNativeGcMark => "Int",
        HostFnId::VmNativeGcCount => "Int",
        HostFnId::VmNativeGcCollect => "Int",
        HostFnId::RandomNextInt64 => "Int",
        HostFnId::RandomNextUInt64 => "UInt64",
        HostFnId::RandomFillBytes => "Array",
        HostFnId::CryptoSha256 => "String",
        HostFnId::CryptoSha512 => "String",
        HostFnId::CryptoHmacSha256 => "String",
        HostFnId::CryptoAesGcmEncrypt => "String",
        HostFnId::SimdFromArray => "Unknown",
        HostFnId::SimdReduceAdd => "Unknown",
        HostFnId::SimdLoadFromArray => "Unknown",
        HostFnId::SimdStoreToArray => "Array",
        HostFnId::StringCharAt => "String",
        HostFnId::ArraySetAt => "Array",
        HostFnId::ArrayAppend => "Nil",
        HostFnId::LeadingZeros128 => "Int",
        HostFnId::SimdI8AllTrue | HostFnId::SimdI8AnyTrue => "Bool",
        HostFnId::SimdShuffle => "Unknown",
        HostFnId::SimdAnd
        | HostFnId::SimdOr
        | HostFnId::SimdXor
        | HostFnId::SimdNot
        | HostFnId::SimdShl
        | HostFnId::SimdShr
        | HostFnId::SimdMin
        | HostFnId::SimdMax
        | HostFnId::SimdF32Add
        | HostFnId::SimdF32Sub
        | HostFnId::SimdF32Mul
        | HostFnId::SimdF32Div
        | HostFnId::SimdF64Add
        | HostFnId::SimdF64Sub
        | HostFnId::SimdF64Mul
        | HostFnId::SimdF64Div
        | HostFnId::SimdI16Add
        | HostFnId::SimdI16Sub
        | HostFnId::SimdI16Mul
        | HostFnId::SimdI8Add
        | HostFnId::SimdI8Sub
        | HostFnId::SimdI8Mul
        | HostFnId::SimdI8And
        | HostFnId::SimdI8Or
        | HostFnId::SimdI8Xor
        | HostFnId::SimdI8Not
        | HostFnId::SimdI8Min
        | HostFnId::SimdI8Max
        | HostFnId::SimdI8UMin
        | HostFnId::SimdI8UMax
        | HostFnId::SimdI8Abs
        | HostFnId::SimdI8Neg
        | HostFnId::SimdI8Replace
        | HostFnId::SimdI8Eq
        | HostFnId::SimdI8Ne
        | HostFnId::SimdI8Le
        | HostFnId::SimdI8Ge
        | HostFnId::SimdI8AvgrU
        | HostFnId::SimdI8Popcnt
        | HostFnId::SimdI8Bitmask
        | HostFnId::SimdI8Lt
        | HostFnId::SimdI8Gt
        | HostFnId::SimdI8AddSat
        | HostFnId::SimdI8SubSat
        | HostFnId::SimdI8Shuffle
        | HostFnId::SimdI8Shl
        | HostFnId::SimdI8Shr
        | HostFnId::SimdI64Add => "Unknown",
        HostFnId::SimdI8Extract => "Int",
        HostFnId::SelectTag => "Object",
        HostFnId::ServiceRegister
        | HostFnId::ServiceRegisterWitness
        | HostFnId::ServiceAddDep
        | HostFnId::ServiceSeal
        | HostFnId::ServiceReset
        | HostFnId::ServiceDestroy
        | HostFnId::ServiceBoot
        | HostFnId::ServiceBeginRequest
        | HostFnId::ServiceEndRequest
        | HostFnId::ServiceRunInRequest
        | HostFnId::ServiceSetProfile
        | HostFnId::ServicePut => "Void",
        HostFnId::ServiceResolve | HostFnId::ServiceResolveDynamic | HostFnId::ServiceLazyProxy => {
            "Object"
        }
        HostFnId::SynuiAppCreate
        | HostFnId::SynuiLabel
        | HostFnId::SynuiButton
        | HostFnId::SynuiColumn
        | HostFnId::SynuiPump
        | HostFnId::SynuiFingerprint
        | HostFnId::SynuiSignalNew
        | HostFnId::SynuiSignalGet
        | HostFnId::SynuiSignalSet
        | HostFnId::SynuiRow
        | HostFnId::SynuiTextField
        | HostFnId::SynuiButtonOnClick
        | HostFnId::SynuiButtonClick
        | HostFnId::SynuiRenderFingerprint
        | HostFnId::SynuiRun
        | HostFnId::SynuiCheckbox
        | HostFnId::SynuiSlider
        | HostFnId::SynuiSignalBoolNew
        | HostFnId::SynuiSignalBoolGet
        | HostFnId::SynuiSignalBoolSet
        | HostFnId::SynuiSignalNumberNew
        | HostFnId::SynuiSignalNumberGet
        | HostFnId::SynuiSignalNumberSet
        | HostFnId::SynuiBindTextField
        | HostFnId::SynuiBindCheckbox
        | HostFnId::SynuiBindSlider
        | HostFnId::SynuiSwitch
        | HostFnId::SynuiRadio
        | HostFnId::SynuiBindSwitch
        | HostFnId::SynuiBindRadio
        | HostFnId::SynuiProgress
        | HostFnId::SynuiPicker
        | HostFnId::SynuiTextArea
        | HostFnId::SynuiList
        | HostFnId::SynuiBindProgress
        | HostFnId::SynuiImage
        | HostFnId::SynuiScrollView
        | HostFnId::SynuiBindPicker
        | HostFnId::SynuiBindTextArea
        | HostFnId::SynuiBindList
        | HostFnId::SynuiSpacer
        | HostFnId::SynuiBindImage
        | HostFnId::SynuiDivider
        | HostFnId::SynuiStack
        | HostFnId::SynuiBindScrollView
        | HostFnId::SynuiTabView
        | HostFnId::SynuiBindTabView
        | HostFnId::SynuiNavigation
        | HostFnId::SynuiBindNavigation => "String",
        HostFnId::OsCpuCount
        | HostFnId::OsMemoryTotal
        | HostFnId::OsMemoryAvailable
        | HostFnId::OsUptimeMs
        | HostFnId::OsDiskTotal
        | HostFnId::OsDiskAvailable
        | HostFnId::OsLoadAverage
        | HostFnId::TaskSetWorkerCount
        | HostFnId::TaskWorkerCount
        | HostFnId::TaskSetQueueCapacity
        | HostFnId::TaskQueueCapacity
        | HostFnId::TaskSetCorePoolSize
        | HostFnId::TaskCorePoolSize
        | HostFnId::TaskSetMaximumPoolSize
        | HostFnId::TaskMaximumPoolSize
        | HostFnId::TaskSetKeepAliveMs
        | HostFnId::TaskKeepAliveMs
        | HostFnId::TaskLiveWorkerCount
        | HostFnId::ActorSetMaxActors
        | HostFnId::ActorMaxActors
        | HostFnId::ActorSetMailboxCapacity
        | HostFnId::ActorMailboxCapacity
        | HostFnId::ActorSetSharedPoolSize
        | HostFnId::ActorSharedPoolSize
        | HostFnId::ChannelLength
        | HostFnId::ChannelCapacity => "Int",
        HostFnId::ActorSetSharedPoolEnabled | HostFnId::ActorSharedPoolEnabled => "Bool",
        HostFnId::TaskSetRejectPolicy | HostFnId::TaskRejectPolicy => "String",
        HostFnId::OsUserName | HostFnId::OsHomeDir | HostFnId::OsHostname | HostFnId::OsPrimaryIpv4 => "String",
        HostFnId::FmtPrint | HostFnId::FmtPrintLine => "Nil",
        HostFnId::FmtFormat | HostFnId::FmtDebug => "String",
        HostFnId::Base64Encode | HostFnId::HexEncode | HostFnId::UrlEncode => "String",
        HostFnId::Base64Decode | HostFnId::HexDecode | HostFnId::UrlDecode => "String",
        HostFnId::MathFloatSin
        | HostFnId::MathFloatCos
        | HostFnId::MathFloatTan
        | HostFnId::MathFloatSqrt
        | HostFnId::MathFloatPow
        | HostFnId::MathFloatFloor
        | HostFnId::MathFloatCeil
        | HostFnId::MathFloatRound
        | HostFnId::MathFloatLog
        | HostFnId::MathFloatAsin
        | HostFnId::MathFloatAcos
        | HostFnId::MathFloatAtan
        | HostFnId::MathFloatAtan2
        | HostFnId::MathFloatExp
        | HostFnId::MathFloatExp2
        | HostFnId::MathFloatLog2
        | HostFnId::MathFloatLog10
        | HostFnId::MathFloatHypot
        | HostFnId::MathFloatCbrt
        | HostFnId::MathFloatTrunc
        | HostFnId::MathFloatAbs
        | HostFnId::MathFloatSinh
        | HostFnId::MathFloatCosh
        | HostFnId::MathFloatTanh
        | HostFnId::MathFloatFmod
        | HostFnId::MathFloatCopysign
        | HostFnId::MathFloatExpm1
        | HostFnId::MathFloatLog1p
        | HostFnId::MathFloatAsinh
        | HostFnId::MathFloatAcosh
        | HostFnId::MathFloatAtanh
        | HostFnId::MathFloatRemainder
        | HostFnId::MathFloatNextUp
        | HostFnId::MathFloatNextDown
        | HostFnId::MathFloatMulAdd
        | HostFnId::MathFloatErf
        | HostFnId::MathFloatErfc
        | HostFnId::MathFloatTgamma
        | HostFnId::MathFloatLgamma => "Float64",
        HostFnId::MathFloat32Sqrt
        | HostFnId::MathFloat32Sin
        | HostFnId::MathFloat32Cos
        | HostFnId::MathFloat32Tan
        | HostFnId::MathFloat32Exp
        | HostFnId::MathFloat32Log
        | HostFnId::MathFloat32Abs
        | HostFnId::MathFloat32Floor
        | HostFnId::MathFloat32Ceil
        | HostFnId::MathFloat32Round
        | HostFnId::MathFloat32Trunc
        | HostFnId::MathFloat32Pow
        | HostFnId::MathFloat32Asin
        | HostFnId::MathFloat32Acos
        | HostFnId::MathFloat32Atan
        | HostFnId::MathFloat32Hypot
        | HostFnId::MathFloat32Atan2
        | HostFnId::MathFloat32Log2
        | HostFnId::MathFloat32Log10
        | HostFnId::MathFloat32Exp2
        | HostFnId::MathFloat32Cbrt
        | HostFnId::MathFloat32Sinh
        | HostFnId::MathFloat32Cosh
        | HostFnId::MathFloat32Tanh
        | HostFnId::MathFloat32Fmod
        | HostFnId::MathFloat32Copysign
        | HostFnId::MathFloat32Expm1
        | HostFnId::MathFloat32Log1p
        | HostFnId::MathFloat32Asinh
        | HostFnId::MathFloat32Acosh
        | HostFnId::MathFloat32Atanh
        | HostFnId::MathFloat32Erf
        | HostFnId::MathFloat32Erfc
        | HostFnId::MathFloat32Tgamma
        | HostFnId::MathFloat32Lgamma
        | HostFnId::MathFloat32Remainder
        | HostFnId::MathFloat32NextUp
        | HostFnId::MathFloat32NextDown
        | HostFnId::MathFloat32MulAdd => "Float32",
        HostFnId::MathFloatIsNan
        | HostFnId::MathFloatIsInfinite
        | HostFnId::MathFloatIsFinite
        | HostFnId::MathFloat32IsNan
        | HostFnId::MathFloat32IsInfinite
        | HostFnId::MathFloat32IsFinite => "Bool",
        HostFnId::ProcessArgs => "Array",
        HostFnId::ProcessEnv => "String",
        HostFnId::ProcessExec => "Int",
        HostFnId::ProcessExecOutput => "String",
        HostFnId::ProcessSpawn => "Int",
        HostFnId::ProcessWait => "Int",
        HostFnId::ProcessExitCode => "Int",
        HostFnId::FileStat => "Object",
        HostFnId::FileWatch => "Array",
        HostFnId::FileSymlink | HostFnId::FileChmod => "Bool",
        HostFnId::FileReadLink => "String",
        HostFnId::UdpBind => "Int",
        HostFnId::UdpSendTo => "Int",
        HostFnId::UdpRecvFrom => "Object",
        HostFnId::DnsResolve => "Array",
        HostFnId::DnsResolveIp => "String",
        HostFnId::CryptoEd25519Sign => "String",
        HostFnId::CryptoEd25519Verify => "Bool",
        HostFnId::CryptoPbkdf2 => "String",
        HostFnId::MutexCreate => "Int",
        HostFnId::MutexLock | HostFnId::MutexUnlock => "Bool",
        HostFnId::WsRecv => "String",
        HostFnId::WsClose => "Void",
        HostFnId::ArraySortBy | HostFnId::ArrayFlatMap | HostFnId::ArrayChunked | HostFnId::ArrayZip => "Array",
        HostFnId::ArrayBinarySearch => "Int",
        HostFnId::LogDebug | HostFnId::LogInfo | HostFnId::LogWarn | HostFnId::LogError => "Nil",
        HostFnId::TestingAssert | HostFnId::TestingAssertEq | HostFnId::TestingTest | HostFnId::TestingExpectEqual => "Nil",
        HostFnId::StringBuilderNew => "Int",
        HostFnId::StringBuilderAppend => "Nil",
        HostFnId::StringBuilderToString => "String",
        HostFnId::BytesFromArray | HostFnId::BytesSlice | HostFnId::BytesConcat => "Array",
        HostFnId::ChannelClose => "Void",
        HostFnId::RegexCompile | HostFnId::RegexReplaceAll => "String",
        HostFnId::EncodingGzipCompress => "Array",
        HostFnId::EncodingGzipDecompress => "String",
        HostFnId::ReflectType => "Object",
        HostFnId::ReflectSpecialization => "Object",
        HostFnId::RegexFindAll => "Array",
        HostFnId::ValueDescription | HostFnId::ValueDebugDescription => "String",
        HostFnId::ValueCompare => "Int",
        HostFnId::EncodingZstdCompress => "Array",
        HostFnId::EncodingZstdDecompress => "String",
        HostFnId::CodableEncode => "String",
        HostFnId::CodableDecode => "Object",
        HostFnId::TestingRunAll => "Int",
        HostFnId::RuntimeCast => "Object",
        HostFnId::CodableEncodeL4 => "String",
        HostFnId::CodableDecodeL4 => "Object",
        HostFnId::JsonEncoderEncode => "String",
        HostFnId::JsonDecoderDecode => "Object",
        HostFnId::JsonStreamOpenWriter => "Int",
        HostFnId::JsonStreamWrite => "Void",
        HostFnId::JsonStreamFinishWriter => "String",
        HostFnId::JsonStreamOpenReader => "Int",
        HostFnId::JsonStreamFeed => "Void",
        HostFnId::JsonStreamDecodeNext => "Object",
        HostFnId::JsonStreamCloseReader => "Void",
        HostFnId::GrpcUnary => "String",
        HostFnId::GrpcStreamClient => "String",
        HostFnId::GrpcStreamServer => "Bool",
        HostFnId::KvoObserve => "Bool",
        HostFnId::SqlExec => "Int",
        HostFnId::SqlQuery => "Array",
        HostFnId::XmlParse => "Object",
        HostFnId::SqlPrepare => "Int",
        HostFnId::SqlOpen => "Int",
        HostFnId::SqlClose => "Void",
        HostFnId::SqlExecOn => "Int",
        HostFnId::SqlQueryOn => "Array",
        HostFnId::SqlPrepareOn => "Int",
        HostFnId::SqlBindStepOn => "Int",
        HostFnId::SqlBindPreparedOn => "Int",
        HostFnId::SqlBeginTransactionOn => "Void",
        HostFnId::SqlCommitTransactionOn => "Void",
        HostFnId::SqlRollbackTransactionOn => "Void",
        HostFnId::OrmSave => "Bool",
        HostFnId::OrmSaveOn => "Bool",
        HostFnId::OrmFindWhereOn => "Array",
        HostFnId::OrmUpdateWhereOn => "Int",
        HostFnId::OrmDeleteWhereOn => "Int",
        HostFnId::OrmFind => "Array",
        HostFnId::CodableSchemaRegister => "Bool",
        HostFnId::XmlXPath => "Array",
        HostFnId::GuiWindowCreate => "String",
        HostFnId::GuiEventPump => "String",
        HostFnId::GuiCanvasDraw => "Bool",
        HostFnId::SqlMigrate => "Int",
        HostFnId::SqlBindStep => "Int",
        HostFnId::OrmFindWhere => "Array",
        HostFnId::OrmUpdateWhere => "Int",
        HostFnId::OrmDeleteWhere => "Int",
        HostFnId::GuiWinitWindowCreate => "String",
        HostFnId::GuiWinitEventPump => "String",
        HostFnId::GuiWinitCanvasDraw => "Bool",
        HostFnId::GuiButtonCreate => "String",
        HostFnId::GuiLabelCreate => "String",
        HostFnId::GuiOnClick => "Bool",
        HostFnId::GuiLayoutCreate => "String",
        HostFnId::GuiLayoutConfigure => "String",
        HostFnId::GuiTextFieldCreate => "String",
        HostFnId::GuiRunLoopOnce => "String",
        HostFnId::GuiPixelRead => "Int",
        HostFnId::GuiClickInject => "Bool",
        HostFnId::GuiKeyInject => "Bool",
        HostFnId::GuiWidgetGetText => "String",
        HostFnId::GuiWidgetSetText => "Bool",
        HostFnId::SqlBeginTransaction => "Bool",
        HostFnId::SqlCommitTransaction => "Bool",
        HostFnId::CodableL14RegisterMigration => "Bool",
        HostFnId::CodableEncodeL14 => "Array",
        HostFnId::CodableDecodeL14 => "String",
        HostFnId::CodableL14RegisterRename => "Bool",
        HostFnId::SqlRollback => "Bool",
        HostFnId::SqlBindMulti => "Bool",
        HostFnId::GuiTouchEvent => "String",
        HostFnId::GuiMobileWindowCreate => "String",
        HostFnId::GuiMobileEventPump => "String",
        HostFnId::GuiMobileCanvasDraw => "Bool",
        HostFnId::GuiMobileRunLoopOnce => "String",
        HostFnId::GuiMobilePixelRead => "Int",
        HostFnId::GcChurnHeap => "String",
        HostFnId::ResultBuilderBuildBlock
        | HostFnId::ResultBuilderBuildEither
        | HostFnId::ResultBuilderBuildArray
        | HostFnId::ResultBuilderBuildOptional => "Array",
        HostFnId::WeakFrom | HostFnId::WeakLoad => "Object",
        HostFnId::WeakAlive => "Bool",
    }
}

pub fn method_from_index(id: u32) -> Option<MethodId> {
    use MethodId::*;
    match id {
        0 => Some(StrSplit),
        1 => Some(StrTrim),
        2 => Some(StrHasPrefix),
        3 => Some(StrLowercased),
        4 => Some(StrUppercased),
        5 => Some(StrReplacingOccurrences),
        6 => Some(StrCount),
        7 => Some(StrSubstring),
        8 => Some(StrMatches),
        9 => Some(ArrLen),
        10 => Some(ArrMap),
        11 => Some(ArrFilter),
        12 => Some(ArrReduce),
        13 => Some(ArrSorted),
        14 => Some(ArrContains),
        15 => Some(ArrReverse),
        16 => Some(ArrJoined),
        17 => Some(HttpResponseJson),
        18 => Some(SetInsert),
        19 => Some(SetRemove),
        20 => Some(SetContains),
        21 => Some(SetCount),
        22 => Some(SetUnion),
        23 => Some(SetIntersection),
        24 => Some(DictContainsKey),
        25 => Some(DictRemove),
        26 => Some(DictCount),
        27 => Some(DictKeys),
        28 => Some(DictValues),
        29 => Some(HttpResponseStatus),
        30 => Some(HttpResponseText),
        31 => Some(HttpResponseBody),
        32 => Some(HttpResponseOk),
        33 => Some(HttpResponseHeader),
        34 => Some(HttpResponseHeaders),
        35 => Some(SqlRowGetString),
        36 => Some(SqlRowGetInt),
        37 => Some(SqlRowGetFloat),
        38 => Some(SqlRowGetBlob),
        39 => Some(SqlRowIsNull),
        40 => Some(SqlRowColumnNames),
        41 => Some(ArrFlatMap),
        _ => None,
    }
}

/// Phase 55: map legacy `MethodId` dispatch to `CallHost` index (MethodCall retirement).
pub fn method_id_call_host_index(method: MethodId) -> Option<u32> {
    let name = match method {
        MethodId::StrSplit => "__syx_str_split",
        MethodId::StrTrim => "__syx_str_trim",
        MethodId::StrHasPrefix => "__syx_str_has_prefix",
        MethodId::StrLowercased => "__syx_str_lowercased",
        MethodId::StrUppercased => "__syx_str_uppercased",
        MethodId::StrMatches => "__syx_str_matches",
        MethodId::StrSubstring => "__syx_str_substring",
        MethodId::StrCount => "__syx_str_length",
        MethodId::ArrLen => "__syx_array_len",
        MethodId::ArrMap => "__syx_array_map",
        MethodId::ArrFilter => "__syx_array_filter",
        MethodId::ArrReduce => "__syx_array_reduce",
        // ArrSorted: MirOp::ArrSorted witness compare loop (Phase 68)
        MethodId::ArrContains => "__syx_array_contains",
        MethodId::ArrReverse => "__syx_array_reverse",
        MethodId::ArrJoined => "__syx_array_joined",
        MethodId::ArrFlatMap => "__syx_array_flat_map",
        MethodId::SetInsert => "__syx_set_insert",
        MethodId::SetContains => "__syx_set_contains",
        MethodId::SetCount => "__syx_set_count",
        MethodId::SetUnion => "__syx_set_union",
        MethodId::SetIntersection => "__syx_set_intersection",
        MethodId::SetRemove => "__syx_set_remove",
        MethodId::DictContainsKey => "__syx_dict_contains",
        MethodId::DictCount => "__syx_dict_count",
        MethodId::DictKeys => "__syx_dict_keys",
        MethodId::DictValues => "__syx_dict_values",
        MethodId::HttpResponseJson => "__syx_http_response_json",
        MethodId::HttpResponseStatus => "__syx_http_response_status",
        MethodId::HttpResponseText => "__syx_http_response_text",
        MethodId::HttpResponseBody => "__syx_http_response_body",
        MethodId::HttpResponseOk => "__syx_http_response_ok",
        MethodId::HttpResponseHeader => "__syx_http_response_header",
        MethodId::HttpResponseHeaders => "__syx_http_response_headers",
        MethodId::SqlRowGetString => "__syx_sql_row_get_string",
        MethodId::SqlRowGetInt => "__syx_sql_row_get_int",
        MethodId::SqlRowGetFloat => "__syx_sql_row_get_float",
        MethodId::SqlRowGetBlob => "__syx_sql_row_get_blob",
        MethodId::SqlRowIsNull => "__syx_sql_row_is_null",
        MethodId::SqlRowColumnNames => "__syx_sql_row_column_names",
        _ => return None,
    };
    host_fn_index_by_name(name)
}
