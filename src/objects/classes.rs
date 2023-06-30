// TODO: only allow specific warnings
#![allow(warnings)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PPtr {
    m_FileID: i64,
    m_PathID: i64,
}
impl PPtr {
    fn get_object_handler<'a, R: std::io::Read + std::io::Seek>(
        &'a self,
        asset: &'a crate::files::SerializedFile,
        reader: &'a mut R,
    ) -> std::io::Result<crate::files::ObjectHandler<R>> {
        match asset.m_Objects.iter().find(|x| x.m_PathID == self.m_PathID) {
            Some(objectinfo) => Ok(asset.get_object_handler(objectinfo, reader)),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Object not found",
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AABB {
    m_Center: Vector3f,
    m_Extent: Vector3f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ASTCImporter {
    m_Name: String,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddedGameObject {
    targetCorrespondingSourceObject: PPtr,
    insertIndex: i32,
    addedObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AimConstraint {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Weight: f32,
    m_RotationAtRest: Vector3f,
    m_RotationOffset: Vector3f,
    m_AimVector: Vector3f,
    m_UpVector: Vector3f,
    m_WorldUpVector: Vector3f,
    m_WorldUpObject: PPtr,
    m_UpType: i32,
    m_AffectRotationX: bool,
    m_AffectRotationY: bool,
    m_AffectRotationZ: bool,
    m_IsContraintActive: Option<bool>,
    m_Sources: Vec<ConstraintSource>,
    m_Active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnchoredJoint2D {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AndroidAssetPackImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UsedFileIDs: Vec<i64>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Animation {
//     m_GameObject: PPtr,
//     m_Enabled: u8,
//     m_Animation: PPtr,
//     m_Animations: Vec<PPtr>,
//     m_WrapMode: i32,
//     m_PlayAutomatically: bool,
//     m_AnimatePhysics: bool,
//     m_AnimateOnlyIfVisible: Option<bool>,
//     m_CullingType: Option<i32>,
//     m_UserAABB: Option<AABB>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct AnimationClip {
//     m_Name: String,
//     m_Compressed: bool,
//     m_RotationCurves: Vec<QuaternionCurve>,
//     m_CompressedRotationCurves: Vec<CompressedAnimationCurve>,
//     m_PositionCurves: Vec<Vector3Curve>,
//     m_ScaleCurves: Vec<Vector3Curve>,
//     m_FloatCurves: Vec<FloatCurve>,
//     m_SampleRate: f32,
//     m_WrapMode: i32,
//     m_Events: Vec<AnimationEvent>,
//     m_Bounds: Option<AABB>,
//     m_AnimationType: Option<i32>,
//     m_MuscleClipSize: Option<u32>,
//     m_MuscleClip: Option<ClipMuscleConstant>,
//     m_UseHighQualityCurve: Option<bool>,
//     m_PPtrCurves: Option<Vec<PPtrCurve>>,
//     m_ClipBindingConstant: Option<AnimationClipBindingConstant>,
//     m_Legacy: Option<bool>,
//     m_EulerCurves: Option<Vec<Vector3Curve>>,
//     m_HasGenericRootTransform: Option<bool>,
//     m_HasMotionFloatCurves: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimationClipBindingConstant {
    genericBindings: Vec<GenericBinding>,
    pptrCurveMapping: Vec<PPtr>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct AnimationClipOverride {
//     m_OriginalClip: PPtr,
//     m_OverrideClip: PPtr,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimationCurve {
    m_Curve: Vec<Keyframe>,
    m_PreInfinity: i32,
    m_PostInfinity: i32,
    m_RotationOrder: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimationEvent {
    time: f32,
    functionName: String,
    data: String,
    objectReferenceParameter: PPtr,
    floatParameter: f32,
    intParameter: i32,
    messageOptions: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimationManager {}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Animator {
//     m_GameObject: PPtr,
//     m_Enabled: u8,
//     m_Avatar: PPtr,
//     m_Controller: (
//         Option<PPtr>,
//         Option<PPtr>,
//     ),
//     m_CullingMode: i32,
//     m_ApplyRootMotion: bool,
//     m_AnimatePhysics: Option<bool>,
//     m_HasTransformHierarchy: Option<bool>,
//     m_UpdateMode: Option<i32>,
//     m_AllowConstantClipSamplingOptimization: Option<bool>,
//     m_LinearVelocityBlending: Option<bool>,
//     m_KeepAnimatorControllerStateOnDisable: Option<bool>,
//     m_StabilizeFeet: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimatorCondition {
    m_ConditionMode: i32,
    m_ConditionEvent: String,
    m_EventTreshold: f32,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct AnimatorController {
//     m_Name: String,
//     m_ControllerSize: u32,
//     m_Controller: ControllerConstant,
//     m_TOS: Vec<(u32, String)>,
//     m_AnimationClips: Vec<PPtr>,
//     m_StateMachineBehaviourVectorDescription: Option<StateMachineBehaviourVectorDescription>,
//     m_StateMachineBehaviours: Option<Vec<PPtr>>,
//     m_MultiThreadedStateMachine: Option<bool>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct AnimatorOverrideController {
//     m_Name: String,
//     m_Controller: PPtr,
//     m_Clips: Vec<AnimationClipOverride>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimatorTransition {
    m_Name: String,
    m_Conditions: Vec<AnimatorCondition>,
    m_DstStateMachine: PPtr,
    m_DstState: PPtr,
    m_Solo: bool,
    m_Mute: bool,
    m_IsExit: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimatorTransitionBase {
    m_Name: String,
    m_Conditions: Vec<AnimatorCondition>,
    m_DstStateMachine: PPtr,
    m_DstState: PPtr,
    m_Solo: bool,
    m_Mute: bool,
    m_IsExit: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Annotation {
    m_IconEnabled: bool,
    m_GizmoEnabled: bool,
    m_ClassID: i32,
    m_ScriptClass: String,
    m_Flags: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnnotationManager {
    m_CurrentPreset_m_AnnotationList: Vec<Annotation>,
    m_RecentlyChanged: Vec<Annotation>,
    m_IconSize: Option<f32>,
    m_WorldIconSize: Option<f32>,
    m_Use3dGizmos: Option<bool>,
    m_ShowGrid: Option<bool>,
    m_ShowSelectionOutline: Option<bool>,
    m_ShowSelectionWire: Option<bool>,
    m_FadeGizmoSize: Option<f32>,
    m_FadeGizmos: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AreaEffector2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_ColliderMask: BitField,
    m_ForceDirection: Option<f32>,
    m_ForceMagnitude: f32,
    m_ForceVariation: f32,
    m_Drag: f32,
    m_AngularDrag: f32,
    m_ForceTarget: (Option<u8>, Option<i32>),
    m_UseColliderMask: Option<bool>,
    m_UseGlobalAngle: Option<bool>,
    m_ForceAngle: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticulationBody {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Mass: f32,
    m_ParentAnchorPosition: Vector3f,
    m_ParentAnchorRotation: Quaternionf,
    m_AnchorPosition: Vector3f,
    m_AnchorRotation: Quaternionf,
    m_ComputeParentAnchor: Option<bool>,
    m_ArticulationJointType: i32,
    m_LinearX: i32,
    m_LinearY: i32,
    m_LinearZ: i32,
    m_SwingY: i32,
    m_SwingZ: i32,
    m_Twist: i32,
    m_XDrive: ArticulationDrive,
    m_YDrive: ArticulationDrive,
    m_ZDrive: ArticulationDrive,
    m_LinearDamping: f32,
    m_AngularDamping: f32,
    m_JointFriction: f32,
    m_Immovable: bool,
    m_UseGravity: Option<bool>,
    m_CollisionDetectionMode: Option<i32>,
    m_MatchAnchors: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticulationDrive {
    lowerLimit: f32,
    upperLimit: f32,
    stiffness: f32,
    damping: f32,
    forceLimit: f32,
    target: f32,
    targetVelocity: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AspectRatios {
    __4x3: bool,
    __5x4: bool,
    __16x10: bool,
    __16x9: bool,
    Others: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssemblyDefinitionReferenceAsset {
    m_Name: String,
    m_Script: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssemblyDefinitionReferenceImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UsedFileIDs: Vec<i64>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssemblyJsonAsset {
    m_Name: String,
    m_Script: String,
    m_PathName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssemblyJsonImporter {
    m_Name: String,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    mainRepresentation: LibraryRepresentation,
    representations: Vec<LibraryRepresentation>,
    children: Vec<GUID>,
    parent: GUID,
    digest: Option<MdFour>,
    __type: i32,
    labels: AssetLabels,
    modificationDate: Option<(u32, u32)>,
    metaModificationDate: Option<(u32, u32)>,
    importerClassId: Option<i32>,
    importerVersionHash: Option<u32>,
    hash: Option<(Option<MdFour>, Option<Hash128>)>,
    assetBundleIndex: Option<i32>,
    scriptedImporterClassID: Option<String>,
    hashOfImportedAssetDependencies: Option<Vec<GUID>>,
    hashOfSourceAssetDependencies: Option<Vec<GUID>>,
    guidOfPathLocationDependencies:
        Option<(Option<Vec<(String, GUID)>>, Option<Vec<(GUID, String)>>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetBundle {
    m_Name: String,
    m_PreloadTable: Vec<PPtr>,
    m_Container: Vec<(String, AssetInfo)>,
    m_MainAsset: AssetInfo,
    m_ScriptCompatibility: Option<Vec<AssetBundleScriptInfo>>,
    m_ClassCompatibility: Option<Vec<(i32, u32)>>,
    m_RuntimeCompatibility: Option<u32>,
    m_AssetBundleName: Option<String>,
    m_Dependencies: Option<Vec<String>>,
    m_IsStreamedSceneAssetBundle: Option<bool>,
    m_ClassVersionMap: Option<Vec<(i32, i32)>>,
    m_PathFlags: Option<i32>,
    m_ExplicitDataLayout: Option<i32>,
    m_SceneHashes: Option<Vec<(String, String)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetBundleFullName {
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetBundleInfo {
    AssetBundleHash: Hash128,
    AssetBundleDependencies: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetBundleManifest {
    m_Name: String,
    AssetBundleNames: Vec<(i32, String)>,
    AssetBundlesWithVariant: Vec<i32>,
    AssetBundleInfos: Vec<(i32, AssetBundleInfo)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetBundleScriptInfo {
    className: String,
    nameSpace: String,
    assemblyName: String,
    hash: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetDatabase {
    m_Assets: Vec<(GUID, Asset)>,
    m_AssetTimeStamps: Option<Vec<(String, AssetTimeStamp)>>,
    m_UnityShadersVersion: Option<i32>,
    m_lastValidVersionHashes: Option<Vec<(i32, u32)>>,
    m_AssetBundleNames: Option<Vec<(i32, AssetBundleFullName)>>,
    m_Metrics: Option<AssetDatabaseMetrics>,
    m_lastValidVersions: Option<Vec<(AssetImporterHashKey, u32)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetDatabaseMetrics {
    totalAssetCount: i32,
    nonProAssetCount: Option<i32>,
    nonProAssetsCreatedAfterProLicense: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetImportInProgressProxy {
    m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetImporter {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetImporterHashKey {
    __type: i32,
    ScriptClass: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetImporterLog {
    m_Name: String,
    m_Logs: Vec<AssetImporter_ImportError>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetImporter_ImportError {
    error: String,
    mode: i32,
    line: i32,
    file: String,
    object: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetInfo {
    preloadIndex: i32,
    preloadSize: i32,
    asset: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetLabels {
    m_Labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetMetaData {
    guid: GUID,
    pathName: String,
    originalChangeset: Option<u32>,
    originalName: String,
    originalParent: Option<GUID>,
    originalDigest: Option<(Option<MdFour>, Option<Hash128>)>,
    labels: Vec<String>,
    assetStoreRef: u64,
    timeCreated: Option<u64>,
    licenseType: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetServerCache {
    m_ObjectHideFlags: Option<u32>,
    m_Items: Vec<(GUID, Item)>,
    m_DeletedItems: Vec<(GUID, DeletedItem)>,
    m_LastCommitMessage: String,
    m_CommitItemSelection: Vec<GUID>,
    m_WorkingItemMetaData: Vec<(GUID, CachedAssetMetaData)>,
    m_LatestServerChangeset: i32,
    m_CachesInitialized: i32,
    m_ModifiedItems: Vec<(GUID, Item)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetTimeStamp {
    modificationDate: (u32, u32),
    metaModificationDate: (u32, u32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioBehaviour {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioBuildInfo {
    m_IsAudioDisabled: bool,
    m_AudioClipCount: i32,
    m_AudioMixerCount: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioChorusFilter {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_DryMix: f32,
    m_WetMix1: f32,
    m_WetMix2: f32,
    m_WetMix3: f32,
    m_Delay: f32,
    m_Rate: f32,
    m_Depth: f32,
    m_FeedBack: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioClip {
    m_Name: String,
    m_Format: Option<i32>,
    m_Type: Option<i32>,
    m_3D: Option<bool>,
    m_UseHardware: Option<bool>,
    m_AudioData: Option<Vec<u8>>,
    m_Stream: Option<(Option<bool>, Option<i32>)>,
    m_LoadType: Option<i32>,
    m_Channels: Option<i32>,
    m_Frequency: Option<i32>,
    m_BitsPerSample: Option<i32>,
    m_Length: Option<f32>,
    m_IsTrackerFormat: Option<bool>,
    m_SubsoundIndex: Option<i32>,
    m_PreloadAudioData: Option<bool>,
    m_LoadInBackground: Option<bool>,
    m_Legacy3D: Option<bool>,
    m_Resource: Option<StreamedResource>,
    m_CompressionFormat: Option<i32>,
    m_Ambisonic: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioDistortionFilter {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_DistortionLevel: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioEchoFilter {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Delay: u32,
    m_DecayRatio: f32,
    m_WetMix: f32,
    m_DryMix: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioFilter {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioHighPassFilter {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_CutoffFrequency: f32,
    m_HighpassResonanceQ: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_Format: Option<i32>,
    m_Quality: Option<f32>,
    m_Stream: Option<i32>,
    m_PreviewDataLength: Option<u32>,
    audio_preview_data: Option<Vec<u8>>,
    m_3D: bool,
    m_ForceToMono: bool,
    m_UseHardware: Option<bool>,
    m_Loopable: Option<bool>,
    m_Output: Option<(Option<AudioImporterOutput>, Option<Output>)>,
    m_UserData: Option<String>,
    m_DefaultSettings: Option<SampleSettings>,
    m_PlatformSettingOverrides: Option<Vec<(i32, SampleSettings)>>,
    m_PreloadAudioData: Option<bool>,
    m_LoadInBackground: Option<bool>,
    m_PreviewData: Option<PreviewData>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_Normalize: Option<bool>,
    m_Ambisonic: Option<bool>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioImporterOutput {
    outputSettings: SampleSettings,
    outputContainerFormat: i32,
    editorOutputSettings: SampleSettings,
    editorOutputContainerFormat: i32,
    playerResource: Option<StreamedResource>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioListener {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_ExtensionPropertyValues: Option<Vec<ExtensionPropertyValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioLowPassFilter {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_CutoffFrequency: Option<f32>,
    m_LowpassResonanceQ: f32,
    lowpassLevelCustomCurve: AnimationCurve,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioManager {
    m_Volume: f32,
    m_SpeedOfSound: Option<f32>,
    Doppler_Factor: f32,
    Default_Speaker_Mode: i32,
    Rolloff_Scale: f32,
    iOS_DSP_Buffer_Size: Option<i32>,
    m_DSPBufferSize: Option<i32>,
    m_DisableAudio: Option<bool>,
    m_SampleRate: Option<i32>,
    m_VirtualVoiceCount: Option<i32>,
    m_RealVoiceCount: Option<i32>,
    m_SpatializerPlugin: Option<String>,
    m_VirtualizeEffects: Option<bool>,
    m_AmbisonicDecoderPlugin: Option<String>,
    m_RequestedDSPBufferSize: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixer {
    m_Name: String,
    m_OutputGroup: PPtr,
    m_MasterGroup: PPtr,
    m_Snapshots: Vec<PPtr>,
    m_StartSnapshot: PPtr,
    m_SuspendThreshold: f32,
    m_EnableSuspend: bool,
    m_MixerConstant: AudioMixerConstant,
    m_UpdateMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerConstant {
    groups: Vec<GroupConstant>,
    groupGUIDs: Vec<GUID>,
    effects: Vec<EffectConstant>,
    effectGUIDs: Vec<GUID>,
    numSideChainBuffers: u32,
    snapshots: Vec<SnapshotConstant>,
    snapshotGUIDs: Vec<GUID>,
    groupNameBuffer: Vec<char>,
    snapshotNameBuffer: Vec<char>,
    pluginEffectNameBuffer: Vec<char>,
    exposedParameterNames: Vec<u32>,
    exposedParameterIndices: Vec<u32>,
    groupConnections: Option<Vec<GroupConnection>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerController {
    m_Name: String,
    m_OutputGroup: PPtr,
    m_MasterGroup: PPtr,
    m_Snapshots: Vec<PPtr>,
    m_StartSnapshot: PPtr,
    m_SuspendThreshold: f32,
    m_EnableSuspend: bool,
    m_MixerConstant: AudioMixerConstant,
    m_UpdateMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerEffectController {
    m_Name: String,
    m_EffectID: GUID,
    m_EffectName: String,
    m_MixLevel: GUID,
    m_Parameters: Vec<Parameter>,
    m_SendTarget: PPtr,
    m_EnableWetMix: bool,
    m_Bypass: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerGroup {
    m_Name: String,
    m_AudioMixer: PPtr,
    m_GroupID: GUID,
    m_Children: Vec<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerGroupController {
    m_Name: String,
    m_AudioMixer: PPtr,
    m_GroupID: GUID,
    m_Children: Vec<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerLiveUpdateBool {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerLiveUpdateFloat {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerSnapshot {
    m_Name: String,
    m_AudioMixer: PPtr,
    m_SnapshotID: GUID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerSnapshotController {
    m_Name: String,
    m_AudioMixer: PPtr,
    m_SnapshotID: GUID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioReverbFilter {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_DryLevel: f32,
    m_Room: f32,
    m_RoomHF: f32,
    m_RoomRolloff: Option<f32>,
    m_DecayTime: f32,
    m_DecayHFRatio: f32,
    m_ReflectionsLevel: f32,
    m_ReverbLevel: f32,
    m_ReverbDelay: f32,
    m_Diffusion: f32,
    m_Density: f32,
    m_HFReference: f32,
    m_RoomLF: f32,
    m_LFReference: f32,
    m_ReflectionsDelay: f32,
    m_ReverbPreset: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioReverbZone {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_MinDistance: f32,
    m_MaxDistance: f32,
    m_ReverbPreset: i32,
    m_Room: i32,
    m_RoomHF: i32,
    m_DecayTime: f32,
    m_DecayHFRatio: f32,
    m_Reflections: i32,
    m_ReflectionsDelay: f32,
    m_Reverb: i32,
    m_ReverbDelay: f32,
    m_HFReference: f32,
    m_RoomRolloffFactor: Option<f32>,
    m_Diffusion: f32,
    m_Density: f32,
    m_LFReference: f32,
    m_RoomLF: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioSource {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_audioClip: PPtr,
    m_PlayOnAwake: bool,
    m_Volume: f32,
    m_Pitch: f32,
    Loop: bool,
    Mute: bool,
    Priority: i32,
    DopplerLevel: f32,
    MinDistance: f32,
    MaxDistance: f32,
    Pan2D: f32,
    rolloffMode: i32,
    BypassEffects: bool,
    rolloffCustomCurve: AnimationCurve,
    panLevelCustomCurve: AnimationCurve,
    spreadCustomCurve: AnimationCurve,
    BypassListenerEffects: Option<bool>,
    BypassReverbZones: Option<bool>,
    OutputAudioMixerGroup: Option<PPtr>,
    reverbZoneMixCustomCurve: Option<AnimationCurve>,
    Spatialize: Option<bool>,
    SpatializePostEffects: Option<bool>,
    m_ExtensionPropertyValues: Option<Vec<ExtensionPropertyValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoOffMeshLinkData {
    m_Start: Vector3f,
    m_End: Vector3f,
    m_Radius: f32,
    m_LinkType: u16,
    m_Area: u8,
    m_LinkDirection: u8,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Avatar {
//     m_Name: String,
//     m_AvatarSize: u32,
//     m_Avatar: AvatarConstant,
//     m_TOS: Vec<(u32, String)>,
//     m_HumanDescription: Option<HumanDescription>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvatarBodyMask {
    m_Name: String,
    m_Mask: Vec<u32>,
    m_Elements: Option<Vec<TransformMaskElement>>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct AvatarConstant {
//     m_Skeleton: Option<OffsetPtr>,
//     m_SkeletonPose: Option<OffsetPtr>,
//     m_Human: OffsetPtr,
//     m_HumanSkeletonIndexArray: Vec<i32>,
//     m_RootMotionBoneIndex: i32,
//     m_RootMotionBoneX: xform,
//     m_AvatarSkeleton: Option<OffsetPtr>,
//     m_AvatarSkeletonPose: Option<OffsetPtr>,
//     m_DefaultPose: Option<OffsetPtr>,
//     m_SkeletonNameIDArray: Option<Vec<u32>>,
//     m_HumanSkeletonReverseIndexArray: Option<Vec<i32>>,
//     m_RootMotionSkeleton: Option<OffsetPtr>,
//     m_RootMotionSkeletonPose: Option<OffsetPtr>,
//     m_RootMotionSkeletonIndexArray: Option<Vec<i32>>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvatarMask {
    m_Name: String,
    m_Mask: Vec<u32>,
    m_Elements: Vec<TransformMaskElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvatarSkeletonMask {
    m_Name: String,
    elements: Vec<AvatarSkeletonMaskElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvatarSkeletonMaskElement {
    path: String,
    weight: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Axes {
    m_PreQ: float4,
    m_PostQ: float4,
    m_Sgn: (Option<float3>, Option<float4>),
    m_Limit: Limit,
    m_Length: f32,
    m_Type: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseAnimationTrack {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseVideoTexture {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Behaviour {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BillboardAsset {
    m_Name: String,
    width: f32,
    bottom: f32,
    height: f32,
    imageTexCoords: Vec<Vector4f>,
    rotated: Option<Vec<u8>>,
    vertices: Vec<Vector2f>,
    indices: Vec<u16>,
    material: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BillboardRenderer {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: u8,
    m_ReceiveShadows: (Option<u8>, Option<bool>),
    m_LightmapIndex: u16,
    m_LightmapIndexDynamic: u16,
    m_LightmapTilingOffset: Vector4f,
    m_LightmapTilingOffsetDynamic: Vector4f,
    m_Materials: Vec<PPtr>,
    m_SubsetIndices: Option<Vec<u32>>,
    m_StaticBatchRoot: PPtr,
    m_UseLightProbes: Option<bool>,
    m_ReflectionProbeUsage: (Option<u8>, Option<i32>),
    m_ProbeAnchor: PPtr,
    m_SortingLayerID: Option<i32>,
    m_SortingOrder: i16,
    m_Billboard: PPtr,
    m_MotionVectors: Option<u8>,
    m_LightProbeUsage: Option<u8>,
    m_LightProbeVolumeOverride: Option<PPtr>,
    m_StaticBatchInfo: Option<StaticBatchInfo>,
    m_SortingLayer: Option<i16>,
    m_DynamicOccludee: Option<u8>,
    m_RenderingLayerMask: Option<u32>,
    m_RendererPriority: Option<i32>,
    m_RayTracingMode: Option<u8>,
    m_RayTraceProcedural: Option<u8>,
    m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BitField {
    m_Bits: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blend1dDataConstant {
    m_ChildThresholdArray: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blend2dDataConstant {
    m_ChildThresholdArray: Option<Vec<f32>>,
    m_ChildPositionArray: Option<Vec<Vector2f>>,
    m_ChildMagnitudeArray: Option<Vec<f32>>,
    m_ChildPairVectorArray: Option<Vec<Vector2f>>,
    m_ChildPairAvgMagInvArray: Option<Vec<f32>>,
    m_ChildNeighborListArray: Option<Vec<MotionNeighborList>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlendDirectDataConstant {
    m_ChildBlendEventIDArray: Vec<u32>,
    m_NormalizedBlendValues: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlendShapeData {
    vertices: Vec<BlendShapeVertex>,
    shapes: Vec<MeshBlendShape>,
    channels: Vec<MeshBlendShapeChannel>,
    fullWeights: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlendShapeVertex {
    vertex: Vector3f,
    normal: Vector3f,
    tangent: Vector3f,
    index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlendTree {
    m_Name: String,
    m_Childs: (Option<Vec<ChildMotion>>, Option<Vec<Child>>),
    m_BlendEvent: Option<String>,
    m_MinThreshold: f32,
    m_MaxThreshold: f32,
    m_UseAutomaticThresholds: bool,
    m_BlendEventY: Option<String>,
    m_BlendType: Option<(Option<u32>, Option<i32>)>,
    m_BlendParameter: Option<String>,
    m_BlendParameterY: Option<String>,
    m_NormalizedBlendValues: Option<bool>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct BlendTreeConstant {
//     m_NodeArray: Vec<OffsetPtr>,
//     m_BlendEventArrayConstant: Option<OffsetPtr>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct BlendTreeNodeConstant {
//     m_BlendEventID: u32,
//     m_ChildIndices: Vec<u32>,
//     m_ChildThresholdArray: Option<Vec<f32>>,
//     m_ClipID: u32,
//     m_Duration: f32,
//     m_BlendType: Option<u32>,
//     m_BlendEventYID: Option<u32>,
//     m_Blend1dData: Option<OffsetPtr>,
//     m_Blend2dData: Option<OffsetPtr>,
//     m_CycleOffset: Option<f32>,
//     m_Mirror: Option<bool>,
//     m_ClipIndex: Option<u32>,
//     m_BlendDirectData: Option<OffsetPtr>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoneInfluence {
    weight: (f32, f32, f32, f32),
    boneIndex: (i32, i32, i32, i32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoneWeights4 {
    weight: (f32, f32, f32, f32),
    boneIndex: (i32, i32, i32, i32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoxCollider {
    m_GameObject: PPtr,
    m_Material: PPtr,
    m_IsTrigger: bool,
    m_Size: Vector3f,
    m_Center: Vector3f,
    m_Enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoxCollider2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Material: PPtr,
    m_IsTrigger: bool,
    m_Size: Vector2f,
    m_Center: Option<Vector2f>,
    m_UsedByEffector: Option<bool>,
    m_Offset: Option<Vector2f>,
    m_Density: Option<f32>,
    m_UsedByComposite: Option<bool>,
    m_AutoTiling: Option<bool>,
    m_SpriteTilingProperty: Option<SpriteTilingProperty>,
    m_EdgeRadius: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BufferBinding {
    m_NameIndex: i32,
    m_Index: i32,
    m_ArraySize: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildReport {
    m_Name: String,
    m_Summary: BuildSummary,
    m_Files: Vec<BuildReportFile>,
    m_BuildSteps: Vec<BuildStepInfo>,
    m_Appendices: Vec<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildReportFile {
    path: String,
    role: String,
    id: u32,
    totalSize: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildReportPackedAssetInfo {
    fileID: i64,
    classID: i32,
    packedSize: (Option<u64>, Option<i32>),
    sourceAssetGUID: GUID,
    buildTimeAssetPath: Option<String>,
    offset: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildReportScenesUsingAsset {
    assetPath: String,
    scenePaths: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildSettings {
    levels: Option<Vec<String>>,
    hasRenderTexture: Option<bool>,
    hasPROVersion: bool,
    hasPublishingRights: bool,
    hasShadows: bool,
    hasAdvancedVersion: bool,
    enableDynamicBatching: bool,
    isDebugBuild: bool,
    m_Version: String,
    m_AuthToken: String,
    isEducationalBuild: Option<bool>,
    isNoWatermarkBuild: Option<bool>,
    isPrototypingBuild: Option<bool>,
    runtimeClassHashes: Option<(Option<Vec<(i32, Hash128)>>, Option<Vec<(i32, u32)>>)>,
    isEmbedded: Option<bool>,
    usesOnMouseEvents: Option<bool>,
    hasSoftShadows: Option<bool>,
    hasLocalLightShadows: Option<bool>,
    hasOculusPlugin: Option<bool>,
    preloadedPlugins: Option<Vec<String>>,
    enableMultipleDisplays: Option<bool>,
    hasClusterRendering: Option<bool>,
    scriptHashes: Option<Vec<(Hash128, Hash128)>>,
    scenes: Option<Vec<String>>,
    m_GraphicsAPIs: Option<Vec<i32>>,
    enabledVRDevices: Option<Vec<String>>,
    buildTags: Option<Vec<String>>,
    buildGUID: Option<(Option<String>, Option<GUID>)>,
    isWsaHolographicRemotingEnabled: Option<bool>,
    isTrial: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildStepInfo {
    stepName: String,
    duration: Option<u64>,
    messages: Vec<BuildStepMessage>,
    durationTicks: Option<u64>,
    depth: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildStepMessage {
    __type: i32,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildSummary {
    name: Option<String>,
    platformName: String,
    options: i32,
    assetBundleOptions: i32,
    outputPath: String,
    crc: u32,
    totalSize: u64,
    totalTimeMS: Option<u64>,
    totalErrors: i32,
    totalWarnings: i32,
    buildType: Option<i32>,
    success: Option<bool>,
    platformGroupName: Option<String>,
    buildGUID: Option<GUID>,
    buildResult: Option<i32>,
    buildStartTime: Option<DateTime>,
    totalTimeTicks: Option<u64>,
    subtarget: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildTargetSettings {
    m_BuildTarget: String,
    m_MaxTextureSize: Option<i32>,
    m_TextureFormat: i32,
    m_CompressionQuality: Option<i32>,
    m_AllowsAlphaSplitting: Option<bool>,
    m_TextureWidth: Option<i32>,
    m_TextureHeight: Option<i32>,
    m_LoadingBehavior: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildTextureStackReference {
    groupName: String,
    itemName: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuiltAssetBundleInfo {
    bundleName: String,
    bundleArchiveFile: u32,
    packagedFileIndices: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuiltAssetBundleInfoSet {
    bundleInfos: Option<Vec<BuiltAssetBundleInfo>>,
    m_Name: Option<String>,
    m_UseLegacyImporter: Option<bool>,
    m_Quality: Option<f32>,
    m_IsColorLinear: Option<bool>,
    m_OriginalWidth: Option<i32>,
    m_OriginalHeight: Option<i32>,
    m_FrameRange: Option<i32>,
    m_StartFrame: Option<i32>,
    m_EndFrame: Option<i32>,
    m_FrameCount: Option<i32>,
    m_FrameRate: Option<f64>,
    m_ColorSpace: Option<i32>,
    m_Deinterlace: Option<i32>,
    m_SourceFileSize: Option<u64>,
    m_EncodeAlpha: Option<bool>,
    m_SourceHasAlpha: Option<bool>,
    m_FlipVertical: Option<bool>,
    m_FlipHorizontal: Option<bool>,
    m_AudioImportMode: Option<i32>,
    m_SourceAudioChannelCount: Option<Vec<u16>>,
    m_SourceAudioSampleRate: Option<Vec<u32>>,
    m_TargetSettings: Option<Vec<(i32, VideoClipImporterTargetSettings)>>,
    m_Output: Option<VideoClipImporterOutput>,
    m_UserData: Option<String>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_ImportAudio: Option<bool>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_PixelAspectRatioNumerator: Option<u32>,
    m_PixelAspectRatioDenominator: Option<u32>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuiltinShaderSettings {
    m_Mode: i32,
    m_Shader: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuoyancyEffector2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_UseColliderMask: bool,
    m_ColliderMask: BitField,
    m_SurfaceLevel: f32,
    m_Density: f32,
    m_LinearDrag: f32,
    m_AngularDrag: f32,
    m_FlowAngle: f32,
    m_FlowMagnitude: f32,
    m_FlowVariation: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CGProgram {
    m_Name: String,
    m_Script: Option<String>,
    m_PathName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachedAssetMetaData {
    guid: GUID,
    pathName: String,
    originalChangeset: u32,
    originalName: String,
    originalParent: GUID,
    originalDigest: (Option<MdFour>, Option<Hash128>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachedSpriteAtlas {
    textures: Vec<PPtr>,
    frames: (
        Option<Vec<((GUID, i64), SpriteRenderData)>>,
        Option<Vec<((GUID, i32), SpriteRenderData)>>,
    ),
    alphaTextures: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachedSpriteAtlasRuntimeData {
    textures: Vec<PPtr>,
    alphaTextures: Vec<PPtr>,
    frames: Vec<((GUID, i64), SpriteAtlasData)>,
    currentPackingHash: Option<Hash128>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Camera {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_ClearFlags: u32,
    m_BackGroundColor: ColorRGBA,
    m_NormalizedViewPortRect: Rectf,
    near_clip_plane: f32,
    far_clip_plane: f32,
    field_of_view: f32,
    orthographic: bool,
    orthographic_size: f32,
    m_Depth: f32,
    m_CullingMask: BitField,
    m_RenderingPath: i32,
    m_TargetTexture: PPtr,
    m_HDR: Option<bool>,
    m_OcclusionCulling: Option<bool>,
    m_TargetDisplay: Option<(Option<u32>, Option<i32>)>,
    m_StereoConvergence: Option<f32>,
    m_StereoSeparation: Option<f32>,
    m_StereoMirrorMode: Option<bool>,
    m_TargetEye: Option<i32>,
    m_AllowMSAA: Option<bool>,
    m_ForceIntoRT: Option<bool>,
    m_AllowDynamicResolution: Option<bool>,
    m_projectionMatrixMode: Option<i32>,
    m_SensorSize: Option<Vector2f>,
    m_LensShift: Option<Vector2f>,
    m_FocalLength: Option<f32>,
    m_GateFitMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Canvas {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Alpha: Option<f32>,
    m_RenderMode: i32,
    m_Camera: PPtr,
    m_Normals: Option<bool>,
    m_PositionUVs: Option<bool>,
    m_PixelPerfect: bool,
    m_PlaneDistance: Option<f32>,
    m_ReceivesEvents: Option<bool>,
    m_OverrideSorting: Option<bool>,
    m_OverridePixelPerfect: Option<bool>,
    m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    m_SortingOrder: Option<i16>,
    m_TargetDisplay: Option<i8>,
    m_SortingBucketNormalizedSize: Option<f32>,
    m_AdditionalShaderChannelsFlag: Option<i32>,
    m_UpdateRectTransformForStandalone: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CanvasGroup {
    m_GameObject: PPtr,
    m_Alpha: f32,
    m_Interactable: bool,
    m_BlocksRaycasts: bool,
    m_IgnoreParentGroups: bool,
    m_Enabled: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapsuleCollider {
    m_GameObject: PPtr,
    m_Material: PPtr,
    m_IsTrigger: bool,
    m_Radius: f32,
    m_Height: f32,
    m_Direction: i32,
    m_Center: Vector3f,
    m_Enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapsuleCollider2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Density: f32,
    m_Material: PPtr,
    m_IsTrigger: bool,
    m_UsedByEffector: bool,
    m_Offset: Vector2f,
    m_Size: Vector2f,
    m_Direction: i32,
    m_UsedByComposite: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    byteOffset: i32,
    curve: AnimationCurve,
    attributeName: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelInfo {
    stream: u8,
    offset: u8,
    format: u8,
    dimension: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterController {
    m_GameObject: PPtr,
    m_Height: f32,
    m_Radius: f32,
    m_SlopeLimit: f32,
    m_StepOffset: f32,
    m_SkinWidth: f32,
    m_MinMoveDistance: f32,
    m_Center: Vector3f,
    m_Material: Option<PPtr>,
    m_IsTrigger: Option<bool>,
    m_Enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterInfo {
    index: u32,
    uv: Rectf,
    vert: Rectf,
    width: Option<f32>,
    flipped: Option<bool>,
    advance: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterJoint {
    m_GameObject: PPtr,
    m_ConnectedBody: PPtr,
    m_Anchor: Vector3f,
    m_Axis: Vector3f,
    m_SwingAxis: Vector3f,
    m_LowTwistLimit: SoftJointLimit,
    m_HighTwistLimit: SoftJointLimit,
    m_Swing1Limit: SoftJointLimit,
    m_Swing2Limit: SoftJointLimit,
    m_BreakForce: f32,
    m_BreakTorque: f32,
    m_AutoConfigureConnectedAnchor: Option<bool>,
    m_ConnectedAnchor: Option<Vector3f>,
    m_EnableCollision: Option<bool>,
    m_TwistLimitSpring: Option<SoftJointLimitSpring>,
    m_SwingLimitSpring: Option<SoftJointLimitSpring>,
    m_EnableProjection: Option<bool>,
    m_ProjectionDistance: Option<f32>,
    m_ProjectionAngle: Option<f32>,
    m_EnablePreprocessing: Option<bool>,
    m_MassScale: Option<f32>,
    m_ConnectedMassScale: Option<f32>,
    m_Enabled: Option<bool>,
    m_ConnectedArticulationBody: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Child {
    m_Motion: PPtr,
    m_Threshold: f32,
    m_TimeScale: f32,
    m_IsAnim: bool,
    m_Position: Option<Vector2f>,
    m_CycleOffset: Option<f32>,
    m_Mirror: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChildAnimatorState {
    m_State: PPtr,
    m_Position: Vector3f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChildAnimatorStateMachine {
    m_StateMachine: PPtr,
    m_Position: Vector3f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChildMotion {
    m_Motion: PPtr,
    m_Threshold: f32,
    m_Position: Vector2f,
    m_TimeScale: f32,
    m_CycleOffset: f32,
    m_DirectBlendParameter: String,
    m_Mirror: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CircleCollider2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Material: PPtr,
    m_IsTrigger: bool,
    m_Radius: f32,
    m_Center: Option<Vector2f>,
    m_UsedByEffector: Option<bool>,
    m_Offset: Option<Vector2f>,
    m_Density: Option<f32>,
    m_UsedByComposite: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClampVelocityModule {
    enabled: bool,
    x: MinMaxCurve,
    y: MinMaxCurve,
    z: MinMaxCurve,
    magnitude: MinMaxCurve,
    separateAxis: bool,
    dampen: f32,
    inWorldSpace: Option<bool>,
    multiplyDragByParticleSize: Option<bool>,
    multiplyDragByParticleVelocity: Option<bool>,
    drag: Option<MinMaxCurve>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassInfo {
    m_AssemblyNameIndex: i32,
    m_NamespaceIndex: i32,
    m_ClassName: String,
    m_NumOfMethods: i32,
    m_MethodIndex: i32,
    m_IsUnityClass: bool,
    m_NamespaceName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassMethodInfo {
    m_ClassIndex: i32,
    m_MethodName: String,
    m_OrderNumber: i32,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Clip {
//     m_StreamedClip: StreamedClip,
//     m_DenseClip: DenseClip,
//     m_Binding: Option<OffsetPtr>,
//     m_ConstantClip: Option<ConstantClip>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClipAnimationInfo {
    name: String,
    firstFrame: (Option<f32>, Option<i32>),
    lastFrame: (Option<f32>, Option<i32>),
    wrapMode: i32,
    __loop: bool,
    takeName: Option<String>,
    orientationOffsetY: Option<f32>,
    level: Option<f32>,
    cycleOffset: Option<f32>,
    loopBlend: Option<bool>,
    loopBlendOrientation: Option<bool>,
    loopBlendPositionY: Option<bool>,
    loopBlendPositionXZ: Option<bool>,
    keepOriginalOrientation: Option<bool>,
    keepOriginalPositionY: Option<bool>,
    keepOriginalPositionXZ: Option<bool>,
    heightFromFeet: Option<bool>,
    mirror: Option<bool>,
    keepAdditionalBonesAnimation: Option<bool>,
    bodyMask: Option<Vec<u32>>,
    curves: Option<Vec<ClipAnimationInfoCurve>>,
    skeletonMaskElements: Option<Vec<AvatarSkeletonMaskElement>>,
    transformMask: Option<Vec<TransformMaskElement>>,
    loopTime: Option<bool>,
    events: Option<Vec<AnimationEvent>>,
    maskType: Option<i32>,
    maskSource: Option<PPtr>,
    hasAdditiveReferencePose: Option<bool>,
    additiveReferencePoseFrame: Option<f32>,
    internalID: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClipAnimationInfoCurve {
    name: String,
    curve: AnimationCurve,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct ClipMuscleConstant {
//     m_DeltaPose: HumanPose,
//     m_StartX: xform,
//     m_LeftFootStartX: xform,
//     m_RightFootStartX: xform,
//     m_MotionStartX: Option<xform>,
//     m_MotionStopX: Option<xform>,
//     m_AverageSpeed: (Option<float3>, Option<float4>),
//     m_Clip: OffsetPtr,
//     m_StartTime: f32,
//     m_StopTime: f32,
//     m_OrientationOffsetY: f32,
//     m_Level: f32,
//     m_CycleOffset: f32,
//     m_AverageAngularSpeed: f32,
//     m_IndexArray: Vec<i32>,
//     m_AdditionalCurveIndexArray: Option<Vec<i32>>,
//     m_ValueArrayDelta: Vec<ValueDelta>,
//     m_Mirror: bool,
//     m_LoopBlend: bool,
//     m_LoopBlendOrientation: bool,
//     m_LoopBlendPositionY: bool,
//     m_LoopBlendPositionXZ: bool,
//     m_KeepOriginalOrientation: bool,
//     m_KeepOriginalPositionY: bool,
//     m_KeepOriginalPositionXZ: bool,
//     m_HeightFromFeet: bool,
//     m_LoopTime: Option<bool>,
//     m_ValueArrayReferencePose: Option<Vec<f32>>,
//     m_StopX: Option<xform>,
//     m_StartAtOrigin: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cloth {
    m_GameObject: PPtr,
    m_Enabled: Option<u8>,
    m_StretchingStiffness: Option<f32>,
    m_BendingStiffness: Option<f32>,
    m_UseTethers: Option<bool>,
    m_UseGravity: Option<bool>,
    m_Damping: Option<f32>,
    m_ExternalAcceleration: Option<Vector3f>,
    m_RandomAcceleration: Option<Vector3f>,
    m_WorldVelocityScale: Option<f32>,
    m_WorldAccelerationScale: Option<f32>,
    m_Friction: Option<f32>,
    m_CollisionMassScale: Option<f32>,
    m_UseContinuousCollision: Option<bool>,
    m_UseVirtualParticles: Option<bool>,
    m_SolverFrequency: Option<(Option<f32>, Option<u32>)>,
    m_SleepThreshold: Option<f32>,
    m_Coefficients: Option<Vec<ClothConstrainCoefficients>>,
    m_CapsuleColliders: Option<Vec<PPtr>>,
    m_SphereColliders: Option<(
        Option<Vec<ClothSphereColliderPair>>,
        Option<Vec<(PPtr, PPtr)>>,
    )>,
    m_SelfCollisionDistance: Option<f32>,
    m_SelfCollisionStiffness: Option<f32>,
    m_SelfAndInterCollisionIndices: Option<Vec<u32>>,
    m_VirtualParticleWeights: Option<Vec<Vector3f>>,
    m_VirtualParticleIndices: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClothAttachment {
    m_Collider: PPtr,
    m_TwoWayInteraction: bool,
    m_Tearable: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClothConstrainCoefficients {
    maxDistance: f32,
    maxDistanceBias: Option<f32>,
    collisionSphereRadius: Option<f32>,
    collisionSphereDistance: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClothRenderer {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: bool,
    m_ReceiveShadows: bool,
    m_LightmapIndex: u8,
    m_LightmapTilingOffset: Vector4f,
    m_Materials: Vec<PPtr>,
    m_SubsetIndices: Vec<u32>,
    m_StaticBatchRoot: PPtr,
    m_PauseWhenNotVisible: bool,
    m_UseLightProbes: Option<bool>,
    m_LightProbeAnchor: Option<PPtr>,
    m_SortingLayer: Option<i16>,
    m_SortingOrder: Option<i16>,
    m_SortingLayerID: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClothSphereColliderPair {
    first: PPtr,
    second: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudServiceHandlerBehaviour {
    m_GameObject: PPtr,
    m_Enabled: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudWebServicesManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClusterInput {
    m_Name: String,
    m_DeviceName: String,
    m_ServerUrl: String,
    m_Index: i32,
    m_Type: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClusterInputManager {
    m_Inputs: Vec<ClusterInput>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollabEditorSettings {
    inProgressEnabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collider {
    m_GameObject: Option<PPtr>,
    m_X: Option<xform>,
    m_Type: Option<u32>,
    m_XMotionType: Option<u32>,
    m_YMotionType: Option<u32>,
    m_ZMotionType: Option<u32>,
    m_MinLimitX: Option<f32>,
    m_MaxLimitX: Option<f32>,
    m_MaxLimitY: Option<f32>,
    m_MaxLimitZ: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collider2D {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collision {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collision2D {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollisionModule {
    enabled: bool,
    __type: i32,
    plane0: Option<PPtr>,
    plane1: Option<PPtr>,
    plane2: Option<PPtr>,
    plane3: Option<PPtr>,
    plane4: Option<PPtr>,
    plane5: Option<PPtr>,
    dampen: Option<f32>,
    bounce: Option<f32>,
    energyLossOnCollision: Option<f32>,
    minKillSpeed: f32,
    particleRadius: Option<f32>,
    collidesWith: Option<BitField>,
    quality: Option<i32>,
    voxelSize: Option<f32>,
    collisionMessages: Option<bool>,
    collisionMode: Option<i32>,
    m_Dampen: Option<MinMaxCurve>,
    m_Bounce: Option<MinMaxCurve>,
    m_EnergyLossOnCollision: Option<MinMaxCurve>,
    radiusScale: Option<f32>,
    maxCollisionShapes: Option<i32>,
    collidesWithDynamic: Option<bool>,
    interiorCollisions: Option<bool>,
    maxKillSpeed: Option<f32>,
    colliderForce: Option<f32>,
    multiplyColliderForceByParticleSize: Option<bool>,
    multiplyColliderForceByParticleSpeed: Option<bool>,
    multiplyColliderForceByCollisionAngle: Option<bool>,
    m_Planes: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorBySpeedModule {
    enabled: bool,
    gradient: MinMaxGradient,
    range: Vector2f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorModule {
    enabled: bool,
    gradient: MinMaxGradient,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorRGBA {
    rgba: Option<u32>,
    r: Option<f32>,
    g: Option<f32>,
    b: Option<f32>,
    a: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Component {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComponentPair {
    component: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompositeCollider2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Density: f32,
    m_Material: PPtr,
    m_IsTrigger: bool,
    m_UsedByEffector: bool,
    m_UsedByComposite: bool,
    m_Offset: Vector2f,
    m_GeometryType: i32,
    m_GenerationType: i32,
    m_ColliderPaths: Vec<SubCollider>,
    m_CompositePaths: Polygon2D,
    m_VertexDistance: f32,
    m_EdgeRadius: Option<f32>,
    m_OffsetDistance: Option<f32>,
    m_UseDelaunayMesh: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompressedAnimationCurve {
    m_Path: String,
    m_Times: PackedBitVector,
    m_Values: PackedBitVector,
    m_Slopes: PackedBitVector,
    m_PreInfinity: i32,
    m_PostInfinity: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompressedMesh {
    m_Vertices: PackedBitVector,
    m_UV: PackedBitVector,
    m_BindPoses: Option<PackedBitVector>,
    m_Normals: PackedBitVector,
    m_Tangents: PackedBitVector,
    m_Weights: PackedBitVector,
    m_NormalSigns: PackedBitVector,
    m_TangentSigns: PackedBitVector,
    m_BoneIndices: PackedBitVector,
    m_Triangles: PackedBitVector,
    m_Colors: Option<PackedBitVector>,
    m_FloatColors: Option<PackedBitVector>,
    m_UVInfo: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeBufferCounter {
    bindpoint: i32,
    offset: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShader {
    m_Name: String,
    kernels: Option<Vec<ComputeShaderKernel>>,
    constantBuffers: Option<Vec<ComputeShaderCB>>,
    variants: Option<(
        Option<Vec<ComputeShaderPlatformVariant>>,
        Option<Vec<ComputeShaderVariant>>,
    )>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderBuiltinSampler {
    sampler: (Option<u32>, Option<i32>),
    bindPoint: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderCB {
    name: (Option<String>, Option<FastPropertyName>),
    byteSize: i32,
    params: Vec<ComputeShaderParam>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderImporter {
    m_Name: String,
    m_UserData: String,
    m_CurrentBuildTarget: Option<i32>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_CurrentAPIMask: Option<u32>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_UsedFileIDs: Option<Vec<i64>>,
    m_PreprocessorOverride: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderKernel {
    name: Option<(Option<String>, Option<FastPropertyName>)>,
    cbs: Vec<ComputeShaderResource>,
    textures: Vec<ComputeShaderResource>,
    builtinSamplers: Vec<ComputeShaderBuiltinSampler>,
    inBuffers: Vec<ComputeShaderResource>,
    outBuffers: Vec<ComputeShaderResource>,
    code: Vec<u8>,
    threadGroupSize: Option<Vec<u32>>,
    cbVariantIndices: Option<Vec<u32>>,
    requirements: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderKernelParent {
    name: String,
    variantMap: Option<Vec<(String, ComputeShaderKernel)>>,
    validKeywords: Option<Vec<String>>,
    globalKeywords: Option<Vec<String>>,
    localKeywords: Option<Vec<String>>,
    uniqueVariants: Option<Vec<ComputeShaderKernel>>,
    variantIndices: Option<Vec<(String, u32)>>,
    dynamicKeywords: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderParam {
    name: (Option<String>, Option<FastPropertyName>),
    __type: i32,
    offset: (Option<u32>, Option<i32>),
    arraySize: (Option<u32>, Option<i32>),
    rowCount: (Option<u32>, Option<i32>),
    colCount: (Option<u32>, Option<i32>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderPlatformVariant {
    targetRenderer: i32,
    targetLevel: i32,
    kernels: Vec<ComputeShaderKernelParent>,
    constantBuffers: Vec<ComputeShaderCB>,
    resourcesResolved: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderResource {
    name: (Option<String>, Option<FastPropertyName>),
    bindPoint: i32,
    generatedName: Option<(Option<String>, Option<FastPropertyName>)>,
    secondaryBindPoint: Option<i32>,
    counter: Option<ComputeBufferCounter>,
    samplerBindPoint: Option<i32>,
    texDimension: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderVariant {
    targetRenderer: i32,
    targetLevel: i32,
    kernels: Vec<ComputeShaderKernel>,
    constantBuffers: Vec<ComputeShaderCB>,
    resourcesResolved: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Condition {
    m_ConditionMode: i32,
    m_ConditionEvent: String,
    m_EventTreshold: f32,
    m_ExitTime: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConditionConstant {
    m_ConditionMode: u32,
    m_EventID: u32,
    m_EventThreshold: f32,
    m_ExitTime: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigSetting {
    value: String,
    flags: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigurableJoint {
    m_GameObject: PPtr,
    m_ConnectedBody: PPtr,
    m_Anchor: Vector3f,
    m_Axis: Vector3f,
    m_SecondaryAxis: Vector3f,
    m_XMotion: i32,
    m_YMotion: i32,
    m_ZMotion: i32,
    m_AngularXMotion: i32,
    m_AngularYMotion: i32,
    m_AngularZMotion: i32,
    m_LinearLimit: SoftJointLimit,
    m_LowAngularXLimit: SoftJointLimit,
    m_HighAngularXLimit: SoftJointLimit,
    m_AngularYLimit: SoftJointLimit,
    m_AngularZLimit: SoftJointLimit,
    m_TargetPosition: Vector3f,
    m_TargetVelocity: Vector3f,
    m_XDrive: JointDrive,
    m_YDrive: JointDrive,
    m_ZDrive: JointDrive,
    m_TargetRotation: Quaternionf,
    m_TargetAngularVelocity: Vector3f,
    m_RotationDriveMode: i32,
    m_AngularXDrive: JointDrive,
    m_AngularYZDrive: JointDrive,
    m_SlerpDrive: JointDrive,
    m_ProjectionMode: i32,
    m_ProjectionDistance: f32,
    m_ProjectionAngle: f32,
    m_ConfiguredInWorldSpace: bool,
    m_BreakForce: f32,
    m_BreakTorque: f32,
    m_SwapBodies: Option<bool>,
    m_AutoConfigureConnectedAnchor: Option<bool>,
    m_ConnectedAnchor: Option<Vector3f>,
    m_EnableCollision: Option<bool>,
    m_LinearLimitSpring: Option<SoftJointLimitSpring>,
    m_AngularXLimitSpring: Option<SoftJointLimitSpring>,
    m_AngularYZLimitSpring: Option<SoftJointLimitSpring>,
    m_EnablePreprocessing: Option<bool>,
    m_MassScale: Option<f32>,
    m_ConnectedMassScale: Option<f32>,
    m_Enabled: Option<bool>,
    m_ConnectedArticulationBody: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstantBuffer {
    m_NameIndex: i32,
    m_MatrixParams: Vec<MatrixParameter>,
    m_VectorParams: Vec<VectorParameter>,
    m_Size: i32,
    m_StructParams: Option<Vec<StructParameter>>,
    m_IsPartialCB: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstantClip {
    data: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstantForce {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Force: Vector3f,
    m_RelativeForce: Vector3f,
    m_Torque: Vector3f,
    m_RelativeTorque: Vector3f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstantForce2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Force: Vector2f,
    m_RelativeForce: Vector2f,
    m_Torque: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstraintSource {
    sourceTransform: PPtr,
    weight: f32,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct ControllerConstant {
//     m_HumanLayerArray: Option<Vec<OffsetPtr>>,
//     m_StateMachineArray: Vec<OffsetPtr>,
//     m_Values: OffsetPtr,
//     m_DefaultValues: OffsetPtr,
//     m_LayerArray: Option<Vec<OffsetPtr>>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrashReportManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrashReportingSettings {
    m_EventUrl: String,
    m_Enabled: bool,
    m_NativeEventUrl: Option<String>,
    m_LogBufferSize: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cubemap {
    m_Name: String,
    m_Width: i32,
    m_Height: i32,
    m_CompleteImageSize: (Option<u32>, Option<i32>),
    m_TextureFormat: i32,
    m_MipMap: Option<bool>,
    m_IsReadable: bool,
    m_ReadAllowed: Option<bool>,
    m_ImageCount: i32,
    m_TextureDimension: i32,
    m_TextureSettings: GLTextureSettings,
    m_LightmapFormat: i32,
    image_data: Vec<u8>,
    m_ColorSpace: Option<i32>,
    m_SourceTextures: Option<Vec<PPtr>>,
    m_MipCount: Option<i32>,
    m_StreamData: Option<StreamingInfo>,
    m_ForcedFallbackFormat: Option<i32>,
    m_DownscaleFallback: Option<bool>,
    m_StreamingMipmaps: Option<bool>,
    m_StreamingMipmapsPriority: Option<i32>,
    m_IgnoreMasterTextureLimit: Option<bool>,
    m_IsPreProcessed: Option<bool>,
    m_MipsStripped: Option<i32>,
    m_IsAlphaChannelOptional: Option<bool>,
    m_PlatformBlob: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CubemapArray {
    m_Name: String,
    m_Width: i32,
    m_CubemapCount: i32,
    m_Format: i32,
    m_MipCount: i32,
    m_DataSize: u32,
    m_TextureSettings: GLTextureSettings,
    m_ColorSpace: i32,
    m_IsReadable: bool,
    image_data: Vec<u8>,
    m_StreamData: Option<StreamingInfo>,
    m_ForcedFallbackFormat: Option<i32>,
    m_DownscaleFallback: Option<bool>,
    m_IsAlphaChannelOptional: Option<bool>,
    m_UsageMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomCollider2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Density: f32,
    m_Material: PPtr,
    m_IsTrigger: bool,
    m_UsedByEffector: bool,
    m_UsedByComposite: bool,
    m_Offset: Vector2f,
    m_CustomShapes: PhysicsShapeGroup2D,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomDataModule {
    enabled: bool,
    mode0: i32,
    vectorComponentCount0: i32,
    color0: MinMaxGradient,
    vector0_0: MinMaxCurve,
    vector0_1: MinMaxCurve,
    vector0_2: MinMaxCurve,
    vector0_3: MinMaxCurve,
    mode1: i32,
    vectorComponentCount1: i32,
    color1: MinMaxGradient,
    vector1_0: MinMaxCurve,
    vector1_1: MinMaxCurve,
    vector1_2: MinMaxCurve,
    vector1_3: MinMaxCurve,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomRenderTexture {
    m_Name: String,
    m_Width: i32,
    m_Height: i32,
    m_AntiAliasing: i32,
    m_DepthFormat: Option<i32>,
    m_ColorFormat: i32,
    m_MipMap: bool,
    m_GenerateMips: bool,
    m_SRGB: bool,
    m_TextureSettings: GLTextureSettings,
    m_Dimension: i32,
    m_VolumeDepth: i32,
    m_Material: PPtr,
    m_InitMaterial: PPtr,
    m_InitColor: ColorRGBA,
    m_InitTexture: PPtr,
    m_UpdateMode: i32,
    m_InitializationMode: i32,
    m_UpdateZoneSpace: i32,
    m_CurrentUpdateZoneSpace: i32,
    m_UpdateZones: Vec<UpdateZoneInfo>,
    m_UpdatePeriod: f32,
    m_ShaderPass: u32,
    m_CubemapFaceMask: u32,
    m_DoubleBuffered: bool,
    m_WrapUpdateZones: bool,
    m_InitSource: Option<i32>,
    m_ForcedFallbackFormat: Option<i32>,
    m_DownscaleFallback: Option<bool>,
    m_UseDynamicScale: Option<bool>,
    m_BindMS: Option<bool>,
    m_EnableCompatibleFormat: Option<bool>,
    m_MipCount: Option<i32>,
    m_IsAlphaChannelOptional: Option<bool>,
    m_DepthStencilFormat: Option<i32>,
    m_ShadowSamplingMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DDSImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_UserData: Option<String>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_IsReadable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataTemplate {
    m_LastMergeIdentifier: Option<GUID>,
    m_Objects: Option<Vec<PPtr>>,
    m_Father: Option<PPtr>,
    m_IsDataTemplate: Option<bool>,
    m_ObjectHideFlags: Option<u32>,
    m_ExtensionPtr: Option<PPtr>,
    m_Name: Option<String>,
    m_Modification: Option<PrefabModification>,
    m_ParentPrefab: Option<PPtr>,
    m_RootGameObject: Option<PPtr>,
    m_IsPrefabParent: Option<bool>,
    m_IsExploded: Option<bool>,
    m_SourcePrefab: Option<PPtr>,
    m_IsPrefabAsset: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateTime {
    ticks: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultAsset {
    m_ObjectHideFlags: Option<u32>,
    m_ExtensionPtr: Option<PPtr>,
    m_Name: String,
    m_Message: Option<String>,
    m_IsWarning: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_UserData: Option<String>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultPreset {
    m_Preset: PPtr,
    m_Filter: Option<String>,
    m_Disabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultPresetList {
    __type: PresetType,
    defaultPresets: Vec<DefaultPreset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DelayedCallManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedItem {
    changeset: i32,
    guid: GUID,
    parent: GUID,
    fullPath: String,
    __type: i32,
    digest: (Option<MdFour>, Option<Hash128>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DenseClip {
    m_FrameCount: i32,
    m_CurveCount: u32,
    m_SampleRate: f32,
    m_BeginTime: f32,
    m_SampleArray: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Derived {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailDatabase {
    m_Patches: Vec<DetailPatch>,
    m_DetailPrototypes: Vec<DetailPrototype>,
    m_PatchCount: i32,
    m_PatchSamples: i32,
    m_RandomRotations: Option<Vec<Vector3f>>,
    WavingGrassTint: ColorRGBA,
    m_WavingGrassStrength: f32,
    m_WavingGrassAmount: f32,
    m_WavingGrassSpeed: f32,
    m_TreeInstances: Vec<TreeInstance>,
    m_TreePrototypes: Vec<TreePrototype>,
    m_PreloadTextureAtlasData: Vec<PPtr>,
    m_DetailBillboardShader: Option<PPtr>,
    m_DetailMeshLitShader: Option<PPtr>,
    m_DetailMeshGrassShader: Option<PPtr>,
    m_DefaultShaders: Option<(PPtr, PPtr, PPtr)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailPatch {
    bounds: Option<AABB>,
    layerIndices: Vec<u8>,
    numberOfObjects: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailPrototype {
    prototype: PPtr,
    prototypeTexture: PPtr,
    minWidth: f32,
    maxWidth: f32,
    minHeight: f32,
    maxHeight: f32,
    noiseSpread: f32,
    bendFactor: Option<f32>,
    healthyColor: ColorRGBA,
    dryColor: ColorRGBA,
    lightmapFactor: Option<f32>,
    renderMode: i32,
    usePrototypeMesh: i32,
    holeTestRadius: Option<f32>,
    noiseSeed: Option<i32>,
    useInstancing: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceNone {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DirectorGenericBinding {
    key: PPtr,
    value: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DirectorPlayer {
    m_GameObject: PPtr,
    m_Enabled: Option<u8>,
    m_PlayableAsset: Option<PPtr>,
    m_InitialState: Option<i32>,
    m_WrapMode: Option<i32>,
    m_DirectorUpdateMode: Option<i32>,
    m_InitialTime: Option<f64>,
    m_SceneBindings: Option<Vec<DirectorGenericBinding>>,
    m_ExposedReferences: Option<ExposedReferenceTable>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DistanceJoint2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_CollideConnected: Option<bool>,
    m_ConnectedRigidBody: PPtr,
    m_Anchor: Vector2f,
    m_ConnectedAnchor: Vector2f,
    m_Distance: f32,
    m_MaxDistanceOnly: Option<bool>,
    m_EnableCollision: Option<bool>,
    m_BreakForce: Option<f32>,
    m_BreakTorque: Option<f32>,
    m_AutoConfigureConnectedAnchor: Option<bool>,
    m_AutoConfigureDistance: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EdgeCollider2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Material: PPtr,
    m_IsTrigger: bool,
    m_Points: Vec<Vector2f>,
    m_UsedByEffector: Option<bool>,
    m_Offset: Option<Vector2f>,
    m_Density: Option<f32>,
    m_UsedByComposite: Option<bool>,
    m_EdgeRadius: Option<f32>,
    m_AdjacentStartPoint: Option<Vector2f>,
    m_AdjacentEndPoint: Option<Vector2f>,
    m_UseAdjacentStartPoint: Option<bool>,
    m_UseAdjacentEndPoint: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorBuildSettings {
    m_ObjectHideFlags: Option<u32>,
    m_Scenes: Vec<Scene>,
    m_BuildLocation: Option<Vec<String>>,
    m_ActiveBuildTarget: Option<i32>,
    m_Development: Option<bool>,
    m_configObjects: Option<Vec<(String, PPtr)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorExtension {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorExtensionImpl {
    m_LastTemplateIdentifier: Option<GUID>,
    m_Object: Option<PPtr>,
    m_TemplateFather: Option<PPtr>,
    m_DataTemplate: Option<PPtr>,
    m_LastTemplateFather: Option<PPtr>,
    m_OverrideVariable: Option<bitset>,
    gFlattenedTypeTree: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorProjectAccess {
    m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorSettings {
    m_ObjectHideFlags: Option<u32>,
    m_ExternalVersionControlSupport: Option<(Option<String>, Option<i32>)>,
    m_WebSecurityEmulationEnabled: Option<i32>,
    m_WebSecurityEmulationHostUrl: Option<String>,
    m_SerializationMode: Option<i32>,
    m_DefaultBehaviorMode: Option<i32>,
    m_SpritePackerMode: Option<i32>,
    m_SpritePackerPaddingPower: Option<i32>,
    m_ProjectGenerationIncludedExtensions: Option<String>,
    m_ProjectGenerationRootNamespace: Option<String>,
    m_UserGeneratedProjectSuffix: Option<String>,
    m_CollabEditorSettings: Option<CollabEditorSettings>,
    m_EtcTextureCompressorBehavior: Option<i32>,
    m_EtcTextureFastCompressor: Option<i32>,
    m_EtcTextureNormalCompressor: Option<i32>,
    m_EtcTextureBestCompressor: Option<i32>,
    m_LineEndingsForNewScripts: Option<i32>,
    m_EnableTextureStreamingInPlayMode: Option<bool>,
    m_PrefabRegularEnvironment: Option<PPtr>,
    m_PrefabUIEnvironment: Option<PPtr>,
    m_EnableTextureStreamingInEditMode: Option<bool>,
    m_AsyncShaderCompilation: Option<bool>,
    m_ShowLightmapResolutionOverlay: Option<bool>,
    m_EnterPlayModeOptionsEnabled: Option<bool>,
    m_EnterPlayModeOptions: Option<i32>,
    m_UseLegacyProbeSampleCount: Option<bool>,
    m_AssetPipelineMode: Option<i32>,
    m_CacheServerMode: Option<i32>,
    m_CacheServerEndpoint: Option<String>,
    m_CacheServerNamespacePrefix: Option<String>,
    m_CacheServerEnableDownload: Option<bool>,
    m_CacheServerEnableUpload: Option<bool>,
    m_SerializeInlineMappingsOnOneLine: Option<bool>,
    m_CacheServerEnableAuth: Option<bool>,
    m_CacheServerEnableTls: Option<bool>,
    m_CachingShaderPreprocessor: Option<bool>,
    m_DisableCookiesInLightmapper: Option<bool>,
    m_GameObjectNamingDigits: Option<i32>,
    m_GameObjectNamingScheme: Option<i32>,
    m_AssetNamingUsesSpace: Option<bool>,
    m_EnableRoslynAnalyzers: Option<bool>,
    m_PrefabModeAllowAutoSave: Option<bool>,
    m_CacheServerValidationMode: Option<i32>,
    m_Bc7TextureCompressor: Option<i32>,
    m_RefreshImportMode: Option<i32>,
    m_SpritePackerCacheSize: Option<i32>,
    m_EnableEditorAsyncCPUTextureLoading: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorUserBuildSettings {
    m_BuildLocation: Vec<String>,
    m_ActiveBuildTarget: i32,
    m_SelectedBuildTargetGroup: i32,
    m_SelectedStandaloneTarget: i32,
    m_ArchitectureFlags: i32,
    m_SelectedWiiSubtarget: Option<i32>,
    m_Development: bool,
    m_ConnectProfiler: bool,
    m_AllowDebugging: bool,
    m_WebPlayerStreamed: Option<bool>,
    m_WebPlayerOfflineDeployment: Option<bool>,
    m_WebPlayerNaClSupport: Option<bool>,
    m_InstallInBuildFolder: bool,
    m_SymlinkLibraries: Option<bool>,
    m_SelectedAndroidSubtarget: i32,
    m_SelectedWiiDebugLevel: Option<i32>,
    m_SelectedXboxSubtarget: Option<i32>,
    m_SelectedXboxRunMethod: Option<i32>,
    m_ExplicitNullChecks: Option<bool>,
    m_XboxCompressedXex: Option<bool>,
    m_WebPlayerDeployOnline: Option<bool>,
    m_SelectedPS3Subtarget: Option<i32>,
    m_SelectedMetroBuildType: Option<i32>,
    m_EnableHeadlessMode: Option<bool>,
    m_SelectedMetroTarget: Option<i32>,
    m_SelectedBlackBerrySubtarget: Option<i32>,
    m_SelectedBlackBerryBuildType: Option<i32>,
    m_SelectedMetroSDK: Option<i32>,
    m_SelectedPSP2Subtarget: Option<i32>,
    m_GenerateMetroReferenceProjects: Option<bool>,
    m_SelectedTizenSubtarget: Option<i32>,
    m_SelectedMetroBuildAndRunDeployTarget: Option<i32>,
    m_SelectedPS4Subtarget: Option<i32>,
    m_SelectedPSMSubtarget: Option<i32>,
    m_NeedSubmissionMaterials: Option<bool>,
    m_CompressWithPsArc: Option<bool>,
    m_BuildScriptsOnly: Option<bool>,
    m_WebGLOptimizationLevel: Option<i32>,
    m_SelectedWSASDK: Option<i32>,
    m_SelectedWSABuildAndRunDeployTarget: Option<i32>,
    m_GenerateWSAReferenceProjects: Option<bool>,
    m_XboxOneStreamingInstallLaunchChunkRange: Option<i32>,
    m_SelectedXboxOneDeployMethod: Option<i32>,
    m_SymlinkTrampoline: Option<bool>,
    m_SelectedIOSBuildType: Option<i32>,
    m_SelectedWiiUDebugLevel: Option<i32>,
    m_SelectedWiiUBuildOutput: Option<i32>,
    m_SelectedWiiUBootMode: Option<i32>,
    m_WiiUEnableNetAPI: Option<bool>,
    m_SelectedWSAUWPBuildType: Option<i32>,
    m_ForceOptimizeScriptCompilation: Option<bool>,
    m_WSADotNetNativeEnabled: Option<Vec<bool>>,
    m_XboxOneUsername: Option<String>,
    m_XboxOneNetworkSharePath: Option<String>,
    m_PS4HardwareTarget: Option<i32>,
    m_WebGLUsePreBuiltUnityEngine: Option<bool>,
    m_ExplicitDivideByZeroChecks: Option<bool>,
    m_PlatformSettings: Option<Vec<(String, PlatformSettingsData)>>,
    m_AndroidBuildSystem: Option<i32>,
    m_ExportAsGoogleAndroidProject: Option<bool>,
    m_SelectedWSASubtarget: Option<i32>,
    m_CompressFilesInPackage: Option<bool>,
    m_SelectedWSAUWPSDK: Option<String>,
    m_ActiveBuildTargetGroup: Option<i32>,
    m_SelectedFacebookTarget: Option<i32>,
    m_CreateSolutionFileForSwitch: Option<bool>,
    m_CreateRomFileForSwitch: Option<bool>,
    m_NVNGraphicsDebuggerForSwitch: Option<bool>,
    m_FacebookCreatePackageForSubmission: Option<bool>,
    m_AndroidBuildType: Option<i32>,
    m_FacebookAccessToken: Option<String>,
    m_EnableDebugPadForSwitch: Option<bool>,
    m_RedirectWritesToHostMountForSwitch: Option<bool>,
    m_ForceInstallation: Option<bool>,
    m_AndroidReleaseMinification: Option<i32>,
    m_AndroidDebugMinification: Option<i32>,
    m_AndroidDeviceSocketAddress: Option<String>,
    m_SelectedCompressionType: Option<Vec<(String, i32)>>,
    m_SelectedAndroidETC2Fallback: Option<i32>,
    m_SelectedWSAUWPVSVersion: Option<String>,
    m_BuildAppBundle: Option<bool>,
    m_AndroidCurrentDeploymentTargetId: Option<String>,
    m_MovePackageToDiscOuterEdge: Option<bool>,
    m_ExplicitArrayBoundsChecks: Option<bool>,
    m_DatalessPlayer: Option<bool>,
    m_WsaHolographicRemoting: Option<bool>,
    m_SelectedXboxOneDeployDrive: Option<i32>,
    m_NVNShaderDebugging: Option<bool>,
    m_NVNDrawValidation: Option<bool>,
    m_EnableHeapInspectorForSwitch: Option<bool>,
    m_AndroidUseLegacySdkTools: Option<bool>,
    m_SelectedWSAMinUWPSDK: Option<String>,
    m_SelectedWSAArchitecture: Option<String>,
    m_AndroidCreateSymbolsZip: Option<bool>,
    m_GenerateNintendoSwitchShaderInfo: Option<bool>,
    m_WaitForPlayerConnection: Option<bool>,
    m_WindowsDevicePortalAddress: Option<String>,
    m_WindowsDevicePortalUsername: Option<String>,
    m_BuildWithDeepProfilingSupport: Option<bool>,
    m_PS5KeepPackageFiles: Option<bool>,
    m_SelectedPS5Subtarget: Option<i32>,
    m_SelectedPS5CompressionType: Option<i32>,
    m_SelectedPS5CompressionLevel: Option<i32>,
    m_PS5WorkspaceName: Option<String>,
    m_UseLegacyNvnPoolAllocatorForSwitch: Option<bool>,
    m_EnableRomCompressionForSwitch: Option<bool>,
    m_SaveADFForSwitch: Option<bool>,
    m_RomCompressionTypeForSwitch: Option<i32>,
    m_RomCompressionLevelForSwitch: Option<i32>,
    m_RomCompressionConfigForSwitch: Option<String>,
    m_NVNDrawValidationLight: Option<bool>,
    m_NVNDrawValidationHeavy: Option<bool>,
    m_HTCSScriptDebuggingForSwitch: Option<bool>,
    m_AndroidCreateSymbols: Option<i32>,
    m_SymlinkSources: Option<bool>,
    m_OverrideMaxTextureSize: Option<i32>,
    m_OverrideTextureCompression: Option<i32>,
    m_macosXcodeBuildConfig: Option<i32>,
    m_Il2CppCodeGeneration: Option<i32>,
    m_ActiveBuildPlatformGroupName: Option<String>,
    m_SelectedBuildPlatformGroupName: Option<String>,
    m_SelectedEmbeddedLinuxArchitecture: Option<i32>,
    m_SelectedStandaloneBuildSubtarget: Option<i32>,
    m_ActiveStandaloneBuildSubtarget: Option<i32>,
    m_SelectedWebGLSubtarget: Option<i32>,
    m_RemoteDeviceInfo: Option<bool>,
    m_RemoteDeviceAddress: Option<String>,
    m_RemoteDeviceUsername: Option<String>,
    m_RemoteDeviceExports: Option<String>,
    m_PathOnRemoteDevice: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorUserSettings {
    m_VCUserName: Option<String>,
    m_VCPassword: Option<String>,
    m_VCServer: Option<String>,
    m_VCWorkspace: Option<String>,
    m_VCAutomaticAdd: bool,
    m_VCDebugCom: bool,
    m_VCDebugCmd: bool,
    m_VCDebugOut: bool,
    m_ConfigValues: Option<Vec<(String, String)>>,
    m_SemanticMergeMode: Option<i32>,
    m_VCShowFailedCheckout: Option<bool>,
    m_ConfigSettings: Option<Vec<(String, ConfigSetting)>>,
    m_VCAllowAsyncUpdate: Option<bool>,
    m_VCOverwriteFailedCheckoutAssets: Option<bool>,
    m_AssetPipelineMode: Option<i32>,
    m_CacheServerMode: Option<i32>,
    m_CacheServers: Option<Vec<String>>,
    m_AssetPipelineMode2: Option<i32>,
    m_VCOverlayIcons: Option<bool>,
    m_VCProjectOverlayIcons: Option<bool>,
    m_VCHierarchyOverlayIcons: Option<bool>,
    m_VCOtherOverlayIcons: Option<bool>,
    m_ArtifactGarbageCollection: Option<bool>,
    m_DesiredImportWorkerCount: Option<i32>,
    m_StandbyImportWorkerCount: Option<i32>,
    m_IdleImportWorkerShutdownDelay: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffectConstant {
    __type: i32,
    groupConstantIndex: u32,
    sendTargetEffectIndex: u32,
    wetMixLevelIndex: u32,
    prevEffectIndex: u32,
    bypass: bool,
    parameterIndices: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effector2D {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EllipsoidParticleEmitter {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_Emit: bool,
    minSize: f32,
    maxSize: f32,
    minEnergy: f32,
    maxEnergy: f32,
    minEmission: f32,
    maxEmission: f32,
    worldVelocity: Vector3f,
    localVelocity: Vector3f,
    rndVelocity: Vector3f,
    emitterVelocityScale: f32,
    tangentVelocity: Vector3f,
    angularVelocity: f32,
    rndAngularVelocity: f32,
    rndRotation: bool,
    Simulate_in_Worldspace: bool,
    m_OneShot: bool,
    m_Ellipsoid: Vector3f,
    m_MinEmitterRange: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddedNativeType {
    m_FloatArray: Vec<f32>,
    m_String: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmissionModule {
    enabled: bool,
    m_Type: Option<i32>,
    rate: Option<MinMaxCurve>,
    cnt0: Option<(Option<i32>, Option<u16>)>,
    cnt1: Option<(Option<i32>, Option<u16>)>,
    cnt2: Option<(Option<i32>, Option<u16>)>,
    cnt3: Option<(Option<i32>, Option<u16>)>,
    time0: Option<f32>,
    time1: Option<f32>,
    time2: Option<f32>,
    time3: Option<f32>,
    m_BurstCount: (Option<u8>, Option<i32>),
    cntmax0: Option<(Option<i32>, Option<u16>)>,
    cntmax1: Option<(Option<i32>, Option<u16>)>,
    cntmax2: Option<(Option<i32>, Option<u16>)>,
    cntmax3: Option<(Option<i32>, Option<u16>)>,
    rateOverTime: Option<MinMaxCurve>,
    rateOverDistance: Option<MinMaxCurve>,
    m_Bursts: Option<Vec<ParticleSystemEmissionBurst>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmptyObject {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnlightenRendererInformation {
    renderer: PPtr,
    dynamicLightmapSTInSystem: Vector4f,
    systemId: i32,
    instanceHash: Hash128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnlightenSceneMapping {
    m_Renderers: Vec<EnlightenRendererInformation>,
    m_Systems: Vec<EnlightenSystemInformation>,
    m_SystemAtlases: Vec<EnlightenSystemAtlasInformation>,
    m_TerrainChunks: Vec<EnlightenTerrainChunksInformation>,
    m_Probesets: Option<Vec<Hash128>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnlightenSystemAtlasInformation {
    atlasSize: i32,
    atlasHash: Hash128,
    firstSystemId: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnlightenSystemInformation {
    rendererIndex: u32,
    rendererSize: u32,
    atlasIndex: i32,
    atlasOffsetX: i32,
    atlasOffsetY: i32,
    inputSystemHash: Hash128,
    radiositySystemHash: Hash128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnlightenTerrainChunksInformation {
    firstSystemId: i32,
    numChunksInX: i32,
    numChunksInY: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpandedData {
    m_InspectorExpanded: bool,
    m_ClassID: i32,
    m_ScriptClass: String,
    m_ExpandedProperties: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExposedReferenceTable {
    m_References: Vec<(String, PPtr)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expression {
    op: i32,
    valueIndex: i32,
    data: (i32, i32, i32, i32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionPropertyValue {
    pluginName: String,
    extensionName: String,
    propertyName: String,
    propertyValue: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalForcesModule {
    enabled: bool,
    multiplier: Option<f32>,
    influenceFilter: Option<i32>,
    influenceMask: Option<BitField>,
    influenceList: Option<Vec<PPtr>>,
    multiplierCurve: Option<MinMaxCurve>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct FBXImporter {
//     m_Name: String,
//     m_FileIDToRecycleName: Option<(Option<Vec<(i64, String)>>, Option<Vec<(i32, String)>>)>,
//     m_OldHashIdentity: Option<MdFour>,
//     m_NewHashIdentity: Option<MdFour>,
//     m_GenerateMaterials: Option<i32>,
//     m_GenerateAnimations: Option<i32>,
//     m_GlobalScale: f32,
//     m_BakeSimulation: bool,
//     m_SplitAnimations: Option<bool>,
//     m_AddColliders: bool,
//     m_UseFileUnits: bool,
//     m_AnimationCompression: i32,
//     m_AnimationRotationError: f32,
//     m_AnimationPositionError: f32,
//     m_AnimationScaleError: f32,
//     m_MeshCompression: i32,
//     m_MeshSettings_swapUVChannels: Option<bool>,
//     m_MeshSettings_generateSecondaryUV: Option<bool>,
//     m_MeshSettings_secondaryUVAngleDistortion: Option<f32>,
//     m_MeshSettings_secondaryUVAreaDistortion: Option<f32>,
//     m_MeshSettings_secondaryUVHardAngle: Option<f32>,
//     m_MeshSettings_secondaryUVPackMargin: Option<f32>,
//     m_MeshSettings_normalImportMode: Option<i32>,
//     m_MeshSettings_tangentImportMode: Option<i32>,
//     m_AnimationWrapMode: i32,
//     m_ClipAnimations: Vec<ClipAnimationInfo>,
//     m_FirstImportVersion: Option<i32>,
//     normalSmoothAngle: f32,
//     splitTangentsAcrossUV: Option<bool>,
//     m_ImportedRoots: Vec<PPtr>,
//     m_HasExtraRoot: bool,
//     m_ImportMaterials: Option<bool>,
//     m_MaterialName: Option<i32>,
//     m_MaterialSearch: Option<i32>,
//     m_LODScreenPercentages: Option<Vec<f32>>,
//     swapUVChannels: Option<bool>,
//     generateSecondaryUV: Option<bool>,
//     optimizeMesh: Option<bool>,
//     secondaryUVAngleDistortion: Option<f32>,
//     secondaryUVAreaDistortion: Option<f32>,
//     secondaryUVHardAngle: Option<f32>,
//     secondaryUVPackMargin: Option<f32>,
//     normalImportMode: Option<i32>,
//     tangentImportMode: Option<i32>,
//     m_LegacyGenerateAnimations: Option<i32>,
//     m_IsReadable: Option<bool>,
//     optimizeMeshForGPU: Option<bool>,
//     m_ImportedTakeInfos: Option<Vec<TakeInfo>>,
//     m_ReferencedClips: Option<Vec<GUID>>,
//     m_ImportAnimation: Option<bool>,
//     m_CopyAvatar: Option<bool>,
//     m_HumanDescription: Option<HumanDescription>,
//     m_LastHumanDescriptionAvatarSource: Option<PPtr>,
//     m_AdditionalBone: Option<bool>,
//     m_AnimationType: Option<i32>,
//     m_UserData: Option<String>,
//     m_ImportBlendShapes: Option<bool>,
//     weldVertices: Option<bool>,
//     m_OptimizeGameObjects: Option<bool>,
//     m_ExtraExposedTransformPaths: Option<Vec<String>>,
//     m_MotionNodeName: Option<String>,
//     keepQuads: Option<bool>,
//     m_UseFileScale: Option<bool>,
//     m_FileScale: Option<f32>,
//     m_AssetBundleName: Option<String>,
//     m_AssetBundleVariant: Option<String>,
//     m_AnimationImportErrors: Option<String>,
//     m_AnimationImportWarnings: Option<String>,
//     m_AnimationRetargetingWarnings: Option<String>,
//     m_AnimationDoRetargetingWarnings: Option<bool>,
//     m_HumanoidOversampling: Option<i32>,
//     m_ResampleRotations: Option<bool>,
//     m_ResampleCurves: Option<bool>,
//     m_RigImportErrors: Option<String>,
//     m_RigImportWarnings: Option<String>,
//     m_ImportVisibility: Option<bool>,
//     m_ImportCameras: Option<bool>,
//     m_ImportLights: Option<bool>,
//     normalCalculationMode: Option<i32>,
//     m_ExtraUserProperties: Option<Vec<String>>,
//     m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
//     m_AutoMapExternalMaterials: Option<bool>,
//     m_MaterialLocation: Option<i32>,
//     m_Materials: Option<Vec<SourceAssetIdentifier>>,
//     m_ImportAnimatedCustomProperties: Option<bool>,
//     m_HasEmbeddedTextures: Option<bool>,
//     m_SupportsEmbeddedMaterials: Option<bool>,
//     m_PreserveHierarchy: Option<bool>,
//     indexFormat: Option<i32>,
//     m_ImportConstraints: Option<bool>,
//     m_PreviousCalculatedGlobalScale: Option<f32>,
//     m_HasPreviousCalculatedGlobalScale: Option<bool>,
//     m_UseSRGBMaterialColor: Option<bool>,
//     m_FileScaleUnit: Option<String>,
//     m_FileScaleFactor: Option<f32>,
//     legacyComputeAllNormalsFromSmoothingGroupsWhenMeshHasBlendShapes: Option<bool>,
//     blendShapeNormalImportMode: Option<i32>,
//     normalSmoothingSource: Option<i32>,
//     m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
//     m_UsedFileIDs: Option<Vec<i64>>,
//     skinWeightsMode: Option<i32>,
//     maxBonesPerVertex: Option<i32>,
//     minBoneWeight: Option<f32>,
//     meshOptimizationFlags: Option<i32>,
//     m_SortHierarchyByName: Option<bool>,
//     m_AvatarSetup: Option<i32>,
//     m_MaterialImportMode: Option<i32>,
//     m_AutoGenerateAvatarMappingIfUnspecified: Option<bool>,
//     m_FileIdsGeneration: Option<i32>,
//     secondaryUVMarginMethod: Option<i32>,
//     secondaryUVMinLightmapResolution: Option<f32>,
//     secondaryUVMinObjectScale: Option<f32>,
//     bakeAxisConversion: Option<bool>,
//     m_AddHumanoidExtraRootOnlyWhenUsingAvatar: Option<bool>,
//     optimizeBones: Option<bool>,
//     m_RemoveConstantScaleCurves: Option<bool>,
//     m_NodeNameCollisionStrategy: Option<i32>,
//     m_StrictVertexDataChecks: Option<bool>,
//     m_ImportBlendShapeDeformPercent: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FakeComponent {
    m_GameObject: PPtr,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct FalloffTable {
//     m_Table: (
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//     ),
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FastPropertyName {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FixedJoint {
    m_GameObject: PPtr,
    m_ConnectedBody: PPtr,
    m_BreakForce: f32,
    m_BreakTorque: f32,
    m_EnableCollision: Option<bool>,
    m_EnablePreprocessing: Option<bool>,
    m_MassScale: Option<f32>,
    m_ConnectedMassScale: Option<f32>,
    m_Enabled: Option<bool>,
    m_ConnectedArticulationBody: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FixedJoint2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_EnableCollision: bool,
    m_ConnectedRigidBody: PPtr,
    m_BreakForce: f32,
    m_BreakTorque: f32,
    m_AutoConfigureConnectedAnchor: bool,
    m_Anchor: Vector2f,
    m_ConnectedAnchor: Vector2f,
    m_DampingRatio: f32,
    m_Frequency: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flare {
    m_Name: String,
    m_FlareTexture: PPtr,
    m_TextureLayout: i32,
    m_Elements: Vec<FlareElement>,
    m_UseFog: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlareElement {
    m_ImageIndex: u32,
    m_Position: f32,
    m_Size: f32,
    m_Color: ColorRGBA,
    m_UseLightColor: bool,
    m_Rotate: bool,
    m_Zoom: bool,
    m_Fade: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlareLayer {
    m_GameObject: PPtr,
    m_Enabled: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FloatCurve {
    curve: AnimationCurve,
    attribute: String,
    path: String,
    classID: i32,
    script: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Font {
    m_Name: String,
    m_AsciiStartOffset: i32,
    m_FontCountX: Option<i32>,
    m_FontCountY: Option<i32>,
    m_Kerning: Option<f32>,
    m_LineSpacing: f32,
    m_PerCharacterKerning: Option<Vec<(i32, f32)>>,
    m_ConvertCase: i32,
    m_DefaultMaterial: PPtr,
    m_CharacterRects: Vec<CharacterInfo>,
    m_Texture: PPtr,
    m_KerningValues: Vec<((u16, u16), f32)>,
    m_GridFont: Option<bool>,
    m_FontData: Vec<char>,
    m_FontSize: f32,
    m_Ascent: f32,
    m_DefaultStyle: u32,
    m_FontNames: Vec<String>,
    m_CharacterSpacing: Option<i32>,
    m_CharacterPadding: Option<i32>,
    m_PixelScale: Option<f32>,
    m_FallbackFonts: Option<Vec<PPtr>>,
    m_FontRenderingMode: Option<i32>,
    m_Tracking: Option<f32>,
    m_Descent: Option<f32>,
    m_UseLegacyBoundsCalculation: Option<bool>,
    m_ShouldRoundAdvanceValue: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForceModule {
    enabled: bool,
    x: MinMaxCurve,
    y: MinMaxCurve,
    z: MinMaxCurve,
    inWorldSpace: bool,
    randomizePerFrame: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrictionJoint2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_EnableCollision: bool,
    m_ConnectedRigidBody: PPtr,
    m_BreakForce: f32,
    m_BreakTorque: f32,
    m_AutoConfigureConnectedAnchor: bool,
    m_Anchor: Vector2f,
    m_ConnectedAnchor: Vector2f,
    m_MaxForce: f32,
    m_MaxTorque: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GISettings {
    m_BounceScale: f32,
    m_IndirectOutputScale: f32,
    m_AlbedoBoost: f32,
    m_TemporalCoherenceThreshold: Option<f32>,
    m_EnvironmentLightingMode: u32,
    m_EnableBakedLightmaps: bool,
    m_EnableRealtimeLightmaps: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GLTextureSettings {
    m_FilterMode: i32,
    m_Aniso: i32,
    m_MipBias: f32,
    m_WrapMode: Option<i32>,
    m_WrapU: Option<i32>,
    m_WrapV: Option<i32>,
    m_WrapW: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GUID {
    data: (u32, u32, u32, u32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GUIDSerializer {
    guidToPath: Vec<(GUID, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GUIElement {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GUILayer {
    m_GameObject: PPtr,
    m_Enabled: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GUIText {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Text: String,
    m_Anchor: i16,
    m_Alignment: i16,
    m_PixelOffset: Vector2f,
    m_LineSpacing: f32,
    m_TabSize: f32,
    m_Font: PPtr,
    m_Material: PPtr,
    m_FontSize: i32,
    m_FontStyle: i32,
    m_PixelCorrect: bool,
    m_RichText: Option<bool>,
    m_Color: Option<ColorRGBA>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GUITexture {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Texture: PPtr,
    m_Color: ColorRGBA,
    m_PixelInset: Rectf,
    m_LeftBorder: i32,
    m_RightBorder: i32,
    m_TopBorder: i32,
    m_BottomBorder: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameObject {
    m_Component: (Option<Vec<ComponentPair>>, Option<Vec<(i32, PPtr)>>),
    m_Layer: u32,
    m_Name: String,
    m_Tag: u16,
    m_IsActive: (Option<u8>, Option<bool>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameObjectRecorder {
    m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericBinding {
    path: u32,
    attribute: u32,
    script: PPtr,
    classID: Option<u16>,
    customType: u8,
    isPPtrCurve: u8,
    typeID: Option<i32>,
    isIntCurve: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalGameManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Google {
    depthFormat: i32,
    enableTransitionView: Option<bool>,
    useSustainedPerformanceMode: Option<bool>,
    enableVideoLayer: Option<bool>,
    useProtectedVideoMemory: Option<bool>,
    minimumSupportedHeadTracking: Option<i32>,
    maximumSupportedHeadTracking: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Gradient {
    m_Color: Option<(ColorRGBA, ColorRGBA, ColorRGBA, ColorRGBA, ColorRGBA)>,
    key0: Option<ColorRGBA>,
    key1: Option<ColorRGBA>,
    key2: Option<ColorRGBA>,
    key3: Option<ColorRGBA>,
    key4: Option<ColorRGBA>,
    key5: Option<ColorRGBA>,
    key6: Option<ColorRGBA>,
    key7: Option<ColorRGBA>,
    ctime0: Option<u16>,
    ctime1: Option<u16>,
    ctime2: Option<u16>,
    ctime3: Option<u16>,
    ctime4: Option<u16>,
    ctime5: Option<u16>,
    ctime6: Option<u16>,
    ctime7: Option<u16>,
    atime0: Option<u16>,
    atime1: Option<u16>,
    atime2: Option<u16>,
    atime3: Option<u16>,
    atime4: Option<u16>,
    atime5: Option<u16>,
    atime6: Option<u16>,
    atime7: Option<u16>,
    m_Mode: Option<i32>,
    m_NumColorKeys: Option<u8>,
    m_NumAlphaKeys: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GradientNEW {
    key0: ColorRGBA,
    key1: ColorRGBA,
    key2: ColorRGBA,
    key3: ColorRGBA,
    key4: ColorRGBA,
    key5: ColorRGBA,
    key6: ColorRGBA,
    key7: ColorRGBA,
    ctime0: u16,
    ctime1: u16,
    ctime2: u16,
    ctime3: u16,
    ctime4: u16,
    ctime5: u16,
    ctime6: u16,
    ctime7: u16,
    atime0: u16,
    atime1: u16,
    atime2: u16,
    atime3: u16,
    atime4: u16,
    atime5: u16,
    atime6: u16,
    atime7: u16,
    m_NumColorKeys: u8,
    m_NumAlphaKeys: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grid {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_CellSize: Vector3f,
    m_CellGap: Vector3f,
    m_CellLayout: i32,
    m_CellSwizzle: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GridLayout {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupConnection {
    sourceGroupIndex: u32,
    targetGroupIndex: u32,
    sendEffectIndex: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupConstant {
    parentConstantIndex: i32,
    volumeIndex: u32,
    pitchIndex: u32,
    mute: bool,
    solo: bool,
    bypassEffects: bool,
    sendIndex: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Halo {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Color: ColorRGBA,
    m_Size: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HaloLayer {
    m_GameObject: PPtr,
    m_Enabled: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HaloManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hand {
    m_HandBoneIndex: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandPose {
    m_GrabX: xform,
    m_DoFArray: Vec<f32>,
    m_Override: f32,
    m_CloseOpen: f32,
    m_InOut: f32,
    m_Grab: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Handle {
    m_X: xform,
    m_ParentHumanIndex: u32,
    m_ID: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hash128 {
    // bytes: (
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    // ),
    bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeightMeshBVNode {
    min: Vector3f,
    max: Vector3f,
    i: i32,
    n: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeightMeshData {
    m_Vertices: Vec<Vector3f>,
    m_Indices: Vec<i32>,
    m_Bounds: AABB,
    m_Nodes: Vec<HeightMeshBVNode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heightmap {
    m_Heights: Vec<i16>,
    m_PrecomputedError: Vec<f32>,
    m_MinMaxPatchHeights: Vec<f32>,
    m_DefaultPhysicMaterial: Option<PPtr>,
    m_Width: Option<i32>,
    m_Height: Option<i32>,
    m_Levels: i32,
    m_Scale: Vector3f,
    m_Thickness: Option<f32>,
    m_SurfaceMask: Option<Vec<u8>>,
    m_SurfaceMaskLOD: Option<Vec<u8>>,
    m_EnableSurfaceMaskTextureCompression: Option<bool>,
    m_Resolution: Option<i32>,
    m_Holes: Option<Vec<u8>>,
    m_HolesLOD: Option<Vec<u8>>,
    m_EnableHolesTextureCompression: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeightmapData {
    position: Vector3f,
    terrainData: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HierarchicalSceneData {
    m_SceneGUID: GUID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HierarchyState {
    m_ObjectHideFlags: Option<u32>,
    expanded: Vec<PPtr>,
    selection: Vec<PPtr>,
    scrollposition_x: f32,
    scrollposition_y: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HingeJoint {
    m_GameObject: PPtr,
    m_ConnectedBody: PPtr,
    m_Anchor: Vector3f,
    m_Axis: Vector3f,
    m_UseSpring: bool,
    m_Spring: JointSpring,
    m_UseMotor: bool,
    m_Motor: JointMotor,
    m_UseLimits: bool,
    m_Limits: JointLimits,
    m_BreakForce: f32,
    m_BreakTorque: f32,
    m_AutoConfigureConnectedAnchor: Option<bool>,
    m_ConnectedAnchor: Option<Vector3f>,
    m_EnableCollision: Option<bool>,
    m_EnablePreprocessing: Option<bool>,
    m_MassScale: Option<f32>,
    m_ConnectedMassScale: Option<f32>,
    m_Enabled: Option<bool>,
    m_ConnectedArticulationBody: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HingeJoint2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_CollideConnected: Option<bool>,
    m_ConnectedRigidBody: PPtr,
    m_Anchor: Vector2f,
    m_ConnectedAnchor: Vector2f,
    m_UseMotor: bool,
    m_Motor: JointMotor2D,
    m_UseLimits: bool,
    m_AngleLimits: (Option<JointAngleLimit2D>, Option<JointAngleLimits2D>),
    m_EnableCollision: Option<bool>,
    m_BreakForce: Option<f32>,
    m_BreakTorque: Option<f32>,
    m_AutoConfigureConnectedAnchor: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HoloLens {
    depthFormat: i32,
    depthBufferSharingEnabled: Option<bool>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Human {
//     m_RootX: xform,
//     m_Skeleton: OffsetPtr,
//     m_SkeletonPose: OffsetPtr,
//     m_LeftHand: OffsetPtr,
//     m_RightHand: OffsetPtr,
//     m_Handles: Option<Vec<Handle>>,
//     m_ColliderArray: Option<Vec<Collider>>,
//     m_HumanBoneIndex: Vec<i32>,
//     m_HumanBoneMass: Vec<f32>,
//     m_ColliderIndex: Option<Vec<i32>>,
//     m_Scale: f32,
//     m_ArmTwist: f32,
//     m_ForeArmTwist: f32,
//     m_UpperLegTwist: f32,
//     m_LegTwist: f32,
//     m_ArmStretch: f32,
//     m_LegStretch: f32,
//     m_FeetSpacing: f32,
//     m_HasLeftHand: bool,
//     m_HasRightHand: bool,
//     m_HasTDoF: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanBone {
    m_BoneName: String,
    m_HumanName: String,
    m_Limit: SkeletonBoneLimit,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanDescription {
    m_Human: Vec<HumanBone>,
    m_Skeleton: Vec<SkeletonBone>,
    m_Handles: Option<Vec<HumanHandle>>,
    m_ArmTwist: f32,
    m_ForeArmTwist: f32,
    m_UpperLegTwist: f32,
    m_LegTwist: f32,
    m_ArmStretch: f32,
    m_LegStretch: f32,
    m_FeetSpacing: f32,
    m_RootMotionBoneName: String,
    m_HasTranslationDoF: Option<bool>,
    m_RootMotionBoneRotation: Option<Quaternionf>,
    m_HasExtraRoot: Option<bool>,
    m_SkeletonHasParents: Option<bool>,
    m_GlobalScale: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanGoal {
    m_X: xform,
    m_WeightT: f32,
    m_WeightR: f32,
    m_HintT: Option<(Option<float3>, Option<float4>)>,
    m_HintWeightT: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanHandle {
    m_Name: String,
    m_BoneName: String,
    m_Position: Vector3f,
    m_Rotation: Quaternionf,
    m_Scale: Vector3f,
    m_LookAt: bool,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct HumanLayerConstant {
//     m_StateMachineIndex: u32,
//     m_StateMachineMotionSetIndex: u32,
//     m_BodyMask: HumanPoseMask,
//     m_SkeletonMask: OffsetPtr,
//     m_Binding: u32,
//     m_LayerBlendingMode: i32,
//     m_IKPass: bool,
//     m_DefaultWeight: Option<f32>,
//     m_SyncedLayerAffectsTiming: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanPose {
    m_RootX: xform,
    m_LookAtPosition: (Option<float3>, Option<float4>),
    m_LookAtWeight: float4,
    m_GoalArray: Vec<HumanGoal>,
    m_LeftHandPose: HandPose,
    m_RightHandPose: HandPose,
    m_DoFArray: Vec<f32>,
    m_TDoFArray: Option<(Option<Vec<float4>>, Option<Vec<float3>>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanPoseMask {
    word0: u32,
    word1: u32,
    word2: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanTemplate {
    m_Name: String,
    m_BoneTemplate: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IConstraint {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IHVImageFormatImporter {
    m_Name: String,
    m_TextureSettings: GLTextureSettings,
    m_IsReadable: bool,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_sRGBTexture: Option<bool>,
    m_StreamingMipmaps: Option<bool>,
    m_StreamingMipmapsPriority: Option<i32>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    m_Format: (Option<u32>, Option<i32>),
    m_Width: i32,
    m_Height: i32,
    m_RowBytes: i32,
    image_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InheritVelocityModule {
    enabled: bool,
    m_Mode: i32,
    m_Curve: MinMaxCurve,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitialModule {
    enabled: bool,
    startLifetime: MinMaxCurve,
    startSpeed: MinMaxCurve,
    startColor: MinMaxGradient,
    startSize: MinMaxCurve,
    startRotation: MinMaxCurve,
    gravityModifier: (Option<f32>, Option<MinMaxCurve>),
    inheritVelocity: Option<f32>,
    maxNumParticles: i32,
    startRotationX: Option<MinMaxCurve>,
    startRotationY: Option<MinMaxCurve>,
    randomizeRotationDirection: Option<f32>,
    rotation3D: Option<bool>,
    startSizeY: Option<MinMaxCurve>,
    startSizeZ: Option<MinMaxCurve>,
    size3D: Option<bool>,
    customEmitterVelocity: Option<Vector3f>,
    gravitySource: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputAxis {
    m_Name: String,
    descriptiveName: String,
    descriptiveNegativeName: String,
    negativeButton: String,
    positiveButton: String,
    altNegativeButton: String,
    altPositiveButton: String,
    gravity: f32,
    dead: f32,
    sensitivity: f32,
    snap: bool,
    invert: bool,
    __type: i32,
    axis: i32,
    joyNum: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputImportSettings {
    name: String,
    value: Option<SubstanceValue>,
    alphaSource: Option<i32>,
    filterMode: Option<i32>,
    aniso: Option<i32>,
    wrapMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputManager {
    m_Axes: Vec<InputAxis>,
    m_UsePhysicalKeys: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InspectorExpandedState {
    m_ObjectHideFlags: Option<u32>,
    m_ExpandedData: Vec<ExpandedData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntPoint {
    X: i64,
    Y: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveCloth {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_BendingStiffness: f32,
    m_StretchingStiffness: f32,
    m_Damping: f32,
    m_Thickness: f32,
    m_UseGravity: bool,
    m_SelfCollision: bool,
    m_ExternalAcceleration: Vector3f,
    m_RandomAcceleration: Vector3f,
    m_Mesh: PPtr,
    m_Friction: f32,
    m_Density: f32,
    m_Pressure: f32,
    m_CollisionResponse: f32,
    m_AttachmentTearFactor: f32,
    m_AttachmentResponse: f32,
    m_TearFactor: f32,
    m_AttachedColliders: Vec<ClothAttachment>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    markedForRemoval: Option<bool>,
    downloadResolution: Option<i32>,
    nameConflictResolution: Option<i32>,
    changeset: Option<i32>,
    guid: Option<GUID>,
    name: Option<String>,
    parent: Option<GUID>,
    __type: Option<i32>,
    digest: Option<(Option<MdFour>, Option<Hash128>)>,
    origin: Option<i32>,
    oldVersion: Option<i32>,
    parentFolderID: Option<i32>,
    changeFlags: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Joint {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Joint2D {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointAngleLimit2D {
    m_LowerAngle: f32,
    m_UpperAngle: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointAngleLimits2D {
    m_LowerAngle: f32,
    m_UpperAngle: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointDrive {
    mode: Option<i32>,
    positionSpring: f32,
    positionDamper: f32,
    maximumForce: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointLimits {
    min: f32,
    max: f32,
    minBounce: Option<f32>,
    maxBounce: Option<f32>,
    contactDistance: Option<f32>,
    bounciness: Option<f32>,
    bounceMinVelocity: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointMotor {
    targetVelocity: f32,
    force: f32,
    freeSpin: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointMotor2D {
    m_MotorSpeed: f32,
    m_MaximumMotorForce: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointSpring {
    spring: f32,
    damper: f32,
    targetPosition: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointSuspension2D {
    m_DampingRatio: f32,
    m_Frequency: f32,
    m_Angle: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointTranslationLimits2D {
    m_LowerTranslation: f32,
    m_UpperTranslation: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KTXImporter {
    m_Name: String,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keyframe {
    time: f32,
    value: (Option<f32>, Option<Quaternionf>, Option<Vector3f>),
    inSlope: (Option<f32>, Option<Quaternionf>, Option<Vector3f>),
    outSlope: (Option<f32>, Option<Quaternionf>, Option<Vector3f>),
    weightedMode: Option<i32>,
    inWeight: Option<(Option<f32>, Option<Quaternionf>, Option<Vector3f>)>,
    outWeight: Option<(Option<f32>, Option<Quaternionf>, Option<Vector3f>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LOD {
    screenRelativeHeight: f32,
    renderers: Vec<LODRenderer>,
    fadeMode: Option<i32>,
    fadeTransitionWidth: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LODGroup {
    m_GameObject: PPtr,
    m_LocalReferencePoint: Vector3f,
    m_Size: f32,
    m_ScreenRelativeTransitionHeight: Option<f32>,
    m_LODs: Vec<LOD>,
    m_Enabled: bool,
    m_FadeMode: Option<i32>,
    m_AnimateCrossFading: Option<bool>,
    m_LastLODIsBillboard: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LODRenderer {
    renderer: PPtr,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct LayerConstant {
//     m_StateMachineIndex: u32,
//     m_StateMachineMotionSetIndex: Option<u32>,
//     m_BodyMask: HumanPoseMask,
//     m_SkeletonMask: OffsetPtr,
//     m_Binding: u32,
//     m_LayerBlendingMode: i32,
//     m_DefaultWeight: f32,
//     m_IKPass: bool,
//     m_SyncedLayerAffectsTiming: bool,
//     m_StateMachineSynchronizedLayerIndex: Option<u32>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayoutDataOne {
    m_FloatArray: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayoutDataThree {
    m_AnotherFloatArray: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayoutDataTwo {
    m_IntegerValue: i32,
    m_FloatValue: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeafInfoConstant {
    m_IDArray: Vec<u32>,
    m_IndexOffset: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LensFlare {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Flare: PPtr,
    m_Color: ColorRGBA,
    m_Brightness: f32,
    m_IgnoreLayers: BitField,
    m_Directional: bool,
    m_FadeSpeed: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelGameManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryAssetImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_UserData: Option<String>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryRepresentation {
    name: String,
    thumbnail: Image,
    object: Option<(Option<PPtr>, Option<PPtr>)>,
    thumbnailClassID: (Option<i16>, Option<i32>),
    scriptClassName: String,
    flags: Option<u16>,
    guid: Option<GUID>,
    path: Option<String>,
    localIdentifier: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LifetimeByEmitterSpeedModule {
    enabled: bool,
    m_Curve: MinMaxCurve,
    m_Range: Vector2f,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Light {
//     m_GameObject: PPtr,
//     m_Enabled: u8,
//     m_Type: i32,
//     m_Color: ColorRGBA,
//     m_Intensity: f32,
//     m_Range: f32,
//     m_SpotAngle: f32,
//     m_CookieSize: f32,
//     m_Shadows: ShadowSettings,
//     m_Cookie: PPtr,
//     m_DrawHalo: bool,
//     m_ActuallyLightmapped: Option<bool>,
//     m_Flare: PPtr,
//     m_RenderMode: i32,
//     m_CullingMask: BitField,
//     m_Lightmapping: i32,
//     m_BounceIntensity: Option<f32>,
//     m_BakedIndex: Option<i32>,
//     m_AreaSize: Option<Vector2f>,
//     m_CCT: Option<f32>,
//     m_BakingOutput: Option<LightBakingOutput>,
//     m_ColorTemperature: Option<f32>,
//     m_UseColorTemperature: Option<bool>,
//     m_FalloffTable: Option<FalloffTable>,
//     m_LightShadowCasterMode: Option<i32>,
//     m_InnerSpotAngle: Option<f32>,
//     m_RenderingLayerMask: Option<u32>,
//     m_BoundingSphereOverride: Option<Vector4f>,
//     m_UseBoundingSphereOverride: Option<bool>,
//     m_Shape: Option<i32>,
//     m_UseViewFrustumForShadowCasterCull: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightBakingOutput {
    probeOcclusionLightIndex: i32,
    shadowMaskChannel: Option<i32>,
    lightmappingMask: Option<i32>,
    occlusionMaskChannel: Option<i32>,
    lightmapBakeMode: Option<LightmapBakeMode>,
    isBaked: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightProbeData {
    m_Tetrahedralization: ProbeSetTetrahedralization,
    m_ProbeSets: Vec<ProbeSetIndex>,
    m_Positions: Vec<Vector3f>,
    m_NonTetrahedralizedProbeSetIndexMap: Vec<(Hash128, i32)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightProbeGroup {
    m_GameObject: PPtr,
    m_Enabled: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightProbeOcclusion {
    m_BakedLightIndex: Option<Vec<i32>>,
    m_Occlusion: Vec<f32>,
    m_ProbeOcclusionLightIndex: Option<Vec<i32>>,
    m_ShadowMaskChannel: Option<Vec<i8>>,
    m_OcclusionMaskChannel: Option<Vec<i8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightProbeProxyVolume {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_BoundingBoxMode: i32,
    m_ResolutionX: u32,
    m_ResolutionY: u32,
    m_ResolutionZ: u32,
    m_ResolutionProbesPerUnit: f32,
    m_BoundingBoxSize: Vector3f,
    m_BoundingBoxOrigin: Vector3f,
    m_ResolutionMode: i32,
    m_ProbePositionMode: i32,
    m_RefreshMode: i32,
    m_QualityMode: Option<i32>,
    m_DataFormat: Option<i32>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct LightProbes {
//     m_Name: String,
//     bakedPositions: Option<Vec<Vector3f>>,
//     bakedCoefficients: Option<Vec<LightmapData>>,
//     tetrahedra: Option<Vec<Tetrahedron>>,
//     hullRays: Option<Vec<Vector3f>>,
//     m_Data: Option<LightProbeData>,
//     m_BakedCoefficients: Option<Vec<SphericalHarmonicsL2>>,
//     m_BakedLightOcclusion: Option<Vec<LightProbeOcclusion>>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightingDataAssetParent {
    m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightingSettings {
    m_Name: String,
    m_GIWorkflowMode: i32,
    m_EnableBakedLightmaps: bool,
    m_EnableRealtimeLightmaps: bool,
    m_RealtimeEnvironmentLighting: bool,
    m_BounceScale: f32,
    m_AlbedoBoost: f32,
    m_UsingShadowmask: bool,
    m_BakeBackend: Option<i32>,
    m_LightmapMaxSize: Option<i32>,
    m_BakeResolution: Option<f32>,
    m_Padding: Option<i32>,
    m_TextureCompression: Option<bool>,
    m_AO: Option<bool>,
    m_AOMaxDistance: Option<f32>,
    m_CompAOExponent: Option<f32>,
    m_CompAOExponentDirect: Option<f32>,
    m_ExtractAO: Option<bool>,
    m_MixedBakeMode: Option<i32>,
    m_LightmapsBakeMode: Option<i32>,
    m_FilterMode: Option<i32>,
    m_LightmapParameters: Option<PPtr>,
    m_ExportTrainingData: Option<bool>,
    m_TrainingDataDestination: Option<String>,
    m_RealtimeResolution: Option<f32>,
    m_ForceWhiteAlbedo: Option<bool>,
    m_ForceUpdates: Option<bool>,
    m_FinalGather: Option<bool>,
    m_FinalGatherRayCount: Option<i32>,
    m_FinalGatherFiltering: Option<bool>,
    m_PVRCulling: Option<bool>,
    m_PVRSampling: Option<i32>,
    m_PVRDirectSampleCount: Option<i32>,
    m_PVRSampleCount: Option<i32>,
    m_PVREnvironmentSampleCount: Option<i32>,
    m_PVRBounces: Option<i32>,
    m_PVREnvironmentMIS: Option<i32>,
    m_PVREnvironmentReferencePointCount: Option<i32>,
    m_PVRFilteringMode: Option<i32>,
    m_PVRDenoiserTypeDirect: Option<i32>,
    m_PVRDenoiserTypeIndirect: Option<i32>,
    m_PVRDenoiserTypeAO: Option<i32>,
    m_PVRFilterTypeDirect: Option<i32>,
    m_PVRFilterTypeIndirect: Option<i32>,
    m_PVRFilterTypeAO: Option<i32>,
    m_PVRFilteringGaussRadiusDirect: Option<i32>,
    m_PVRFilteringGaussRadiusIndirect: Option<i32>,
    m_PVRFilteringGaussRadiusAO: Option<i32>,
    m_PVRFilteringAtrousPositionSigmaDirect: Option<f32>,
    m_PVRFilteringAtrousPositionSigmaIndirect: Option<f32>,
    m_PVRFilteringAtrousPositionSigmaAO: Option<f32>,
    m_IndirectOutputScale: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightmapBakeMode {
    lightmapBakeType: i32,
    mixedLightingMode: i32,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct LightmapData {
//     m_Lightmap: Option<PPtr>,
//     m_IndirectLightmap: Option<PPtr>,
//     m_DirLightmap: Option<PPtr>,
//     m_ShadowMask: Option<PPtr>,
//     sh: Option<(
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//     )>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightmapParameters {
    m_Name: String,
    resolution: f32,
    clusterResolution: f32,
    irradianceBudget: i32,
    irradianceQuality: i32,
    backFaceTolerance: f32,
    isTransparent: i32,
    modellingTolerance: f32,
    systemTag: i32,
    edgeStitching: i32,
    blurRadius: i32,
    directLightQuality: i32,
    antiAliasingSamples: i32,
    AOQuality: i32,
    AOAntiAliasingSamples: i32,
    bakedLightmapTag: i32,
    pushoff: Option<f32>,
    limitLightmapCount: Option<bool>,
    maxLightmapCount: Option<i32>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct LightmapSettings {
//     m_Lightmaps: Vec<LightmapData>,
//     m_LightmapsMode: i32,
//     m_UseDualLightmapsInForward: Option<bool>,
//     m_LightProbes: Option<PPtr>,
//     m_BakedColorSpace: Option<i32>,
//     m_EnlightenSceneMapping: Option<EnlightenSceneMapping>,
//     m_GISettings: Option<GISettings>,
//     m_RuntimeCPUUsage: Option<i32>,
//     m_ShadowMaskMode: Option<i32>,
//     m_UseShadowmask: Option<bool>,
//     m_LightingSettings: Option<PPtr>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct LightmapSnapshot {
//     m_Name: String,
//     m_Lightmaps: Vec<LightmapData>,
//     m_LightProbes: PPtr,
//     m_BakedAmbientProbesInLinear: Option<Vec<SphericalHarmonicsL2>>,
//     m_BakedAmbientProbesInGamma: Option<Vec<SphericalHarmonicsL2>>,
//     m_LightmappedRendererData: Vec<RendererData>,
//     m_LightmappedRendererDataIDs: Vec<SceneObjectIdentifier>,
//     m_EnlightenSceneMapping: EnlightenSceneMapping,
//     m_EnlightenSceneMappingRendererIDs: Vec<SceneObjectIdentifier>,
//     m_Lights: Vec<SceneObjectIdentifier>,
//     m_BakedReflectionProbeCubemaps: Vec<PPtr>,
//     m_BakedSkyboxProbeCubemaps: Option<Vec<PPtr>>,
//     m_BakedReflectionProbes: Vec<SceneObjectIdentifier>,
//     m_EnlightenData: Vec<u8>,
//     m_SceneGUID: Option<GUID>,
//     m_BakedAmbientProbeInLinear: Option<SphericalHarmonicsL2>,
//     m_BakedAmbientProbeInGamma: Option<SphericalHarmonicsL2>,
//     m_EnlightenDataVersion: Option<i32>,
//     m_LightmapsMode: Option<i32>,
//     m_BakedLightIndices: Option<Vec<i32>>,
//     m_LightBakingOutputs: Option<Vec<LightBakingOutput>>,
//     m_Scene: Option<PPtr>,
//     m_LightmapsCacheFiles: Option<Vec<String>>,
//     m_BakedReflectionProbeCubemapCacheFiles: Option<Vec<String>>,
//     m_AOTextures: Option<Vec<PPtr>>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightsModule {
    enabled: bool,
    ratio: f32,
    light: PPtr,
    randomDistribution: bool,
    color: bool,
    range: bool,
    intensity: bool,
    rangeCurve: MinMaxCurve,
    intensityCurve: MinMaxCurve,
    maxLights: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Limit {
    m_Min: (Option<float3>, Option<float4>),
    m_Max: (Option<float3>, Option<float4>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineParameters {
    widthMultiplier: Option<f32>,
    widthCurve: Option<AnimationCurve>,
    colorGradient: Option<Gradient>,
    numCornerVertices: Option<i32>,
    numCapVertices: Option<i32>,
    alignment: Option<i32>,
    textureMode: Option<i32>,
    generateLightingData: Option<bool>,
    shadowBias: Option<f32>,
    textureScale: Option<Vector2f>,
    startWidth: Option<f32>,
    endWidth: Option<f32>,
    m_StartColor: Option<ColorRGBA>,
    m_EndColor: Option<ColorRGBA>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineRenderer {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: (Option<u8>, Option<bool>),
    m_ReceiveShadows: (Option<u8>, Option<bool>),
    m_LightmapIndex: (Option<u8>, Option<u16>),
    m_LightmapTilingOffset: Vector4f,
    m_Materials: Vec<PPtr>,
    m_SubsetIndices: Option<Vec<u32>>,
    m_StaticBatchRoot: PPtr,
    m_Positions: Vec<Vector3f>,
    m_Parameters: LineParameters,
    m_UseWorldSpace: bool,
    m_UseLightProbes: Option<bool>,
    m_LightProbeAnchor: Option<PPtr>,
    m_SortingLayer: Option<i16>,
    m_SortingOrder: Option<i16>,
    m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    m_LightmapIndexDynamic: Option<u16>,
    m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    m_ProbeAnchor: Option<PPtr>,
    m_MotionVectors: Option<u8>,
    m_LightProbeUsage: Option<u8>,
    m_LightProbeVolumeOverride: Option<PPtr>,
    m_StaticBatchInfo: Option<StaticBatchInfo>,
    m_Loop: Option<bool>,
    m_DynamicOccludee: Option<u8>,
    m_RenderingLayerMask: Option<u32>,
    m_RendererPriority: Option<i32>,
    m_RayTracingMode: Option<u8>,
    m_RayTraceProcedural: Option<u8>,
    m_StaticShadowCaster: Option<u8>,
    m_MaskInteraction: Option<i32>,
    m_ApplyActiveColorSpace: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalizationAsset {
    m_Name: String,
    Locale_ISO_Code: String,
    Editor_Asset: bool,
    String_Table: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalizationImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LookAtConstraint {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Weight: f32,
    m_RotationAtRest: Vector3f,
    m_RotationOffset: Vector3f,
    m_Roll: f32,
    m_WorldUpObject: PPtr,
    m_UseUpObject: bool,
    m_IsContraintActive: Option<bool>,
    m_Sources: Vec<ConstraintSource>,
    m_Active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LowerResBlitTexture {
    m_Name: String,
    m_ForcedFallbackFormat: Option<i32>,
    m_DownscaleFallback: Option<bool>,
    m_IsAlphaChannelOptional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lumin {
    depthFormat: i32,
    frameTiming: i32,
    enableGLCache: bool,
    glCacheMaxBlobSize: u32,
    glCacheMaxFileSize: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MasterServerInterface {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Material {
    m_Name: String,
    m_Shader: PPtr,
    m_SavedProperties: UnityPropertySheet,
    m_ShaderKeywords: Option<(Option<Vec<String>>, Option<String>)>,
    m_CustomRenderQueue: Option<i32>,
    m_LightmapFlags: Option<u32>,
    stringTagMap: Option<Vec<(String, String)>>,
    m_EnableInstancingVariants: Option<bool>,
    disabledShaderPasses: Option<Vec<String>>,
    m_DoubleSidedGI: Option<bool>,
    m_BuildTextureStacks: Option<Vec<BuildTextureStackReference>>,
    m_ValidKeywords: Option<Vec<String>>,
    m_InvalidKeywords: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaterialImportOutput {
    currentSettings: BuildTargetSettings,
    baked: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaterialInstanceSettings {
    name: String,
    prototypeName: String,
    shaderName: Option<String>,
    inputs: Vec<InputImportSettings>,
    materialInformation: ProceduralMaterialInformation,
    materialProperties: UnityPropertySheet,
    textureParameters: Vec<InputImportSettings>,
    buildTargetSettings: Vec<BuildTargetSettings>,
    shader: Option<PPtr>,
    textureAssignments: Option<Vec<ProceduralTextureAssignment>>,
    shaderKeywords: Option<String>,
    renderQueue: Option<i32>,
    lightmapFlags: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Matrix3x4f {
    e00: f32,
    e01: f32,
    e02: f32,
    e03: f32,
    e10: f32,
    e11: f32,
    e12: f32,
    e13: f32,
    e20: f32,
    e21: f32,
    e22: f32,
    e23: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Matrix4x4f {
    e00: f32,
    e01: f32,
    e02: f32,
    e03: f32,
    e10: f32,
    e11: f32,
    e12: f32,
    e13: f32,
    e20: f32,
    e21: f32,
    e22: f32,
    e23: f32,
    e30: f32,
    e31: f32,
    e32: f32,
    e33: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatrixParameter {
    m_NameIndex: i32,
    m_Index: i32,
    m_ArraySize: i32,
    m_Type: i8,
    m_RowCount: i8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MdFour {
    md4_hash: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemorySettings {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mesh {
    m_Name: String,
    m_Use16BitIndices: Option<i32>,
    m_SubMeshes: Vec<SubMesh>,
    m_MeshCompression: u8,
    m_IndexBuffer: Vec<u8>,
    m_Vertices: Option<Vec<Vector3f>>,
    m_Skin: Option<(Option<Vec<BoneInfluence>>, Option<Vec<BoneWeights4>>)>,
    m_BindPose: Vec<Matrix4x4f>,
    m_UV: Option<Vec<Vector2f>>,
    m_UV1: Option<Vec<Vector2f>>,
    m_Tangents: Option<Vec<Vector4f>>,
    m_Normals: Option<Vec<Vector3f>>,
    m_CompressedMesh: CompressedMesh,
    m_LocalAABB: AABB,
    m_Colors: Option<Vec<ColorRGBA>>,
    m_CollisionTriangles: Option<Vec<u32>>,
    m_CollisionVertexCount: Option<i32>,
    m_MeshUsageFlags: i32,
    m_VertexData: Option<VertexData>,
    m_StreamCompression: Option<u8>,
    m_IsReadable: Option<bool>,
    m_KeepVertices: Option<bool>,
    m_KeepIndices: Option<bool>,
    m_Shapes: Option<(Option<Vec<MeshBlendShape>>, Option<BlendShapeData>)>,
    m_ShapeVertices: Option<Vec<MeshBlendShapeVertex>>,
    m_BoneNameHashes: Option<Vec<u32>>,
    m_RootBoneNameHash: Option<u32>,
    m_BakedConvexCollisionMesh: Option<Vec<u8>>,
    m_BakedTriangleCollisionMesh: Option<Vec<u8>>,
    m_IndexFormat: Option<i32>,
    m_MeshMetrics: Option<(f32, f32)>,
    m_StreamData: Option<StreamingInfo>,
    m_BonesAABB: Option<Vec<MinMaxAABB>>,
    m_VariableBoneCountWeights: Option<VariableBoneCountWeights>,
    m_CookingOptions: Option<i32>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Mesh3DSImporter {
//     m_Name: String,
//     m_FileIDToRecycleName: Option<(Option<Vec<(i64, String)>>, Option<Vec<(i32, String)>>)>,
//     m_OldHashIdentity: Option<MdFour>,
//     m_NewHashIdentity: Option<MdFour>,
//     m_GenerateMaterials: Option<i32>,
//     m_GenerateAnimations: Option<i32>,
//     m_GlobalScale: f32,
//     m_BakeSimulation: bool,
//     m_SplitAnimations: Option<bool>,
//     m_AddColliders: bool,
//     m_UseFileUnits: bool,
//     m_AnimationCompression: i32,
//     m_AnimationRotationError: f32,
//     m_AnimationPositionError: f32,
//     m_AnimationScaleError: f32,
//     m_MeshCompression: i32,
//     m_MeshSettings_swapUVChannels: Option<bool>,
//     m_MeshSettings_generateSecondaryUV: Option<bool>,
//     m_MeshSettings_secondaryUVAngleDistortion: Option<f32>,
//     m_MeshSettings_secondaryUVAreaDistortion: Option<f32>,
//     m_MeshSettings_secondaryUVHardAngle: Option<f32>,
//     m_MeshSettings_secondaryUVPackMargin: Option<f32>,
//     m_MeshSettings_normalImportMode: Option<i32>,
//     m_MeshSettings_tangentImportMode: Option<i32>,
//     m_AnimationWrapMode: i32,
//     m_ClipAnimations: Vec<ClipAnimationInfo>,
//     m_FirstImportVersion: Option<i32>,
//     normalSmoothAngle: f32,
//     splitTangentsAcrossUV: Option<bool>,
//     m_ImportedRoots: Vec<PPtr>,
//     m_HasExtraRoot: bool,
//     m_ImportMaterials: Option<bool>,
//     m_MaterialName: Option<i32>,
//     m_MaterialSearch: Option<i32>,
//     m_LODScreenPercentages: Option<Vec<f32>>,
//     swapUVChannels: Option<bool>,
//     generateSecondaryUV: Option<bool>,
//     optimizeMesh: Option<bool>,
//     secondaryUVAngleDistortion: Option<f32>,
//     secondaryUVAreaDistortion: Option<f32>,
//     secondaryUVHardAngle: Option<f32>,
//     secondaryUVPackMargin: Option<f32>,
//     normalImportMode: Option<i32>,
//     tangentImportMode: Option<i32>,
//     m_LegacyGenerateAnimations: Option<i32>,
//     m_IsReadable: Option<bool>,
//     optimizeMeshForGPU: Option<bool>,
//     m_ImportedTakeInfos: Option<Vec<TakeInfo>>,
//     m_ReferencedClips: Option<Vec<GUID>>,
//     m_ImportAnimation: Option<bool>,
//     m_CopyAvatar: Option<bool>,
//     m_HumanDescription: Option<HumanDescription>,
//     m_LastHumanDescriptionAvatarSource: Option<PPtr>,
//     m_AdditionalBone: Option<bool>,
//     m_AnimationType: Option<i32>,
//     m_UserData: Option<String>,
//     m_ImportBlendShapes: Option<bool>,
//     weldVertices: Option<bool>,
//     m_OptimizeGameObjects: Option<bool>,
//     m_ExtraExposedTransformPaths: Option<Vec<String>>,
//     m_MotionNodeName: Option<String>,
//     keepQuads: Option<bool>,
//     m_UseFileScale: Option<bool>,
//     m_FileScale: Option<f32>,
//     m_AssetBundleName: Option<String>,
//     m_AssetBundleVariant: Option<String>,
//     m_AnimationImportErrors: Option<String>,
//     m_AnimationImportWarnings: Option<String>,
//     m_AnimationRetargetingWarnings: Option<String>,
//     m_AnimationDoRetargetingWarnings: Option<bool>,
//     m_HumanoidOversampling: Option<i32>,
//     m_ResampleRotations: Option<bool>,
//     m_ResampleCurves: Option<bool>,
//     m_RigImportErrors: Option<String>,
//     m_RigImportWarnings: Option<String>,
//     m_ImportVisibility: Option<bool>,
//     m_ImportCameras: Option<bool>,
//     m_ImportLights: Option<bool>,
//     normalCalculationMode: Option<i32>,
//     m_ExtraUserProperties: Option<Vec<String>>,
//     m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
//     m_AutoMapExternalMaterials: Option<bool>,
//     m_MaterialLocation: Option<i32>,
//     m_Materials: Option<Vec<SourceAssetIdentifier>>,
//     m_ImportAnimatedCustomProperties: Option<bool>,
//     m_HasEmbeddedTextures: Option<bool>,
//     m_SupportsEmbeddedMaterials: Option<bool>,
//     m_PreserveHierarchy: Option<bool>,
//     indexFormat: Option<i32>,
//     m_ImportConstraints: Option<bool>,
//     m_PreviousCalculatedGlobalScale: Option<f32>,
//     m_HasPreviousCalculatedGlobalScale: Option<bool>,
//     m_UseSRGBMaterialColor: Option<bool>,
//     m_FileScaleUnit: Option<String>,
//     m_FileScaleFactor: Option<f32>,
//     legacyComputeAllNormalsFromSmoothingGroupsWhenMeshHasBlendShapes: Option<bool>,
//     blendShapeNormalImportMode: Option<i32>,
//     normalSmoothingSource: Option<i32>,
//     m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
//     m_UsedFileIDs: Option<Vec<i64>>,
//     skinWeightsMode: Option<i32>,
//     maxBonesPerVertex: Option<i32>,
//     minBoneWeight: Option<f32>,
//     meshOptimizationFlags: Option<i32>,
//     m_SortHierarchyByName: Option<bool>,
//     m_AvatarSetup: Option<i32>,
//     m_MaterialImportMode: Option<i32>,
//     m_AutoGenerateAvatarMappingIfUnspecified: Option<bool>,
//     m_FileIdsGeneration: Option<i32>,
//     secondaryUVMarginMethod: Option<i32>,
//     secondaryUVMinLightmapResolution: Option<f32>,
//     secondaryUVMinObjectScale: Option<f32>,
//     bakeAxisConversion: Option<bool>,
//     m_AddHumanoidExtraRootOnlyWhenUsingAvatar: Option<bool>,
//     optimizeBones: Option<bool>,
//     m_RemoveConstantScaleCurves: Option<bool>,
//     m_NodeNameCollisionStrategy: Option<i32>,
//     m_StrictVertexDataChecks: Option<bool>,
//     m_ImportBlendShapeDeformPercent: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshBlendShape {
    name: Option<String>,
    firstVertex: u32,
    vertexCount: u32,
    aabbMinDelta: Option<Vector3f>,
    aabbMaxDelta: Option<Vector3f>,
    hasNormals: bool,
    hasTangents: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshBlendShapeChannel {
    name: String,
    nameHash: u32,
    frameIndex: i32,
    frameCount: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshBlendShapeVertex {
    vertex: Vector3f,
    normal: Vector3f,
    tangent: Vector3f,
    index: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshCollider {
    m_GameObject: PPtr,
    m_Material: PPtr,
    m_IsTrigger: bool,
    m_SmoothSphereCollisions: Option<bool>,
    m_Convex: bool,
    m_Mesh: PPtr,
    m_Enabled: Option<bool>,
    m_InflateMesh: Option<bool>,
    m_SkinWidth: Option<f32>,
    m_CookingOptions: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshFilter {
    m_GameObject: PPtr,
    m_Mesh: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshParticleEmitter {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_Emit: bool,
    minSize: f32,
    maxSize: f32,
    minEnergy: f32,
    maxEnergy: f32,
    minEmission: f32,
    maxEmission: f32,
    worldVelocity: Vector3f,
    localVelocity: Vector3f,
    rndVelocity: Vector3f,
    emitterVelocityScale: f32,
    tangentVelocity: Vector3f,
    angularVelocity: f32,
    rndAngularVelocity: f32,
    rndRotation: bool,
    Simulate_in_Worldspace: bool,
    m_OneShot: bool,
    m_InterpolateTriangles: bool,
    m_Systematic: bool,
    m_MinNormalVelocity: f32,
    m_MaxNormalVelocity: f32,
    m_Mesh: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshRenderer {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: (Option<u8>, Option<bool>),
    m_ReceiveShadows: (Option<u8>, Option<bool>),
    m_LightmapIndex: (Option<u8>, Option<u16>),
    m_LightmapTilingOffset: Vector4f,
    m_Materials: Vec<PPtr>,
    m_SubsetIndices: Option<Vec<u32>>,
    m_StaticBatchRoot: PPtr,
    m_UseLightProbes: Option<bool>,
    m_LightProbeAnchor: Option<PPtr>,
    m_SortingLayer: Option<i16>,
    m_SortingOrder: Option<i16>,
    m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    m_LightmapIndexDynamic: Option<u16>,
    m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    m_ProbeAnchor: Option<PPtr>,
    m_AdditionalVertexStreams: Option<PPtr>,
    m_MotionVectors: Option<u8>,
    m_LightProbeUsage: Option<u8>,
    m_LightProbeVolumeOverride: Option<PPtr>,
    m_StaticBatchInfo: Option<StaticBatchInfo>,
    m_DynamicOccludee: Option<u8>,
    m_RenderingLayerMask: Option<u32>,
    m_RendererPriority: Option<i32>,
    m_RayTracingMode: Option<u8>,
    m_EnlightenVertexStream: Option<PPtr>,
    m_RayTraceProcedural: Option<u8>,
    m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinMaxAABB {
    m_Min: Vector3f,
    m_Max: Vector3f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinMaxCurve {
    scalar: f32,
    maxCurve: AnimationCurve,
    minCurve: AnimationCurve,
    minMaxState: (Option<i16>, Option<u16>),
    minScalar: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinMaxGradient {
    maxGradient: (Option<Gradient>, Option<GradientNEW>),
    minGradient: (Option<Gradient>, Option<GradientNEW>),
    minColor: ColorRGBA,
    maxColor: ColorRGBA,
    minMaxState: (Option<i16>, Option<u16>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelImporter {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Module {
    name: String,
    dependencies: Vec<String>,
    strippable: bool,
    controlledByBuiltinPackage: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonoAssemblyImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_ExecutionOrder: Vec<(String, i32)>,
    m_IconMap: Vec<(String, PPtr)>,
    m_UserData: Option<String>,
    m_IsPreloaded: Option<bool>,
    m_PlatformData: Option<(
        Option<Vec<(String, PlatformSettingsData)>>,
        Option<Vec<((String, String), PlatformSettingsData)>>,
    )>,
    m_Output: Option<PluginImportOutput>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_IsOverridable: Option<bool>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_DefineConstraints: Option<Vec<String>>,
    m_IsExplicitlyReferenced: Option<bool>,
    m_ValidateReferences: Option<bool>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonoBehaviour {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Script: PPtr,
    m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonoImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_DefaultReferences: Vec<(String, PPtr)>,
    executionOrder: i16,
    icon: PPtr,
    m_UserData: Option<String>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonoManager {
    m_Scripts: Vec<PPtr>,
    m_AssemblyNames: Option<Vec<String>>,
    m_AssemblyTypes: Option<Vec<i32>>,
    m_ScriptHashes: Option<Vec<(Hash128, Hash128)>>,
    m_RuntimeClassHashes: Option<Vec<(i32, Hash128)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonoObject {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonoScript {
    m_Name: String,
    m_ClassName: String,
    m_Namespace: String,
    m_AssemblyName: String,
    m_IsEditorScript: Option<bool>,
    m_ExecutionOrder: Option<i32>,
    m_PropertiesHash: Option<(Option<u32>, Option<Hash128>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Motion {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MotionNeighborList {
    m_NeighborArray: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MovieImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_Quality: f32,
    m_LinearTexture: Option<bool>,
    m_UserData: Option<String>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MovieTexture {
    m_Name: String,
    m_Loop: Option<bool>,
    m_AudioClip: Option<PPtr>,
    m_MovieData: Option<Vec<u8>>,
    m_ColorSpace: Option<i32>,
    m_ForcedFallbackFormat: Option<i32>,
    m_DownscaleFallback: Option<bool>,
    m_IsAlphaChannelOptional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiArtifactTestImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UsedFileIDs: Vec<i64>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiModeParameter {
    value: Option<f32>,
    mode: i32,
    spread: f32,
    speed: MinMaxCurve,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NScreenBridge {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameToObjectMap {
    m_ObjectToName: Vec<(PPtr, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NamedObject {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NativeFormatImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_UserData: Option<String>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_MainObjectFileID: Option<i64>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NativeObjectType {
    m_Inner: NativeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NativeType {
    a: i32,
    b: f32,
    embedded: EmbeddedNativeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMesh {
    m_Name: String,
    m_MeshData: Option<Vec<u8>>,
    m_Heightmaps: Option<Vec<HeightmapData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshAgent {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Radius: f32,
    m_Speed: f32,
    m_Acceleration: f32,
    m_AngularSpeed: f32,
    m_StoppingDistance: f32,
    m_AutoTraverseOffMeshLink: bool,
    m_AutoRepath: bool,
    m_Height: f32,
    m_BaseOffset: f32,
    m_WalkableMask: u32,
    m_ObstacleAvoidanceType: i32,
    avoidancePriority: Option<i32>,
    m_AutoBraking: Option<bool>,
    m_AgentTypeID: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshAreaData {
    name: String,
    cost: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshBuildDebugSettings {
    m_Flags: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshBuildSettings {
    agentTypeID: i32,
    agentRadius: f32,
    agentHeight: f32,
    agentSlope: f32,
    agentClimb: f32,
    ledgeDropHeight: f32,
    maxJumpAcrossDistance: f32,
    minRegionArea: f32,
    manualCellSize: (Option<bool>, Option<i32>),
    cellSize: f32,
    manualTileSize: (Option<bool>, Option<i32>),
    tileSize: i32,
    accuratePlacement: (Option<bool>, Option<i32>),
    debug: Option<NavMeshBuildDebugSettings>,
    maxJobWorkers: Option<u32>,
    keepTiles: Option<i32>,
    preserveTilesOutsideBounds: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshData {
    m_Name: String,
    m_NavMeshTiles: Vec<NavMeshTileData>,
    m_NavMeshParams: Option<NavMeshParams>,
    m_Heightmaps: Vec<HeightmapData>,
    m_HeightMeshes: Vec<HeightMeshData>,
    m_OffMeshLinks: Vec<AutoOffMeshLinkData>,
    m_NavMeshBuildSettings: Option<NavMeshBuildSettings>,
    m_SourceBounds: Option<AABB>,
    m_Rotation: Option<Quaternionf>,
    m_Position: Option<Vector3f>,
    m_AgentTypeID: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshLayerData {
    name: String,
    cost: f32,
    editType: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshLayers {
    Built_in_Layer_0: Option<NavMeshLayerData>,
    Built_in_Layer_1: Option<NavMeshLayerData>,
    Built_in_Layer_2: Option<NavMeshLayerData>,
    User_Layer_0: Option<NavMeshLayerData>,
    User_Layer_1: Option<NavMeshLayerData>,
    User_Layer_2: Option<NavMeshLayerData>,
    User_Layer_3: Option<NavMeshLayerData>,
    User_Layer_4: Option<NavMeshLayerData>,
    User_Layer_5: Option<NavMeshLayerData>,
    User_Layer_6: Option<NavMeshLayerData>,
    User_Layer_7: Option<NavMeshLayerData>,
    User_Layer_8: Option<NavMeshLayerData>,
    User_Layer_9: Option<NavMeshLayerData>,
    User_Layer_10: Option<NavMeshLayerData>,
    User_Layer_11: Option<NavMeshLayerData>,
    User_Layer_12: Option<NavMeshLayerData>,
    User_Layer_13: Option<NavMeshLayerData>,
    User_Layer_14: Option<NavMeshLayerData>,
    User_Layer_15: Option<NavMeshLayerData>,
    User_Layer_16: Option<NavMeshLayerData>,
    User_Layer_17: Option<NavMeshLayerData>,
    User_Layer_18: Option<NavMeshLayerData>,
    User_Layer_19: Option<NavMeshLayerData>,
    User_Layer_20: Option<NavMeshLayerData>,
    User_Layer_21: Option<NavMeshLayerData>,
    User_Layer_22: Option<NavMeshLayerData>,
    User_Layer_23: Option<NavMeshLayerData>,
    User_Layer_24: Option<NavMeshLayerData>,
    User_Layer_25: Option<NavMeshLayerData>,
    User_Layer_26: Option<NavMeshLayerData>,
    User_Layer_27: Option<NavMeshLayerData>,
    User_Layer_28: Option<NavMeshLayerData>,
    areas: Option<Vec<NavMeshAreaData>>,
    m_LastAgentTypeID: Option<i32>,
    m_Settings: Option<Vec<NavMeshBuildSettings>>,
    m_SettingNames: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshObstacle {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Radius: Option<f32>,
    m_Height: Option<f32>,
    m_MoveThreshold: Option<f32>,
    m_Carve: Option<bool>,
    m_Shape: Option<i32>,
    m_Extents: Option<Vector3f>,
    m_CarveOnlyStationary: Option<bool>,
    m_Center: Option<Vector3f>,
    m_TimeToStationary: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshParams {
    tileSize: f32,
    walkableHeight: f32,
    walkableRadius: f32,
    walkableClimb: f32,
    cellSize: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshSettings {
    m_NavMesh: Option<PPtr>,
    m_NavMeshData: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshTileData {
    m_MeshData: Vec<u8>,
    m_Hash: Option<Hash128>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkManager {
    m_DebugLevel: i32,
    m_Sendrate: f32,
    m_AssetToPrefab: Vec<(GUID, PPtr)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkView {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_StateSynchronization: i32,
    m_Observed: PPtr,
    m_ViewID: NetworkViewID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkViewID {
    m_ID: u32,
    m_Type: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewAnimationTrack {
    m_ObjectHideFlags: Option<u32>,
    m_ExtensionPtr: Option<PPtr>,
    m_Name: String,
    m_Curves: Vec<Channel>,
    m_ClassID: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    m_ParentId: i32,
    m_AxesId: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoiseModule {
    enabled: bool,
    strength: MinMaxCurve,
    strengthY: MinMaxCurve,
    strengthZ: MinMaxCurve,
    separateAxes: bool,
    frequency: f32,
    damping: bool,
    octaves: i32,
    octaveMultiplier: f32,
    octaveScale: f32,
    quality: i32,
    scrollSpeed: MinMaxCurve,
    remap: MinMaxCurve,
    remapY: MinMaxCurve,
    remapZ: MinMaxCurve,
    remapEnabled: bool,
    positionAmount: Option<MinMaxCurve>,
    rotationAmount: Option<MinMaxCurve>,
    sizeAmount: Option<MinMaxCurve>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NonAlignedStruct {
    m_Bool: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcclusionArea {
    m_GameObject: PPtr,
    m_Size: Vector3f,
    m_Center: Vector3f,
    m_IsViewVolume: bool,
    m_IsTargetVolume: Option<bool>,
    m_TargetResolution: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcclusionCullingData {
    m_Name: String,
    m_PVSData: Vec<u8>,
    m_Scenes: Vec<OcclusionScene>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcclusionPortal {
    m_GameObject: PPtr,
    m_Open: bool,
    m_Center: Vector3f,
    m_Size: Vector3f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcclusionScene {
    indexRenderers: i32,
    sizeRenderers: i32,
    indexPortals: i32,
    sizePortals: i32,
    scene: GUID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Oculus {
    sharedDepthBuffer: bool,
    dashSupport: bool,
    lowOverheadMode: Option<bool>,
    protectedContext: Option<bool>,
    v2Signing: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OffMeshLink {
    m_GameObject: PPtr,
    m_Start: PPtr,
    m_End: PPtr,
    m_DtPolyRef: Option<u32>,
    m_CostOverride: f32,
    m_BiDirectional: bool,
    m_Activated: bool,
    m_NavMeshLayer: Option<u32>,
    m_Enabled: Option<u8>,
    m_AutoUpdatePositions: Option<bool>,
    m_AreaIndex: Option<u32>,
    m_AgentTypeID: Option<i32>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct OffsetPtr {
//     data: Box<(
//         Option<Skeleton>,
//         Option<StateConstant>,
//         Option<Hand>,
//         Option<ValueArray>,
//         Option<Blend1dDataConstant>,
//         Option<TransitionConstant>,
//         Option<SkeletonPose>,
//         Option<SkeletonMask>,
//         Option<Clip>,
//         Option<ConditionConstant>,
//         Option<ValueArrayConstant>,
//         Option<LayerConstant>,
//         Option<HumanLayerConstant>,
//         Option<Blend2dDataConstant>,
//         Option<Human>,
//         Option<SelectorTransitionConstant>,
//         Option<BlendDirectDataConstant>,
//         Option<BlendTreeNodeConstant>,
//         Option<SelectorStateConstant>,
//         Option<StateMachineConstant>,
//         Option<BlendTreeConstant>,
//     )>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    previewData: Option<Vec<f32>>,
    importedType: Option<i32>,
    hasEmptyFontData: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PPtrCurve {
    curve: Vec<PPtrKeyframe>,
    attribute: String,
    path: String,
    classID: i32,
    script: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PPtrKeyframe {
    time: f32,
    value: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PVRImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_UserData: Option<String>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageManifest {
    m_Name: String,
    m_Script: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageManifestImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UsedFileIDs: Vec<i64>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackedAssets {
    m_File: Option<u32>,
    m_ShortPath: String,
    m_Overhead: u64,
    m_Contents: Vec<BuildReportPackedAssetInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackedBitVector {
    m_NumItems: u32,
    m_Range: Option<f32>,
    m_Start: Option<f32>,
    m_Data: Vec<u8>,
    m_BitSize: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackingSettings {
    padding: i32,
    blockOffset: i32,
    allowAlphaSplitting: bool,
    enableRotation: bool,
    enableTightPacking: bool,
    enableAlphaDilation: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parameter {
    m_ParameterName: String,
    m_GUID: GUID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParentConstraint {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Weight: f32,
    m_TranslationAtRest: Vector3f,
    m_RotationAtRest: Vector3f,
    m_TranslationOffsets: Vec<Vector3f>,
    m_RotationOffsets: Vec<Vector3f>,
    m_AffectTranslationX: bool,
    m_AffectTranslationY: bool,
    m_AffectTranslationZ: bool,
    m_AffectRotationX: bool,
    m_AffectRotationY: bool,
    m_AffectRotationZ: bool,
    m_IsContraintActive: Option<bool>,
    m_Sources: Vec<ConstraintSource>,
    m_Active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParserBindChannels {
    m_Channels: Vec<ShaderBindChannel>,
    m_SourceMap: (Option<u32>, Option<i32>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleAnimator {
    m_GameObject: PPtr,
    Does_Animate_Color: bool,
    worldRotationAxis: Vector3f,
    localRotationAxis: Vector3f,
    sizeGrow: f32,
    rndForce: Vector3f,
    force: Vector3f,
    damping: f32,
    stopSimulation: bool,
    autodestruct: bool,
    colorAnimation: (ColorRGBA, ColorRGBA, ColorRGBA, ColorRGBA, ColorRGBA),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleEmitter {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleRenderer {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: (Option<u8>, Option<bool>),
    m_ReceiveShadows: (Option<u8>, Option<bool>),
    m_LightmapIndex: (Option<u8>, Option<u16>),
    m_LightmapTilingOffset: Vector4f,
    m_Materials: Vec<PPtr>,
    m_SubsetIndices: Option<Vec<u32>>,
    m_StaticBatchRoot: PPtr,
    m_CameraVelocityScale: f32,
    m_StretchParticles: i32,
    m_LengthScale: f32,
    m_VelocityScale: f32,
    m_MaxParticleSize: f32,
    UV_Animation: UVAnimation,
    m_UseLightProbes: Option<bool>,
    m_LightProbeAnchor: Option<PPtr>,
    m_SortingLayer: Option<i16>,
    m_SortingOrder: Option<i16>,
    m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    m_LightmapIndexDynamic: Option<u16>,
    m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    m_ProbeAnchor: Option<PPtr>,
    m_MotionVectors: Option<u8>,
    m_LightProbeUsage: Option<u8>,
    m_LightProbeVolumeOverride: Option<PPtr>,
    m_StaticBatchInfo: Option<StaticBatchInfo>,
    m_DynamicOccludee: Option<u8>,
    m_RenderingLayerMask: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleSystem {
    m_GameObject: PPtr,
    lengthInSec: f32,
    startDelay: (Option<f32>, Option<MinMaxCurve>),
    speed: Option<f32>,
    randomSeed: (Option<u32>, Option<i32>),
    looping: bool,
    prewarm: bool,
    playOnAwake: bool,
    moveWithTransform: (Option<bool>, Option<i32>),
    InitialModule: InitialModule,
    ShapeModule: ShapeModule,
    EmissionModule: EmissionModule,
    SizeModule: SizeModule,
    RotationModule: RotationModule,
    ColorModule: ColorModule,
    UVModule: UVModule,
    VelocityModule: VelocityModule,
    ForceModule: ForceModule,
    ClampVelocityModule: ClampVelocityModule,
    SizeBySpeedModule: SizeBySpeedModule,
    RotationBySpeedModule: RotationBySpeedModule,
    ColorBySpeedModule: ColorBySpeedModule,
    CollisionModule: CollisionModule,
    SubModule: SubModule,
    ExternalForcesModule: Option<ExternalForcesModule>,
    scalingMode: Option<i32>,
    InheritVelocityModule: Option<InheritVelocityModule>,
    TriggerModule: Option<TriggerModule>,
    autoRandomSeed: Option<bool>,
    moveWithCustomTransform: Option<PPtr>,
    NoiseModule: Option<NoiseModule>,
    LightsModule: Option<LightsModule>,
    TrailModule: Option<TrailModule>,
    simulationSpeed: Option<f32>,
    CustomDataModule: Option<CustomDataModule>,
    useUnscaledTime: Option<bool>,
    useRigidbodyForVelocity: Option<bool>,
    stopAction: Option<i32>,
    cullingMode: Option<i32>,
    ringBufferMode: Option<i32>,
    ringBufferLoopRange: Option<Vector2f>,
    LifetimeByEmitterSpeedModule: Option<LifetimeByEmitterSpeedModule>,
    emitterVelocityMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleSystemEmissionBurst {
    time: f32,
    minCount: Option<u32>,
    maxCount: Option<u32>,
    cycleCount: (Option<u32>, Option<i32>),
    repeatInterval: f32,
    countCurve: Option<MinMaxCurve>,
    probability: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleSystemForceField {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Parameters: ParticleSystemForceFieldParameters,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleSystemForceFieldParameters {
    m_Shape: i32,
    m_StartRange: f32,
    m_EndRange: f32,
    m_Length: f32,
    m_GravityFocus: f32,
    m_RotationRandomness: Vector2f,
    m_DirectionCurveX: MinMaxCurve,
    m_DirectionCurveY: MinMaxCurve,
    m_DirectionCurveZ: MinMaxCurve,
    m_GravityCurve: MinMaxCurve,
    m_RotationSpeedCurve: MinMaxCurve,
    m_RotationAttractionCurve: MinMaxCurve,
    m_DragCurve: MinMaxCurve,
    m_VectorField: PPtr,
    m_VectorFieldSpeedCurve: MinMaxCurve,
    m_VectorFieldAttractionCurve: MinMaxCurve,
    m_MultiplyDragByParticleSize: bool,
    m_MultiplyDragByParticleVelocity: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleSystemRenderer {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: (Option<u8>, Option<bool>),
    m_ReceiveShadows: (Option<u8>, Option<bool>),
    m_LightmapIndex: (Option<u8>, Option<u16>),
    m_LightmapTilingOffset: Vector4f,
    m_Materials: Vec<PPtr>,
    m_SubsetIndices: Option<Vec<u32>>,
    m_StaticBatchRoot: PPtr,
    m_UseLightProbes: Option<bool>,
    m_LightProbeAnchor: Option<PPtr>,
    m_RenderMode: (Option<u16>, Option<i32>),
    m_MaxParticleSize: f32,
    m_CameraVelocityScale: f32,
    m_VelocityScale: f32,
    m_LengthScale: f32,
    m_SortingFudge: f32,
    m_SortMode: (Option<u8>, Option<u16>, Option<i32>),
    m_Mesh: PPtr,
    m_NormalDirection: Option<f32>,
    m_Mesh1: Option<PPtr>,
    m_Mesh2: Option<PPtr>,
    m_Mesh3: Option<PPtr>,
    m_SortingLayer: Option<i16>,
    m_SortingOrder: Option<i16>,
    m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    m_LightmapIndexDynamic: Option<u16>,
    m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    m_ProbeAnchor: Option<PPtr>,
    m_MinParticleSize: Option<f32>,
    m_RenderAlignment: Option<i32>,
    m_Pivot: Option<Vector3f>,
    m_MotionVectors: Option<u8>,
    m_LightProbeUsage: Option<u8>,
    m_LightProbeVolumeOverride: Option<PPtr>,
    m_StaticBatchInfo: Option<StaticBatchInfo>,
    m_UseCustomVertexStreams: Option<bool>,
    m_VertexStreamMask: Option<i32>,
    m_VertexStreams: Option<Vec<u8>>,
    m_MaskInteraction: Option<i32>,
    m_DynamicOccludee: Option<u8>,
    m_RenderingLayerMask: Option<u32>,
    m_EnableGPUInstancing: Option<bool>,
    m_ApplyActiveColorSpace: Option<bool>,
    m_RendererPriority: Option<i32>,
    m_ShadowBias: Option<f32>,
    m_Flip: Option<Vector3f>,
    m_AllowRoll: Option<bool>,
    m_RayTracingMode: Option<u8>,
    m_FreeformStretching: Option<bool>,
    m_RotateWithStretchDirection: Option<bool>,
    m_RayTraceProcedural: Option<u8>,
    m_StaticShadowCaster: Option<u8>,
    m_MeshDistribution: Option<u8>,
    m_MeshWeighting: Option<f32>,
    m_MeshWeighting1: Option<f32>,
    m_MeshWeighting2: Option<f32>,
    m_MeshWeighting3: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerLODSettings {
    height: f32,
    castShadows: bool,
    receiveShadows: bool,
    useLightProbes: bool,
    reflectionProbeUsage: i32,
    enableBump: bool,
    enableHue: bool,
    windQuality: i32,
    enableSubsurface: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerformanceReportingManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerformanceReportingSettings {
    m_Enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicMaterial {
    m_Name: String,
    dynamicFriction: f32,
    staticFriction: f32,
    bounciness: f32,
    frictionCombine: i32,
    bounceCombine: i32,
    frictionDirection2: Option<Vector3f>,
    dynamicFriction2: Option<f32>,
    staticFriction2: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Physics2DSettings {
    m_Gravity: Vector2f,
    m_DefaultMaterial: PPtr,
    m_VelocityIterations: i32,
    m_PositionIterations: i32,
    m_RaycastsHitTriggers: Option<bool>,
    m_LayerCollisionMatrix: Vec<u32>,
    m_VelocityThreshold: Option<f32>,
    m_MaxLinearCorrection: Option<f32>,
    m_MaxAngularCorrection: Option<f32>,
    m_MaxTranslationSpeed: Option<f32>,
    m_MaxRotationSpeed: Option<f32>,
    m_BaumgarteScale: Option<f32>,
    m_BaumgarteTimeOfImpactScale: Option<f32>,
    m_TimeToSleep: Option<f32>,
    m_LinearSleepTolerance: Option<f32>,
    m_AngularSleepTolerance: Option<f32>,
    m_DeleteStopsCallbacks: Option<bool>,
    m_MinPenetrationForPenalty: Option<f32>,
    m_RaycastsStartInColliders: Option<bool>,
    m_ChangeStopsCallbacks: Option<bool>,
    m_QueriesHitTriggers: Option<bool>,
    m_QueriesStartInColliders: Option<bool>,
    m_DefaultContactOffset: Option<f32>,
    m_CallbacksOnDisable: Option<bool>,
    m_AutoSimulation: Option<bool>,
    m_AutoSyncTransforms: Option<bool>,
    m_JobOptions: Option<PhysicsJobOptions2D>,
    m_ReuseCollisionCallbacks: Option<bool>,
    m_SimulationMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsJobOptions2D {
    m_UseMultithreading: Option<bool>,
    m_UseConsistencySorting: Option<bool>,
    m_InterpolationPosesPerJob: i32,
    m_NewContactsPerJob: i32,
    m_CollideContactsPerJob: i32,
    m_ClearFlagsPerJob: i32,
    m_ClearBodyForcesPerJob: i32,
    m_SyncDiscreteFixturesPerJob: i32,
    m_SyncContinuousFixturesPerJob: i32,
    m_FindNearestContactsPerJob: i32,
    m_UpdateTriggerContactsPerJob: i32,
    m_IslandSolverCostThreshold: i32,
    m_IslandSolverBodyCostScale: i32,
    m_IslandSolverContactCostScale: i32,
    m_IslandSolverJointCostScale: i32,
    m_IslandSolverBodiesPerJob: i32,
    m_IslandSolverContactsPerJob: i32,
    useMultithreading: Option<bool>,
    useConsistencySorting: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsManager {
    m_Gravity: Vector3f,
    m_DefaultMaterial: PPtr,
    m_BounceThreshold: f32,
    m_SleepVelocity: Option<f32>,
    m_SleepAngularVelocity: Option<f32>,
    m_MaxAngularVelocity: Option<f32>,
    m_MinPenetrationForPenalty: Option<f32>,
    m_SolverIterationCount: Option<i32>,
    m_RaycastsHitTriggers: Option<bool>,
    m_LayerCollisionMatrix: Vec<u32>,
    m_SleepThreshold: Option<f32>,
    m_DefaultContactOffset: Option<f32>,
    m_EnableAdaptiveForce: Option<bool>,
    m_QueriesHitTriggers: Option<bool>,
    m_SolverVelocityIterations: Option<i32>,
    m_DefaultSolverIterations: Option<i32>,
    m_DefaultSolverVelocityIterations: Option<i32>,
    m_QueriesHitBackfaces: Option<bool>,
    m_EnablePCM: Option<bool>,
    m_AutoSimulation: Option<bool>,
    m_AutoSyncTransforms: Option<bool>,
    m_ClothInterCollisionDistance: Option<f32>,
    m_ClothInterCollisionStiffness: Option<f32>,
    m_ContactsGeneration: Option<i32>,
    m_ClothInterCollisionSettingsToggle: Option<bool>,
    m_ContactPairsMode: Option<i32>,
    m_BroadphaseType: Option<i32>,
    m_WorldBounds: Option<AABB>,
    m_WorldSubdivisions: Option<i32>,
    m_FrictionType: Option<i32>,
    m_EnableEnhancedDeterminism: Option<bool>,
    m_EnableUnifiedHeightmaps: Option<bool>,
    m_ReuseCollisionCallbacks: Option<bool>,
    m_DefaultMaxAngluarSpeed: Option<f32>,
    m_ClothGravity: Option<Vector3f>,
    m_DefaultMaxAngularSpeed: Option<f32>,
    m_SolverType: Option<i32>,
    m_DefaultMaxDepenetrationVelocity: Option<f32>,
    m_ImprovedPatchFriction: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsMaterial2D {
    m_Name: String,
    friction: f32,
    bounciness: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsShape {
    m_ShapeType: i32,
    m_Radius: f32,
    m_VertexStartIndex: i32,
    m_VertexCount: i32,
    m_UseAdjacentStart: i32,
    m_UseAdjacentEnd: i32,
    m_AdjacentStart: Vector2f,
    m_AdjacentEnd: Vector2f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsShapeGroup2D {
    m_Shapes: Vec<PhysicsShape>,
    m_Vertices: Vec<Vector2f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsUpdateBehaviour2D {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pipeline {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformEffector2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_ColliderMask: BitField,
    m_OneWay: Option<bool>,
    m_SideFriction: Option<bool>,
    m_SideBounce: Option<bool>,
    m_SideAngleVariance: Option<f32>,
    m_UseOneWay: Option<bool>,
    m_UseSideFriction: Option<bool>,
    m_UseSideBounce: Option<bool>,
    m_UseColliderMask: Option<bool>,
    m_SurfaceArc: Option<f32>,
    m_SideArc: Option<f32>,
    m_UseOneWayGrouping: Option<bool>,
    m_RotationalOffset: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformModuleSetup {
    modules: Vec<Module>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformSettings {
    m_BuildTarget: String,
    m_MaxTextureSize: i32,
    m_TextureFormat: i32,
    m_TextureCompression: i32,
    m_CompressionQuality: i32,
    m_CrunchedCompression: bool,
    m_AllowsAlphaSplitting: bool,
    m_Overridden: bool,
    m_ResizeAlgorithm: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformSettingsData {
    enabled: Option<bool>,
    settings: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformShaderDefines {
    shaderPlatform: i32,
    defines_Tier1: Vec<u32>,
    defines_Tier2: Vec<u32>,
    defines_Tier3: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformShaderSettings {
    useScreenSpaceShadows: Option<bool>,
    useCascadedShadowMaps: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerSettings {
    AndroidLicensePublicKey: Option<String>,
    defaultScreenOrientation: i32,
    targetDevice: i32,
    targetPlatform: Option<i32>,
    targetResolution: Option<i32>,
    Override_IPod_Music: Option<bool>,
    companyName: String,
    productName: String,
    defaultScreenWidth: i32,
    defaultScreenHeight: i32,
    defaultScreenWidthWeb: i32,
    defaultScreenHeightWeb: i32,
    m_RenderingPath: Option<i32>,
    displayResolutionDialog: Option<i32>,
    defaultIsFullScreen: Option<bool>,
    useAlphaInDashboard: Option<bool>,
    runInBackground: bool,
    captureSingleScreen: bool,
    alwaysDisplayWatermark: Option<bool>,
    m_SupportedAspectRatios: AspectRatios,
    firstStreamedLevelWithResources: Option<i32>,
    unityRebuildLibraryVersion: Option<i32>,
    unityForwardCompatibleVersion: Option<i32>,
    unityStandardAssetsVersion: Option<i32>,
    AndroidProfiler: Option<bool>,
    usePlayerLog: Option<bool>,
    allowedAutorotateToPortrait: Option<bool>,
    allowedAutorotateToPortraitUpsideDown: Option<bool>,
    allowedAutorotateToLandscapeRight: Option<bool>,
    allowedAutorotateToLandscapeLeft: Option<bool>,
    useOSAutorotation: Option<bool>,
    use32BitDisplayBuffer: Option<bool>,
    useMacAppStoreValidation: Option<bool>,
    xboxSkinOnGPU: Option<bool>,
    accelerometerFrequency: Option<i32>,
    m_ActiveColorSpace: Option<i32>,
    m_MTRendering: Option<bool>,
    iosShowActivityIndicatorOnLoading: Option<i32>,
    androidShowActivityIndicatorOnLoading: Option<i32>,
    use24BitDepthBuffer: Option<bool>,
    xboxEnableAvatar: Option<bool>,
    xboxEnableKinect: Option<bool>,
    xboxEnableKinectAutoTracking: Option<bool>,
    xboxEnableSpeech: Option<bool>,
    wiiHio2Usage: Option<i32>,
    wiiLoadingScreenRectPlacement: Option<i32>,
    wiiLoadingScreenBackground: Option<ColorRGBA>,
    wiiLoadingScreenPeriod: Option<i32>,
    wiiLoadingScreenFileName: Option<String>,
    wiiLoadingScreenRect: Option<Rectf>,
    debugUnloadMode: Option<i32>,
    targetGlesGraphics: Option<i32>,
    defaultCursor: Option<PPtr>,
    cursorHotspot: Option<Vector2f>,
    m_UseDX11: Option<bool>,
    stripPhysics: Option<bool>,
    resizableWindow: Option<bool>,
    xboxEnableFitness: Option<bool>,
    macFullscreenMode: Option<i32>,
    xboxSpeechDB: Option<u32>,
    enableHWStatistics: Option<bool>,
    xboxEnableHeadOrientation: Option<bool>,
    iPhoneBundleIdentifier: Option<String>,
    defaultIsNativeResolution: Option<bool>,
    Prepare_IOS_For_Recording: Option<bool>,
    forceSingleInstance: Option<bool>,
    gpuSkinning: Option<bool>,
    m_MobileRenderingPath: Option<i32>,
    m_MobileMTRendering: Option<bool>,
    xboxPIXTextureCapture: Option<bool>,
    xboxEnableGuest: Option<bool>,
    metroEnableIndependentInputSource: Option<bool>,
    metroEnableLowLatencyPresentationAPI: Option<bool>,
    m_Stereoscopic3D: Option<bool>,
    videoMemoryForVertexBuffers: Option<i32>,
    d3d9FullscreenMode: Option<i32>,
    visibleInBackground: Option<bool>,
    d3d11ForceExclusiveMode: Option<bool>,
    targetIOSGraphics: Option<i32>,
    cloudProjectId: Option<String>,
    iosAppInBackgroundBehavior: Option<i32>,
    disableDepthAndStencilBuffers: Option<bool>,
    submitAnalytics: Option<bool>,
    bakeCollisionMeshes: Option<bool>,
    d3d11FullscreenMode: Option<i32>,
    xboxOneResolution: Option<i32>,
    ps3SplashScreen: Option<PPtr>,
    psp2PowerMode: Option<i32>,
    psp2AcquireBGM: Option<bool>,
    bundleIdentifier: Option<String>,
    bundleVersion: Option<String>,
    preloadedAssets: Option<Vec<PPtr>>,
    xboxOneDisableKinectGpuReservation: Option<bool>,
    m_ShowUnitySplashScreen: Option<bool>,
    projectId: Option<String>,
    projectName: Option<String>,
    organizationId: Option<String>,
    cloudEnabled: Option<bool>,
    virtualRealitySupported: Option<bool>,
    n3dsDisableStereoscopicView: Option<bool>,
    n3dsEnableSharedListOpt: Option<bool>,
    n3dsEnableVSync: Option<bool>,
    wiiUTVResolution: Option<i32>,
    wiiUGamePadMSAA: Option<i32>,
    wiiUSupportsNunchuk: Option<bool>,
    wiiUSupportsClassicController: Option<bool>,
    wiiUSupportsBalanceBoard: Option<bool>,
    wiiUSupportsMotionPlus: Option<bool>,
    wiiUSupportsProController: Option<bool>,
    wiiUAllowScreenCapture: Option<bool>,
    wiiUControllerCount: Option<i32>,
    useOnDemandResources: Option<bool>,
    iosAllowHTTPDownload: Option<bool>,
    m_VirtualRealitySplashScreen: Option<PPtr>,
    allowFullscreenSwitch: Option<bool>,
    uiUse16BitDepthBuffer: Option<bool>,
    xboxEnablePIXSampling: Option<bool>,
    ignoreAlphaClear: Option<bool>,
    xboxEnableEnableRenderThreadRunsJobs: Option<bool>,
    xboxOneMonoLoggingLevel: Option<i32>,
    xboxOneLoggingLevel: Option<i32>,
    muteOtherAudioSources: Option<bool>,
    m_SplashScreenStyle: Option<i32>,
    m_StackTraceTypes: Option<Vec<i32>>,
    graphicsJobs: Option<bool>,
    singlePassStereoRendering: Option<bool>,
    protectGraphicsMemory: Option<bool>,
    productGUID: Option<GUID>,
    tizenShowActivityIndicatorOnLoading: Option<i32>,
    m_SplashScreenBackgroundColor: Option<ColorRGBA>,
    m_ShowUnitySplashLogo: Option<bool>,
    m_SplashScreenOverlayOpacity: Option<f32>,
    m_SplashScreenAnimation: Option<i32>,
    m_SplashScreenLogoStyle: Option<i32>,
    m_SplashScreenDrawMode: Option<i32>,
    m_SplashScreenBackgroundAnimationZoom: Option<f32>,
    m_SplashScreenLogoAnimationZoom: Option<f32>,
    m_SplashScreenBackgroundLandscapeAspect: Option<f32>,
    m_SplashScreenBackgroundPortraitAspect: Option<f32>,
    m_SplashScreenBackgroundLandscapeUvs: Option<Rectf>,
    m_SplashScreenBackgroundPortraitUvs: Option<Rectf>,
    m_SplashScreenLogos: Option<Vec<SplashScreenLogo>>,
    m_SplashScreenBackgroundLandscape: Option<PPtr>,
    m_SplashScreenBackgroundPortrait: Option<PPtr>,
    m_HolographicTrackingLossScreen: Option<PPtr>,
    m_StereoRenderingPath: Option<i32>,
    metroInputSource: Option<i32>,
    m_HolographicPauseOnTrackingLoss: Option<bool>,
    graphicsJobMode: Option<i32>,
    useHDRDisplay: Option<bool>,
    xboxOneSResolution: Option<i32>,
    xboxOneXResolution: Option<i32>,
    xboxOneEnable7thCore: Option<bool>,
    vrSettings: Option<VRSettings>,
    enableNewInputSystem: Option<bool>,
    macAppStoreCategory: Option<String>,
    deferSystemGesturesMode: Option<i32>,
    hideHomeButton: Option<bool>,
    Force_IOS_Speakers_When_Recording: Option<bool>,
    targetPixelDensity: Option<i32>,
    resolutionScalingMode: Option<i32>,
    enableNativePlatformBackendsForNewInputSystem: Option<bool>,
    disableOldInputManagerSupport: Option<bool>,
    xboxOneDisableEsram: Option<bool>,
    AndroidFilterTouchesWhenObscured: Option<bool>,
    mobileMTRenderingBaked: Option<bool>,
    androidBlitType: Option<i32>,
    macRetinaSupport: Option<bool>,
    metalFramebufferOnly: Option<bool>,
    xboxOnePresentImmediateThreshold: Option<u32>,
    m_ColorGamuts: Option<Vec<i32>>,
    androidSupportedAspectRatio: Option<i32>,
    androidMaxAspectRatio: Option<f32>,
    wsaTransparentSwapchain: Option<bool>,
    preserveFramebufferAlpha: Option<bool>,
    iosUseCustomAppBackgroundBehavior: Option<bool>,
    AndroidEnableSustainedPerformanceMode: Option<bool>,
    fullscreenMode: Option<i32>,
    switchQueueCommandMemory: Option<i32>,
    vulkanEnableSetSRGBWrite: Option<bool>,
    vulkanUseSWCommandBuffers: Option<bool>,
    isWsaHolographicRemotingEnabled: Option<bool>,
    androidStartInFullscreen: Option<bool>,
    androidRenderOutsideSafeArea: Option<bool>,
    enableFrameTimingStats: Option<bool>,
    framebufferDepthMemorylessMode: Option<i32>,
    legacyClampBlendShapeWeights: Option<bool>,
    switchQueueControlMemory: Option<i32>,
    switchQueueComputeMemory: Option<i32>,
    switchNVNShaderPoolsGranularity: Option<i32>,
    switchNVNDefaultPoolsGranularity: Option<i32>,
    switchNVNOtherPoolsGranularity: Option<i32>,
    xboxOneEnableTypeOptimization: Option<bool>,
    switchNVNMaxPublicTextureIDCount: Option<i32>,
    switchNVNMaxPublicSamplerIDCount: Option<i32>,
    useFlipModelSwapchain: Option<bool>,
    androidUseSwappy: Option<bool>,
    D3DHDRBitDepth: Option<i32>,
    vulkanNumSwapchainBuffers: Option<u32>,
    stadiaPresentMode: Option<i32>,
    stadiaTargetFramerate: Option<i32>,
    vulkanEnableLateAcquireNextImage: Option<bool>,
    androidResizableWindow: Option<bool>,
    androidDefaultWindowWidth: Option<i32>,
    androidDefaultWindowHeight: Option<i32>,
    androidMinimumWindowWidth: Option<i32>,
    androidMinimumWindowHeight: Option<i32>,
    androidFullscreenMode: Option<i32>,
    virtualTexturingSupportEnabled: Option<bool>,
    mipStripping: Option<bool>,
    numberOfMipsStripped: Option<i32>,
    vulkanEnablePreTransform: Option<bool>,
    AID: Option<Hash128>,
    qualitySettingsNames: Option<Vec<String>>,
    activeInputHandler: Option<i32>,
    playerMinOpenGLESVersion: Option<i32>,
    vulkanEnableCommandBufferRecycling: Option<bool>,
    resetResolutionOnWindowResize: Option<bool>,
    playerDataPath: Option<String>,
    forceSRGBBlit: Option<bool>,
    uploadClearedTextureDataAfterCreationFromScript: Option<bool>,
    enableOpenGLProfilerGPURecorders: Option<bool>,
    switchNVNGraphicsFirmwareMemory: Option<i32>,
    insecureHttpOption: Option<i32>,
    m_SpriteBatchVertexThreshold: Option<i32>,
    switchGpuScratchPoolGranularity: Option<i32>,
    switchAllowGpuScratchShrinking: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginBuildInfo {
    m_RuntimePlugins: Vec<String>,
    m_EditorPlugins: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginImportOutput {
    pluginType: Option<i32>,
    dllType: Option<i32>,
    scriptingRuntimeVersion: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PointEffector2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_ColliderMask: BitField,
    m_ForceMagnitude: f32,
    m_ForceVariation: f32,
    m_DistanceScale: f32,
    m_Drag: f32,
    m_AngularDrag: f32,
    m_ForceSource: (Option<u8>, Option<i32>),
    m_ForceTarget: (Option<u8>, Option<i32>),
    m_ForceMode: (Option<u8>, Option<i32>),
    m_UseColliderMask: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Polygon2D {
    m_Paths: Vec<Vec<Vector2f>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolygonCollider2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Material: PPtr,
    m_IsTrigger: bool,
    m_Poly: Option<Polygon2D>,
    m_UsedByEffector: Option<bool>,
    m_Offset: Option<Vector2f>,
    m_Points: Option<Polygon2D>,
    m_Density: Option<f32>,
    m_UsedByComposite: Option<bool>,
    m_AutoTiling: Option<bool>,
    m_SpriteTilingProperty: Option<SpriteTilingProperty>,
    m_UseDelaunayMesh: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolygonColliderBase2D {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PositionConstraint {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Weight: f32,
    m_TranslationAtRest: Vector3f,
    m_TranslationOffset: Vector3f,
    m_AffectTranslationX: bool,
    m_AffectTranslationY: bool,
    m_AffectTranslationZ: bool,
    m_IsContraintActive: Option<bool>,
    m_Sources: Vec<ConstraintSource>,
    m_Active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prefab {
    m_RootGameObject: PPtr,
    m_HideFlagsBehaviour: Option<i32>,
    m_ContainsMissingSerializeReferenceTypes: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrefabImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_AddedObjectFileIDs: Vec<i64>,
    m_IsPrefabVariant: bool,
    m_UnableToImportOnPreviousDomainReload: Option<bool>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrefabModification {
    m_TransformParent: PPtr,
    m_Modifications: Vec<PropertyModification>,
    m_RemovedComponents: (Option<Vec<PPtr>>, Option<Vec<PPtr>>),
    m_AddedGameObjects: Option<Vec<AddedGameObject>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreloadData {
    m_Name: String,
    m_Assets: Vec<PPtr>,
    m_Dependencies: Option<Vec<String>>,
    m_ExplicitDataLayout: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preset {
    m_Name: String,
    m_TargetType: PresetType,
    m_Properties: Vec<PropertyModification>,
    m_ExcludedProperties: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresetManager {
    m_DefaultList: Option<Vec<DefaultPresetList>>,
    m_DefaultPresets: Option<Vec<(PresetType, Vec<DefaultPreset>)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresetType {
    m_NativeTypeID: i32,
    m_ManagedTypePPtr: PPtr,
    m_ManagedTypeFallback: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct PreviewAnimationClip {
//     m_Name: String,
//     m_Legacy: bool,
//     m_Compressed: bool,
//     m_UseHighQualityCurve: bool,
//     m_RotationCurves: Vec<QuaternionCurve>,
//     m_CompressedRotationCurves: Vec<CompressedAnimationCurve>,
//     m_EulerCurves: Vec<Vector3Curve>,
//     m_PositionCurves: Vec<Vector3Curve>,
//     m_ScaleCurves: Vec<Vector3Curve>,
//     m_FloatCurves: Vec<FloatCurve>,
//     m_PPtrCurves: Vec<PPtrCurve>,
//     m_SampleRate: f32,
//     m_WrapMode: i32,
//     m_Bounds: AABB,
//     m_MuscleClipSize: u32,
//     m_MuscleClip: ClipMuscleConstant,
//     m_ClipBindingConstant: AnimationClipBindingConstant,
//     m_Events: Vec<AnimationEvent>,
//     m_HasGenericRootTransform: Option<bool>,
//     m_HasMotionFloatCurves: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreviewData {
    m_PreviewData: Vec<f32>,
    m_OrigSize: i32,
    m_CompSize: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreviewImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UsedFileIDs: Vec<i64>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProbeSetIndex {
    m_Hash: Hash128,
    m_Offset: i32,
    m_Size: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProbeSetTetrahedralization {
    m_Tetrahedra: Vec<Tetrahedron>,
    m_HullRays: Vec<Vector3f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProceduralMaterial {
    m_Name: String,
    m_Shader: PPtr,
    m_SavedProperties: UnityPropertySheet,
    m_SubstancePackage: Option<PPtr>,
    m_Width: Option<i32>,
    m_Height: Option<i32>,
    m_Textures: Option<Vec<PPtr>>,
    m_Inputs: Option<Vec<SubstanceInput>>,
    m_Flags: Option<u32>,
    m_AnimationUpdateRate: Option<i32>,
    m_MaximumSize: Option<i32>,
    m_CacheSize: Option<i32>,
    m_LoadingBehavior: Option<i32>,
    m_ShaderKeywords: Option<(Option<Vec<String>>, Option<String>)>,
    m_Hash: Option<Hash128>,
    m_PrototypeName: Option<String>,
    m_CustomRenderQueue: Option<i32>,
    m_GenerateMipmaps: Option<bool>,
    m_LightmapFlags: Option<u32>,
    stringTagMap: Option<Vec<(String, String)>>,
    m_EnableInstancingVariants: Option<bool>,
    disabledShaderPasses: Option<Vec<String>>,
    m_DoubleSidedGI: Option<bool>,
    m_BuildTextureStacks: Option<Vec<BuildTextureStackReference>>,
    m_ValidKeywords: Option<Vec<String>>,
    m_InvalidKeywords: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProceduralMaterialInformation {
    m_Offset: Vector2f,
    m_Scale: Vector2f,
    m_GeneratedAtLoading: Option<i32>,
    m_GenerateAllOutputs: Option<i32>,
    m_AnimationUpdateRate: Option<i32>,
    m_GenerateMipmaps: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProceduralTexture {
    m_Name: String,
    m_SubstanceMaterial: Option<PPtr>,
    m_SubstanceTextureUID: Option<u64>,
    Type: Option<i32>,
    AlphaSource: Option<i32>,
    Format: Option<i32>,
    m_TextureParameters: Option<TextureParameters>,
    m_TextureSettings: Option<GLTextureSettings>,
    m_BakedData: Option<Vec<u8>>,
    m_BakedParameters: Option<TextureParameters>,
    m_LightmapFormat: Option<i32>,
    m_ColorSpace: Option<i32>,
    m_AlphaSourceUID: Option<u64>,
    AlphaSourceIsGrayscale: Option<bool>,
    m_Mipmaps: Option<i32>,
    m_AlphaSourceIsInverted: Option<bool>,
    m_ForcedFallbackFormat: Option<i32>,
    m_DownscaleFallback: Option<bool>,
    m_IsAlphaChannelOptional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProceduralTextureAssignment {
    shaderProp: (Option<String>, Option<FastPropertyName>),
    material: PPtr,
    baseUID: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Projector {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_NearClipPlane: f32,
    m_FarClipPlane: f32,
    m_FieldOfView: f32,
    m_AspectRatio: f32,
    m_Orthographic: bool,
    m_OrthographicSize: f32,
    m_Material: PPtr,
    m_IgnoreLayers: BitField,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropertyModification {
    target: PPtr,
    propertyPath: String,
    value: String,
    objectReference: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropertyModificationsTargetTestNativeObject {
    m_IntegerValue: i32,
    m_FloatValue: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropertyModificationsTargetTestObject {
    m_Data: PropertyModificationsTargetTestNativeObject,
    m_Array: Vec<PropertyModificationsTargetTestNativeObject>,
    m_FloatTestValue: f32,
    m_Bytes: Option<Vec<u8>>,
    m_Floats: Option<Vec<f32>>,
    m_BytesSize: Option<u32>,
    byte_data: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualitySetting {
    pixelLightCount: i32,
    shadows: i32,
    shadowResolution: i32,
    shadowCascades: i32,
    shadowDistance: f32,
    blendWeights: Option<i32>,
    textureQuality: i32,
    anisotropicTextures: i32,
    antiAliasing: i32,
    softParticles: bool,
    softVegetation: bool,
    syncToVBL: Option<bool>,
    shadowProjection: Option<i32>,
    vSyncCount: Option<i32>,
    name: Option<String>,
    lodBias: Option<f32>,
    maximumLODLevel: Option<i32>,
    particleRaycastBudget: Option<i32>,
    shadowCascade2Split: Option<f32>,
    shadowCascade4Split: Option<Vector3f>,
    realtimeReflectionProbes: Option<bool>,
    billboardsFaceCameraPosition: Option<bool>,
    shadowNearPlaneOffset: Option<f32>,
    asyncUploadTimeSlice: Option<i32>,
    asyncUploadBufferSize: Option<i32>,
    shadowmaskMode: Option<i32>,
    resolutionScalingFixedDPIFactor: Option<f32>,
    streamingMipmapsActive: Option<bool>,
    streamingMipmapsAddAllCameras: Option<bool>,
    streamingMipmapsMemoryBudget: Option<f32>,
    streamingMipmapsRenderersPerFrame: Option<i32>,
    streamingMipmapsMaxLevelReduction: Option<i32>,
    streamingMipmapsMaxFileIORequests: Option<i32>,
    asyncUploadPersistentBuffer: Option<bool>,
    skinWeights: Option<i32>,
    customRenderPipeline: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualitySettings {
    m_DefaultStandaloneQuality: Option<i32>,
    m_DefaultWebPlayerQuality: Option<i32>,
    m_EditorQuality: Option<i32>,
    Fastest: Option<QualitySetting>,
    Fast: Option<QualitySetting>,
    Simple: Option<QualitySetting>,
    Good: Option<QualitySetting>,
    Beautiful: Option<QualitySetting>,
    Fantastic: Option<QualitySetting>,
    m_DefaultMobileQuality: Option<i32>,
    m_CurrentQuality: Option<i32>,
    m_QualitySettings: Option<Vec<QualitySetting>>,
    m_StrippedMaximumLODLevel: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuaternionCurve {
    curve: AnimationCurve,
    path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quaternionf {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShader {
    m_Name: String,
    variants: Vec<RayTracingShaderVariant>,
    m_MaxRecursionDepth: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderBuiltinSampler {
    sampler: u32,
    bindPoint: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderConstantBuffer {
    name: String,
    byteSize: i32,
    params: Vec<RayTracingShaderParam>,
    hash: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderFunctionDesc {
    identifier: RayTracingShaderID,
    payloadSizeInBytes: u32,
    attributeSizeInBytes: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderID {
    __type: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UsedFileIDs: Vec<i64>,
    m_CurrentAPIMask: Option<u32>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderParam {
    name: String,
    __type: Option<i32>,
    offset: (Option<u32>, Option<i32>),
    arraySize: (Option<u32>, Option<i32>),
    rowCount: (Option<u8>, Option<i32>),
    colCount: (Option<u8>, Option<i32>),
    dataType: Option<i32>,
    dataSize: Option<u32>,
    propertySheetType: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderReflectionData {
    functions: Vec<RayTracingShaderFunctionDesc>,
    localResources: RayTracingShaderResources,
    globalResources: RayTracingShaderResources,
    hasErrors: bool,
    code: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderResource {
    name: String,
    bindPoint: i32,
    samplerBindPoint: i32,
    texDimension: i32,
    rayGenMask: u64,
    arraySize: Option<i32>,
    multisampled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderResources {
    textures: Vec<RayTracingShaderResource>,
    builtinSamplers: Vec<RayTracingShaderBuiltinSampler>,
    inputBuffers: Vec<RayTracingShaderResource>,
    outputBuffers: Vec<RayTracingShaderResource>,
    constantBuffersDesc: Vec<RayTracingShaderConstantBuffer>,
    constantBuffers: Vec<RayTracingShaderResource>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderVariant {
    targetRenderer: i32,
    resourceReflectionData: RayTracingShaderReflectionData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaycastCollider {
    m_GameObject: PPtr,
    m_Material: PPtr,
    m_IsTrigger: bool,
    m_Center: Vector3f,
    m_Length: f32,
    m_Enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RectTransform {
    m_GameObject: PPtr,
    m_AnchorMin: Vector2f,
    m_AnchorMax: Vector2f,
    m_Position: Option<Vector2f>,
    m_SizeDelta: Vector2f,
    m_Pivot: Vector2f,
    m_LocalRotation: Option<Quaternionf>,
    m_LocalPosition: Option<Vector3f>,
    m_LocalScale: Option<Vector3f>,
    m_Children: Option<Vec<PPtr>>,
    m_Father: Option<PPtr>,
    m_AnchoredPosition: Option<Vector2f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rectf {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReferencesArtifactGenerator {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UsedFileIDs: Vec<i64>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReflectionProbe {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Type: i32,
    m_Mode: i32,
    m_RefreshMode: i32,
    m_TimeSlicingMode: i32,
    m_Resolution: i32,
    m_UpdateFrequency: i32,
    m_Importance: (Option<i16>, Option<i32>),
    m_BoxSize: Vector3f,
    m_BoxOffset: Vector3f,
    m_NearClip: f32,
    m_FarClip: f32,
    m_ShadowDistance: f32,
    m_ClearFlags: u32,
    m_BackGroundColor: ColorRGBA,
    m_CullingMask: BitField,
    m_IntensityMultiplier: f32,
    m_HDR: bool,
    m_BoxProjection: bool,
    m_RenderDynamicObjects: bool,
    m_UseOcclusionCulling: bool,
    m_CustomBakedTexture: PPtr,
    m_BakedTexture: PPtr,
    m_BlendDistance: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelativeJoint2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_EnableCollision: bool,
    m_ConnectedRigidBody: PPtr,
    m_BreakForce: f32,
    m_BreakTorque: f32,
    m_MaxForce: f32,
    m_MaxTorque: f32,
    m_CorrectionScale: f32,
    m_AutoConfigureOffset: bool,
    m_LinearOffset: Vector2f,
    m_AngularOffset: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderManager {
    m_AlwaysIncludedShaders: Option<Vec<PPtr>>,
    m_Deferred: Option<BuiltinShaderSettings>,
    m_LegacyDeferred: Option<BuiltinShaderSettings>,
    m_PreloadedShaders: Option<Vec<PPtr>>,
    m_DeferredReflections: Option<BuiltinShaderSettings>,
    m_ShaderSettings: Option<PlatformShaderSettings>,
    m_ScreenSpaceShadows: Option<BuiltinShaderSettings>,
    m_DepthNormals: Option<BuiltinShaderSettings>,
    m_MotionVectors: Option<BuiltinShaderSettings>,
    m_LightHalo: Option<BuiltinShaderSettings>,
    m_LensFlare: Option<BuiltinShaderSettings>,
    m_ShaderSettings_Tier1: Option<PlatformShaderSettings>,
    m_ShaderSettings_Tier2: Option<PlatformShaderSettings>,
    m_ShaderSettings_Tier3: Option<PlatformShaderSettings>,
    m_SpritesDefaultMaterial: Option<PPtr>,
    m_TierSettings_Tier1: Option<TierGraphicsSettings>,
    m_TierSettings_Tier2: Option<TierGraphicsSettings>,
    m_TierSettings_Tier3: Option<TierGraphicsSettings>,
    m_TransparencySortMode: Option<i32>,
    m_TransparencySortAxis: Option<Vector3f>,
    m_LightsUseLinearIntensity: Option<bool>,
    m_LightsUseCCT: Option<bool>,
    m_CustomRenderPipeline: Option<PPtr>,
    m_LightsUseColorTemperature: Option<bool>,
    m_ShaderDefinesPerShaderCompiler: Option<Vec<PlatformShaderDefines>>,
    m_LogWhenShaderIsCompiled: Option<bool>,
    m_AllowEnlightenSupportForUpgradedProject: Option<bool>,
    m_VideoShadersIncludeMode: Option<i32>,
    m_DefaultRenderingLayerMask: Option<u32>,
    m_SRPDefaultSettings: Option<Vec<(String, PPtr)>>,
    m_PreloadShadersBatchTimeLimit: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderPassAttachment {}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct RenderSettings {
//     m_Fog: bool,
//     m_FogColor: ColorRGBA,
//     m_FogDensity: f32,
//     m_AmbientLight: Option<ColorRGBA>,
//     m_SkyboxMaterial: PPtr,
//     m_HaloStrength: f32,
//     m_FlareStrength: f32,
//     m_HaloTexture: PPtr,
//     m_SpotCookie: PPtr,
//     m_FogMode: Option<i32>,
//     m_LinearFogStart: Option<f32>,
//     m_LinearFogEnd: Option<f32>,
//     m_FlareFadeSpeed: Option<f32>,
//     m_AmbientSkyColor: Option<ColorRGBA>,
//     m_AmbientEquatorColor: Option<ColorRGBA>,
//     m_AmbientGroundColor: Option<ColorRGBA>,
//     m_AmbientIntensity: Option<f32>,
//     m_AmbientMode: Option<i32>,
//     m_DefaultReflectionMode: Option<i32>,
//     m_DefaultReflectionResolution: Option<i32>,
//     m_ReflectionBounces: Option<i32>,
//     m_ReflectionIntensity: Option<f32>,
//     m_CustomReflection: Option<(Option<PPtr>, Option<PPtr>)>,
//     m_AmbientProbe: Option<SphericalHarmonicsL2>,
//     m_AmbientProbeInGamma: Option<SphericalHarmonicsL2>,
//     m_GeneratedSkyboxReflection: Option<PPtr>,
//     m_Sun: Option<PPtr>,
//     m_IndirectSpecularColor: Option<ColorRGBA>,
//     m_SubtractiveShadowColor: Option<ColorRGBA>,
//     m_UseRadianceAmbientProbe: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderTexture {
    m_Name: String,
    m_Width: i32,
    m_Height: i32,
    m_DepthFormat: Option<i32>,
    m_ColorFormat: i32,
    m_IsPowerOfTwo: Option<bool>,
    m_IsCubemap: Option<bool>,
    m_MipMap: bool,
    m_TextureSettings: GLTextureSettings,
    m_SRGB: Option<bool>,
    m_AntiAliasing: Option<i32>,
    m_GenerateMips: Option<bool>,
    m_Dimension: Option<i32>,
    m_VolumeDepth: Option<i32>,
    m_ForcedFallbackFormat: Option<i32>,
    m_DownscaleFallback: Option<bool>,
    m_UseDynamicScale: Option<bool>,
    m_BindMS: Option<bool>,
    m_EnableCompatibleFormat: Option<bool>,
    m_MipCount: Option<i32>,
    m_IsAlphaChannelOptional: Option<bool>,
    m_DepthStencilFormat: Option<i32>,
    m_ShadowSamplingMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Renderer {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RendererData {
    uvMesh: PPtr,
    terrainDynamicUVST: Vector4f,
    terrainChunkDynamicUVST: Vector4f,
    lightmapIndex: u16,
    lightmapIndexDynamic: u16,
    lightmapST: Vector4f,
    lightmapSTDynamic: Vector4f,
    explicitProbeSetHash: Option<Hash128>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RendererFake {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: u8,
    m_ReceiveShadows: u8,
    m_DynamicOccludee: u8,
    m_MotionVectors: u8,
    m_LightProbeUsage: u8,
    m_ReflectionProbeUsage: u8,
    m_RenderingLayerMask: u32,
    m_RendererPriority: i32,
    m_LightmapIndex: u16,
    m_LightmapIndexDynamic: u16,
    m_LightmapTilingOffset: Vector4f,
    m_LightmapTilingOffsetDynamic: Vector4f,
    m_Materials: Vec<PPtr>,
    m_StaticBatchInfo: StaticBatchInfo,
    m_StaticBatchRoot: PPtr,
    m_ProbeAnchor: PPtr,
    m_LightProbeVolumeOverride: PPtr,
    m_SortingLayerID: i32,
    m_SortingLayer: i16,
    m_SortingOrder: i16,
    m_Positions: Vec<Vector3f>,
    m_Parameters: LineParameters,
    m_UseWorldSpace: bool,
    m_Loop: bool,
    m_RayTracingMode: Option<u8>,
    m_RayTraceProcedural: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceManager {
    m_Container: Vec<(String, PPtr)>,
    m_DependentAssets: Option<Vec<ResourceManager_Dependency>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceManager_Dependency {
    m_Object: PPtr,
    m_Dependencies: Vec<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rigidbody {
    m_GameObject: PPtr,
    m_Mass: f32,
    m_Drag: f32,
    m_AngularDrag: f32,
    m_UseGravity: bool,
    m_IsKinematic: bool,
    m_Interpolate: u8,
    m_FreezeRotation: Option<bool>,
    m_CollisionDetection: i32,
    m_Constraints: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rigidbody2D {
    m_GameObject: PPtr,
    m_Mass: f32,
    m_LinearDrag: f32,
    m_AngularDrag: f32,
    m_GravityScale: f32,
    m_FixedAngle: Option<bool>,
    m_IsKinematic: Option<bool>,
    m_Interpolate: (Option<u8>, Option<i32>),
    m_SleepingMode: (Option<u8>, Option<i32>),
    m_CollisionDetection: (Option<u8>, Option<i32>),
    m_Constraints: Option<i32>,
    m_UseAutoMass: Option<bool>,
    m_BodyType: Option<i32>,
    m_Simulated: Option<bool>,
    m_UseFullKinematicContacts: Option<bool>,
    m_Material: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RootMotionData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoslynAdditionalFileAsset {
    m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoslynAdditionalFileImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UsedFileIDs: Vec<i64>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoslynAnalyzerConfigAsset {
    m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoslynAnalyzerConfigImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UsedFileIDs: Vec<i64>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotationBySpeedModule {
    enabled: bool,
    curve: MinMaxCurve,
    range: Vector2f,
    x: Option<MinMaxCurve>,
    y: Option<MinMaxCurve>,
    separateAxes: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotationConstraint {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Weight: f32,
    m_RotationAtRest: Vector3f,
    m_RotationOffset: Vector3f,
    m_AffectRotationX: bool,
    m_AffectRotationY: bool,
    m_AffectRotationZ: bool,
    m_IsContraintActive: Option<bool>,
    m_Sources: Vec<ConstraintSource>,
    m_Active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotationModule {
    enabled: bool,
    curve: MinMaxCurve,
    x: Option<MinMaxCurve>,
    y: Option<MinMaxCurve>,
    separateAxes: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleSetFileAsset {
    m_Name: String,
    m_Script: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleSetFileImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UsedFileIDs: Vec<i64>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct RuntimeAnimatorController {
//     m_Name: String,
//     m_ControllerSize: u32,
//     m_Controller: ControllerConstant,
//     m_TOS: Vec<(u32, String)>,
//     m_AnimationClips: Vec<PPtr>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuntimeInitializeOnLoadManager {
    m_AssemblyNames: Option<Vec<String>>,
    m_NamespaceNames: Option<Vec<String>>,
    m_ClassInfos: Option<Vec<ClassInfo>>,
    m_ClassMethodInfos: Option<Vec<ClassMethodInfo>>,
    m_UnityMethodExecutionOrders: Option<Vec<i32>>,
    m_MethodExecutionOrders: Option<Vec<i32>>,
    m_BeforeUnityMethodExecutionOrders: Option<Vec<i32>>,
    m_AfterUnityMethodExecutionOrders: Option<Vec<i32>>,
    m_BeforeMethodExecutionOrders: Option<Vec<i32>>,
    m_AfterMethodExecutionOrders: Option<Vec<i32>>,
    m_AfterAssembliesLoadedUnityMethodExecutionOrders: Option<Vec<i32>>,
    m_AfterAssembliesLoadedMethodExecutionOrders: Option<Vec<i32>>,
    m_BeforeSplashScreenUnityMethodExecutionOrders: Option<Vec<i32>>,
    m_BeforeSplashScreenMethodExecutionOrders: Option<Vec<i32>>,
    m_SubsystemRegistrationUnityMethodExecutionOrders: Option<Vec<i32>>,
    m_SubsystemRegistrationMethodExecutionOrders: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SBranchWindLevel {
    m_afDistance_0: f32,
    m_afDistance_1: f32,
    m_afDistance_2: f32,
    m_afDistance_3: f32,
    m_afDistance_4: f32,
    m_afDistance_5: f32,
    m_afDistance_6: f32,
    m_afDistance_7: f32,
    m_afDistance_8: f32,
    m_afDistance_9: f32,
    m_afDirectionAdherence_0: f32,
    m_afDirectionAdherence_1: f32,
    m_afDirectionAdherence_2: f32,
    m_afDirectionAdherence_3: f32,
    m_afDirectionAdherence_4: f32,
    m_afDirectionAdherence_5: f32,
    m_afDirectionAdherence_6: f32,
    m_afDirectionAdherence_7: f32,
    m_afDirectionAdherence_8: f32,
    m_afDirectionAdherence_9: f32,
    m_afWhip_0: f32,
    m_afWhip_1: f32,
    m_afWhip_2: f32,
    m_afWhip_3: f32,
    m_afWhip_4: f32,
    m_afWhip_5: f32,
    m_afWhip_6: f32,
    m_afWhip_7: f32,
    m_afWhip_8: f32,
    m_afWhip_9: f32,
    m_fTurbulence: f32,
    m_fTwitch: f32,
    m_fTwitchFreqScale: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SParams {
    m_fStrengthResponse: f32,
    m_fDirectionResponse: f32,
    m_fAnchorOffset: f32,
    m_fAnchorDistanceScale: f32,
    Oscillation0_0: f32,
    Oscillation0_1: f32,
    Oscillation0_2: f32,
    Oscillation0_3: f32,
    Oscillation0_4: f32,
    Oscillation0_5: f32,
    Oscillation0_6: f32,
    Oscillation0_7: f32,
    Oscillation0_8: f32,
    Oscillation0_9: f32,
    Oscillation1_0: f32,
    Oscillation1_1: f32,
    Oscillation1_2: f32,
    Oscillation1_3: f32,
    Oscillation1_4: f32,
    Oscillation1_5: f32,
    Oscillation1_6: f32,
    Oscillation1_7: f32,
    Oscillation1_8: f32,
    Oscillation1_9: f32,
    Oscillation2_0: f32,
    Oscillation2_1: f32,
    Oscillation2_2: f32,
    Oscillation2_3: f32,
    Oscillation2_4: f32,
    Oscillation2_5: f32,
    Oscillation2_6: f32,
    Oscillation2_7: f32,
    Oscillation2_8: f32,
    Oscillation2_9: f32,
    Oscillation3_0: f32,
    Oscillation3_1: f32,
    Oscillation3_2: f32,
    Oscillation3_3: f32,
    Oscillation3_4: f32,
    Oscillation3_5: f32,
    Oscillation3_6: f32,
    Oscillation3_7: f32,
    Oscillation3_8: f32,
    Oscillation3_9: f32,
    Oscillation4_0: f32,
    Oscillation4_1: f32,
    Oscillation4_2: f32,
    Oscillation4_3: f32,
    Oscillation4_4: f32,
    Oscillation4_5: f32,
    Oscillation4_6: f32,
    Oscillation4_7: f32,
    Oscillation4_8: f32,
    Oscillation4_9: f32,
    Oscillation5_0: f32,
    Oscillation5_1: f32,
    Oscillation5_2: f32,
    Oscillation5_3: f32,
    Oscillation5_4: f32,
    Oscillation5_5: f32,
    Oscillation5_6: f32,
    Oscillation5_7: f32,
    Oscillation5_8: f32,
    Oscillation5_9: f32,
    Oscillation6_0: f32,
    Oscillation6_1: f32,
    Oscillation6_2: f32,
    Oscillation6_3: f32,
    Oscillation6_4: f32,
    Oscillation6_5: f32,
    Oscillation6_6: f32,
    Oscillation6_7: f32,
    Oscillation6_8: f32,
    Oscillation6_9: f32,
    Oscillation7_0: f32,
    Oscillation7_1: f32,
    Oscillation7_2: f32,
    Oscillation7_3: f32,
    Oscillation7_4: f32,
    Oscillation7_5: f32,
    Oscillation7_6: f32,
    Oscillation7_7: f32,
    Oscillation7_8: f32,
    Oscillation7_9: f32,
    Oscillation8_0: f32,
    Oscillation8_1: f32,
    Oscillation8_2: f32,
    Oscillation8_3: f32,
    Oscillation8_4: f32,
    Oscillation8_5: f32,
    Oscillation8_6: f32,
    Oscillation8_7: f32,
    Oscillation8_8: f32,
    Oscillation8_9: f32,
    Oscillation9_0: f32,
    Oscillation9_1: f32,
    Oscillation9_2: f32,
    Oscillation9_3: f32,
    Oscillation9_4: f32,
    Oscillation9_5: f32,
    Oscillation9_6: f32,
    Oscillation9_7: f32,
    Oscillation9_8: f32,
    Oscillation9_9: f32,
    m_fGlobalHeight: f32,
    m_fGlobalHeightExponent: f32,
    m_afGlobalDistance_0: f32,
    m_afGlobalDistance_1: f32,
    m_afGlobalDistance_2: f32,
    m_afGlobalDistance_3: f32,
    m_afGlobalDistance_4: f32,
    m_afGlobalDistance_5: f32,
    m_afGlobalDistance_6: f32,
    m_afGlobalDistance_7: f32,
    m_afGlobalDistance_8: f32,
    m_afGlobalDistance_9: f32,
    m_afGlobalDirectionAdherence_0: f32,
    m_afGlobalDirectionAdherence_1: f32,
    m_afGlobalDirectionAdherence_2: f32,
    m_afGlobalDirectionAdherence_3: f32,
    m_afGlobalDirectionAdherence_4: f32,
    m_afGlobalDirectionAdherence_5: f32,
    m_afGlobalDirectionAdherence_6: f32,
    m_afGlobalDirectionAdherence_7: f32,
    m_afGlobalDirectionAdherence_8: f32,
    m_afGlobalDirectionAdherence_9: f32,
    BranchLevel1: SBranchWindLevel,
    BranchLevel2: SBranchWindLevel,
    LeafGroup1: SWindGroup,
    LeafGroup2: SWindGroup,
    m_afFrondRippleDistance_0: f32,
    m_afFrondRippleDistance_1: f32,
    m_afFrondRippleDistance_2: f32,
    m_afFrondRippleDistance_3: f32,
    m_afFrondRippleDistance_4: f32,
    m_afFrondRippleDistance_5: f32,
    m_afFrondRippleDistance_6: f32,
    m_afFrondRippleDistance_7: f32,
    m_afFrondRippleDistance_8: f32,
    m_afFrondRippleDistance_9: f32,
    m_fFrondRippleTile: f32,
    m_fFrondRippleLightingScalar: f32,
    m_fRollingNoiseSize: f32,
    m_fRollingNoiseTwist: f32,
    m_fRollingNoiseTurbulence: f32,
    m_fRollingNoisePeriod: f32,
    m_fRollingNoiseSpeed: f32,
    m_fRollingBranchFieldMin: f32,
    m_fRollingBranchLightingAdjust: f32,
    m_fRollingBranchVerticalOffset: f32,
    m_fRollingLeafRippleMin: f32,
    m_fRollingLeafTumbleMin: f32,
    m_fGustFrequency: f32,
    m_fGustStrengthMin: f32,
    m_fGustStrengthMax: f32,
    m_fGustDurationMin: f32,
    m_fGustDurationMax: f32,
    m_fGustRiseScalar: f32,
    m_fGustFallScalar: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SWindGroup {
    m_afRippleDistance_0: f32,
    m_afRippleDistance_1: f32,
    m_afRippleDistance_2: f32,
    m_afRippleDistance_3: f32,
    m_afRippleDistance_4: f32,
    m_afRippleDistance_5: f32,
    m_afRippleDistance_6: f32,
    m_afRippleDistance_7: f32,
    m_afRippleDistance_8: f32,
    m_afRippleDistance_9: f32,
    m_afTumbleFlip_0: f32,
    m_afTumbleFlip_1: f32,
    m_afTumbleFlip_2: f32,
    m_afTumbleFlip_3: f32,
    m_afTumbleFlip_4: f32,
    m_afTumbleFlip_5: f32,
    m_afTumbleFlip_6: f32,
    m_afTumbleFlip_7: f32,
    m_afTumbleFlip_8: f32,
    m_afTumbleFlip_9: f32,
    m_afTumbleTwist_0: f32,
    m_afTumbleTwist_1: f32,
    m_afTumbleTwist_2: f32,
    m_afTumbleTwist_3: f32,
    m_afTumbleTwist_4: f32,
    m_afTumbleTwist_5: f32,
    m_afTumbleTwist_6: f32,
    m_afTumbleTwist_7: f32,
    m_afTumbleTwist_8: f32,
    m_afTumbleTwist_9: f32,
    m_afTumbleDirectionAdherence_0: f32,
    m_afTumbleDirectionAdherence_1: f32,
    m_afTumbleDirectionAdherence_2: f32,
    m_afTumbleDirectionAdherence_3: f32,
    m_afTumbleDirectionAdherence_4: f32,
    m_afTumbleDirectionAdherence_5: f32,
    m_afTumbleDirectionAdherence_6: f32,
    m_afTumbleDirectionAdherence_7: f32,
    m_afTumbleDirectionAdherence_8: f32,
    m_afTumbleDirectionAdherence_9: f32,
    m_afTwitchThrow_0: f32,
    m_afTwitchThrow_1: f32,
    m_afTwitchThrow_2: f32,
    m_afTwitchThrow_3: f32,
    m_afTwitchThrow_4: f32,
    m_afTwitchThrow_5: f32,
    m_afTwitchThrow_6: f32,
    m_afTwitchThrow_7: f32,
    m_afTwitchThrow_8: f32,
    m_afTwitchThrow_9: f32,
    m_fTwitchSharpness: f32,
    m_fRollMaxScale: f32,
    m_fRollMinScale: f32,
    m_fRollSpeed: f32,
    m_fRollSeparation: f32,
    m_fLeewardScalar: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SampleClip {
    m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SampleSettings {
    loadType: i32,
    sampleRateSetting: i32,
    sampleRateOverride: u32,
    compressionFormat: i32,
    quality: f32,
    conversionMode: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SamplerParameter {
    sampler: u32,
    bindPoint: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScaleConstraint {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Weight: f32,
    m_ScaleAtRest: Vector3f,
    m_ScaleOffset: Vector3f,
    m_AffectScalingX: bool,
    m_AffectScalingY: bool,
    m_AffectScalingZ: bool,
    m_IsContraintActive: Option<bool>,
    m_Sources: Vec<ConstraintSource>,
    m_Active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scene {
    m_PVSData: Option<Vec<u8>>,
    m_PVSObjectsArray: Option<Vec<PPtr>>,
    m_QueryMode: Option<u32>,
    m_PVSPortalsArray: Option<Vec<PPtr>>,
    m_SceneGUID: Option<GUID>,
    m_OcclusionCullingData: Option<PPtr>,
    m_StaticRenderers: Option<Vec<PPtr>>,
    m_Portals: Option<Vec<PPtr>>,
    enabled: Option<bool>,
    path: Option<String>,
    guid: Option<GUID>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneAsset {
    m_Name: String,
    m_Message: Option<String>,
    m_IsWarning: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneDataContainer {
    m_SceneData: Vec<(SceneIdentifier, HierarchicalSceneData)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneIdentifier {
    guid: GUID,
    handle: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneObjectIdentifier {
    targetObject: i64,
    targetPrefab: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneVisibilityData {
    m_SceneGUID: GUID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneVisibilityState {
    m_SceneData: Option<Vec<(SceneIdentifier, SceneVisibilityData)>>,
    m_MainStageIsolated: Option<bool>,
    m_PrefabStageIsolated: Option<bool>,
    m_SceneVisibilityData: Option<SceneDataContainer>,
    m_SceneVisibilityDataIsolated: Option<SceneDataContainer>,
    m_ScenePickingData: Option<SceneDataContainer>,
    m_IsolationMode: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScenesUsingAssets {
    m_ScenesUsingAssets: Vec<BuildReportScenesUsingAsset>,
    m_ListOfScenesUsingEachAsset: Vec<(String, Vec<String>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScriptMapper {
    m_Shaders: NameToObjectMap,
    m_PreloadShaders: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScriptableCamera {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_ClearFlags: u32,
    m_BackGroundColor: ColorRGBA,
    m_projectionMatrixMode: i32,
    m_GateFitMode: i32,
    m_SensorSize: Vector2f,
    m_LensShift: Vector2f,
    m_FocalLength: f32,
    m_NormalizedViewPortRect: Rectf,
    near_clip_plane: f32,
    far_clip_plane: f32,
    field_of_view: f32,
    orthographic: bool,
    orthographic_size: f32,
    m_Depth: f32,
    m_CullingMask: BitField,
    m_RenderingPath: i32,
    m_TargetTexture: PPtr,
    m_TargetDisplay: i32,
    m_TargetEye: i32,
    m_HDR: bool,
    m_AllowMSAA: bool,
    m_AllowDynamicResolution: bool,
    m_ForceIntoRT: bool,
    m_OcclusionCulling: bool,
    m_StereoConvergence: f32,
    m_StereoSeparation: f32,
    m_Script: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScriptedImporter {
    m_Name: String,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
    m_Script: PPtr,
    m_FileIDToRecycleName: Option<Vec<(i64, String)>>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecondarySpriteTexture {
    texture: PPtr,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecondaryTextureSettings {
    platformSettings: Vec<TextureImporterPlatformSettings>,
    sRGB: Option<bool>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct SelectorStateConstant {
//     m_TransitionConstantArray: Vec<OffsetPtr>,
//     m_FullPathID: u32,
//     m_IsEntry: bool,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct SelectorTransitionConstant {
//     m_Destination: u32,
//     m_ConditionConstantArray: Vec<OffsetPtr>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializableManagedHost {
    m_Script: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializableManagedRefTestClass {
    m_Script: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedCustomEditorForRenderPipeline {
    customEditorName: String,
    renderPipelineType: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedPass {
    m_NameIndices: Vec<(String, i32)>,
    m_Type: i32,
    m_State: SerializedShaderState,
    m_ProgramMask: u32,
    progVertex: SerializedProgram,
    progFragment: SerializedProgram,
    progGeometry: SerializedProgram,
    progHull: SerializedProgram,
    progDomain: SerializedProgram,
    m_HasInstancingVariant: bool,
    m_UseName: String,
    m_Name: String,
    m_TextureName: String,
    m_Tags: SerializedTagMap,
    m_HasProceduralInstancingVariant: Option<bool>,
    progRayTracing: Option<SerializedProgram>,
    m_EditorDataHash: Option<Vec<Hash128>>,
    m_Platforms: Option<Vec<u8>>,
    m_LocalKeywordMask: Option<Vec<u16>>,
    m_GlobalKeywordMask: Option<Vec<u16>>,
    m_SerializedKeywordStateMask: Option<Vec<u16>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedProgram {
    m_SubPrograms: Vec<SerializedSubProgram>,
    m_CommonParameters: Option<SerializedProgramParameters>,
    m_SerializedKeywordStateMask: Option<Vec<u16>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedProgramParameters {
    m_VectorParams: Vec<VectorParameter>,
    m_MatrixParams: Vec<MatrixParameter>,
    m_TextureParams: Vec<TextureParameter>,
    m_BufferParams: Vec<BufferBinding>,
    m_ConstantBuffers: Vec<ConstantBuffer>,
    m_ConstantBufferBindings: Vec<BufferBinding>,
    m_UAVParams: Vec<UAVParameter>,
    m_Samplers: Vec<SamplerParameter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedProperties {
    m_Props: Vec<SerializedProperty>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedProperty {
    m_Name: String,
    m_Description: String,
    m_Attributes: Vec<String>,
    m_Type: i32,
    m_Flags: u32,
    m_DefTexture: SerializedTextureProperty,
    m_DefValue: (f32, f32, f32, f32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedShader {
    m_PropInfo: SerializedProperties,
    m_SubShaders: Vec<SerializedSubShader>,
    m_Name: String,
    m_CustomEditorName: String,
    m_FallbackName: String,
    m_Dependencies: Vec<SerializedShaderDependency>,
    m_DisableNoSubshadersMessage: bool,
    m_CustomEditorForRenderPipelines: Option<Vec<SerializedCustomEditorForRenderPipeline>>,
    m_KeywordNames: Option<Vec<String>>,
    m_KeywordFlags: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedShaderDependency {
    from: String,
    to: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedShaderFloatValue {
    val: f32,
    name: (Option<String>, Option<FastPropertyName>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedShaderRTBlendState {
    srcBlend: SerializedShaderFloatValue,
    destBlend: SerializedShaderFloatValue,
    srcBlendAlpha: SerializedShaderFloatValue,
    destBlendAlpha: SerializedShaderFloatValue,
    blendOp: SerializedShaderFloatValue,
    blendOpAlpha: SerializedShaderFloatValue,
    colMask: SerializedShaderFloatValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedShaderState {
    m_Name: String,
    rtBlend0: SerializedShaderRTBlendState,
    rtBlend1: SerializedShaderRTBlendState,
    rtBlend2: SerializedShaderRTBlendState,
    rtBlend3: SerializedShaderRTBlendState,
    rtBlend4: SerializedShaderRTBlendState,
    rtBlend5: SerializedShaderRTBlendState,
    rtBlend6: SerializedShaderRTBlendState,
    rtBlend7: SerializedShaderRTBlendState,
    rtSeparateBlend: bool,
    zTest: SerializedShaderFloatValue,
    zWrite: SerializedShaderFloatValue,
    culling: SerializedShaderFloatValue,
    offsetFactor: SerializedShaderFloatValue,
    offsetUnits: SerializedShaderFloatValue,
    alphaToMask: SerializedShaderFloatValue,
    stencilOp: SerializedStencilOp,
    stencilOpFront: SerializedStencilOp,
    stencilOpBack: SerializedStencilOp,
    stencilReadMask: SerializedShaderFloatValue,
    stencilWriteMask: SerializedShaderFloatValue,
    stencilRef: SerializedShaderFloatValue,
    fogStart: SerializedShaderFloatValue,
    fogEnd: SerializedShaderFloatValue,
    fogDensity: SerializedShaderFloatValue,
    fogColor: SerializedShaderVectorValue,
    fogMode: i32,
    gpuProgramID: i32,
    m_Tags: SerializedTagMap,
    m_LOD: i32,
    lighting: bool,
    zClip: Option<SerializedShaderFloatValue>,
    conservative: Option<SerializedShaderFloatValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedShaderVectorValue {
    x: SerializedShaderFloatValue,
    y: SerializedShaderFloatValue,
    z: SerializedShaderFloatValue,
    w: SerializedShaderFloatValue,
    name: (Option<String>, Option<FastPropertyName>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedStencilOp {
    pass: SerializedShaderFloatValue,
    fail: SerializedShaderFloatValue,
    zFail: SerializedShaderFloatValue,
    comp: SerializedShaderFloatValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedSubProgram {
    m_BlobIndex: u32,
    m_Channels: ParserBindChannels,
    m_KeywordIndices: Option<Vec<u16>>,
    m_ShaderHardwareTier: i8,
    m_GpuProgramType: i8,
    m_VectorParams: Option<Vec<VectorParameter>>,
    m_MatrixParams: Option<Vec<MatrixParameter>>,
    m_TextureParams: Option<Vec<TextureParameter>>,
    m_BufferParams: Option<Vec<BufferBinding>>,
    m_ConstantBuffers: Option<Vec<ConstantBuffer>>,
    m_ConstantBufferBindings: Option<Vec<BufferBinding>>,
    m_UAVParams: Option<Vec<UAVParameter>>,
    m_Samplers: Option<Vec<SamplerParameter>>,
    m_ShaderRequirements: Option<(Option<i64>, Option<i32>)>,
    m_GlobalKeywordIndices: Option<Vec<u16>>,
    m_LocalKeywordIndices: Option<Vec<u16>>,
    m_Parameters: Option<SerializedProgramParameters>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedSubShader {
    m_Passes: Vec<SerializedPass>,
    m_Tags: SerializedTagMap,
    m_LOD: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedTagMap {
    tags: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedTextureProperty {
    m_DefaultName: String,
    m_TexDim: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shader {
    m_Name: String,
    m_Script: Option<String>,
    m_PathName: Option<String>,
    m_DefaultProperties: Option<UnityPropertySheet>,
    m_Dependencies: Option<Vec<PPtr>>,
    m_ShaderIsBaked: Option<bool>,
    decompressedSize: Option<u32>,
    m_SubProgramBlob: Option<Vec<u8>>,
    m_ParsedForm: Option<SerializedShader>,
    platforms: Option<Vec<u32>>,
    offsets: Option<(Option<Vec<u32>>, Option<Vec<Vec<u32>>>)>,
    compressedLengths: Option<(Option<Vec<u32>>, Option<Vec<Vec<u32>>>)>,
    decompressedLengths: Option<(Option<Vec<u32>>, Option<Vec<Vec<u32>>>)>,
    compressedBlob: Option<Vec<u8>>,
    m_NonModifiableTextures: Option<Vec<(String, PPtr)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShaderBindChannel {
    source: i8,
    target: i8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShaderImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_UserData: Option<String>,
    m_DefaultTextures: Option<Vec<(String, PPtr)>>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_NonModifiableTextures: Option<Vec<(String, PPtr)>>,
    m_UsedFileIDs: Option<Vec<i64>>,
    m_PreprocessorOverride: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShaderIncludeImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UsedFileIDs: Vec<i64>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShaderInfo {
    variants: Vec<VariantInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShaderVariantCollection {
    m_Name: String,
    m_Shaders: Vec<(PPtr, ShaderInfo)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShadowSettings {
    m_Type: i32,
    m_Resolution: i32,
    m_Strength: f32,
    m_Projection: Option<i32>,
    m_Bias: f32,
    m_Softness: Option<f32>,
    m_SoftnessFade: Option<f32>,
    m_NormalBias: Option<f32>,
    m_NearPlane: Option<f32>,
    m_CustomResolution: Option<i32>,
    m_CullingMatrixOverride: Option<Matrix4x4f>,
    m_UseCullingMatrixOverride: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShapeModule {
    enabled: bool,
    __type: i32,
    radius: (Option<MultiModeParameter>, Option<f32>),
    angle: f32,
    boxX: Option<f32>,
    boxY: Option<f32>,
    boxZ: Option<f32>,
    placementMode: i32,
    m_Mesh: PPtr,
    randomDirection: Option<bool>,
    length: Option<f32>,
    arc: Option<(Option<MultiModeParameter>, Option<f32>)>,
    m_MeshRenderer: Option<PPtr>,
    m_SkinnedMeshRenderer: Option<PPtr>,
    m_MeshMaterialIndex: Option<i32>,
    m_MeshNormalOffset: Option<f32>,
    m_UseMeshMaterialIndex: Option<bool>,
    m_UseMeshColors: Option<bool>,
    m_MeshScale: Option<f32>,
    alignToDirection: Option<bool>,
    randomDirectionAmount: Option<f32>,
    sphericalDirectionAmount: Option<f32>,
    boxThickness: Option<Vector3f>,
    radiusThickness: Option<f32>,
    donutRadius: Option<f32>,
    m_Position: Option<Vector3f>,
    m_Rotation: Option<Vector3f>,
    m_Scale: Option<Vector3f>,
    randomPositionAmount: Option<f32>,
    m_Texture: Option<PPtr>,
    m_TextureClipChannel: Option<i32>,
    m_TextureClipThreshold: Option<f32>,
    m_TextureUVChannel: Option<i32>,
    m_TextureColorAffectsParticles: Option<bool>,
    m_TextureAlphaAffectsParticles: Option<bool>,
    m_TextureBilinearFiltering: Option<bool>,
    m_Sprite: Option<PPtr>,
    m_SpriteRenderer: Option<PPtr>,
    m_MeshSpawn: Option<MultiModeParameter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiblingDerived {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SizeBySpeedModule {
    enabled: bool,
    curve: MinMaxCurve,
    range: Vector2f,
    y: Option<MinMaxCurve>,
    z: Option<MinMaxCurve>,
    separateAxes: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SizeModule {
    enabled: bool,
    curve: MinMaxCurve,
    y: Option<MinMaxCurve>,
    z: Option<MinMaxCurve>,
    separateAxes: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skeleton {
    m_Node: Vec<Node>,
    m_ID: Vec<u32>,
    m_AxesArray: Vec<Axes>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkeletonBone {
    m_Name: String,
    m_Position: Vector3f,
    m_Rotation: Quaternionf,
    m_Scale: Vector3f,
    m_TransformModified: Option<bool>,
    m_ParentName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkeletonBoneLimit {
    m_Min: Vector3f,
    m_Max: Vector3f,
    m_Value: Vector3f,
    m_PreQ: Option<Quaternionf>,
    m_PostQ: Option<Quaternionf>,
    m_Length: f32,
    m_Modified: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkeletonMask {
    m_Data: Vec<SkeletonMaskElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkeletonMaskElement {
    m_Index: Option<u32>,
    m_Weight: f32,
    m_PathHash: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkeletonPose {
    m_X: Vec<xform>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SketchUpImportCamera {
    position: Vector3f,
    lookAt: Vector3f,
    up: Vector3f,
    fov: f32,
    aspectRatio: f32,
    orthoSize: f32,
    isPerspective: i32,
    nearPlane: Option<f32>,
    farPlane: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SketchUpImportData {
    defaultCamera: SketchUpImportCamera,
    scenes: Vec<SketchUpImportScene>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SketchUpImportScene {
    camera: SketchUpImportCamera,
    name: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct SketchUpImporter {
//     m_GenerateBackFace: bool,
//     m_MergeCoplanarFaces: bool,
//     m_SelectedNodes: Vec<i32>,
//     m_AssetHash: Hash128,
//     m_Longitude: f64,
//     m_Latitude: f64,
//     m_NorthCorrection: f64,
//     m_FileUnit: i32,
//     m_SketchUpImportData: SketchUpImportData,
//     m_Name: String,
//     m_FileIDToRecycleName: Option<Vec<(i64, String)>>,
//     m_ImportMaterials: Option<bool>,
//     m_MaterialName: i32,
//     m_MaterialSearch: i32,
//     m_LegacyGenerateAnimations: i32,
//     m_BakeSimulation: bool,
//     m_OptimizeGameObjects: bool,
//     m_MotionNodeName: String,
//     m_AnimationImportErrors: String,
//     m_AnimationImportWarnings: String,
//     m_AnimationRetargetingWarnings: String,
//     m_AnimationDoRetargetingWarnings: bool,
//     m_AnimationCompression: i32,
//     m_AnimationRotationError: f32,
//     m_AnimationPositionError: f32,
//     m_AnimationScaleError: f32,
//     m_AnimationWrapMode: i32,
//     m_ExtraExposedTransformPaths: Vec<String>,
//     m_ClipAnimations: Vec<ClipAnimationInfo>,
//     m_IsReadable: bool,
//     m_LODScreenPercentages: Vec<f32>,
//     m_GlobalScale: f32,
//     m_MeshCompression: i32,
//     m_AddColliders: bool,
//     m_ImportBlendShapes: bool,
//     swapUVChannels: bool,
//     generateSecondaryUV: bool,
//     m_UseFileUnits: bool,
//     optimizeMeshForGPU: Option<bool>,
//     keepQuads: bool,
//     weldVertices: bool,
//     secondaryUVAngleDistortion: f32,
//     secondaryUVAreaDistortion: f32,
//     secondaryUVHardAngle: f32,
//     secondaryUVPackMargin: f32,
//     m_UseFileScale: bool,
//     m_FileScale: f32,
//     normalSmoothAngle: f32,
//     splitTangentsAcrossUV: Option<bool>,
//     normalImportMode: i32,
//     tangentImportMode: i32,
//     m_ImportedTakeInfos: Vec<TakeInfo>,
//     m_ReferencedClips: Vec<GUID>,
//     m_ImportedRoots: Vec<PPtr>,
//     m_HasExtraRoot: bool,
//     m_ImportAnimation: bool,
//     m_CopyAvatar: Option<bool>,
//     m_HumanDescription: HumanDescription,
//     m_LastHumanDescriptionAvatarSource: PPtr,
//     m_AnimationType: i32,
//     m_AdditionalBone: bool,
//     m_UserData: String,
//     m_AssetBundleName: String,
//     m_AssetBundleVariant: String,
//     m_HumanoidOversampling: Option<i32>,
//     m_ResampleRotations: Option<bool>,
//     m_ResampleCurves: Option<bool>,
//     m_RigImportErrors: Option<String>,
//     m_RigImportWarnings: Option<String>,
//     m_ImportVisibility: Option<bool>,
//     m_ImportCameras: Option<bool>,
//     m_ImportLights: Option<bool>,
//     normalCalculationMode: Option<i32>,
//     m_ExtraUserProperties: Option<Vec<String>>,
//     m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
//     m_AutoMapExternalMaterials: Option<bool>,
//     m_MaterialLocation: Option<i32>,
//     m_Materials: Option<Vec<SourceAssetIdentifier>>,
//     m_ImportAnimatedCustomProperties: Option<bool>,
//     m_HasEmbeddedTextures: Option<bool>,
//     m_SupportsEmbeddedMaterials: Option<bool>,
//     m_PreserveHierarchy: Option<bool>,
//     indexFormat: Option<i32>,
//     m_ImportConstraints: Option<bool>,
//     m_PreviousCalculatedGlobalScale: Option<f32>,
//     m_HasPreviousCalculatedGlobalScale: Option<bool>,
//     m_UseSRGBMaterialColor: Option<bool>,
//     m_FileScaleUnit: Option<String>,
//     m_FileScaleFactor: Option<f32>,
//     legacyComputeAllNormalsFromSmoothingGroupsWhenMeshHasBlendShapes: Option<bool>,
//     blendShapeNormalImportMode: Option<i32>,
//     normalSmoothingSource: Option<i32>,
//     m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
//     m_UsedFileIDs: Option<Vec<i64>>,
//     skinWeightsMode: Option<i32>,
//     maxBonesPerVertex: Option<i32>,
//     minBoneWeight: Option<f32>,
//     meshOptimizationFlags: Option<i32>,
//     m_SortHierarchyByName: Option<bool>,
//     m_AvatarSetup: Option<i32>,
//     m_MaterialImportMode: Option<i32>,
//     m_AutoGenerateAvatarMappingIfUnspecified: Option<bool>,
//     m_FileIdsGeneration: Option<i32>,
//     secondaryUVMarginMethod: Option<i32>,
//     secondaryUVMinLightmapResolution: Option<f32>,
//     secondaryUVMinObjectScale: Option<f32>,
//     bakeAxisConversion: Option<bool>,
//     m_AddHumanoidExtraRootOnlyWhenUsingAvatar: Option<bool>,
//     optimizeBones: Option<bool>,
//     m_RemoveConstantScaleCurves: Option<bool>,
//     m_NodeNameCollisionStrategy: Option<i32>,
//     m_StrictVertexDataChecks: Option<bool>,
//     m_ImportBlendShapeDeformPercent: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkinnedCloth {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_BendingStiffness: f32,
    m_StretchingStiffness: f32,
    m_Damping: f32,
    m_Thickness: f32,
    m_UseGravity: bool,
    m_SelfCollision: bool,
    m_ExternalAcceleration: Vector3f,
    m_RandomAcceleration: Vector3f,
    m_WorldVelocityScale: f32,
    m_WorldAccelerationScale: f32,
    m_Coefficients: Vec<ClothConstrainCoefficients>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkinnedMeshRenderer {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: (Option<u8>, Option<bool>),
    m_ReceiveShadows: (Option<u8>, Option<bool>),
    m_LightmapIndex: (Option<u8>, Option<u16>),
    m_LightmapTilingOffset: Vector4f,
    m_Materials: Vec<PPtr>,
    m_SubsetIndices: Option<Vec<u32>>,
    m_StaticBatchRoot: PPtr,
    m_Quality: i32,
    m_UpdateWhenOffscreen: bool,
    m_SkinNormals: Option<bool>,
    m_Mesh: PPtr,
    m_Bones: Vec<PPtr>,
    m_AABB: Option<AABB>,
    m_DirtyAABB: Option<bool>,
    m_UseLightProbes: Option<bool>,
    m_LightProbeAnchor: Option<PPtr>,
    m_RootBone: Option<PPtr>,
    m_SortingLayer: Option<i16>,
    m_SortingOrder: Option<i16>,
    m_BlendShapeWeights: Option<Vec<f32>>,
    m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    m_LightmapIndexDynamic: Option<u16>,
    m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    m_ProbeAnchor: Option<PPtr>,
    m_MotionVectors: Option<u8>,
    m_LightProbeUsage: Option<u8>,
    m_LightProbeVolumeOverride: Option<PPtr>,
    m_SkinnedMotionVectors: Option<bool>,
    m_StaticBatchInfo: Option<StaticBatchInfo>,
    m_DynamicOccludee: Option<u8>,
    m_RenderingLayerMask: Option<u32>,
    m_RendererPriority: Option<i32>,
    m_RayTracingMode: Option<u8>,
    m_RayTraceProcedural: Option<u8>,
    m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skybox {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_CustomSkybox: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SliderJoint2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_CollideConnected: Option<bool>,
    m_ConnectedRigidBody: PPtr,
    m_Anchor: Vector2f,
    m_ConnectedAnchor: Vector2f,
    m_Angle: f32,
    m_UseMotor: bool,
    m_Motor: JointMotor2D,
    m_UseLimits: bool,
    m_TranslationLimits: JointTranslationLimits2D,
    m_EnableCollision: Option<bool>,
    m_BreakForce: Option<f32>,
    m_BreakTorque: Option<f32>,
    m_AutoConfigureConnectedAnchor: Option<bool>,
    m_AutoConfigureAngle: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnapshotConstant {
    nameHash: u32,
    values: Vec<f32>,
    transitionTypes: Vec<u32>,
    transitionIndices: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoftJointLimit {
    limit: f32,
    bounciness: f32,
    spring: Option<f32>,
    damper: Option<f32>,
    contactDistance: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoftJointLimitSpring {
    spring: f32,
    damper: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SortingGroup {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_SortingLayer: i16,
    m_SortingOrder: i16,
    m_SortingLayerID: Option<i32>,
    m_SortAtRoot: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SortingLayerEntry {
    name: String,
    userID: Option<u32>,
    uniqueID: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceAssetIdentifier {
    __type: String,
    assembly: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTextureInformation {
    width: i32,
    height: i32,
    doesTextureContainAlpha: bool,
    doesTextureContainColor: Option<bool>,
    sourceWasHDR: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SparseTexture {
    m_Name: String,
    m_Width: i32,
    m_Height: i32,
    m_MipCount: i32,
    m_Format: (Option<u32>, Option<i32>),
    m_ColorSpace: i32,
    m_TextureSettings: GLTextureSettings,
    m_ForcedFallbackFormat: Option<i32>,
    m_DownscaleFallback: Option<bool>,
    m_IsAlphaChannelOptional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpeedTreeImporter {
    m_Name: String,
    m_MainColor: ColorRGBA,
    m_SpecColor: Option<ColorRGBA>,
    m_HueVariation: ColorRGBA,
    m_Shininess: Option<f32>,
    m_AlphaTestRef: f32,
    m_BestWindQuality: i32,
    m_HasBillboard: bool,
    m_LODSettings: Vec<PerLODSettings>,
    m_EnableSmoothLODTransition: bool,
    m_BillboardTransitionCrossFadeWidth: f32,
    m_FadeOutWidth: f32,
    m_ScaleFactor: f32,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
    m_AnimateCrossFading: Option<bool>,
    m_MaterialVersion: Option<i32>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_MaterialLocation: Option<i32>,
    m_Materials: Option<Vec<SourceAssetIdentifier>>,
    m_SupportsEmbeddedMaterials: Option<bool>,
    m_UsedFileIDs: Option<Vec<i64>>,
    m_FileIDType: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpeedTreeWind {
    m_sParams: SParams,
    BranchWindAnchor0: f32,
    BranchWindAnchor1: f32,
    BranchWindAnchor2: f32,
    m_fMaxBranchLevel1Length: f32,
    GLOBAL_WIND: bool,
    GLOBAL_PRESERVE_SHAPE: bool,
    BRANCH_SIMPLE_1: bool,
    BRANCH_DIRECTIONAL_1: bool,
    BRANCH_DIRECTIONAL_FROND_1: bool,
    BRANCH_TURBULENCE_1: bool,
    BRANCH_WHIP_1: bool,
    BRANCH_OSC_COMPLEX_1: bool,
    BRANCH_SIMPLE_2: bool,
    BRANCH_DIRECTIONAL_2: bool,
    BRANCH_DIRECTIONAL_FROND_2: bool,
    BRANCH_TURBULENCE_2: bool,
    BRANCH_WHIP_2: bool,
    BRANCH_OSC_COMPLEX_2: bool,
    LEAF_RIPPLE_VERTEX_NORMAL_1: bool,
    LEAF_RIPPLE_COMPUTED_1: bool,
    LEAF_TUMBLE_1: bool,
    LEAF_TWITCH_1: bool,
    LEAF_OCCLUSION_1: bool,
    LEAF_RIPPLE_VERTEX_NORMAL_2: bool,
    LEAF_RIPPLE_COMPUTED_2: bool,
    LEAF_TUMBLE_2: bool,
    LEAF_TWITCH_2: bool,
    LEAF_OCCLUSION_2: bool,
    FROND_RIPPLE_ONE_SIDED: bool,
    FROND_RIPPLE_TWO_SIDED: bool,
    FROND_RIPPLE_ADJUST_LIGHTING: bool,
    ROLLING: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpeedTreeWindAsset {
    m_Name: String,
    m_Wind: SpeedTreeWind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SphereCollider {
    m_GameObject: PPtr,
    m_Material: PPtr,
    m_IsTrigger: bool,
    m_Radius: f32,
    m_Center: Vector3f,
    m_Enabled: Option<bool>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct SphericalHarmonicsL2 {
//     sh: (
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//         f32,
//     ),
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SplashScreenLogo {
    logo: PPtr,
    duration: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SplatDatabase {
    m_Splats: Option<Vec<SplatPrototype>>,
    m_AlphaTextures: Vec<PPtr>,
    m_AlphamapResolution: i32,
    m_BaseMapResolution: i32,
    m_ColorSpace: Option<i32>,
    m_MaterialRequiresMetallic: Option<bool>,
    m_MaterialRequiresSmoothness: Option<bool>,
    m_TerrainLayers: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SplatPrototype {
    texture: PPtr,
    tileSize: Vector2f,
    tileOffset: Vector2f,
    normalMap: Option<PPtr>,
    specularMetallic: Option<Vector4f>,
    smoothness: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpringJoint {
    m_GameObject: PPtr,
    m_ConnectedBody: PPtr,
    m_Anchor: Vector3f,
    m_Spring: f32,
    m_Damper: f32,
    m_MinDistance: f32,
    m_MaxDistance: f32,
    m_BreakForce: f32,
    m_BreakTorque: f32,
    m_AutoConfigureConnectedAnchor: Option<bool>,
    m_ConnectedAnchor: Option<Vector3f>,
    m_EnableCollision: Option<bool>,
    m_EnablePreprocessing: Option<bool>,
    m_Tolerance: Option<f32>,
    m_MassScale: Option<f32>,
    m_ConnectedMassScale: Option<f32>,
    m_Enabled: Option<bool>,
    m_ConnectedArticulationBody: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpringJoint2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_CollideConnected: Option<bool>,
    m_ConnectedRigidBody: PPtr,
    m_Anchor: Vector2f,
    m_ConnectedAnchor: Vector2f,
    m_Distance: f32,
    m_DampingRatio: f32,
    m_Frequency: f32,
    m_EnableCollision: Option<bool>,
    m_BreakForce: Option<f32>,
    m_BreakTorque: Option<f32>,
    m_AutoConfigureConnectedAnchor: Option<bool>,
    m_AutoConfigureDistance: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sprite {
    m_Name: String,
    m_Rect: Rectf,
    m_Offset: Vector2f,
    m_PixelsToUnits: f32,
    m_Extrude: u32,
    m_RD: SpriteRenderData,
    m_Border: Option<Vector4f>,
    m_IsPolygon: Option<bool>,
    m_Pivot: Option<Vector2f>,
    m_RenderDataKey: Option<(GUID, i64)>,
    m_AtlasTags: Option<Vec<String>>,
    m_SpriteAtlas: Option<PPtr>,
    m_PhysicsShape: Option<Vec<Vec<Vector2f>>>,
    m_Bones: Option<Vec<SpriteBone>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlas {
    m_Name: String,
    m_PackedSprites: Vec<PPtr>,
    m_PackedSpriteNamesToIndex: Vec<String>,
    m_RenderDataMap: Vec<((GUID, i64), SpriteAtlasData)>,
    m_Tag: String,
    m_IsVariant: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlasAsset {
    m_Name: String,
    m_MasterAtlas: PPtr,
    m_ImporterData: (Option<SpriteAtlasAssetData>, Option<SpriteAtlasEditorData>),
    m_IsVariant: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlasAssetData {
    packables: Vec<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlasData {
    texture: PPtr,
    alphaTexture: PPtr,
    textureRect: Rectf,
    textureRectOffset: Vector2f,
    uvTransform: Vector4f,
    downscaleMultiplier: f32,
    settingsRaw: u32,
    atlasRectOffset: Option<Vector2f>,
    secondaryTextures: Option<Vec<SecondarySpriteTexture>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlasDatabase {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlasEditorData {
    textureSettings: TextureSettings,
    platformSettings: Vec<TextureImporterPlatformSettings>,
    packingSettings: PackingSettings,
    variantMultiplier: f32,
    packables: Vec<PPtr>,
    totalSpriteSurfaceArea: Option<u32>,
    bindAsDefault: bool,
    storedHash: Option<Hash128>,
    isAtlasV2: bool,
    cachedData: PPtr,
    secondaryTextureSettings: Option<Vec<(String, SecondaryTextureSettings)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlasImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UsedFileIDs: Vec<i64>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
    m_TextureSettings: Option<TextureSettings>,
    m_PlatformSettings: Option<Vec<TextureImporterPlatformSettings>>,
    m_PackingSettings: Option<PackingSettings>,
    m_SecondaryTextureSettings: Option<Vec<(String, SecondaryTextureSettings)>>,
    m_VariantMultiplier: Option<f32>,
    m_BindAsDefault: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteBone {
    name: String,
    position: Vector3f,
    rotation: Quaternionf,
    length: f32,
    parentId: i32,
    guid: Option<String>,
    color: Option<ColorRGBA>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteData {
    sprite: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteMask {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: u8,
    m_ReceiveShadows: u8,
    m_MotionVectors: u8,
    m_LightProbeUsage: u8,
    m_ReflectionProbeUsage: u8,
    m_LightmapIndex: u16,
    m_LightmapIndexDynamic: u16,
    m_LightmapTilingOffset: Vector4f,
    m_LightmapTilingOffsetDynamic: Vector4f,
    m_Materials: Vec<PPtr>,
    m_StaticBatchInfo: StaticBatchInfo,
    m_StaticBatchRoot: PPtr,
    m_ProbeAnchor: PPtr,
    m_LightProbeVolumeOverride: PPtr,
    m_SortingLayerID: i32,
    m_SortingLayer: i16,
    m_SortingOrder: i16,
    m_Sprite: PPtr,
    m_MaskAlphaCutoff: f32,
    m_FrontSortingLayer: i16,
    m_BackSortingLayer: i16,
    m_FrontSortingOrder: i16,
    m_BackSortingOrder: i16,
    m_IsCustomRangeActive: bool,
    m_DynamicOccludee: Option<u8>,
    m_RenderingLayerMask: Option<u32>,
    m_SpriteSortPoint: Option<i32>,
    m_RendererPriority: Option<i32>,
    m_FrontSortingLayerID: Option<i32>,
    m_BackSortingLayerID: Option<i32>,
    m_RayTracingMode: Option<u8>,
    m_RayTraceProcedural: Option<u8>,
    m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteMetaData {
    m_Name: String,
    m_Rect: Rectf,
    m_Alignment: i32,
    m_Pivot: Vector2f,
    m_Border: Option<Vector4f>,
    m_Outline: Option<Vec<Vec<Vector2f>>>,
    m_TessellationDetail: Option<f32>,
    m_PhysicsShape: Option<Vec<Vec<Vector2f>>>,
    m_Bones: Option<Vec<SpriteBone>>,
    m_SpriteID: Option<String>,
    m_Vertices: Option<Vec<Vector2f>>,
    m_Indices: Option<Vec<i32>>,
    m_Edges: Option<Vec<int2_storage>>,
    m_Weights: Option<Vec<BoneWeights4>>,
    m_InternalID: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteRenderData {
    texture: PPtr,
    vertices: Option<Vec<SpriteVertex>>,
    indices: Option<Vec<u16>>,
    textureRect: Rectf,
    textureRectOffset: Vector2f,
    settingsRaw: u32,
    uvTransform: Option<Vector4f>,
    alphaTexture: Option<PPtr>,
    atlasRectOffset: Option<Vector2f>,
    m_SubMeshes: Option<Vec<SubMesh>>,
    m_IndexBuffer: Option<Vec<u8>>,
    m_VertexData: Option<VertexData>,
    downscaleMultiplier: Option<f32>,
    m_Bindpose: Option<Vec<Matrix4x4f>>,
    m_SourceSkin: Option<Vec<BoneWeights4>>,
    secondaryTextures: Option<Vec<SecondarySpriteTexture>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteRenderer {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: (Option<u8>, Option<bool>),
    m_ReceiveShadows: (Option<u8>, Option<bool>),
    m_LightmapIndex: (Option<u8>, Option<u16>),
    m_LightmapTilingOffset: Vector4f,
    m_Materials: Vec<PPtr>,
    m_SubsetIndices: Option<Vec<u32>>,
    m_StaticBatchRoot: PPtr,
    m_UseLightProbes: Option<bool>,
    m_LightProbeAnchor: Option<PPtr>,
    m_SortingLayer: Option<i16>,
    m_SortingOrder: i16,
    m_Sprite: PPtr,
    m_Color: ColorRGBA,
    m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    m_LightmapIndexDynamic: Option<u16>,
    m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    m_ProbeAnchor: Option<PPtr>,
    m_FlipX: Option<bool>,
    m_FlipY: Option<bool>,
    m_MotionVectors: Option<u8>,
    m_LightProbeUsage: Option<u8>,
    m_LightProbeVolumeOverride: Option<PPtr>,
    m_StaticBatchInfo: Option<StaticBatchInfo>,
    m_DrawMode: Option<i32>,
    m_Size: Option<Vector2f>,
    m_AdaptiveModeThreshold: Option<f32>,
    m_SpriteTileMode: Option<i32>,
    m_WasSpriteAssigned: Option<bool>,
    m_MaskInteraction: Option<i32>,
    m_DynamicOccludee: Option<u8>,
    m_RenderingLayerMask: Option<u32>,
    m_SpriteSortPoint: Option<i32>,
    m_RendererPriority: Option<i32>,
    m_RayTracingMode: Option<u8>,
    m_RayTraceProcedural: Option<u8>,
    m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteShapeRenderer {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: u8,
    m_ReceiveShadows: u8,
    m_DynamicOccludee: u8,
    m_MotionVectors: u8,
    m_LightProbeUsage: u8,
    m_ReflectionProbeUsage: u8,
    m_RenderingLayerMask: u32,
    m_LightmapIndex: u16,
    m_LightmapIndexDynamic: u16,
    m_LightmapTilingOffset: Vector4f,
    m_LightmapTilingOffsetDynamic: Vector4f,
    m_Materials: Vec<PPtr>,
    m_StaticBatchInfo: StaticBatchInfo,
    m_StaticBatchRoot: PPtr,
    m_ProbeAnchor: PPtr,
    m_LightProbeVolumeOverride: PPtr,
    m_SortingLayerID: i32,
    m_SortingLayer: i16,
    m_SortingOrder: i16,
    m_Color: ColorRGBA,
    m_MaskInteraction: i32,
    m_ShapeTexture: PPtr,
    m_Sprites: Vec<PPtr>,
    m_LocalAABB: AABB,
    m_RendererPriority: Option<i32>,
    m_RayTracingMode: Option<u8>,
    m_RayTraceProcedural: Option<u8>,
    m_StaticShadowCaster: Option<u8>,
    m_SpriteSortPoint: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteSheetMetaData {
    m_Sprites: Vec<SpriteMetaData>,
    m_Outline: Option<Vec<Vec<Vector2f>>>,
    m_PhysicsShape: Option<Vec<Vec<Vector2f>>>,
    m_Bones: Option<Vec<SpriteBone>>,
    m_SpriteID: Option<String>,
    m_Vertices: Option<Vec<Vector2f>>,
    m_Indices: Option<Vec<i32>>,
    m_Edges: Option<Vec<int2_storage>>,
    m_Weights: Option<Vec<BoneWeights4>>,
    m_InternalID: Option<i64>,
    m_SecondaryTextures: Option<Vec<SecondarySpriteTexture>>,
    m_NameFileIdTable: Option<Vec<(String, i64)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteTilingProperty {
    border: Vector4f,
    pivot: Vector2f,
    oldSize: Vector2f,
    newSize: Vector2f,
    adaptiveTilingThreshold: f32,
    drawMode: i32,
    adaptiveTiling: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteVertex {
    pos: Vector3f,
    uv: Option<Vector2f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    m_Name: String,
    m_Speed: f32,
    m_Motions: Option<Vec<PPtr>>,
    m_ParentStateMachine: Option<PPtr>,
    m_Position: Vector3f,
    m_IKOnFeet: bool,
    m_Tag: String,
    m_CycleOffset: Option<f32>,
    m_Mirror: Option<bool>,
    m_Transitions: Option<Vec<PPtr>>,
    m_StateMachineBehaviours: Option<Vec<PPtr>>,
    m_WriteDefaultValues: Option<bool>,
    m_Motion: Option<PPtr>,
    m_SpeedParameterActive: Option<bool>,
    m_MirrorParameterActive: Option<bool>,
    m_CycleOffsetParameterActive: Option<bool>,
    m_SpeedParameter: Option<String>,
    m_MirrorParameter: Option<String>,
    m_CycleOffsetParameter: Option<String>,
    m_TimeParameterActive: Option<bool>,
    m_TimeParameter: Option<String>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct StateConstant {
//     m_TransitionConstantArray: Vec<OffsetPtr>,
//     m_BlendTreeConstantIndexArray: Vec<i32>,
//     m_LeafInfoArray: Option<Vec<LeafInfoConstant>>,
//     m_BlendTreeConstantArray: Vec<OffsetPtr>,
//     m_ID: Option<u32>,
//     m_TagID: u32,
//     m_Speed: f32,
//     m_IKOnFeet: bool,
//     m_Loop: bool,
//     m_CycleOffset: Option<f32>,
//     m_Mirror: Option<bool>,
//     m_NameID: Option<u32>,
//     m_PathID: Option<u32>,
//     m_FullPathID: Option<u32>,
//     m_WriteDefaultValues: Option<bool>,
//     m_SpeedParamID: Option<u32>,
//     m_MirrorParamID: Option<u32>,
//     m_CycleOffsetParamID: Option<u32>,
//     m_TimeParamID: Option<u32>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateKey {
    m_StateID: u32,
    m_LayerIndex: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateMachine {
    m_Name: String,
    m_DefaultState: (Option<PPtr>, Option<PPtr>),
    m_States: Option<Vec<PPtr>>,
    m_ChildStateMachine: Option<Vec<PPtr>>,
    m_ChildStateMachinePosition: Option<Vec<Vector3f>>,
    m_LocalTransitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    m_OrderedTransitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    m_MotionSetCount: Option<i32>,
    m_AnyStatePosition: Vector3f,
    m_ParentStateMachinePosition: Vector3f,
    m_ChildStates: Option<Vec<ChildAnimatorState>>,
    m_ChildStateMachines: Option<Vec<ChildAnimatorStateMachine>>,
    m_AnyStateTransitions: Option<Vec<PPtr>>,
    m_EntryTransitions: Option<Vec<PPtr>>,
    m_StateMachineTransitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    m_StateMachineBehaviours: Option<Vec<PPtr>>,
    m_EntryPosition: Option<Vector3f>,
    m_ExitPosition: Option<Vector3f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateMachineBehaviourVectorDescription {
    m_StateMachineBehaviourRanges: Vec<(StateKey, StateRange)>,
    m_StateMachineBehaviourIndices: Vec<u32>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct StateMachineConstant {
//     m_StateConstantArray: Vec<OffsetPtr>,
//     m_AnyStateTransitionConstantArray: Vec<OffsetPtr>,
//     m_DefaultState: u32,
//     m_MotionSetCount: Option<u32>,
//     m_SelectorStateConstantArray: Option<Vec<OffsetPtr>>,
//     m_SynchronizedLayerCount: Option<u32>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateRange {
    m_StartIndex: u32,
    m_Count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaticBatchInfo {
    firstSubMesh: u16,
    subMeshCount: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamInfo {
    channelMask: u32,
    offset: u32,
    stride: (Option<u8>, Option<u32>),
    align: Option<u32>,
    dividerOp: Option<u8>,
    frequency: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamedClip {
    data: Vec<u32>,
    curveCount: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamedResource {
    m_Source: String,
    m_Offset: (Option<u64>, Option<i32>),
    m_Size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamingController {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_StreamingMipmapBias: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamingInfo {
    offset: (Option<u64>, Option<u32>),
    size: u32,
    path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamingManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructParameter {
    m_NameIndex: i32,
    m_Index: i32,
    m_ArraySize: i32,
    m_StructSize: i32,
    m_VectorMembers: Vec<VectorParameter>,
    m_MatrixMembers: Vec<MatrixParameter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StyleSheetImporter {
    m_Name: String,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubCollider {
    m_Collider: PPtr,
    m_ColliderPaths: Vec<Vec<IntPoint>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubDerived {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubEmitterData {
    emitter: PPtr,
    __type: i32,
    properties: i32,
    emitProbability: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubMesh {
    firstByte: u32,
    indexCount: u32,
    isTriStrip: Option<u32>,
    triangleCount: Option<u32>,
    firstVertex: u32,
    vertexCount: u32,
    localAABB: AABB,
    topology: Option<i32>,
    baseVertex: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubModule {
    enabled: bool,
    subEmitterBirth: Option<PPtr>,
    subEmitterDeath: Option<PPtr>,
    subEmitterCollision: Option<PPtr>,
    subEmitterBirth1: Option<PPtr>,
    subEmitterCollision1: Option<PPtr>,
    subEmitterDeath1: Option<PPtr>,
    subEmitters: Option<Vec<SubEmitterData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubstanceArchive {
    m_Name: String,
    m_PackageData: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubstanceEnumItem {
    value: i32,
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubstanceImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_MaterialInstances: Option<Vec<MaterialInstanceSettings>>,
    m_IsFirstImport: Option<i32>,
    m_DeletedPrototypes: Option<Vec<String>>,
    m_MaterialImportOutputs: Option<Vec<MaterialImportOutput>>,
    m_UserData: Option<String>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubstanceInput {
    name: String,
    group: Option<String>,
    __type: i32,
    value: SubstanceValue,
    internalType: i32,
    internalIndex: u32,
    internalIdentifier: Option<u32>,
    minimum: f32,
    maximum: f32,
    step: f32,
    flags: u32,
    alteredTexturesUID: Vec<u32>,
    enumValues: Vec<SubstanceEnumItem>,
    label: Option<String>,
    componentLabels: Option<Vec<String>>,
    visibleIf: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubstanceValue {
    texture: PPtr,
    scalar: (f32, f32, f32, f32),
    stringvalue: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SurfaceEffector2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_ColliderMask: BitField,
    m_Speed: f32,
    m_SpeedVariation: f32,
    m_ForceScale: Option<f32>,
    m_UseContactForce: Option<bool>,
    m_UseFriction: Option<bool>,
    m_UseBounce: Option<bool>,
    m_UseColliderMask: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagManager {
    tags: Vec<String>,
    Builtin_Layer_0: Option<String>,
    Builtin_Layer_1: Option<String>,
    Builtin_Layer_2: Option<String>,
    Builtin_Layer_3: Option<String>,
    Builtin_Layer_4: Option<String>,
    Builtin_Layer_5: Option<String>,
    Builtin_Layer_6: Option<String>,
    Builtin_Layer_7: Option<String>,
    User_Layer_8: Option<String>,
    User_Layer_9: Option<String>,
    User_Layer_10: Option<String>,
    User_Layer_11: Option<String>,
    User_Layer_12: Option<String>,
    User_Layer_13: Option<String>,
    User_Layer_14: Option<String>,
    User_Layer_15: Option<String>,
    User_Layer_16: Option<String>,
    User_Layer_17: Option<String>,
    User_Layer_18: Option<String>,
    User_Layer_19: Option<String>,
    User_Layer_20: Option<String>,
    User_Layer_21: Option<String>,
    User_Layer_22: Option<String>,
    User_Layer_23: Option<String>,
    User_Layer_24: Option<String>,
    User_Layer_25: Option<String>,
    User_Layer_26: Option<String>,
    User_Layer_27: Option<String>,
    User_Layer_28: Option<String>,
    User_Layer_29: Option<String>,
    User_Layer_30: Option<String>,
    User_Layer_31: Option<String>,
    m_SortingLayers: Option<Vec<SortingLayerEntry>>,
    layers: Option<Vec<String>>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct TakeInfo {
//     name: String,
//     defaultClipName: String,
//     startTime: f32,
//     stopTime: f32,
//     bakeStartTime: f32,
//     bakeStopTime: f32,
//     sampleRate: f32,
//     clip: PPtr,
//     internalID: Option<i64>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TargetJoint2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_EnableCollision: bool,
    m_ConnectedRigidBody: PPtr,
    m_BreakForce: f32,
    m_BreakTorque: f32,
    m_Anchor: Vector2f,
    m_Target: Vector2f,
    m_AutoConfigureTarget: bool,
    m_MaxForce: f32,
    m_DampingRatio: f32,
    m_Frequency: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Terrain {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_TerrainData: PPtr,
    m_TreeDistance: f32,
    m_TreeBillboardDistance: f32,
    m_TreeCrossFadeLength: f32,
    m_TreeMaximumFullLODCount: i32,
    m_DetailObjectDistance: f32,
    m_DetailObjectDensity: f32,
    m_HeightmapPixelError: f32,
    m_SplatMapDistance: f32,
    m_HeightmapMaximumLOD: i32,
    m_CastShadows: Option<bool>,
    m_DrawHeightmap: bool,
    m_DrawTreesAndFoliage: bool,
    m_ReflectionProbeUsage: i32,
    m_MaterialType: Option<i32>,
    m_LegacySpecular: Option<ColorRGBA>,
    m_LegacyShininess: Option<f32>,
    m_UseDefaultSmoothness: Option<bool>,
    m_DefaultSmoothness: Option<f32>,
    m_MaterialTemplate: PPtr,
    m_LightmapIndex: u16,
    m_LightmapIndexDynamic: u16,
    m_LightmapTilingOffset: Vector4f,
    m_LightmapTilingOffsetDynamic: Vector4f,
    m_BakeLightProbesForTrees: bool,
    m_DynamicUVST: Vector4f,
    m_ChunkDynamicUVST: Vector4f,
    m_ExplicitProbeSetHash: Option<Hash128>,
    m_PreserveTreePrototypeLayers: Option<bool>,
    m_DrawInstanced: Option<bool>,
    m_GroupingID: Option<i32>,
    m_AllowAutoConnect: Option<bool>,
    m_ShadowCastingMode: Option<i32>,
    m_RenderingLayerMask: Option<u32>,
    m_StaticShadowCaster: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerrainCollider {
    m_GameObject: PPtr,
    m_Material: Option<PPtr>,
    m_IsTrigger: Option<bool>,
    m_TerrainData: PPtr,
    m_CreateTreeColliders: Option<bool>,
    m_Enabled: Option<bool>,
    m_EnableTreeColliders: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerrainData {
    m_Name: String,
    m_SplatDatabase: SplatDatabase,
    m_DetailDatabase: DetailDatabase,
    m_Heightmap: Heightmap,
    m_PreloadShaders: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerrainLayer {
    m_Name: String,
    m_DiffuseTexture: PPtr,
    m_NormalMapTexture: PPtr,
    m_MaskMapTexture: PPtr,
    m_TileSize: Vector2f,
    m_TileOffset: Vector2f,
    m_Specular: ColorRGBA,
    m_Metallic: f32,
    m_Smoothness: f32,
    m_NormalScale: f32,
    m_DiffuseRemapMin: Vector4f,
    m_DiffuseRemapMax: Vector4f,
    m_MaskMapRemapMin: Vector4f,
    m_MaskMapRemapMax: Vector4f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectVectorPairStringBool {
    m_Map: Vec<(String, bool)>,
    m_String: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectWithSerializedAnimationCurve {
    m_Curve: AnimationCurve,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectWithSerializedArray {
    m_IntegerArray: Vec<i32>,
    m_ClampTestValue: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectWithSerializedMapStringBool {
    m_Map: Vec<(String, bool)>,
    m_String: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectWithSerializedMapStringNonAlignedStruct {
    m_Map: Vec<(String, NonAlignedStruct)>,
    m_String: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectWithSpecialLayoutOne {
    differentLayout: LayoutDataOne,
    sameLayout: LayoutDataOne,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectWithSpecialLayoutTwo {
    differentLayout: LayoutDataTwo,
    sameLayout: LayoutDataThree,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tetrahedron {
    matrix: Matrix3x4f,
    indices: (i32, i32, i32, i32),
    neighbors: (i32, i32, i32, i32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextAsset {
    m_Name: String,
    m_Script: String,
    m_PathName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextMesh {
    m_GameObject: PPtr,
    m_Text: String,
    m_OffsetZ: f32,
    m_CharacterSize: f32,
    m_LineSpacing: f32,
    m_Anchor: i16,
    m_Alignment: i16,
    m_TabSize: f32,
    m_FontSize: i32,
    m_FontStyle: i32,
    m_Font: PPtr,
    m_RichText: Option<bool>,
    m_Color: Option<ColorRGBA>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextScriptImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_UserData: Option<String>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Texture {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Texture2D {
    m_Name: String,
    m_Width: i32,
    m_Height: i32,
    m_CompleteImageSize: (Option<u32>, Option<i32>),
    m_TextureFormat: i32,
    m_MipMap: Option<bool>,
    m_IsReadable: bool,
    m_ReadAllowed: Option<bool>,
    m_ImageCount: i32,
    m_TextureDimension: i32,
    m_TextureSettings: GLTextureSettings,
    m_LightmapFormat: i32,
    image_data: Vec<u8>,
    m_ColorSpace: Option<i32>,
    m_MipCount: Option<i32>,
    m_StreamData: Option<StreamingInfo>,
    m_ForcedFallbackFormat: Option<i32>,
    m_DownscaleFallback: Option<bool>,
    m_StreamingMipmaps: Option<bool>,
    m_StreamingMipmapsPriority: Option<i32>,
    m_IgnoreMasterTextureLimit: Option<bool>,
    m_IsPreProcessed: Option<bool>,
    m_MipsStripped: Option<i32>,
    m_IsAlphaChannelOptional: Option<bool>,
    m_PlatformBlob: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Texture2DArray {
    m_Name: String,
    m_Width: i32,
    m_Height: i32,
    m_Depth: i32,
    m_Format: i32,
    m_MipCount: i32,
    m_DataSize: u32,
    m_TextureSettings: GLTextureSettings,
    m_ColorSpace: i32,
    m_IsReadable: bool,
    image_data: Vec<u8>,
    m_StreamData: Option<StreamingInfo>,
    m_ForcedFallbackFormat: Option<i32>,
    m_DownscaleFallback: Option<bool>,
    m_IsAlphaChannelOptional: Option<bool>,
    m_UsageMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Texture3D {
    m_Name: String,
    m_Width: i32,
    m_Height: i32,
    m_Depth: Option<i32>,
    m_Format: Option<(Option<u32>, Option<i32>)>,
    m_MipMap: Option<bool>,
    m_DataSize: Option<u32>,
    m_TextureSettings: GLTextureSettings,
    image_data: Vec<u8>,
    m_CompleteImageSize: Option<i32>,
    m_TextureFormat: Option<i32>,
    m_IsReadable: Option<bool>,
    m_ReadAllowed: Option<bool>,
    m_ImageCount: Option<i32>,
    m_TextureDimension: Option<i32>,
    m_LightmapFormat: Option<i32>,
    m_ColorSpace: Option<i32>,
    m_MipCount: Option<i32>,
    m_StreamData: Option<StreamingInfo>,
    m_ForcedFallbackFormat: Option<i32>,
    m_DownscaleFallback: Option<bool>,
    m_IsAlphaChannelOptional: Option<bool>,
    m_UsageMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureImportInstructions {
    compressedFormat: i32,
    uncompressedFormat: i32,
    recommendedFormat: Option<i32>,
    usageMode: i32,
    colorSpace: i32,
    width: i32,
    height: i32,
    compressionQuality: i32,
    desiredFormat: Option<i32>,
    androidETC2FallbackFormat: Option<i32>,
    androidETC2FallbackDownscale: Option<bool>,
    vtOnly: Option<bool>,
    depth: Option<i32>,
    cubeIntermediateSize: Option<i32>,
    cubeMode: Option<i32>,
    cubeLayout: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureImportOutput {
    textureImportInstructions: TextureImportInstructions,
    sourceTextureInformation: SourceTextureInformation,
    importInspectorWarnings: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<(Option<Vec<(i64, String)>>, Option<Vec<(i32, String)>>)>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_MipMapMode: i32,
    m_EnableMipMap: i32,
    m_CorrectGamma: Option<i32>,
    m_FadeOut: i32,
    m_BorderMipMap: i32,
    m_MipMapFadeDistanceStart: i32,
    m_MipMapFadeDistanceEnd: i32,
    m_ConvertToNormalMap: i32,
    m_ExternalNormalMap: i32,
    m_IsReadable: i32,
    m_HeightScale: f32,
    m_NormalMapFilter: i32,
    m_GrayScaleToAlpha: i32,
    m_GenerateCubemap: i32,
    m_TextureFormat: i32,
    m_MaxTextureSize: i32,
    m_TextureSettings: GLTextureSettings,
    m_NPOTScale: i32,
    m_Lightmap: i32,
    m_TextureType: i32,
    m_RecommendedTextureFormat: Option<i32>,
    m_SourceTextureInformation: Option<SourceTextureInformation>,
    m_BuildTargetSettings: Option<Vec<BuildTargetSettings>>,
    m_LinearTexture: Option<i32>,
    correctGamma: Option<i32>,
    m_CompressionQuality: Option<i32>,
    m_SeamlessCubemap: Option<i32>,
    m_Output: Option<TextureImportOutput>,
    m_UserData: Option<String>,
    m_AlphaIsTransparency: Option<i32>,
    m_SpriteMode: Option<i32>,
    m_SpriteExtrude: Option<u32>,
    m_SpriteMeshType: Option<i32>,
    m_Alignment: Option<i32>,
    m_SpritePivot: Option<Vector2f>,
    m_SpritePixelsToUnits: Option<f32>,
    m_SpriteSheet: Option<SpriteSheetMetaData>,
    m_SpritePackingTag: Option<String>,
    m_SpriteBorder: Option<Vector4f>,
    m_CubemapConvolution: Option<i32>,
    m_CubemapConvolutionSteps: Option<i32>,
    m_CubemapConvolutionExponent: Option<f32>,
    m_RGBM: Option<i32>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_AllowsAlphaSplitting: Option<i32>,
    m_SpriteTessellationDetail: Option<f32>,
    m_sRGBTexture: Option<i32>,
    m_AlphaUsage: Option<i32>,
    m_TextureShape: Option<i32>,
    m_MaxTextureSizeSet: Option<i32>,
    m_CompressionQualitySet: Option<i32>,
    m_TextureFormatSet: Option<i32>,
    m_PlatformSettings: Option<(
        Option<Vec<PlatformSettings>>,
        Option<Vec<TextureImporterPlatformSettings>>,
    )>,
    m_MipMapsPreserveCoverage: Option<i32>,
    m_AlphaTestReferenceValue: Option<f32>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_SpriteGenerateFallbackPhysicsShape: Option<i32>,
    m_SingleChannelComponent: Option<i32>,
    m_StreamingMipmaps: Option<i32>,
    m_StreamingMipmapsPriority: Option<i32>,
    m_PushPullDilation: Option<i32>,
    m_PSDRemoveMatte: Option<bool>,
    m_PSDShowRemoveMatteOption: Option<bool>,
    m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
    m_UsedFileIDs: Option<Vec<i64>>,
    m_ApplyGammaDecoding: Option<i32>,
    m_IgnorePngGamma: Option<(Option<bool>, Option<i32>)>,
    m_VTOnly: Option<i32>,
    m_FlipbookRows: Option<i32>,
    m_FlipbookColumns: Option<i32>,
    m_IgnoreMasterTextureLimit: Option<i32>,
    m_FlipGreenChannel: Option<i32>,
    m_Swizzle: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureImporterPlatformSettings {
    m_BuildTarget: String,
    m_MaxTextureSize: i32,
    m_ResizeAlgorithm: i32,
    m_TextureFormat: i32,
    m_TextureCompression: i32,
    m_CompressionQuality: i32,
    m_CrunchedCompression: bool,
    m_AllowsAlphaSplitting: bool,
    m_Overridden: bool,
    m_AndroidETC2FallbackOverride: i32,
    m_ForceMaximumCompressionQuality_BC6H_BC7: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureParameter {
    m_NameIndex: i32,
    m_Index: i32,
    m_SamplerIndex: i32,
    m_Dim: i8,
    m_MultiSampled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureParameters {
    width: i32,
    height: i32,
    mipLevels: i32,
    textureFormat: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureSettings {
    anisoLevel: i32,
    compressionQuality: i32,
    maxTextureSize: i32,
    textureCompression: i32,
    filterMode: i32,
    generateMipMaps: bool,
    readable: bool,
    crunchedCompression: bool,
    sRGB: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TierGraphicsSettings {
    renderingPath: i32,
    useCascadedShadowMaps: bool,
    hdrMode: Option<i32>,
    useHDR: Option<bool>,
    realtimeGICPUUsage: Option<i32>,
    enableLPPV: Option<bool>,
    prefer32BitShadowMaps: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tile {
    m_TileIndex: u32,
    m_TileSpriteIndex: u32,
    m_TileMatrixIndex: (Option<u32>, Option<u16>),
    m_TileColorIndex: (Option<u32>, Option<u16>),
    m_ObjectToInstantiate: Option<PPtr>,
    m_TileFlags: Option<(Option<u32>, Option<i32>)>,
    m_ColliderType: Option<i32>,
    m_TileObjectToInstantiateIndex: Option<u16>,
    m_AllTileFlags: Option<u32>,
    dummyAlignment: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TileAnimationData {
    m_AnimatedSprites: Vec<PPtr>,
    m_AnimationSpeed: f32,
    m_AnimationTimeOffset: f32,
    m_IsLooping: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tilemap {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Tiles: Vec<(int3_storage, Tile)>,
    m_AnimatedTiles: Vec<(int3_storage, TileAnimationData)>,
    m_TileAssetArray: Vec<TilemapRefCountedData>,
    m_TileSpriteArray: Vec<TilemapRefCountedData>,
    m_TileMatrixArray: Vec<TilemapRefCountedData>,
    m_TileColorArray: Vec<TilemapRefCountedData>,
    m_AnimationFrameRate: f32,
    m_Color: ColorRGBA,
    m_Origin: int3_storage,
    m_Size: int3_storage,
    m_TileAnchor: Vector3f,
    m_TileOrientation: i32,
    m_TileOrientationMatrix: Matrix4x4f,
    m_TileObjectToInstantiateArray: Option<Vec<TilemapRefCountedData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TilemapCollider2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Density: f32,
    m_Material: PPtr,
    m_IsTrigger: bool,
    m_UsedByEffector: bool,
    m_UsedByComposite: bool,
    m_Offset: Vector2f,
    m_MaximumTileChangeCount: Option<u32>,
    m_ExtrusionFactor: Option<f32>,
    m_UseDelaunayMesh: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TilemapEditorUserSettings {
    m_LastUsedPalette: PPtr,
    m_FocusMode: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TilemapRefCountedData {
    m_RefCount: u32,
    m_Data: (
        Option<PPtr>,
        Option<PPtr>,
        Option<Matrix4x4f>,
        Option<ColorRGBA>,
        Option<PPtr>,
    ),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TilemapRenderer {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: u8,
    m_ReceiveShadows: u8,
    m_DynamicOccludee: u8,
    m_MotionVectors: u8,
    m_LightProbeUsage: u8,
    m_ReflectionProbeUsage: u8,
    m_LightmapIndex: u16,
    m_LightmapIndexDynamic: u16,
    m_LightmapTilingOffset: Vector4f,
    m_LightmapTilingOffsetDynamic: Vector4f,
    m_Materials: Vec<PPtr>,
    m_StaticBatchInfo: StaticBatchInfo,
    m_StaticBatchRoot: PPtr,
    m_ProbeAnchor: PPtr,
    m_LightProbeVolumeOverride: PPtr,
    m_SortingLayerID: i32,
    m_SortingLayer: i16,
    m_SortingOrder: i16,
    m_ChunkSize: int3_storage,
    m_MaxChunkCount: u32,
    m_MaxFrameAge: u32,
    m_SortOrder: i32,
    m_MaskInteraction: i32,
    m_ChunkCullingBounds: Option<Vector3f>,
    m_RenderingLayerMask: Option<u32>,
    m_DetectChunkCullingBounds: Option<i32>,
    m_RendererPriority: Option<i32>,
    m_Mode: Option<i32>,
    m_RayTracingMode: Option<u8>,
    m_RayTraceProcedural: Option<u8>,
    m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeManager {
    Fixed_Timestep: f32,
    Maximum_Allowed_Timestep: f32,
    m_TimeScale: f32,
    Maximum_Particle_Timestep: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrailModule {
    enabled: bool,
    ratio: f32,
    lifetime: MinMaxCurve,
    minVertexDistance: f32,
    textureMode: i32,
    worldSpace: bool,
    dieWithParticles: bool,
    sizeAffectsWidth: bool,
    sizeAffectsLifetime: bool,
    inheritParticleColor: bool,
    colorOverLifetime: MinMaxGradient,
    widthOverTrail: MinMaxCurve,
    colorOverTrail: MinMaxGradient,
    generateLightingData: Option<bool>,
    mode: Option<i32>,
    ribbonCount: Option<i32>,
    splitSubEmitterRibbons: Option<bool>,
    shadowBias: Option<f32>,
    attachRibbonsToTransform: Option<bool>,
    textureScale: Option<Vector2f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrailRenderer {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: (Option<u8>, Option<bool>),
    m_ReceiveShadows: (Option<u8>, Option<bool>),
    m_LightmapIndex: (Option<u8>, Option<u16>),
    m_LightmapTilingOffset: Vector4f,
    m_Materials: Vec<PPtr>,
    m_SubsetIndices: Option<Vec<u32>>,
    m_StaticBatchRoot: PPtr,
    m_Time: f32,
    m_StartWidth: Option<f32>,
    m_EndWidth: Option<f32>,
    m_Colors: Option<Gradient>,
    m_MinVertexDistance: f32,
    m_Autodestruct: bool,
    m_UseLightProbes: Option<bool>,
    m_LightProbeAnchor: Option<PPtr>,
    m_SortingLayer: Option<i16>,
    m_SortingOrder: Option<i16>,
    m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    m_LightmapIndexDynamic: Option<u16>,
    m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    m_ProbeAnchor: Option<PPtr>,
    m_MotionVectors: Option<u8>,
    m_LightProbeUsage: Option<u8>,
    m_LightProbeVolumeOverride: Option<PPtr>,
    m_StaticBatchInfo: Option<StaticBatchInfo>,
    m_Parameters: Option<LineParameters>,
    m_DynamicOccludee: Option<u8>,
    m_RenderingLayerMask: Option<u32>,
    m_Emitting: Option<bool>,
    m_RendererPriority: Option<i32>,
    m_RayTracingMode: Option<u8>,
    m_RayTraceProcedural: Option<u8>,
    m_StaticShadowCaster: Option<u8>,
    m_MaskInteraction: Option<i32>,
    m_ApplyActiveColorSpace: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transform {
    m_GameObject: PPtr,
    m_LocalRotation: Quaternionf,
    m_LocalPosition: Vector3f,
    m_LocalScale: Vector3f,
    m_Children: Vec<PPtr>,
    m_Father: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransformMaskElement {
    m_Path: String,
    m_Weight: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transition {
    m_Name: String,
    m_SrcState: Option<PPtr>,
    m_DstState: (Option<PPtr>, Option<PPtr>),
    m_TransitionDuration: f32,
    m_TransitionOffset: f32,
    m_Conditions: (Option<Vec<AnimatorCondition>>, Option<Vec<Condition>>),
    m_Atomic: Option<bool>,
    m_Solo: bool,
    m_Mute: bool,
    m_CanTransitionToSelf: Option<bool>,
    m_DstStateMachine: Option<PPtr>,
    m_IsExit: Option<bool>,
    m_ExitTime: Option<f32>,
    m_HasExitTime: Option<bool>,
    m_InterruptionSource: Option<i32>,
    m_OrderedInterruption: Option<bool>,
    m_HasFixedDuration: Option<bool>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct TransitionConstant {
//     m_ConditionConstantArray: Vec<OffsetPtr>,
//     m_DestinationState: u32,
//     m_ID: u32,
//     m_UserID: u32,
//     m_TransitionDuration: f32,
//     m_TransitionOffset: f32,
//     m_Atomic: Option<bool>,
//     m_CanTransitionToSelf: Option<bool>,
//     m_FullPathID: Option<u32>,
//     m_ExitTime: Option<f32>,
//     m_HasExitTime: Option<bool>,
//     m_InterruptionSource: Option<i32>,
//     m_OrderedInterruption: Option<bool>,
//     m_HasFixedDuration: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tree {
    m_GameObject: PPtr,
    m_SpeedTreeWindAsset: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TreeInstance {
    position: Vector3f,
    widthScale: f32,
    heightScale: f32,
    color: ColorRGBA,
    lightmapColor: ColorRGBA,
    index: i32,
    rotation: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TreePrototype {
    prefab: PPtr,
    bendFactor: f32,
    navMeshLod: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerModule {
    enabled: bool,
    collisionShape0: Option<PPtr>,
    collisionShape1: Option<PPtr>,
    collisionShape2: Option<PPtr>,
    collisionShape3: Option<PPtr>,
    collisionShape4: Option<PPtr>,
    collisionShape5: Option<PPtr>,
    inside: i32,
    outside: i32,
    enter: i32,
    exit: i32,
    radiusScale: f32,
    colliderQueryMode: Option<i32>,
    primitives: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrueTypeFontImporter {
    m_Name: String,
    m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    m_OldHashIdentity: Option<MdFour>,
    m_NewHashIdentity: Option<MdFour>,
    m_FontSize: i32,
    m_ForceTextureCase: i32,
    m_RenderMode: Option<i32>,
    m_Style: Option<i32>,
    m_IncludeFontData: bool,
    m_Use2xBehaviour: Option<bool>,
    m_FontNames: Vec<String>,
    m_FontColor: Option<ColorRGBA>,
    m_CustomCharacters: Option<String>,
    m_CharacterSpacing: Option<i32>,
    m_CharacterPadding: Option<i32>,
    m_FontRenderingMode: Option<i32>,
    m_Output: Option<Output>,
    m_UserData: Option<String>,
    m_AssetBundleName: Option<String>,
    m_AssetBundleVariant: Option<String>,
    m_FallbackFontReferences: Option<Vec<PPtr>>,
    m_AscentCalculationMode: Option<i32>,
    m_FontName: Option<String>,
    m_UseLegacyBoundsCalculation: Option<bool>,
    m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    m_ShouldRoundAdvanceValue: Option<bool>,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UAVParameter {
    m_NameIndex: i32,
    m_Index: i32,
    m_OriginalIndex: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UIRenderer {
    m_GameObject: PPtr,
    m_CullTransparentMesh: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UVAnimation {
    x_Tile: i32,
    y_Tile: i32,
    cycles: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UVModule {
    enabled: bool,
    frameOverTime: MinMaxCurve,
    tilesX: i32,
    tilesY: i32,
    animationType: i32,
    rowIndex: i32,
    cycles: f32,
    randomRow: Option<bool>,
    startFrame: Option<MinMaxCurve>,
    uvChannelMask: Option<i32>,
    flipU: Option<f32>,
    flipV: Option<f32>,
    mode: Option<i32>,
    sprites: Option<Vec<SpriteData>>,
    timeMode: Option<i32>,
    fps: Option<f32>,
    speedRange: Option<Vector2f>,
    rowMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityAdsSettings {
    m_Enabled: Option<bool>,
    m_InitializeOnStartup: Option<bool>,
    m_TestMode: Option<bool>,
    m_EnabledPlatforms: Option<u32>,
    m_IosGameId: Option<String>,
    m_AndroidGameId: Option<String>,
    m_GameId: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityAnalyticsManager {
    m_Enabled: Option<bool>,
    m_InitializeOnStartup: Option<bool>,
    m_TestMode: Option<bool>,
    m_TestEventUrl: Option<String>,
    m_TestConfigUrl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityAnalyticsSettings {
    m_Enabled: bool,
    m_InitializeOnStartup: bool,
    m_TestMode: bool,
    m_TestEventUrl: Option<String>,
    m_TestConfigUrl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityConnectSettings {
    UnityPurchasingSettings: UnityPurchasingSettings,
    UnityAnalyticsSettings: UnityAnalyticsSettings,
    m_Enabled: Option<bool>,
    m_TestMode: Option<bool>,
    m_TestEventUrl: Option<String>,
    m_TestConfigUrl: Option<String>,
    CrashReportingSettings: Option<CrashReportingSettings>,
    UnityAdsSettings: Option<UnityAdsSettings>,
    PerformanceReportingSettings: Option<PerformanceReportingSettings>,
    m_TestInitMode: Option<i32>,
    m_EventOldUrl: Option<String>,
    m_EventUrl: Option<String>,
    m_ConfigUrl: Option<String>,
    m_DashboardUrl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityPropertySheet {
    m_TexEnvs: (
        Option<Vec<(String, UnityTexEnv)>>,
        Option<Vec<(FastPropertyName, UnityTexEnv)>>,
    ),
    m_Floats: (
        Option<Vec<(FastPropertyName, f32)>>,
        Option<Vec<(String, f32)>>,
    ),
    m_Colors: (
        Option<Vec<(String, ColorRGBA)>>,
        Option<Vec<(FastPropertyName, ColorRGBA)>>,
    ),
    m_Ints: Option<Vec<(String, i32)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityPurchasingSettings {
    m_Enabled: bool,
    m_TestMode: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityTexEnv {
    m_Texture: PPtr,
    m_Scale: Vector2f,
    m_Offset: Vector2f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateZoneInfo {
    updateZoneCenter: Vector3f,
    updateZoneSize: Vector3f,
    rotation: f32,
    passIndex: i32,
    needSwap: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXCPUBufferData {
    data: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXCPUBufferDesc {
    capacity: u32,
    stride: u32,
    layout: Vec<VFXLayoutElementDesc>,
    initialData: VFXCPUBufferData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXEditorSystemDesc {
    __type: i32,
    flags: i32,
    capacity: u32,
    layer: u32,
    buffers: Vec<VFXMapping>,
    values: Vec<VFXMapping>,
    tasks: Vec<VFXEditorTaskDesc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXEditorTaskDesc {
    __type: i32,
    buffers: Vec<VFXMapping>,
    values: Vec<VFXMapping>,
    params: Vec<VFXMapping>,
    processor: PPtr,
    shaderSourceIndex: i32,
    temporaryBuffers: Option<Vec<VFXMappingTemporary>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXEntryExposed {
    m_Value: (
        Option<Vector4f>,
        Option<Vector2f>,
        Option<u32>,
        Option<Gradient>,
        Option<f32>,
        Option<bool>,
        Option<Vector3f>,
        Option<Matrix4x4f>,
        Option<i32>,
        Option<PPtr>,
        Option<PPtr>,
        Option<AnimationCurve>,
    ),
    m_Name: String,
    m_Overridden: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXEntryExpressionValue {
    m_ExpressionIndex: u32,
    m_Value: (
        Option<Vector4f>,
        Option<Vector2f>,
        Option<u32>,
        Option<Gradient>,
        Option<f32>,
        Option<bool>,
        Option<Vector3f>,
        Option<Matrix4x4f>,
        Option<i32>,
        Option<PPtr>,
        Option<PPtr>,
        Option<AnimationCurve>,
    ),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXEventDesc {
    name: String,
    playSystems: Vec<u32>,
    stopSystems: Vec<u32>,
    initSystems: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXExpressionContainer {
    m_Expressions: Vec<Expression>,
    m_NeedsLocalToWorld: bool,
    m_NeedsWorldToLocal: bool,
    m_NeededMainCameraBuffers: Option<i32>,
    m_MaxCommonExpressionsIndex: Option<u32>,
    m_NeedsMainCamera: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXField {
    m_Array: (
        Option<Vec<VFXEntryExpressionValue>>,
        Option<Vec<VFXEntryExposed>>,
    ),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXGPUBufferDesc {
    __type: i32,
    size: u32,
    layout: Vec<VFXLayoutElementDesc>,
    capacity: u32,
    stride: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXLayoutElementDesc {
    name: String,
    __type: i32,
    offset: VFXLayoutOffset,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXLayoutOffset {
    bucket: u32,
    structure: u32,
    element: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXManager {
    m_IndirectShader: PPtr,
    m_CopyBufferShader: PPtr,
    m_SortShader: PPtr,
    m_RenderPipeSettingsPath: String,
    m_FixedTimeStep: f32,
    m_MaxDeltaTime: f32,
    m_StripUpdateShader: Option<PPtr>,
    m_CompiledVersion: Option<u32>,
    m_RuntimeVersion: Option<u32>,
    m_RuntimeResources: Option<PPtr>,
    m_MaxScrubTime: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXMapping {
    nameId: String,
    index: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXMappingTemporary {
    mapping: VFXMapping,
    pastFrameIndex: u32,
    perCameraBuffer: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXPropertySheetSerializedBase {
    m_Float: VFXField,
    m_Vector2f: VFXField,
    m_Vector3f: VFXField,
    m_Vector4f: VFXField,
    m_Uint: VFXField,
    m_Int: VFXField,
    m_Matrix4x4f: VFXField,
    m_AnimationCurve: VFXField,
    m_Gradient: VFXField,
    m_NamedObject: VFXField,
    m_Bool: VFXField,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXRenderer {
    m_GameObject: PPtr,
    m_Enabled: bool,
    m_CastShadows: u8,
    m_ReceiveShadows: u8,
    m_DynamicOccludee: u8,
    m_MotionVectors: u8,
    m_LightProbeUsage: u8,
    m_ReflectionProbeUsage: u8,
    m_RenderingLayerMask: u32,
    m_RendererPriority: i32,
    m_LightmapIndex: u16,
    m_LightmapIndexDynamic: u16,
    m_LightmapTilingOffset: Vector4f,
    m_LightmapTilingOffsetDynamic: Vector4f,
    m_Materials: Option<Vec<PPtr>>,
    m_StaticBatchInfo: StaticBatchInfo,
    m_StaticBatchRoot: PPtr,
    m_ProbeAnchor: PPtr,
    m_LightProbeVolumeOverride: PPtr,
    m_SortingLayerID: i32,
    m_SortingLayer: i16,
    m_SortingOrder: i16,
    m_RayTracingMode: Option<u8>,
    m_RayTraceProcedural: Option<u8>,
    m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXRendererSettings {
    motionVectorGenerationMode: i32,
    shadowCastingMode: i32,
    receiveShadows: bool,
    reflectionProbeUsage: i32,
    lightProbeUsage: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXShaderSourceDesc {
    compute: bool,
    name: String,
    source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXSystemDesc {
    __type: i32,
    flags: i32,
    capacity: u32,
    layer: u32,
    buffers: Vec<VFXMapping>,
    values: Vec<VFXMapping>,
    tasks: Vec<VFXTaskDesc>,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXTaskDesc {
    __type: i32,
    buffers: Vec<VFXMapping>,
    values: Vec<VFXMapping>,
    params: Vec<VFXMapping>,
    processor: PPtr,
    temporaryBuffers: Option<Vec<VFXMappingTemporary>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXTemporaryGPUBufferDesc {
    desc: VFXGPUBufferDesc,
    frameCount: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VRSettings {
    none: Option<DeviceNone>,
    cardboard: Option<Google>,
    daydream: Option<Google>,
    hololens: Option<HoloLens>,
    oculus: Option<Oculus>,
    enable360StereoCapture: Option<bool>,
    lumin: Option<Lumin>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueArray {
    m_BoolValues: Vec<bool>,
    m_IntValues: Vec<i32>,
    m_FloatValues: Vec<f32>,
    m_VectorValues: Option<Vec<float4>>,
    m_PositionValues: Option<(Option<Vec<float4>>, Option<Vec<float3>>)>,
    m_QuaternionValues: Option<Vec<float4>>,
    m_ScaleValues: Option<(Option<Vec<float4>>, Option<Vec<float3>>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueArrayConstant {
    m_ValueArray: Vec<ValueConstant>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueConstant {
    m_ID: u32,
    m_TypeID: Option<u32>,
    m_Type: u32,
    m_Index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueDelta {
    m_Start: f32,
    m_Stop: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariableBoneCountWeights {
    m_Data: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariantInfo {
    keywords: String,
    passType: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector2f {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector3Curve {
    curve: AnimationCurve,
    path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector3f {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector4f {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VectorParameter {
    m_NameIndex: i32,
    m_Index: i32,
    m_ArraySize: i32,
    m_Type: i8,
    m_Dim: i8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VelocityModule {
    enabled: bool,
    x: MinMaxCurve,
    y: MinMaxCurve,
    z: MinMaxCurve,
    inWorldSpace: bool,
    speedModifier: Option<MinMaxCurve>,
    orbitalX: Option<MinMaxCurve>,
    orbitalY: Option<MinMaxCurve>,
    orbitalZ: Option<MinMaxCurve>,
    orbitalOffsetX: Option<MinMaxCurve>,
    orbitalOffsetY: Option<MinMaxCurve>,
    orbitalOffsetZ: Option<MinMaxCurve>,
    radial: Option<MinMaxCurve>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionControlSettings {
    m_Mode: String,
    m_CollabEditorSettings: CollabEditorSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VertexData {
    m_CurrentChannels: Option<(Option<u32>, Option<i32>)>,
    m_VertexCount: u32,
    m_DataSize: Vec<u8>,
    m_Streams: Option<(
        Option<(StreamInfo, StreamInfo, StreamInfo, StreamInfo)>,
        Option<Vec<StreamInfo>>,
    )>,
    m_Channels: Option<Vec<ChannelInfo>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoBuildInfo {
    m_VideoClipCount: i32,
    m_IsVideoModuleDisabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoClip {
    m_Name: String,
    m_OriginalPath: String,
    m_ProxyWidth: u32,
    m_ProxyHeight: u32,
    Width: u32,
    Height: u32,
    m_FrameRate: f64,
    m_FrameCount: u64,
    m_Format: i32,
    m_AudioChannelCount: Vec<u16>,
    m_AudioSampleRate: Vec<u32>,
    m_AudioLanguage: Vec<String>,
    m_ExternalResources: StreamedResource,
    m_HasSplitAlpha: bool,
    m_PixelAspecRatioNum: Option<u32>,
    m_PixelAspecRatioDen: Option<u32>,
    m_sRGB: Option<bool>,
    m_VideoShaders: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoClipImporterOutput {
    format: Option<i32>,
    settings: Option<VideoClipImporterTargetSettings>,
    encodedWidth: Option<i32>,
    encodedHeight: Option<i32>,
    encodedStartFrame: Option<i32>,
    encodedEndFrame: Option<i32>,
    streamedResource: Option<StreamedResource>,
    sourceFrameRate: Option<f64>,
    sourceFileSize: Option<u64>,
    sourceAudioChannelCount: Option<Vec<u16>>,
    sourceHasAlpha: Option<bool>,
    sourceAudioSampleRate: Option<Vec<u32>>,
    originalWidth: Option<i32>,
    originalHeight: Option<i32>,
    originalFrameCount: Option<i32>,
    encodedSettings: Option<VideoClipImporterTargetSettings>,
    sourcePixelAspectRatioNumerator: Option<u32>,
    sourcePixelAspectRatioDenominator: Option<u32>,
    transcodeSkipped: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoClipImporterTargetSettings {
    enableTranscoding: bool,
    codec: i32,
    resizeFormat: i32,
    aspectRatio: i32,
    customWidth: i32,
    customHeight: i32,
    bitrateMode: i32,
    spatialQuality: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoPlayer {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_VideoClip: PPtr,
    m_TargetCameraAlpha: f32,
    m_TargetCamera: PPtr,
    m_TargetTexture: PPtr,
    m_TargetMaterialRenderer: PPtr,
    m_RenderMode: i32,
    m_AspectRatio: i32,
    m_DataSource: i32,
    m_PlaybackSpeed: f32,
    m_AudioOutputMode: i32,
    m_TargetAudioSources: Vec<PPtr>,
    m_DirectAudioVolumes: Vec<f32>,
    m_Url: String,
    m_TargetMaterialName: Option<String>,
    m_TargetMaterialProperty: String,
    m_EnabledAudioTracks: Vec<bool>,
    m_DirectAudioMutes: Vec<bool>,
    m_ControlledAudioTrackCount: u16,
    m_PlayOnAwake: bool,
    m_SkipOnDrop: bool,
    m_Looping: bool,
    m_WaitForFirstFrame: bool,
    m_FrameReadyEventEnabled: bool,
    m_TimeReference: Option<i32>,
    m_TargetCamera3DLayout: Option<i32>,
    m_VideoShaders: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffect {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Asset: PPtr,
    m_StartSeed: u32,
    m_ResetSeedOnPlay: (Option<u8>, Option<bool>),
    m_PropertySheet: VFXPropertySheetSerializedBase,
    m_InitialEventName: Option<String>,
    m_InitialEventNameOverriden: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectAsset {
    m_Name: String,
    m_Infos: VisualEffectInfo,
    m_Systems: Vec<VFXSystemDesc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectImporter {
    m_Name: String,
    m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    m_UserData: String,
    m_AssetBundleName: String,
    m_AssetBundleVariant: String,
    m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectInfo {
    m_Expressions: VFXExpressionContainer,
    m_PropertySheet: VFXPropertySheetSerializedBase,
    m_ExposedExpressions: Vec<VFXMapping>,
    m_Buffers: Vec<VFXGPUBufferDesc>,
    m_CPUBuffers: Vec<VFXCPUBufferDesc>,
    m_Events: Vec<VFXEventDesc>,
    m_RendererSettings: VFXRendererSettings,
    m_CullingFlags: i32,
    m_UpdateMode: i32,
    m_RuntimeVersion: Option<u32>,
    m_PreWarmDeltaTime: Option<f32>,
    m_PreWarmStepCount: Option<u32>,
    m_TemporaryBuffers: Option<Vec<VFXTemporaryGPUBufferDesc>>,
    m_InitialEventName: Option<String>,
    m_CompilationVersion: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectObject {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectResource {
    m_Name: String,
    m_Graph: PPtr,
    m_ShaderSources: Option<Vec<VFXShaderSourceDesc>>,
    m_Infos: (Option<VisualEffectInfo>, Option<VisualEffectSettings>),
    m_Systems: Option<Vec<VFXEditorSystemDesc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectSettings {
    m_RendererSettings: VFXRendererSettings,
    m_CullingFlags: i32,
    m_UpdateMode: i32,
    m_PreWarmDeltaTime: f32,
    m_PreWarmStepCount: u32,
    m_InitialEventName: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectSubgraph {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectSubgraphBlock {
    m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectSubgraphOperator {
    m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebCamTexture {
    m_Name: String,
    m_ForcedFallbackFormat: Option<i32>,
    m_DownscaleFallback: Option<bool>,
    m_IsAlphaChannelOptional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WheelCollider {
    m_GameObject: PPtr,
    m_Center: Vector3f,
    m_Radius: f32,
    m_SuspensionDistance: f32,
    m_SuspensionSpring: JointSpring,
    m_Mass: f32,
    m_ForwardFriction: WheelFrictionCurve,
    m_SidewaysFriction: WheelFrictionCurve,
    m_Enabled: Option<bool>,
    m_ForceAppPointDistance: Option<f32>,
    m_WheelDampingRate: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WheelFrictionCurve {
    extremumSlip: Option<f32>,
    extremumValue: Option<f32>,
    asymptoteSlip: Option<f32>,
    asymptoteValue: Option<f32>,
    stiffnessFactor: Option<f32>,
    m_ExtremumSlip: Option<f32>,
    m_ExtremumValue: Option<f32>,
    m_AsymptoteSlip: Option<f32>,
    m_AsymptoteValue: Option<f32>,
    m_Stiffness: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WheelJoint2D {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_CollideConnected: Option<bool>,
    m_ConnectedRigidBody: PPtr,
    m_Anchor: Vector2f,
    m_ConnectedAnchor: Vector2f,
    m_Suspension: JointSuspension2D,
    m_UseMotor: bool,
    m_Motor: JointMotor2D,
    m_EnableCollision: Option<bool>,
    m_BreakForce: Option<f32>,
    m_BreakTorque: Option<f32>,
    m_AutoConfigureConnectedAnchor: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindZone {
    m_GameObject: PPtr,
    m_Enabled: u8,
    m_Mode: i32,
    m_Radius: f32,
    m_WindMain: f32,
    m_WindTurbulence: f32,
    m_WindPulseMagnitude: f32,
    m_WindPulseFrequency: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldAnchor {
    m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldParticleCollider {
    m_GameObject: PPtr,
    m_BounceFactor: f32,
    m_CollisionEnergyLoss: f32,
    m_CollidesWith: BitField,
    m_SendCollisionMessage: bool,
    m_MinKillVelocity: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct bitset {
    bitCount: i32,
    bitblocks: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct float3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct float4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct int2_storage {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct int3_storage {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct void {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct xform {
    t: (Option<float3>, Option<float4>),
    q: float4,
    s: (Option<float3>, Option<float4>),
}

use AnimatorTransition as AnimatorStateTransition;
use State as AnimatorState;
use StateMachine as AnimatorStateMachine;
