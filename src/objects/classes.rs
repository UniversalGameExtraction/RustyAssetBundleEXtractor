// TODO: only allow specific warnings
#![allow(warnings)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PPtr {
    pub m_FileID: i64,
    pub m_PathID: i64,
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
    pub m_Center: Vector3f,
    pub m_Extent: Vector3f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ASTCImporter {
    pub m_Name: String,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddedGameObject {
    pub targetCorrespondingSourceObject: PPtr,
    pub insertIndex: i32,
    pub addedObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AimConstraint {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Weight: f32,
    pub m_RotationAtRest: Vector3f,
    pub m_RotationOffset: Vector3f,
    pub m_AimVector: Vector3f,
    pub m_UpVector: Vector3f,
    pub m_WorldUpVector: Vector3f,
    pub m_WorldUpObject: PPtr,
    pub m_UpType: i32,
    pub m_AffectRotationX: bool,
    pub m_AffectRotationY: bool,
    pub m_AffectRotationZ: bool,
    pub m_IsContraintActive: Option<bool>,
    pub m_Sources: Vec<ConstraintSource>,
    pub m_Active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnchoredJoint2D {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AndroidAssetPackImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Animation {
//     pub m_GameObject: PPtr,
//     pub m_Enabled: u8,
//     pub m_Animation: PPtr,
//     pub m_Animations: Vec<PPtr>,
//     pub m_WrapMode: i32,
//     pub m_PlayAutomatically: bool,
//     pub m_AnimatePhysics: bool,
//     pub m_AnimateOnlyIfVisible: Option<bool>,
//     pub m_CullingType: Option<i32>,
//     pub m_UserAABB: Option<AABB>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct AnimationClip {
//     pub m_Name: String,
//     pub m_Compressed: bool,
//     pub m_RotationCurves: Vec<QuaternionCurve>,
//     pub m_CompressedRotationCurves: Vec<CompressedAnimationCurve>,
//     pub m_PositionCurves: Vec<Vector3Curve>,
//     pub m_ScaleCurves: Vec<Vector3Curve>,
//     pub m_FloatCurves: Vec<FloatCurve>,
//     pub m_SampleRate: f32,
//     pub m_WrapMode: i32,
//     pub m_Events: Vec<AnimationEvent>,
//     pub m_Bounds: Option<AABB>,
//     pub m_AnimationType: Option<i32>,
//     pub m_MuscleClipSize: Option<u32>,
//     pub m_MuscleClip: Option<ClipMuscleConstant>,
//     pub m_UseHighQualityCurve: Option<bool>,
//     pub m_PPtrCurves: Option<Vec<PPtrCurve>>,
//     pub m_ClipBindingConstant: Option<AnimationClipBindingConstant>,
//     pub m_Legacy: Option<bool>,
//     pub m_EulerCurves: Option<Vec<Vector3Curve>>,
//     pub m_HasGenericRootTransform: Option<bool>,
//     pub m_HasMotionFloatCurves: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimationClipBindingConstant {
    pub genericBindings: Vec<GenericBinding>,
    pub pptrCurveMapping: Vec<PPtr>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct AnimationClipOverride {
//     pub m_OriginalClip: PPtr,
//     pub m_OverrideClip: PPtr,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimationCurve {
    pub m_Curve: Vec<Keyframe>,
    pub m_PreInfinity: i32,
    pub m_PostInfinity: i32,
    pub m_RotationOrder: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimationEvent {
    pub time: f32,
    pub functionName: String,
    pub data: String,
    pub objectReferenceParameter: PPtr,
    pub floatParameter: f32,
    pub intParameter: i32,
    pub messageOptions: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimationManager {}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Animator {
//     pub m_GameObject: PPtr,
//     pub m_Enabled: u8,
//     pub m_Avatar: PPtr,
//     pub m_Controller: (
//         Option<PPtr>,
//         Option<PPtr>,
//     ),
//     pub m_CullingMode: i32,
//     pub m_ApplyRootMotion: bool,
//     pub m_AnimatePhysics: Option<bool>,
//     pub m_HasTransformHierarchy: Option<bool>,
//     pub m_UpdateMode: Option<i32>,
//     pub m_AllowConstantClipSamplingOptimization: Option<bool>,
//     pub m_LinearVelocityBlending: Option<bool>,
//     pub m_KeepAnimatorControllerStateOnDisable: Option<bool>,
//     pub m_StabilizeFeet: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimatorCondition {
    pub m_ConditionMode: i32,
    pub m_ConditionEvent: String,
    pub m_EventTreshold: f32,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct AnimatorController {
//     pub m_Name: String,
//     pub m_ControllerSize: u32,
//     pub m_Controller: ControllerConstant,
//     pub m_TOS: Vec<(u32, String)>,
//     pub m_AnimationClips: Vec<PPtr>,
//     pub m_StateMachineBehaviourVectorDescription: Option<StateMachineBehaviourVectorDescription>,
//     pub m_StateMachineBehaviours: Option<Vec<PPtr>>,
//     pub m_MultiThreadedStateMachine: Option<bool>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct AnimatorOverrideController {
//     pub m_Name: String,
//     pub m_Controller: PPtr,
//     pub m_Clips: Vec<AnimationClipOverride>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimatorTransition {
    pub m_Name: String,
    pub m_Conditions: Vec<AnimatorCondition>,
    pub m_DstStateMachine: PPtr,
    pub m_DstState: PPtr,
    pub m_Solo: bool,
    pub m_Mute: bool,
    pub m_IsExit: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimatorTransitionBase {
    pub m_Name: String,
    pub m_Conditions: Vec<AnimatorCondition>,
    pub m_DstStateMachine: PPtr,
    pub m_DstState: PPtr,
    pub m_Solo: bool,
    pub m_Mute: bool,
    pub m_IsExit: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Annotation {
    pub m_IconEnabled: bool,
    pub m_GizmoEnabled: bool,
    pub m_ClassID: i32,
    pub m_ScriptClass: String,
    pub m_Flags: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnnotationManager {
    pub m_CurrentPreset_m_AnnotationList: Vec<Annotation>,
    pub m_RecentlyChanged: Vec<Annotation>,
    pub m_IconSize: Option<f32>,
    pub m_WorldIconSize: Option<f32>,
    pub m_Use3dGizmos: Option<bool>,
    pub m_ShowGrid: Option<bool>,
    pub m_ShowSelectionOutline: Option<bool>,
    pub m_ShowSelectionWire: Option<bool>,
    pub m_FadeGizmoSize: Option<f32>,
    pub m_FadeGizmos: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AreaEffector2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_ColliderMask: BitField,
    pub m_ForceDirection: Option<f32>,
    pub m_ForceMagnitude: f32,
    pub m_ForceVariation: f32,
    pub m_Drag: f32,
    pub m_AngularDrag: f32,
    pub m_ForceTarget: (Option<u8>, Option<i32>),
    pub m_UseColliderMask: Option<bool>,
    pub m_UseGlobalAngle: Option<bool>,
    pub m_ForceAngle: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticulationBody {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Mass: f32,
    pub m_ParentAnchorPosition: Vector3f,
    pub m_ParentAnchorRotation: Quaternionf,
    pub m_AnchorPosition: Vector3f,
    pub m_AnchorRotation: Quaternionf,
    pub m_ComputeParentAnchor: Option<bool>,
    pub m_ArticulationJointType: i32,
    pub m_LinearX: i32,
    pub m_LinearY: i32,
    pub m_LinearZ: i32,
    pub m_SwingY: i32,
    pub m_SwingZ: i32,
    pub m_Twist: i32,
    pub m_XDrive: ArticulationDrive,
    pub m_YDrive: ArticulationDrive,
    pub m_ZDrive: ArticulationDrive,
    pub m_LinearDamping: f32,
    pub m_AngularDamping: f32,
    pub m_JointFriction: f32,
    pub m_Immovable: bool,
    pub m_UseGravity: Option<bool>,
    pub m_CollisionDetectionMode: Option<i32>,
    pub m_MatchAnchors: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticulationDrive {
    pub lowerLimit: f32,
    pub upperLimit: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub forceLimit: f32,
    pub target: f32,
    pub targetVelocity: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AspectRatios {
    pub __4x3: bool,
    pub __5x4: bool,
    pub __16x10: bool,
    pub __16x9: bool,
    pub Others: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssemblyDefinitionReferenceAsset {
    pub m_Name: String,
    pub m_Script: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssemblyDefinitionReferenceImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssemblyJsonAsset {
    pub m_Name: String,
    pub m_Script: String,
    pub m_PathName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssemblyJsonImporter {
    pub m_Name: String,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub mainRepresentation: LibraryRepresentation,
    pub representations: Vec<LibraryRepresentation>,
    pub children: Vec<GUID>,
    pub parent: GUID,
    pub digest: Option<MdFour>,
    pub __type: i32,
    pub labels: AssetLabels,
    pub modificationDate: Option<(u32, u32)>,
    pub metaModificationDate: Option<(u32, u32)>,
    pub importerClassId: Option<i32>,
    pub importerVersionHash: Option<u32>,
    pub hash: Option<(Option<MdFour>, Option<Hash128>)>,
    pub assetBundleIndex: Option<i32>,
    pub scriptedImporterClassID: Option<String>,
    pub hashOfImportedAssetDependencies: Option<Vec<GUID>>,
    pub hashOfSourceAssetDependencies: Option<Vec<GUID>>,
    guidOfPathLocationDependencies:
        Option<(Option<Vec<(String, GUID)>>, Option<Vec<(GUID, String)>>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetBundle {
    pub m_Name: String,
    pub m_PreloadTable: Vec<PPtr>,
    pub m_Container: Vec<(String, AssetInfo)>,
    pub m_MainAsset: AssetInfo,
    pub m_ScriptCompatibility: Option<Vec<AssetBundleScriptInfo>>,
    pub m_ClassCompatibility: Option<Vec<(i32, u32)>>,
    pub m_RuntimeCompatibility: Option<u32>,
    pub m_AssetBundleName: Option<String>,
    pub m_Dependencies: Option<Vec<String>>,
    pub m_IsStreamedSceneAssetBundle: Option<bool>,
    pub m_ClassVersionMap: Option<Vec<(i32, i32)>>,
    pub m_PathFlags: Option<i32>,
    pub m_ExplicitDataLayout: Option<i32>,
    pub m_SceneHashes: Option<Vec<(String, String)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetBundleFullName {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetBundleInfo {
    pub AssetBundleHash: Hash128,
    pub AssetBundleDependencies: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetBundleManifest {
    pub m_Name: String,
    pub AssetBundleNames: Vec<(i32, String)>,
    pub AssetBundlesWithVariant: Vec<i32>,
    pub AssetBundleInfos: Vec<(i32, AssetBundleInfo)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetBundleScriptInfo {
    pub className: String,
    pub nameSpace: String,
    pub assemblyName: String,
    pub hash: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetDatabase {
    pub m_Assets: Vec<(GUID, Asset)>,
    pub m_AssetTimeStamps: Option<Vec<(String, AssetTimeStamp)>>,
    pub m_UnityShadersVersion: Option<i32>,
    pub m_lastValidVersionHashes: Option<Vec<(i32, u32)>>,
    pub m_AssetBundleNames: Option<Vec<(i32, AssetBundleFullName)>>,
    pub m_Metrics: Option<AssetDatabaseMetrics>,
    pub m_lastValidVersions: Option<Vec<(AssetImporterHashKey, u32)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetDatabaseMetrics {
    pub totalAssetCount: i32,
    pub nonProAssetCount: Option<i32>,
    pub nonProAssetsCreatedAfterProLicense: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetImportInProgressProxy {
    pub m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetImporter {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetImporterHashKey {
    pub __type: i32,
    pub ScriptClass: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetImporterLog {
    pub m_Name: String,
    pub m_Logs: Vec<AssetImporter_ImportError>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetImporter_ImportError {
    pub error: String,
    pub mode: i32,
    pub line: i32,
    pub file: String,
    pub object: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetInfo {
    pub preloadIndex: i32,
    pub preloadSize: i32,
    pub asset: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetLabels {
    pub m_Labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetMetaData {
    pub guid: GUID,
    pub pathName: String,
    pub originalChangeset: Option<u32>,
    pub originalName: String,
    pub originalParent: Option<GUID>,
    pub originalDigest: Option<(Option<MdFour>, Option<Hash128>)>,
    pub labels: Vec<String>,
    pub assetStoreRef: u64,
    pub timeCreated: Option<u64>,
    pub licenseType: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetServerCache {
    pub m_ObjectHideFlags: Option<u32>,
    pub m_Items: Vec<(GUID, Item)>,
    pub m_DeletedItems: Vec<(GUID, DeletedItem)>,
    pub m_LastCommitMessage: String,
    pub m_CommitItemSelection: Vec<GUID>,
    pub m_WorkingItemMetaData: Vec<(GUID, CachedAssetMetaData)>,
    pub m_LatestServerChangeset: i32,
    pub m_CachesInitialized: i32,
    pub m_ModifiedItems: Vec<(GUID, Item)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetTimeStamp {
    pub modificationDate: (u32, u32),
    pub metaModificationDate: (u32, u32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioBehaviour {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioBuildInfo {
    pub m_IsAudioDisabled: bool,
    pub m_AudioClipCount: i32,
    pub m_AudioMixerCount: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioChorusFilter {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_DryMix: f32,
    pub m_WetMix1: f32,
    pub m_WetMix2: f32,
    pub m_WetMix3: f32,
    pub m_Delay: f32,
    pub m_Rate: f32,
    pub m_Depth: f32,
    pub m_FeedBack: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioClip {
    pub m_Name: String,
    pub m_Format: Option<i32>,
    pub m_Type: Option<i32>,
    pub m_3D: Option<bool>,
    pub m_UseHardware: Option<bool>,
    pub m_AudioData: Option<Vec<u8>>,
    pub m_Stream: Option<(Option<bool>, Option<i32>)>,
    pub m_LoadType: Option<i32>,
    pub m_Channels: Option<i32>,
    pub m_Frequency: Option<i32>,
    pub m_BitsPerSample: Option<i32>,
    pub m_Length: Option<f32>,
    pub m_IsTrackerFormat: Option<bool>,
    pub m_SubsoundIndex: Option<i32>,
    pub m_PreloadAudioData: Option<bool>,
    pub m_LoadInBackground: Option<bool>,
    pub m_Legacy3D: Option<bool>,
    pub m_Resource: Option<StreamedResource>,
    pub m_CompressionFormat: Option<i32>,
    pub m_Ambisonic: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioDistortionFilter {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_DistortionLevel: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioEchoFilter {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Delay: u32,
    pub m_DecayRatio: f32,
    pub m_WetMix: f32,
    pub m_DryMix: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioFilter {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioHighPassFilter {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_CutoffFrequency: f32,
    pub m_HighpassResonanceQ: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_Format: Option<i32>,
    pub m_Quality: Option<f32>,
    pub m_Stream: Option<i32>,
    pub m_PreviewDataLength: Option<u32>,
    pub audio_preview_data: Option<Vec<u8>>,
    pub m_3D: bool,
    pub m_ForceToMono: bool,
    pub m_UseHardware: Option<bool>,
    pub m_Loopable: Option<bool>,
    pub m_Output: Option<(Option<AudioImporterOutput>, Option<Output>)>,
    pub m_UserData: Option<String>,
    pub m_DefaultSettings: Option<SampleSettings>,
    pub m_PlatformSettingOverrides: Option<Vec<(i32, SampleSettings)>>,
    pub m_PreloadAudioData: Option<bool>,
    pub m_LoadInBackground: Option<bool>,
    pub m_PreviewData: Option<PreviewData>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_Normalize: Option<bool>,
    pub m_Ambisonic: Option<bool>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioImporterOutput {
    pub outputSettings: SampleSettings,
    pub outputContainerFormat: i32,
    pub editorOutputSettings: SampleSettings,
    pub editorOutputContainerFormat: i32,
    pub playerResource: Option<StreamedResource>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioListener {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_ExtensionPropertyValues: Option<Vec<ExtensionPropertyValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioLowPassFilter {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_CutoffFrequency: Option<f32>,
    pub m_LowpassResonanceQ: f32,
    pub lowpassLevelCustomCurve: AnimationCurve,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioManager {
    pub m_Volume: f32,
    pub m_SpeedOfSound: Option<f32>,
    pub Doppler_Factor: f32,
    pub Default_Speaker_Mode: i32,
    pub Rolloff_Scale: f32,
    pub iOS_DSP_Buffer_Size: Option<i32>,
    pub m_DSPBufferSize: Option<i32>,
    pub m_DisableAudio: Option<bool>,
    pub m_SampleRate: Option<i32>,
    pub m_VirtualVoiceCount: Option<i32>,
    pub m_RealVoiceCount: Option<i32>,
    pub m_SpatializerPlugin: Option<String>,
    pub m_VirtualizeEffects: Option<bool>,
    pub m_AmbisonicDecoderPlugin: Option<String>,
    pub m_RequestedDSPBufferSize: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixer {
    pub m_Name: String,
    pub m_OutputGroup: PPtr,
    pub m_MasterGroup: PPtr,
    pub m_Snapshots: Vec<PPtr>,
    pub m_StartSnapshot: PPtr,
    pub m_SuspendThreshold: f32,
    pub m_EnableSuspend: bool,
    pub m_MixerConstant: AudioMixerConstant,
    pub m_UpdateMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerConstant {
    pub groups: Vec<GroupConstant>,
    pub groupGUIDs: Vec<GUID>,
    pub effects: Vec<EffectConstant>,
    pub effectGUIDs: Vec<GUID>,
    pub numSideChainBuffers: u32,
    pub snapshots: Vec<SnapshotConstant>,
    pub snapshotGUIDs: Vec<GUID>,
    pub groupNameBuffer: Vec<char>,
    pub snapshotNameBuffer: Vec<char>,
    pub pluginEffectNameBuffer: Vec<char>,
    pub exposedParameterNames: Vec<u32>,
    pub exposedParameterIndices: Vec<u32>,
    pub groupConnections: Option<Vec<GroupConnection>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerController {
    pub m_Name: String,
    pub m_OutputGroup: PPtr,
    pub m_MasterGroup: PPtr,
    pub m_Snapshots: Vec<PPtr>,
    pub m_StartSnapshot: PPtr,
    pub m_SuspendThreshold: f32,
    pub m_EnableSuspend: bool,
    pub m_MixerConstant: AudioMixerConstant,
    pub m_UpdateMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerEffectController {
    pub m_Name: String,
    pub m_EffectID: GUID,
    pub m_EffectName: String,
    pub m_MixLevel: GUID,
    pub m_Parameters: Vec<Parameter>,
    pub m_SendTarget: PPtr,
    pub m_EnableWetMix: bool,
    pub m_Bypass: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerGroup {
    pub m_Name: String,
    pub m_AudioMixer: PPtr,
    pub m_GroupID: GUID,
    pub m_Children: Vec<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerGroupController {
    pub m_Name: String,
    pub m_AudioMixer: PPtr,
    pub m_GroupID: GUID,
    pub m_Children: Vec<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerLiveUpdateBool {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerLiveUpdateFloat {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerSnapshot {
    pub m_Name: String,
    pub m_AudioMixer: PPtr,
    pub m_SnapshotID: GUID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioMixerSnapshotController {
    pub m_Name: String,
    pub m_AudioMixer: PPtr,
    pub m_SnapshotID: GUID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioReverbFilter {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_DryLevel: f32,
    pub m_Room: f32,
    pub m_RoomHF: f32,
    pub m_RoomRolloff: Option<f32>,
    pub m_DecayTime: f32,
    pub m_DecayHFRatio: f32,
    pub m_ReflectionsLevel: f32,
    pub m_ReverbLevel: f32,
    pub m_ReverbDelay: f32,
    pub m_Diffusion: f32,
    pub m_Density: f32,
    pub m_HFReference: f32,
    pub m_RoomLF: f32,
    pub m_LFReference: f32,
    pub m_ReflectionsDelay: f32,
    pub m_ReverbPreset: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioReverbZone {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_MinDistance: f32,
    pub m_MaxDistance: f32,
    pub m_ReverbPreset: i32,
    pub m_Room: i32,
    pub m_RoomHF: i32,
    pub m_DecayTime: f32,
    pub m_DecayHFRatio: f32,
    pub m_Reflections: i32,
    pub m_ReflectionsDelay: f32,
    pub m_Reverb: i32,
    pub m_ReverbDelay: f32,
    pub m_HFReference: f32,
    pub m_RoomRolloffFactor: Option<f32>,
    pub m_Diffusion: f32,
    pub m_Density: f32,
    pub m_LFReference: f32,
    pub m_RoomLF: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioSource {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_audioClip: PPtr,
    pub m_PlayOnAwake: bool,
    pub m_Volume: f32,
    pub m_Pitch: f32,
    pub Loop: bool,
    pub Mute: bool,
    pub Priority: i32,
    pub DopplerLevel: f32,
    pub MinDistance: f32,
    pub MaxDistance: f32,
    pub Pan2D: f32,
    pub rolloffMode: i32,
    pub BypassEffects: bool,
    pub rolloffCustomCurve: AnimationCurve,
    pub panLevelCustomCurve: AnimationCurve,
    pub spreadCustomCurve: AnimationCurve,
    pub BypassListenerEffects: Option<bool>,
    pub BypassReverbZones: Option<bool>,
    pub OutputAudioMixerGroup: Option<PPtr>,
    pub reverbZoneMixCustomCurve: Option<AnimationCurve>,
    pub Spatialize: Option<bool>,
    pub SpatializePostEffects: Option<bool>,
    pub m_ExtensionPropertyValues: Option<Vec<ExtensionPropertyValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoOffMeshLinkData {
    pub m_Start: Vector3f,
    pub m_End: Vector3f,
    pub m_Radius: f32,
    pub m_LinkType: u16,
    pub m_Area: u8,
    pub m_LinkDirection: u8,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Avatar {
//     pub m_Name: String,
//     pub m_AvatarSize: u32,
//     pub m_Avatar: AvatarConstant,
//     pub m_TOS: Vec<(u32, String)>,
//     pub m_HumanDescription: Option<HumanDescription>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvatarBodyMask {
    pub m_Name: String,
    pub m_Mask: Vec<u32>,
    pub m_Elements: Option<Vec<TransformMaskElement>>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct AvatarConstant {
//     pub m_Skeleton: Option<OffsetPtr>,
//     pub m_SkeletonPose: Option<OffsetPtr>,
//     pub m_Human: OffsetPtr,
//     pub m_HumanSkeletonIndexArray: Vec<i32>,
//     pub m_RootMotionBoneIndex: i32,
//     pub m_RootMotionBoneX: xform,
//     pub m_AvatarSkeleton: Option<OffsetPtr>,
//     pub m_AvatarSkeletonPose: Option<OffsetPtr>,
//     pub m_DefaultPose: Option<OffsetPtr>,
//     pub m_SkeletonNameIDArray: Option<Vec<u32>>,
//     pub m_HumanSkeletonReverseIndexArray: Option<Vec<i32>>,
//     pub m_RootMotionSkeleton: Option<OffsetPtr>,
//     pub m_RootMotionSkeletonPose: Option<OffsetPtr>,
//     pub m_RootMotionSkeletonIndexArray: Option<Vec<i32>>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvatarMask {
    pub m_Name: String,
    pub m_Mask: Vec<u32>,
    pub m_Elements: Vec<TransformMaskElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvatarSkeletonMask {
    pub m_Name: String,
    pub elements: Vec<AvatarSkeletonMaskElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvatarSkeletonMaskElement {
    pub path: String,
    pub weight: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Axes {
    pub m_PreQ: float4,
    pub m_PostQ: float4,
    pub m_Sgn: (Option<float3>, Option<float4>),
    pub m_Limit: Limit,
    pub m_Length: f32,
    pub m_Type: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseAnimationTrack {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseVideoTexture {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Behaviour {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BillboardAsset {
    pub m_Name: String,
    pub width: f32,
    pub bottom: f32,
    pub height: f32,
    pub imageTexCoords: Vec<Vector4f>,
    pub rotated: Option<Vec<u8>>,
    pub vertices: Vec<Vector2f>,
    pub indices: Vec<u16>,
    pub material: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BillboardRenderer {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: u8,
    pub m_ReceiveShadows: (Option<u8>, Option<bool>),
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    pub m_Materials: Vec<PPtr>,
    pub m_SubsetIndices: Option<Vec<u32>>,
    pub m_StaticBatchRoot: PPtr,
    pub m_UseLightProbes: Option<bool>,
    pub m_ReflectionProbeUsage: (Option<u8>, Option<i32>),
    pub m_ProbeAnchor: PPtr,
    pub m_SortingLayerID: Option<i32>,
    pub m_SortingOrder: i16,
    pub m_Billboard: PPtr,
    pub m_MotionVectors: Option<u8>,
    pub m_LightProbeUsage: Option<u8>,
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    pub m_SortingLayer: Option<i16>,
    pub m_DynamicOccludee: Option<u8>,
    pub m_RenderingLayerMask: Option<u32>,
    pub m_RendererPriority: Option<i32>,
    pub m_RayTracingMode: Option<u8>,
    pub m_RayTraceProcedural: Option<u8>,
    pub m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BitField {
    pub m_Bits: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blend1dDataConstant {
    pub m_ChildThresholdArray: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blend2dDataConstant {
    pub m_ChildThresholdArray: Option<Vec<f32>>,
    pub m_ChildPositionArray: Option<Vec<Vector2f>>,
    pub m_ChildMagnitudeArray: Option<Vec<f32>>,
    pub m_ChildPairVectorArray: Option<Vec<Vector2f>>,
    pub m_ChildPairAvgMagInvArray: Option<Vec<f32>>,
    pub m_ChildNeighborListArray: Option<Vec<MotionNeighborList>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlendDirectDataConstant {
    pub m_ChildBlendEventIDArray: Vec<u32>,
    pub m_NormalizedBlendValues: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlendShapeData {
    pub vertices: Vec<BlendShapeVertex>,
    pub shapes: Vec<MeshBlendShape>,
    pub channels: Vec<MeshBlendShapeChannel>,
    pub fullWeights: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlendShapeVertex {
    pub vertex: Vector3f,
    pub normal: Vector3f,
    pub tangent: Vector3f,
    pub index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlendTree {
    pub m_Name: String,
    pub m_Childs: (Option<Vec<ChildMotion>>, Option<Vec<Child>>),
    pub m_BlendEvent: Option<String>,
    pub m_MinThreshold: f32,
    pub m_MaxThreshold: f32,
    pub m_UseAutomaticThresholds: bool,
    pub m_BlendEventY: Option<String>,
    pub m_BlendType: Option<(Option<u32>, Option<i32>)>,
    pub m_BlendParameter: Option<String>,
    pub m_BlendParameterY: Option<String>,
    pub m_NormalizedBlendValues: Option<bool>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct BlendTreeConstant {
//     pub m_NodeArray: Vec<OffsetPtr>,
//     pub m_BlendEventArrayConstant: Option<OffsetPtr>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct BlendTreeNodeConstant {
//     pub m_BlendEventID: u32,
//     pub m_ChildIndices: Vec<u32>,
//     pub m_ChildThresholdArray: Option<Vec<f32>>,
//     pub m_ClipID: u32,
//     pub m_Duration: f32,
//     pub m_BlendType: Option<u32>,
//     pub m_BlendEventYID: Option<u32>,
//     pub m_Blend1dData: Option<OffsetPtr>,
//     pub m_Blend2dData: Option<OffsetPtr>,
//     pub m_CycleOffset: Option<f32>,
//     pub m_Mirror: Option<bool>,
//     pub m_ClipIndex: Option<u32>,
//     pub m_BlendDirectData: Option<OffsetPtr>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoneInfluence {
    pub weight: (f32, f32, f32, f32),
    pub boneIndex: (i32, i32, i32, i32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoneWeights4 {
    pub weight: (f32, f32, f32, f32),
    pub boneIndex: (i32, i32, i32, i32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoxCollider {
    pub m_GameObject: PPtr,
    pub m_Material: PPtr,
    pub m_IsTrigger: bool,
    pub m_Size: Vector3f,
    pub m_Center: Vector3f,
    pub m_Enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoxCollider2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Material: PPtr,
    pub m_IsTrigger: bool,
    pub m_Size: Vector2f,
    pub m_Center: Option<Vector2f>,
    pub m_UsedByEffector: Option<bool>,
    pub m_Offset: Option<Vector2f>,
    pub m_Density: Option<f32>,
    pub m_UsedByComposite: Option<bool>,
    pub m_AutoTiling: Option<bool>,
    pub m_SpriteTilingProperty: Option<SpriteTilingProperty>,
    pub m_EdgeRadius: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BufferBinding {
    pub m_NameIndex: i32,
    pub m_Index: i32,
    pub m_ArraySize: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildReport {
    pub m_Name: String,
    pub m_Summary: BuildSummary,
    pub m_Files: Vec<BuildReportFile>,
    pub m_BuildSteps: Vec<BuildStepInfo>,
    pub m_Appendices: Vec<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildReportFile {
    pub path: String,
    pub role: String,
    pub id: u32,
    pub totalSize: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildReportPackedAssetInfo {
    pub fileID: i64,
    pub classID: i32,
    pub packedSize: (Option<u64>, Option<i32>),
    pub sourceAssetGUID: GUID,
    pub buildTimeAssetPath: Option<String>,
    pub offset: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildReportScenesUsingAsset {
    pub assetPath: String,
    pub scenePaths: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildSettings {
    pub levels: Option<Vec<String>>,
    pub hasRenderTexture: Option<bool>,
    pub hasPROVersion: bool,
    pub hasPublishingRights: bool,
    pub hasShadows: bool,
    pub hasAdvancedVersion: bool,
    pub enableDynamicBatching: bool,
    pub isDebugBuild: bool,
    pub m_Version: String,
    pub m_AuthToken: String,
    pub isEducationalBuild: Option<bool>,
    pub isNoWatermarkBuild: Option<bool>,
    pub isPrototypingBuild: Option<bool>,
    pub runtimeClassHashes: Option<(Option<Vec<(i32, Hash128)>>, Option<Vec<(i32, u32)>>)>,
    pub isEmbedded: Option<bool>,
    pub usesOnMouseEvents: Option<bool>,
    pub hasSoftShadows: Option<bool>,
    pub hasLocalLightShadows: Option<bool>,
    pub hasOculusPlugin: Option<bool>,
    pub preloadedPlugins: Option<Vec<String>>,
    pub enableMultipleDisplays: Option<bool>,
    pub hasClusterRendering: Option<bool>,
    pub scriptHashes: Option<Vec<(Hash128, Hash128)>>,
    pub scenes: Option<Vec<String>>,
    pub m_GraphicsAPIs: Option<Vec<i32>>,
    pub enabledVRDevices: Option<Vec<String>>,
    pub buildTags: Option<Vec<String>>,
    pub buildGUID: Option<(Option<String>, Option<GUID>)>,
    pub isWsaHolographicRemotingEnabled: Option<bool>,
    pub isTrial: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildStepInfo {
    pub stepName: String,
    pub duration: Option<u64>,
    pub messages: Vec<BuildStepMessage>,
    pub durationTicks: Option<u64>,
    pub depth: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildStepMessage {
    pub __type: i32,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildSummary {
    pub name: Option<String>,
    pub platformName: String,
    pub options: i32,
    pub assetBundleOptions: i32,
    pub outputPath: String,
    pub crc: u32,
    pub totalSize: u64,
    pub totalTimeMS: Option<u64>,
    pub totalErrors: i32,
    pub totalWarnings: i32,
    pub buildType: Option<i32>,
    pub success: Option<bool>,
    pub platformGroupName: Option<String>,
    pub buildGUID: Option<GUID>,
    pub buildResult: Option<i32>,
    pub buildStartTime: Option<DateTime>,
    pub totalTimeTicks: Option<u64>,
    pub subtarget: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildTargetSettings {
    pub m_BuildTarget: String,
    pub m_MaxTextureSize: Option<i32>,
    pub m_TextureFormat: i32,
    pub m_CompressionQuality: Option<i32>,
    pub m_AllowsAlphaSplitting: Option<bool>,
    pub m_TextureWidth: Option<i32>,
    pub m_TextureHeight: Option<i32>,
    pub m_LoadingBehavior: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildTextureStackReference {
    pub groupName: String,
    pub itemName: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuiltAssetBundleInfo {
    pub bundleName: String,
    pub bundleArchiveFile: u32,
    pub packagedFileIndices: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuiltAssetBundleInfoSet {
    pub bundleInfos: Option<Vec<BuiltAssetBundleInfo>>,
    pub m_Name: Option<String>,
    pub m_UseLegacyImporter: Option<bool>,
    pub m_Quality: Option<f32>,
    pub m_IsColorLinear: Option<bool>,
    pub m_OriginalWidth: Option<i32>,
    pub m_OriginalHeight: Option<i32>,
    pub m_FrameRange: Option<i32>,
    pub m_StartFrame: Option<i32>,
    pub m_EndFrame: Option<i32>,
    pub m_FrameCount: Option<i32>,
    pub m_FrameRate: Option<f64>,
    pub m_ColorSpace: Option<i32>,
    pub m_Deinterlace: Option<i32>,
    pub m_SourceFileSize: Option<u64>,
    pub m_EncodeAlpha: Option<bool>,
    pub m_SourceHasAlpha: Option<bool>,
    pub m_FlipVertical: Option<bool>,
    pub m_FlipHorizontal: Option<bool>,
    pub m_AudioImportMode: Option<i32>,
    pub m_SourceAudioChannelCount: Option<Vec<u16>>,
    pub m_SourceAudioSampleRate: Option<Vec<u32>>,
    pub m_TargetSettings: Option<Vec<(i32, VideoClipImporterTargetSettings)>>,
    pub m_Output: Option<VideoClipImporterOutput>,
    pub m_UserData: Option<String>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_ImportAudio: Option<bool>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_PixelAspectRatioNumerator: Option<u32>,
    pub m_PixelAspectRatioDenominator: Option<u32>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuiltinShaderSettings {
    pub m_Mode: i32,
    pub m_Shader: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuoyancyEffector2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_UseColliderMask: bool,
    pub m_ColliderMask: BitField,
    pub m_SurfaceLevel: f32,
    pub m_Density: f32,
    pub m_LinearDrag: f32,
    pub m_AngularDrag: f32,
    pub m_FlowAngle: f32,
    pub m_FlowMagnitude: f32,
    pub m_FlowVariation: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CGProgram {
    pub m_Name: String,
    pub m_Script: Option<String>,
    pub m_PathName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachedAssetMetaData {
    pub guid: GUID,
    pub pathName: String,
    pub originalChangeset: u32,
    pub originalName: String,
    pub originalParent: GUID,
    pub originalDigest: (Option<MdFour>, Option<Hash128>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachedSpriteAtlas {
    pub textures: Vec<PPtr>,
    pub frames: (
        Option<Vec<((GUID, i64), SpriteRenderData)>>,
        Option<Vec<((GUID, i32), SpriteRenderData)>>,
    ),
    pub alphaTextures: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachedSpriteAtlasRuntimeData {
    pub textures: Vec<PPtr>,
    pub alphaTextures: Vec<PPtr>,
    pub frames: Vec<((GUID, i64), SpriteAtlasData)>,
    pub currentPackingHash: Option<Hash128>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Camera {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_ClearFlags: u32,
    pub m_BackGroundColor: ColorRGBA,
    pub m_NormalizedViewPortRect: Rectf,
    pub near_clip_plane: f32,
    pub far_clip_plane: f32,
    pub field_of_view: f32,
    pub orthographic: bool,
    pub orthographic_size: f32,
    pub m_Depth: f32,
    pub m_CullingMask: BitField,
    pub m_RenderingPath: i32,
    pub m_TargetTexture: PPtr,
    pub m_HDR: Option<bool>,
    pub m_OcclusionCulling: Option<bool>,
    pub m_TargetDisplay: Option<(Option<u32>, Option<i32>)>,
    pub m_StereoConvergence: Option<f32>,
    pub m_StereoSeparation: Option<f32>,
    pub m_StereoMirrorMode: Option<bool>,
    pub m_TargetEye: Option<i32>,
    pub m_AllowMSAA: Option<bool>,
    pub m_ForceIntoRT: Option<bool>,
    pub m_AllowDynamicResolution: Option<bool>,
    pub m_projectionMatrixMode: Option<i32>,
    pub m_SensorSize: Option<Vector2f>,
    pub m_LensShift: Option<Vector2f>,
    pub m_FocalLength: Option<f32>,
    pub m_GateFitMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Canvas {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Alpha: Option<f32>,
    pub m_RenderMode: i32,
    pub m_Camera: PPtr,
    pub m_Normals: Option<bool>,
    pub m_PositionUVs: Option<bool>,
    pub m_PixelPerfect: bool,
    pub m_PlaneDistance: Option<f32>,
    pub m_ReceivesEvents: Option<bool>,
    pub m_OverrideSorting: Option<bool>,
    pub m_OverridePixelPerfect: Option<bool>,
    pub m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    pub m_SortingOrder: Option<i16>,
    pub m_TargetDisplay: Option<i8>,
    pub m_SortingBucketNormalizedSize: Option<f32>,
    pub m_AdditionalShaderChannelsFlag: Option<i32>,
    pub m_UpdateRectTransformForStandalone: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CanvasGroup {
    pub m_GameObject: PPtr,
    pub m_Alpha: f32,
    pub m_Interactable: bool,
    pub m_BlocksRaycasts: bool,
    pub m_IgnoreParentGroups: bool,
    pub m_Enabled: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapsuleCollider {
    pub m_GameObject: PPtr,
    pub m_Material: PPtr,
    pub m_IsTrigger: bool,
    pub m_Radius: f32,
    pub m_Height: f32,
    pub m_Direction: i32,
    pub m_Center: Vector3f,
    pub m_Enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapsuleCollider2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Density: f32,
    pub m_Material: PPtr,
    pub m_IsTrigger: bool,
    pub m_UsedByEffector: bool,
    pub m_Offset: Vector2f,
    pub m_Size: Vector2f,
    pub m_Direction: i32,
    pub m_UsedByComposite: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    pub byteOffset: i32,
    pub curve: AnimationCurve,
    pub attributeName: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelInfo {
    pub stream: u8,
    pub offset: u8,
    pub format: u8,
    pub dimension: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterController {
    pub m_GameObject: PPtr,
    pub m_Height: f32,
    pub m_Radius: f32,
    pub m_SlopeLimit: f32,
    pub m_StepOffset: f32,
    pub m_SkinWidth: f32,
    pub m_MinMoveDistance: f32,
    pub m_Center: Vector3f,
    pub m_Material: Option<PPtr>,
    pub m_IsTrigger: Option<bool>,
    pub m_Enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterInfo {
    pub index: u32,
    pub uv: Rectf,
    pub vert: Rectf,
    pub width: Option<f32>,
    pub flipped: Option<bool>,
    pub advance: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterJoint {
    pub m_GameObject: PPtr,
    pub m_ConnectedBody: PPtr,
    pub m_Anchor: Vector3f,
    pub m_Axis: Vector3f,
    pub m_SwingAxis: Vector3f,
    pub m_LowTwistLimit: SoftJointLimit,
    pub m_HighTwistLimit: SoftJointLimit,
    pub m_Swing1Limit: SoftJointLimit,
    pub m_Swing2Limit: SoftJointLimit,
    pub m_BreakForce: f32,
    pub m_BreakTorque: f32,
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    pub m_ConnectedAnchor: Option<Vector3f>,
    pub m_EnableCollision: Option<bool>,
    pub m_TwistLimitSpring: Option<SoftJointLimitSpring>,
    pub m_SwingLimitSpring: Option<SoftJointLimitSpring>,
    pub m_EnableProjection: Option<bool>,
    pub m_ProjectionDistance: Option<f32>,
    pub m_ProjectionAngle: Option<f32>,
    pub m_EnablePreprocessing: Option<bool>,
    pub m_MassScale: Option<f32>,
    pub m_ConnectedMassScale: Option<f32>,
    pub m_Enabled: Option<bool>,
    pub m_ConnectedArticulationBody: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Child {
    pub m_Motion: PPtr,
    pub m_Threshold: f32,
    pub m_TimeScale: f32,
    pub m_IsAnim: bool,
    pub m_Position: Option<Vector2f>,
    pub m_CycleOffset: Option<f32>,
    pub m_Mirror: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChildAnimatorState {
    pub m_State: PPtr,
    pub m_Position: Vector3f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChildAnimatorStateMachine {
    pub m_StateMachine: PPtr,
    pub m_Position: Vector3f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChildMotion {
    pub m_Motion: PPtr,
    pub m_Threshold: f32,
    pub m_Position: Vector2f,
    pub m_TimeScale: f32,
    pub m_CycleOffset: f32,
    pub m_DirectBlendParameter: String,
    pub m_Mirror: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CircleCollider2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Material: PPtr,
    pub m_IsTrigger: bool,
    pub m_Radius: f32,
    pub m_Center: Option<Vector2f>,
    pub m_UsedByEffector: Option<bool>,
    pub m_Offset: Option<Vector2f>,
    pub m_Density: Option<f32>,
    pub m_UsedByComposite: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClampVelocityModule {
    pub enabled: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    pub magnitude: MinMaxCurve,
    pub separateAxis: bool,
    pub dampen: f32,
    pub inWorldSpace: Option<bool>,
    pub multiplyDragByParticleSize: Option<bool>,
    pub multiplyDragByParticleVelocity: Option<bool>,
    pub drag: Option<MinMaxCurve>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassInfo {
    pub m_AssemblyNameIndex: i32,
    pub m_NamespaceIndex: i32,
    pub m_ClassName: String,
    pub m_NumOfMethods: i32,
    pub m_MethodIndex: i32,
    pub m_IsUnityClass: bool,
    pub m_NamespaceName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassMethodInfo {
    pub m_ClassIndex: i32,
    pub m_MethodName: String,
    pub m_OrderNumber: i32,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Clip {
//     pub m_StreamedClip: StreamedClip,
//     pub m_DenseClip: DenseClip,
//     pub m_Binding: Option<OffsetPtr>,
//     pub m_ConstantClip: Option<ConstantClip>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClipAnimationInfo {
    pub name: String,
    pub firstFrame: (Option<f32>, Option<i32>),
    pub lastFrame: (Option<f32>, Option<i32>),
    pub wrapMode: i32,
    pub __loop: bool,
    pub takeName: Option<String>,
    pub orientationOffsetY: Option<f32>,
    pub level: Option<f32>,
    pub cycleOffset: Option<f32>,
    pub loopBlend: Option<bool>,
    pub loopBlendOrientation: Option<bool>,
    pub loopBlendPositionY: Option<bool>,
    pub loopBlendPositionXZ: Option<bool>,
    pub keepOriginalOrientation: Option<bool>,
    pub keepOriginalPositionY: Option<bool>,
    pub keepOriginalPositionXZ: Option<bool>,
    pub heightFromFeet: Option<bool>,
    pub mirror: Option<bool>,
    pub keepAdditionalBonesAnimation: Option<bool>,
    pub bodyMask: Option<Vec<u32>>,
    pub curves: Option<Vec<ClipAnimationInfoCurve>>,
    pub skeletonMaskElements: Option<Vec<AvatarSkeletonMaskElement>>,
    pub transformMask: Option<Vec<TransformMaskElement>>,
    pub loopTime: Option<bool>,
    pub events: Option<Vec<AnimationEvent>>,
    pub maskType: Option<i32>,
    pub maskSource: Option<PPtr>,
    pub hasAdditiveReferencePose: Option<bool>,
    pub additiveReferencePoseFrame: Option<f32>,
    pub internalID: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClipAnimationInfoCurve {
    pub name: String,
    pub curve: AnimationCurve,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct ClipMuscleConstant {
//     pub m_DeltaPose: HumanPose,
//     pub m_StartX: xform,
//     pub m_LeftFootStartX: xform,
//     pub m_RightFootStartX: xform,
//     pub m_MotionStartX: Option<xform>,
//     pub m_MotionStopX: Option<xform>,
//     pub m_AverageSpeed: (Option<float3>, Option<float4>),
//     pub m_Clip: OffsetPtr,
//     pub m_StartTime: f32,
//     pub m_StopTime: f32,
//     pub m_OrientationOffsetY: f32,
//     pub m_Level: f32,
//     pub m_CycleOffset: f32,
//     pub m_AverageAngularSpeed: f32,
//     pub m_IndexArray: Vec<i32>,
//     pub m_AdditionalCurveIndexArray: Option<Vec<i32>>,
//     pub m_ValueArrayDelta: Vec<ValueDelta>,
//     pub m_Mirror: bool,
//     pub m_LoopBlend: bool,
//     pub m_LoopBlendOrientation: bool,
//     pub m_LoopBlendPositionY: bool,
//     pub m_LoopBlendPositionXZ: bool,
//     pub m_KeepOriginalOrientation: bool,
//     pub m_KeepOriginalPositionY: bool,
//     pub m_KeepOriginalPositionXZ: bool,
//     pub m_HeightFromFeet: bool,
//     pub m_LoopTime: Option<bool>,
//     pub m_ValueArrayReferencePose: Option<Vec<f32>>,
//     pub m_StopX: Option<xform>,
//     pub m_StartAtOrigin: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cloth {
    pub m_GameObject: PPtr,
    pub m_Enabled: Option<u8>,
    pub m_StretchingStiffness: Option<f32>,
    pub m_BendingStiffness: Option<f32>,
    pub m_UseTethers: Option<bool>,
    pub m_UseGravity: Option<bool>,
    pub m_Damping: Option<f32>,
    pub m_ExternalAcceleration: Option<Vector3f>,
    pub m_RandomAcceleration: Option<Vector3f>,
    pub m_WorldVelocityScale: Option<f32>,
    pub m_WorldAccelerationScale: Option<f32>,
    pub m_Friction: Option<f32>,
    pub m_CollisionMassScale: Option<f32>,
    pub m_UseContinuousCollision: Option<bool>,
    pub m_UseVirtualParticles: Option<bool>,
    pub m_SolverFrequency: Option<(Option<f32>, Option<u32>)>,
    pub m_SleepThreshold: Option<f32>,
    pub m_Coefficients: Option<Vec<ClothConstrainCoefficients>>,
    pub m_CapsuleColliders: Option<Vec<PPtr>>,
    pub m_SphereColliders: Option<(
        Option<Vec<ClothSphereColliderPair>>,
        Option<Vec<(PPtr, PPtr)>>,
    )>,
    pub m_SelfCollisionDistance: Option<f32>,
    pub m_SelfCollisionStiffness: Option<f32>,
    pub m_SelfAndInterCollisionIndices: Option<Vec<u32>>,
    pub m_VirtualParticleWeights: Option<Vec<Vector3f>>,
    pub m_VirtualParticleIndices: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClothAttachment {
    pub m_Collider: PPtr,
    pub m_TwoWayInteraction: bool,
    pub m_Tearable: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClothConstrainCoefficients {
    pub maxDistance: f32,
    pub maxDistanceBias: Option<f32>,
    pub collisionSphereRadius: Option<f32>,
    pub collisionSphereDistance: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClothRenderer {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: bool,
    pub m_ReceiveShadows: bool,
    pub m_LightmapIndex: u8,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_Materials: Vec<PPtr>,
    pub m_SubsetIndices: Vec<u32>,
    pub m_StaticBatchRoot: PPtr,
    pub m_PauseWhenNotVisible: bool,
    pub m_UseLightProbes: Option<bool>,
    pub m_LightProbeAnchor: Option<PPtr>,
    pub m_SortingLayer: Option<i16>,
    pub m_SortingOrder: Option<i16>,
    pub m_SortingLayerID: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClothSphereColliderPair {
    pub first: PPtr,
    pub second: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudServiceHandlerBehaviour {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudWebServicesManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClusterInput {
    pub m_Name: String,
    pub m_DeviceName: String,
    pub m_ServerUrl: String,
    pub m_Index: i32,
    pub m_Type: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClusterInputManager {
    pub m_Inputs: Vec<ClusterInput>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollabEditorSettings {
    pub inProgressEnabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collider {
    pub m_GameObject: Option<PPtr>,
    pub m_X: Option<xform>,
    pub m_Type: Option<u32>,
    pub m_XMotionType: Option<u32>,
    pub m_YMotionType: Option<u32>,
    pub m_ZMotionType: Option<u32>,
    pub m_MinLimitX: Option<f32>,
    pub m_MaxLimitX: Option<f32>,
    pub m_MaxLimitY: Option<f32>,
    pub m_MaxLimitZ: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collider2D {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collision {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collision2D {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollisionModule {
    pub enabled: bool,
    pub __type: i32,
    pub plane0: Option<PPtr>,
    pub plane1: Option<PPtr>,
    pub plane2: Option<PPtr>,
    pub plane3: Option<PPtr>,
    pub plane4: Option<PPtr>,
    pub plane5: Option<PPtr>,
    pub dampen: Option<f32>,
    pub bounce: Option<f32>,
    pub energyLossOnCollision: Option<f32>,
    pub minKillSpeed: f32,
    pub particleRadius: Option<f32>,
    pub collidesWith: Option<BitField>,
    pub quality: Option<i32>,
    pub voxelSize: Option<f32>,
    pub collisionMessages: Option<bool>,
    pub collisionMode: Option<i32>,
    pub m_Dampen: Option<MinMaxCurve>,
    pub m_Bounce: Option<MinMaxCurve>,
    pub m_EnergyLossOnCollision: Option<MinMaxCurve>,
    pub radiusScale: Option<f32>,
    pub maxCollisionShapes: Option<i32>,
    pub collidesWithDynamic: Option<bool>,
    pub interiorCollisions: Option<bool>,
    pub maxKillSpeed: Option<f32>,
    pub colliderForce: Option<f32>,
    pub multiplyColliderForceByParticleSize: Option<bool>,
    pub multiplyColliderForceByParticleSpeed: Option<bool>,
    pub multiplyColliderForceByCollisionAngle: Option<bool>,
    pub m_Planes: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorBySpeedModule {
    pub enabled: bool,
    pub gradient: MinMaxGradient,
    pub range: Vector2f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorModule {
    pub enabled: bool,
    pub gradient: MinMaxGradient,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorRGBA {
    pub rgba: Option<u32>,
    pub r: Option<f32>,
    pub g: Option<f32>,
    pub b: Option<f32>,
    pub a: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Component {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComponentPair {
    pub component: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompositeCollider2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Density: f32,
    pub m_Material: PPtr,
    pub m_IsTrigger: bool,
    pub m_UsedByEffector: bool,
    pub m_UsedByComposite: bool,
    pub m_Offset: Vector2f,
    pub m_GeometryType: i32,
    pub m_GenerationType: i32,
    pub m_ColliderPaths: Vec<SubCollider>,
    pub m_CompositePaths: Polygon2D,
    pub m_VertexDistance: f32,
    pub m_EdgeRadius: Option<f32>,
    pub m_OffsetDistance: Option<f32>,
    pub m_UseDelaunayMesh: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompressedAnimationCurve {
    pub m_Path: String,
    pub m_Times: PackedBitVector,
    pub m_Values: PackedBitVector,
    pub m_Slopes: PackedBitVector,
    pub m_PreInfinity: i32,
    pub m_PostInfinity: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompressedMesh {
    pub m_Vertices: PackedBitVector,
    pub m_UV: PackedBitVector,
    pub m_BindPoses: Option<PackedBitVector>,
    pub m_Normals: PackedBitVector,
    pub m_Tangents: PackedBitVector,
    pub m_Weights: PackedBitVector,
    pub m_NormalSigns: PackedBitVector,
    pub m_TangentSigns: PackedBitVector,
    pub m_BoneIndices: PackedBitVector,
    pub m_Triangles: PackedBitVector,
    pub m_Colors: Option<PackedBitVector>,
    pub m_FloatColors: Option<PackedBitVector>,
    pub m_UVInfo: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeBufferCounter {
    pub bindpoint: i32,
    pub offset: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShader {
    pub m_Name: String,
    pub kernels: Option<Vec<ComputeShaderKernel>>,
    pub constantBuffers: Option<Vec<ComputeShaderCB>>,
    pub variants: Option<(
        Option<Vec<ComputeShaderPlatformVariant>>,
        Option<Vec<ComputeShaderVariant>>,
    )>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderBuiltinSampler {
    pub sampler: (Option<u32>, Option<i32>),
    pub bindPoint: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderCB {
    pub name: (Option<String>, Option<FastPropertyName>),
    pub byteSize: i32,
    pub params: Vec<ComputeShaderParam>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderImporter {
    pub m_Name: String,
    pub m_UserData: String,
    pub m_CurrentBuildTarget: Option<i32>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_CurrentAPIMask: Option<u32>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
    pub m_PreprocessorOverride: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderKernel {
    pub name: Option<(Option<String>, Option<FastPropertyName>)>,
    pub cbs: Vec<ComputeShaderResource>,
    pub textures: Vec<ComputeShaderResource>,
    pub builtinSamplers: Vec<ComputeShaderBuiltinSampler>,
    pub inBuffers: Vec<ComputeShaderResource>,
    pub outBuffers: Vec<ComputeShaderResource>,
    pub code: Vec<u8>,
    pub threadGroupSize: Option<Vec<u32>>,
    pub cbVariantIndices: Option<Vec<u32>>,
    pub requirements: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderKernelParent {
    pub name: String,
    pub variantMap: Option<Vec<(String, ComputeShaderKernel)>>,
    pub validKeywords: Option<Vec<String>>,
    pub globalKeywords: Option<Vec<String>>,
    pub localKeywords: Option<Vec<String>>,
    pub uniqueVariants: Option<Vec<ComputeShaderKernel>>,
    pub variantIndices: Option<Vec<(String, u32)>>,
    pub dynamicKeywords: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderParam {
    pub name: (Option<String>, Option<FastPropertyName>),
    pub __type: i32,
    pub offset: (Option<u32>, Option<i32>),
    pub arraySize: (Option<u32>, Option<i32>),
    pub rowCount: (Option<u32>, Option<i32>),
    pub colCount: (Option<u32>, Option<i32>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderPlatformVariant {
    pub targetRenderer: i32,
    pub targetLevel: i32,
    pub kernels: Vec<ComputeShaderKernelParent>,
    pub constantBuffers: Vec<ComputeShaderCB>,
    pub resourcesResolved: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderResource {
    pub name: (Option<String>, Option<FastPropertyName>),
    pub bindPoint: i32,
    pub generatedName: Option<(Option<String>, Option<FastPropertyName>)>,
    pub secondaryBindPoint: Option<i32>,
    pub counter: Option<ComputeBufferCounter>,
    pub samplerBindPoint: Option<i32>,
    pub texDimension: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeShaderVariant {
    pub targetRenderer: i32,
    pub targetLevel: i32,
    pub kernels: Vec<ComputeShaderKernel>,
    pub constantBuffers: Vec<ComputeShaderCB>,
    pub resourcesResolved: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Condition {
    pub m_ConditionMode: i32,
    pub m_ConditionEvent: String,
    pub m_EventTreshold: f32,
    pub m_ExitTime: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConditionConstant {
    pub m_ConditionMode: u32,
    pub m_EventID: u32,
    pub m_EventThreshold: f32,
    pub m_ExitTime: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigSetting {
    pub value: String,
    pub flags: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigurableJoint {
    pub m_GameObject: PPtr,
    pub m_ConnectedBody: PPtr,
    pub m_Anchor: Vector3f,
    pub m_Axis: Vector3f,
    pub m_SecondaryAxis: Vector3f,
    pub m_XMotion: i32,
    pub m_YMotion: i32,
    pub m_ZMotion: i32,
    pub m_AngularXMotion: i32,
    pub m_AngularYMotion: i32,
    pub m_AngularZMotion: i32,
    pub m_LinearLimit: SoftJointLimit,
    pub m_LowAngularXLimit: SoftJointLimit,
    pub m_HighAngularXLimit: SoftJointLimit,
    pub m_AngularYLimit: SoftJointLimit,
    pub m_AngularZLimit: SoftJointLimit,
    pub m_TargetPosition: Vector3f,
    pub m_TargetVelocity: Vector3f,
    pub m_XDrive: JointDrive,
    pub m_YDrive: JointDrive,
    pub m_ZDrive: JointDrive,
    pub m_TargetRotation: Quaternionf,
    pub m_TargetAngularVelocity: Vector3f,
    pub m_RotationDriveMode: i32,
    pub m_AngularXDrive: JointDrive,
    pub m_AngularYZDrive: JointDrive,
    pub m_SlerpDrive: JointDrive,
    pub m_ProjectionMode: i32,
    pub m_ProjectionDistance: f32,
    pub m_ProjectionAngle: f32,
    pub m_ConfiguredInWorldSpace: bool,
    pub m_BreakForce: f32,
    pub m_BreakTorque: f32,
    pub m_SwapBodies: Option<bool>,
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    pub m_ConnectedAnchor: Option<Vector3f>,
    pub m_EnableCollision: Option<bool>,
    pub m_LinearLimitSpring: Option<SoftJointLimitSpring>,
    pub m_AngularXLimitSpring: Option<SoftJointLimitSpring>,
    pub m_AngularYZLimitSpring: Option<SoftJointLimitSpring>,
    pub m_EnablePreprocessing: Option<bool>,
    pub m_MassScale: Option<f32>,
    pub m_ConnectedMassScale: Option<f32>,
    pub m_Enabled: Option<bool>,
    pub m_ConnectedArticulationBody: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstantBuffer {
    pub m_NameIndex: i32,
    pub m_MatrixParams: Vec<MatrixParameter>,
    pub m_VectorParams: Vec<VectorParameter>,
    pub m_Size: i32,
    pub m_StructParams: Option<Vec<StructParameter>>,
    pub m_IsPartialCB: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstantClip {
    pub data: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstantForce {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Force: Vector3f,
    pub m_RelativeForce: Vector3f,
    pub m_Torque: Vector3f,
    pub m_RelativeTorque: Vector3f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstantForce2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Force: Vector2f,
    pub m_RelativeForce: Vector2f,
    pub m_Torque: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstraintSource {
    pub sourceTransform: PPtr,
    pub weight: f32,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct ControllerConstant {
//     pub m_HumanLayerArray: Option<Vec<OffsetPtr>>,
//     pub m_StateMachineArray: Vec<OffsetPtr>,
//     pub m_Values: OffsetPtr,
//     pub m_DefaultValues: OffsetPtr,
//     pub m_LayerArray: Option<Vec<OffsetPtr>>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrashReportManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrashReportingSettings {
    pub m_EventUrl: String,
    pub m_Enabled: bool,
    pub m_NativeEventUrl: Option<String>,
    pub m_LogBufferSize: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cubemap {
    pub m_Name: String,
    pub m_Width: i32,
    pub m_Height: i32,
    pub m_CompleteImageSize: (Option<u32>, Option<i32>),
    pub m_TextureFormat: i32,
    pub m_MipMap: Option<bool>,
    pub m_IsReadable: bool,
    pub m_ReadAllowed: Option<bool>,
    pub m_ImageCount: i32,
    pub m_TextureDimension: i32,
    pub m_TextureSettings: GLTextureSettings,
    pub m_LightmapFormat: i32,
    pub image_data: Vec<u8>,
    pub m_ColorSpace: Option<i32>,
    pub m_SourceTextures: Option<Vec<PPtr>>,
    pub m_MipCount: Option<i32>,
    pub m_StreamData: Option<StreamingInfo>,
    pub m_ForcedFallbackFormat: Option<i32>,
    pub m_DownscaleFallback: Option<bool>,
    pub m_StreamingMipmaps: Option<bool>,
    pub m_StreamingMipmapsPriority: Option<i32>,
    pub m_IgnoreMasterTextureLimit: Option<bool>,
    pub m_IsPreProcessed: Option<bool>,
    pub m_MipsStripped: Option<i32>,
    pub m_IsAlphaChannelOptional: Option<bool>,
    pub m_PlatformBlob: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CubemapArray {
    pub m_Name: String,
    pub m_Width: i32,
    pub m_CubemapCount: i32,
    pub m_Format: i32,
    pub m_MipCount: i32,
    pub m_DataSize: u32,
    pub m_TextureSettings: GLTextureSettings,
    pub m_ColorSpace: i32,
    pub m_IsReadable: bool,
    pub image_data: Vec<u8>,
    pub m_StreamData: Option<StreamingInfo>,
    pub m_ForcedFallbackFormat: Option<i32>,
    pub m_DownscaleFallback: Option<bool>,
    pub m_IsAlphaChannelOptional: Option<bool>,
    pub m_UsageMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomCollider2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Density: f32,
    pub m_Material: PPtr,
    pub m_IsTrigger: bool,
    pub m_UsedByEffector: bool,
    pub m_UsedByComposite: bool,
    pub m_Offset: Vector2f,
    pub m_CustomShapes: PhysicsShapeGroup2D,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomDataModule {
    pub enabled: bool,
    pub mode0: i32,
    pub vectorComponentCount0: i32,
    pub color0: MinMaxGradient,
    pub vector0_0: MinMaxCurve,
    pub vector0_1: MinMaxCurve,
    pub vector0_2: MinMaxCurve,
    pub vector0_3: MinMaxCurve,
    pub mode1: i32,
    pub vectorComponentCount1: i32,
    pub color1: MinMaxGradient,
    pub vector1_0: MinMaxCurve,
    pub vector1_1: MinMaxCurve,
    pub vector1_2: MinMaxCurve,
    pub vector1_3: MinMaxCurve,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomRenderTexture {
    pub m_Name: String,
    pub m_Width: i32,
    pub m_Height: i32,
    pub m_AntiAliasing: i32,
    pub m_DepthFormat: Option<i32>,
    pub m_ColorFormat: i32,
    pub m_MipMap: bool,
    pub m_GenerateMips: bool,
    pub m_SRGB: bool,
    pub m_TextureSettings: GLTextureSettings,
    pub m_Dimension: i32,
    pub m_VolumeDepth: i32,
    pub m_Material: PPtr,
    pub m_InitMaterial: PPtr,
    pub m_InitColor: ColorRGBA,
    pub m_InitTexture: PPtr,
    pub m_UpdateMode: i32,
    pub m_InitializationMode: i32,
    pub m_UpdateZoneSpace: i32,
    pub m_CurrentUpdateZoneSpace: i32,
    pub m_UpdateZones: Vec<UpdateZoneInfo>,
    pub m_UpdatePeriod: f32,
    pub m_ShaderPass: u32,
    pub m_CubemapFaceMask: u32,
    pub m_DoubleBuffered: bool,
    pub m_WrapUpdateZones: bool,
    pub m_InitSource: Option<i32>,
    pub m_ForcedFallbackFormat: Option<i32>,
    pub m_DownscaleFallback: Option<bool>,
    pub m_UseDynamicScale: Option<bool>,
    pub m_BindMS: Option<bool>,
    pub m_EnableCompatibleFormat: Option<bool>,
    pub m_MipCount: Option<i32>,
    pub m_IsAlphaChannelOptional: Option<bool>,
    pub m_DepthStencilFormat: Option<i32>,
    pub m_ShadowSamplingMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DDSImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_UserData: Option<String>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_IsReadable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataTemplate {
    pub m_LastMergeIdentifier: Option<GUID>,
    pub m_Objects: Option<Vec<PPtr>>,
    pub m_Father: Option<PPtr>,
    pub m_IsDataTemplate: Option<bool>,
    pub m_ObjectHideFlags: Option<u32>,
    pub m_ExtensionPtr: Option<PPtr>,
    pub m_Name: Option<String>,
    pub m_Modification: Option<PrefabModification>,
    pub m_ParentPrefab: Option<PPtr>,
    pub m_RootGameObject: Option<PPtr>,
    pub m_IsPrefabParent: Option<bool>,
    pub m_IsExploded: Option<bool>,
    pub m_SourcePrefab: Option<PPtr>,
    pub m_IsPrefabAsset: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateTime {
    pub ticks: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultAsset {
    pub m_ObjectHideFlags: Option<u32>,
    pub m_ExtensionPtr: Option<PPtr>,
    pub m_Name: String,
    pub m_Message: Option<String>,
    pub m_IsWarning: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_UserData: Option<String>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultPreset {
    pub m_Preset: PPtr,
    pub m_Filter: Option<String>,
    pub m_Disabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultPresetList {
    pub __type: PresetType,
    pub defaultPresets: Vec<DefaultPreset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DelayedCallManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedItem {
    pub changeset: i32,
    pub guid: GUID,
    pub parent: GUID,
    pub fullPath: String,
    pub __type: i32,
    pub digest: (Option<MdFour>, Option<Hash128>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DenseClip {
    pub m_FrameCount: i32,
    pub m_CurveCount: u32,
    pub m_SampleRate: f32,
    pub m_BeginTime: f32,
    pub m_SampleArray: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Derived {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailDatabase {
    pub m_Patches: Vec<DetailPatch>,
    pub m_DetailPrototypes: Vec<DetailPrototype>,
    pub m_PatchCount: i32,
    pub m_PatchSamples: i32,
    pub m_RandomRotations: Option<Vec<Vector3f>>,
    pub WavingGrassTint: ColorRGBA,
    pub m_WavingGrassStrength: f32,
    pub m_WavingGrassAmount: f32,
    pub m_WavingGrassSpeed: f32,
    pub m_TreeInstances: Vec<TreeInstance>,
    pub m_TreePrototypes: Vec<TreePrototype>,
    pub m_PreloadTextureAtlasData: Vec<PPtr>,
    pub m_DetailBillboardShader: Option<PPtr>,
    pub m_DetailMeshLitShader: Option<PPtr>,
    pub m_DetailMeshGrassShader: Option<PPtr>,
    pub m_DefaultShaders: Option<(PPtr, PPtr, PPtr)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailPatch {
    pub bounds: Option<AABB>,
    pub layerIndices: Vec<u8>,
    pub numberOfObjects: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailPrototype {
    pub prototype: PPtr,
    pub prototypeTexture: PPtr,
    pub minWidth: f32,
    pub maxWidth: f32,
    pub minHeight: f32,
    pub maxHeight: f32,
    pub noiseSpread: f32,
    pub bendFactor: Option<f32>,
    pub healthyColor: ColorRGBA,
    pub dryColor: ColorRGBA,
    pub lightmapFactor: Option<f32>,
    pub renderMode: i32,
    pub usePrototypeMesh: i32,
    pub holeTestRadius: Option<f32>,
    pub noiseSeed: Option<i32>,
    pub useInstancing: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceNone {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DirectorGenericBinding {
    pub key: PPtr,
    pub value: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DirectorPlayer {
    pub m_GameObject: PPtr,
    pub m_Enabled: Option<u8>,
    pub m_PlayableAsset: Option<PPtr>,
    pub m_InitialState: Option<i32>,
    pub m_WrapMode: Option<i32>,
    pub m_DirectorUpdateMode: Option<i32>,
    pub m_InitialTime: Option<f64>,
    pub m_SceneBindings: Option<Vec<DirectorGenericBinding>>,
    pub m_ExposedReferences: Option<ExposedReferenceTable>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DistanceJoint2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_CollideConnected: Option<bool>,
    pub m_ConnectedRigidBody: PPtr,
    pub m_Anchor: Vector2f,
    pub m_ConnectedAnchor: Vector2f,
    pub m_Distance: f32,
    pub m_MaxDistanceOnly: Option<bool>,
    pub m_EnableCollision: Option<bool>,
    pub m_BreakForce: Option<f32>,
    pub m_BreakTorque: Option<f32>,
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    pub m_AutoConfigureDistance: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EdgeCollider2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Material: PPtr,
    pub m_IsTrigger: bool,
    pub m_Points: Vec<Vector2f>,
    pub m_UsedByEffector: Option<bool>,
    pub m_Offset: Option<Vector2f>,
    pub m_Density: Option<f32>,
    pub m_UsedByComposite: Option<bool>,
    pub m_EdgeRadius: Option<f32>,
    pub m_AdjacentStartPoint: Option<Vector2f>,
    pub m_AdjacentEndPoint: Option<Vector2f>,
    pub m_UseAdjacentStartPoint: Option<bool>,
    pub m_UseAdjacentEndPoint: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorBuildSettings {
    pub m_ObjectHideFlags: Option<u32>,
    pub m_Scenes: Vec<Scene>,
    pub m_BuildLocation: Option<Vec<String>>,
    pub m_ActiveBuildTarget: Option<i32>,
    pub m_Development: Option<bool>,
    pub m_configObjects: Option<Vec<(String, PPtr)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorExtension {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorExtensionImpl {
    pub m_LastTemplateIdentifier: Option<GUID>,
    pub m_Object: Option<PPtr>,
    pub m_TemplateFather: Option<PPtr>,
    pub m_DataTemplate: Option<PPtr>,
    pub m_LastTemplateFather: Option<PPtr>,
    pub m_OverrideVariable: Option<bitset>,
    pub gFlattenedTypeTree: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorProjectAccess {
    pub m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorSettings {
    pub m_ObjectHideFlags: Option<u32>,
    pub m_ExternalVersionControlSupport: Option<(Option<String>, Option<i32>)>,
    pub m_WebSecurityEmulationEnabled: Option<i32>,
    pub m_WebSecurityEmulationHostUrl: Option<String>,
    pub m_SerializationMode: Option<i32>,
    pub m_DefaultBehaviorMode: Option<i32>,
    pub m_SpritePackerMode: Option<i32>,
    pub m_SpritePackerPaddingPower: Option<i32>,
    pub m_ProjectGenerationIncludedExtensions: Option<String>,
    pub m_ProjectGenerationRootNamespace: Option<String>,
    pub m_UserGeneratedProjectSuffix: Option<String>,
    pub m_CollabEditorSettings: Option<CollabEditorSettings>,
    pub m_EtcTextureCompressorBehavior: Option<i32>,
    pub m_EtcTextureFastCompressor: Option<i32>,
    pub m_EtcTextureNormalCompressor: Option<i32>,
    pub m_EtcTextureBestCompressor: Option<i32>,
    pub m_LineEndingsForNewScripts: Option<i32>,
    pub m_EnableTextureStreamingInPlayMode: Option<bool>,
    pub m_PrefabRegularEnvironment: Option<PPtr>,
    pub m_PrefabUIEnvironment: Option<PPtr>,
    pub m_EnableTextureStreamingInEditMode: Option<bool>,
    pub m_AsyncShaderCompilation: Option<bool>,
    pub m_ShowLightmapResolutionOverlay: Option<bool>,
    pub m_EnterPlayModeOptionsEnabled: Option<bool>,
    pub m_EnterPlayModeOptions: Option<i32>,
    pub m_UseLegacyProbeSampleCount: Option<bool>,
    pub m_AssetPipelineMode: Option<i32>,
    pub m_CacheServerMode: Option<i32>,
    pub m_CacheServerEndpoint: Option<String>,
    pub m_CacheServerNamespacePrefix: Option<String>,
    pub m_CacheServerEnableDownload: Option<bool>,
    pub m_CacheServerEnableUpload: Option<bool>,
    pub m_SerializeInlineMappingsOnOneLine: Option<bool>,
    pub m_CacheServerEnableAuth: Option<bool>,
    pub m_CacheServerEnableTls: Option<bool>,
    pub m_CachingShaderPreprocessor: Option<bool>,
    pub m_DisableCookiesInLightmapper: Option<bool>,
    pub m_GameObjectNamingDigits: Option<i32>,
    pub m_GameObjectNamingScheme: Option<i32>,
    pub m_AssetNamingUsesSpace: Option<bool>,
    pub m_EnableRoslynAnalyzers: Option<bool>,
    pub m_PrefabModeAllowAutoSave: Option<bool>,
    pub m_CacheServerValidationMode: Option<i32>,
    pub m_Bc7TextureCompressor: Option<i32>,
    pub m_RefreshImportMode: Option<i32>,
    pub m_SpritePackerCacheSize: Option<i32>,
    pub m_EnableEditorAsyncCPUTextureLoading: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorUserBuildSettings {
    pub m_BuildLocation: Vec<String>,
    pub m_ActiveBuildTarget: i32,
    pub m_SelectedBuildTargetGroup: i32,
    pub m_SelectedStandaloneTarget: i32,
    pub m_ArchitectureFlags: i32,
    pub m_SelectedWiiSubtarget: Option<i32>,
    pub m_Development: bool,
    pub m_ConnectProfiler: bool,
    pub m_AllowDebugging: bool,
    pub m_WebPlayerStreamed: Option<bool>,
    pub m_WebPlayerOfflineDeployment: Option<bool>,
    pub m_WebPlayerNaClSupport: Option<bool>,
    pub m_InstallInBuildFolder: bool,
    pub m_SymlinkLibraries: Option<bool>,
    pub m_SelectedAndroidSubtarget: i32,
    pub m_SelectedWiiDebugLevel: Option<i32>,
    pub m_SelectedXboxSubtarget: Option<i32>,
    pub m_SelectedXboxRunMethod: Option<i32>,
    pub m_ExplicitNullChecks: Option<bool>,
    pub m_XboxCompressedXex: Option<bool>,
    pub m_WebPlayerDeployOnline: Option<bool>,
    pub m_SelectedPS3Subtarget: Option<i32>,
    pub m_SelectedMetroBuildType: Option<i32>,
    pub m_EnableHeadlessMode: Option<bool>,
    pub m_SelectedMetroTarget: Option<i32>,
    pub m_SelectedBlackBerrySubtarget: Option<i32>,
    pub m_SelectedBlackBerryBuildType: Option<i32>,
    pub m_SelectedMetroSDK: Option<i32>,
    pub m_SelectedPSP2Subtarget: Option<i32>,
    pub m_GenerateMetroReferenceProjects: Option<bool>,
    pub m_SelectedTizenSubtarget: Option<i32>,
    pub m_SelectedMetroBuildAndRunDeployTarget: Option<i32>,
    pub m_SelectedPS4Subtarget: Option<i32>,
    pub m_SelectedPSMSubtarget: Option<i32>,
    pub m_NeedSubmissionMaterials: Option<bool>,
    pub m_CompressWithPsArc: Option<bool>,
    pub m_BuildScriptsOnly: Option<bool>,
    pub m_WebGLOptimizationLevel: Option<i32>,
    pub m_SelectedWSASDK: Option<i32>,
    pub m_SelectedWSABuildAndRunDeployTarget: Option<i32>,
    pub m_GenerateWSAReferenceProjects: Option<bool>,
    pub m_XboxOneStreamingInstallLaunchChunkRange: Option<i32>,
    pub m_SelectedXboxOneDeployMethod: Option<i32>,
    pub m_SymlinkTrampoline: Option<bool>,
    pub m_SelectedIOSBuildType: Option<i32>,
    pub m_SelectedWiiUDebugLevel: Option<i32>,
    pub m_SelectedWiiUBuildOutput: Option<i32>,
    pub m_SelectedWiiUBootMode: Option<i32>,
    pub m_WiiUEnableNetAPI: Option<bool>,
    pub m_SelectedWSAUWPBuildType: Option<i32>,
    pub m_ForceOptimizeScriptCompilation: Option<bool>,
    pub m_WSADotNetNativeEnabled: Option<Vec<bool>>,
    pub m_XboxOneUsername: Option<String>,
    pub m_XboxOneNetworkSharePath: Option<String>,
    pub m_PS4HardwareTarget: Option<i32>,
    pub m_WebGLUsePreBuiltUnityEngine: Option<bool>,
    pub m_ExplicitDivideByZeroChecks: Option<bool>,
    pub m_PlatformSettings: Option<Vec<(String, PlatformSettingsData)>>,
    pub m_AndroidBuildSystem: Option<i32>,
    pub m_ExportAsGoogleAndroidProject: Option<bool>,
    pub m_SelectedWSASubtarget: Option<i32>,
    pub m_CompressFilesInPackage: Option<bool>,
    pub m_SelectedWSAUWPSDK: Option<String>,
    pub m_ActiveBuildTargetGroup: Option<i32>,
    pub m_SelectedFacebookTarget: Option<i32>,
    pub m_CreateSolutionFileForSwitch: Option<bool>,
    pub m_CreateRomFileForSwitch: Option<bool>,
    pub m_NVNGraphicsDebuggerForSwitch: Option<bool>,
    pub m_FacebookCreatePackageForSubmission: Option<bool>,
    pub m_AndroidBuildType: Option<i32>,
    pub m_FacebookAccessToken: Option<String>,
    pub m_EnableDebugPadForSwitch: Option<bool>,
    pub m_RedirectWritesToHostMountForSwitch: Option<bool>,
    pub m_ForceInstallation: Option<bool>,
    pub m_AndroidReleaseMinification: Option<i32>,
    pub m_AndroidDebugMinification: Option<i32>,
    pub m_AndroidDeviceSocketAddress: Option<String>,
    pub m_SelectedCompressionType: Option<Vec<(String, i32)>>,
    pub m_SelectedAndroidETC2Fallback: Option<i32>,
    pub m_SelectedWSAUWPVSVersion: Option<String>,
    pub m_BuildAppBundle: Option<bool>,
    pub m_AndroidCurrentDeploymentTargetId: Option<String>,
    pub m_MovePackageToDiscOuterEdge: Option<bool>,
    pub m_ExplicitArrayBoundsChecks: Option<bool>,
    pub m_DatalessPlayer: Option<bool>,
    pub m_WsaHolographicRemoting: Option<bool>,
    pub m_SelectedXboxOneDeployDrive: Option<i32>,
    pub m_NVNShaderDebugging: Option<bool>,
    pub m_NVNDrawValidation: Option<bool>,
    pub m_EnableHeapInspectorForSwitch: Option<bool>,
    pub m_AndroidUseLegacySdkTools: Option<bool>,
    pub m_SelectedWSAMinUWPSDK: Option<String>,
    pub m_SelectedWSAArchitecture: Option<String>,
    pub m_AndroidCreateSymbolsZip: Option<bool>,
    pub m_GenerateNintendoSwitchShaderInfo: Option<bool>,
    pub m_WaitForPlayerConnection: Option<bool>,
    pub m_WindowsDevicePortalAddress: Option<String>,
    pub m_WindowsDevicePortalUsername: Option<String>,
    pub m_BuildWithDeepProfilingSupport: Option<bool>,
    pub m_PS5KeepPackageFiles: Option<bool>,
    pub m_SelectedPS5Subtarget: Option<i32>,
    pub m_SelectedPS5CompressionType: Option<i32>,
    pub m_SelectedPS5CompressionLevel: Option<i32>,
    pub m_PS5WorkspaceName: Option<String>,
    pub m_UseLegacyNvnPoolAllocatorForSwitch: Option<bool>,
    pub m_EnableRomCompressionForSwitch: Option<bool>,
    pub m_SaveADFForSwitch: Option<bool>,
    pub m_RomCompressionTypeForSwitch: Option<i32>,
    pub m_RomCompressionLevelForSwitch: Option<i32>,
    pub m_RomCompressionConfigForSwitch: Option<String>,
    pub m_NVNDrawValidationLight: Option<bool>,
    pub m_NVNDrawValidationHeavy: Option<bool>,
    pub m_HTCSScriptDebuggingForSwitch: Option<bool>,
    pub m_AndroidCreateSymbols: Option<i32>,
    pub m_SymlinkSources: Option<bool>,
    pub m_OverrideMaxTextureSize: Option<i32>,
    pub m_OverrideTextureCompression: Option<i32>,
    pub m_macosXcodeBuildConfig: Option<i32>,
    pub m_Il2CppCodeGeneration: Option<i32>,
    pub m_ActiveBuildPlatformGroupName: Option<String>,
    pub m_SelectedBuildPlatformGroupName: Option<String>,
    pub m_SelectedEmbeddedLinuxArchitecture: Option<i32>,
    pub m_SelectedStandaloneBuildSubtarget: Option<i32>,
    pub m_ActiveStandaloneBuildSubtarget: Option<i32>,
    pub m_SelectedWebGLSubtarget: Option<i32>,
    pub m_RemoteDeviceInfo: Option<bool>,
    pub m_RemoteDeviceAddress: Option<String>,
    pub m_RemoteDeviceUsername: Option<String>,
    pub m_RemoteDeviceExports: Option<String>,
    pub m_PathOnRemoteDevice: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorUserSettings {
    pub m_VCUserName: Option<String>,
    pub m_VCPassword: Option<String>,
    pub m_VCServer: Option<String>,
    pub m_VCWorkspace: Option<String>,
    pub m_VCAutomaticAdd: bool,
    pub m_VCDebugCom: bool,
    pub m_VCDebugCmd: bool,
    pub m_VCDebugOut: bool,
    pub m_ConfigValues: Option<Vec<(String, String)>>,
    pub m_SemanticMergeMode: Option<i32>,
    pub m_VCShowFailedCheckout: Option<bool>,
    pub m_ConfigSettings: Option<Vec<(String, ConfigSetting)>>,
    pub m_VCAllowAsyncUpdate: Option<bool>,
    pub m_VCOverwriteFailedCheckoutAssets: Option<bool>,
    pub m_AssetPipelineMode: Option<i32>,
    pub m_CacheServerMode: Option<i32>,
    pub m_CacheServers: Option<Vec<String>>,
    pub m_AssetPipelineMode2: Option<i32>,
    pub m_VCOverlayIcons: Option<bool>,
    pub m_VCProjectOverlayIcons: Option<bool>,
    pub m_VCHierarchyOverlayIcons: Option<bool>,
    pub m_VCOtherOverlayIcons: Option<bool>,
    pub m_ArtifactGarbageCollection: Option<bool>,
    pub m_DesiredImportWorkerCount: Option<i32>,
    pub m_StandbyImportWorkerCount: Option<i32>,
    pub m_IdleImportWorkerShutdownDelay: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffectConstant {
    pub __type: i32,
    pub groupConstantIndex: u32,
    pub sendTargetEffectIndex: u32,
    pub wetMixLevelIndex: u32,
    pub prevEffectIndex: u32,
    pub bypass: bool,
    pub parameterIndices: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effector2D {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EllipsoidParticleEmitter {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_Emit: bool,
    pub minSize: f32,
    pub maxSize: f32,
    pub minEnergy: f32,
    pub maxEnergy: f32,
    pub minEmission: f32,
    pub maxEmission: f32,
    pub worldVelocity: Vector3f,
    pub localVelocity: Vector3f,
    pub rndVelocity: Vector3f,
    pub emitterVelocityScale: f32,
    pub tangentVelocity: Vector3f,
    pub angularVelocity: f32,
    pub rndAngularVelocity: f32,
    pub rndRotation: bool,
    pub Simulate_in_Worldspace: bool,
    pub m_OneShot: bool,
    pub m_Ellipsoid: Vector3f,
    pub m_MinEmitterRange: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddedNativeType {
    pub m_FloatArray: Vec<f32>,
    pub m_String: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmissionModule {
    pub enabled: bool,
    pub m_Type: Option<i32>,
    pub rate: Option<MinMaxCurve>,
    pub cnt0: Option<(Option<i32>, Option<u16>)>,
    pub cnt1: Option<(Option<i32>, Option<u16>)>,
    pub cnt2: Option<(Option<i32>, Option<u16>)>,
    pub cnt3: Option<(Option<i32>, Option<u16>)>,
    pub time0: Option<f32>,
    pub time1: Option<f32>,
    pub time2: Option<f32>,
    pub time3: Option<f32>,
    pub m_BurstCount: (Option<u8>, Option<i32>),
    pub cntmax0: Option<(Option<i32>, Option<u16>)>,
    pub cntmax1: Option<(Option<i32>, Option<u16>)>,
    pub cntmax2: Option<(Option<i32>, Option<u16>)>,
    pub cntmax3: Option<(Option<i32>, Option<u16>)>,
    pub rateOverTime: Option<MinMaxCurve>,
    pub rateOverDistance: Option<MinMaxCurve>,
    pub m_Bursts: Option<Vec<ParticleSystemEmissionBurst>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmptyObject {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnlightenRendererInformation {
    pub renderer: PPtr,
    pub dynamicLightmapSTInSystem: Vector4f,
    pub systemId: i32,
    pub instanceHash: Hash128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnlightenSceneMapping {
    pub m_Renderers: Vec<EnlightenRendererInformation>,
    pub m_Systems: Vec<EnlightenSystemInformation>,
    pub m_SystemAtlases: Vec<EnlightenSystemAtlasInformation>,
    pub m_TerrainChunks: Vec<EnlightenTerrainChunksInformation>,
    pub m_Probesets: Option<Vec<Hash128>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnlightenSystemAtlasInformation {
    pub atlasSize: i32,
    pub atlasHash: Hash128,
    pub firstSystemId: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnlightenSystemInformation {
    pub rendererIndex: u32,
    pub rendererSize: u32,
    pub atlasIndex: i32,
    pub atlasOffsetX: i32,
    pub atlasOffsetY: i32,
    pub inputSystemHash: Hash128,
    pub radiositySystemHash: Hash128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnlightenTerrainChunksInformation {
    pub firstSystemId: i32,
    pub numChunksInX: i32,
    pub numChunksInY: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpandedData {
    pub m_InspectorExpanded: bool,
    pub m_ClassID: i32,
    pub m_ScriptClass: String,
    pub m_ExpandedProperties: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExposedReferenceTable {
    pub m_References: Vec<(String, PPtr)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expression {
    pub op: i32,
    pub valueIndex: i32,
    pub data: (i32, i32, i32, i32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionPropertyValue {
    pub pluginName: String,
    pub extensionName: String,
    pub propertyName: String,
    pub propertyValue: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalForcesModule {
    pub enabled: bool,
    pub multiplier: Option<f32>,
    pub influenceFilter: Option<i32>,
    pub influenceMask: Option<BitField>,
    pub influenceList: Option<Vec<PPtr>>,
    pub multiplierCurve: Option<MinMaxCurve>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct FBXImporter {
//     pub m_Name: String,
//     pub m_FileIDToRecycleName: Option<(Option<Vec<(i64, String)>>, Option<Vec<(i32, String)>>)>,
//     pub m_OldHashIdentity: Option<MdFour>,
//     pub m_NewHashIdentity: Option<MdFour>,
//     pub m_GenerateMaterials: Option<i32>,
//     pub m_GenerateAnimations: Option<i32>,
//     pub m_GlobalScale: f32,
//     pub m_BakeSimulation: bool,
//     pub m_SplitAnimations: Option<bool>,
//     pub m_AddColliders: bool,
//     pub m_UseFileUnits: bool,
//     pub m_AnimationCompression: i32,
//     pub m_AnimationRotationError: f32,
//     pub m_AnimationPositionError: f32,
//     pub m_AnimationScaleError: f32,
//     pub m_MeshCompression: i32,
//     pub m_MeshSettings_swapUVChannels: Option<bool>,
//     pub m_MeshSettings_generateSecondaryUV: Option<bool>,
//     pub m_MeshSettings_secondaryUVAngleDistortion: Option<f32>,
//     pub m_MeshSettings_secondaryUVAreaDistortion: Option<f32>,
//     pub m_MeshSettings_secondaryUVHardAngle: Option<f32>,
//     pub m_MeshSettings_secondaryUVPackMargin: Option<f32>,
//     pub m_MeshSettings_normalImportMode: Option<i32>,
//     pub m_MeshSettings_tangentImportMode: Option<i32>,
//     pub m_AnimationWrapMode: i32,
//     pub m_ClipAnimations: Vec<ClipAnimationInfo>,
//     pub m_FirstImportVersion: Option<i32>,
//     pub normalSmoothAngle: f32,
//     pub splitTangentsAcrossUV: Option<bool>,
//     pub m_ImportedRoots: Vec<PPtr>,
//     pub m_HasExtraRoot: bool,
//     pub m_ImportMaterials: Option<bool>,
//     pub m_MaterialName: Option<i32>,
//     pub m_MaterialSearch: Option<i32>,
//     pub m_LODScreenPercentages: Option<Vec<f32>>,
//     pub swapUVChannels: Option<bool>,
//     pub generateSecondaryUV: Option<bool>,
//     pub optimizeMesh: Option<bool>,
//     pub secondaryUVAngleDistortion: Option<f32>,
//     pub secondaryUVAreaDistortion: Option<f32>,
//     pub secondaryUVHardAngle: Option<f32>,
//     pub secondaryUVPackMargin: Option<f32>,
//     pub normalImportMode: Option<i32>,
//     pub tangentImportMode: Option<i32>,
//     pub m_LegacyGenerateAnimations: Option<i32>,
//     pub m_IsReadable: Option<bool>,
//     pub optimizeMeshForGPU: Option<bool>,
//     pub m_ImportedTakeInfos: Option<Vec<TakeInfo>>,
//     pub m_ReferencedClips: Option<Vec<GUID>>,
//     pub m_ImportAnimation: Option<bool>,
//     pub m_CopyAvatar: Option<bool>,
//     pub m_HumanDescription: Option<HumanDescription>,
//     pub m_LastHumanDescriptionAvatarSource: Option<PPtr>,
//     pub m_AdditionalBone: Option<bool>,
//     pub m_AnimationType: Option<i32>,
//     pub m_UserData: Option<String>,
//     pub m_ImportBlendShapes: Option<bool>,
//     pub weldVertices: Option<bool>,
//     pub m_OptimizeGameObjects: Option<bool>,
//     pub m_ExtraExposedTransformPaths: Option<Vec<String>>,
//     pub m_MotionNodeName: Option<String>,
//     pub keepQuads: Option<bool>,
//     pub m_UseFileScale: Option<bool>,
//     pub m_FileScale: Option<f32>,
//     pub m_AssetBundleName: Option<String>,
//     pub m_AssetBundleVariant: Option<String>,
//     pub m_AnimationImportErrors: Option<String>,
//     pub m_AnimationImportWarnings: Option<String>,
//     pub m_AnimationRetargetingWarnings: Option<String>,
//     pub m_AnimationDoRetargetingWarnings: Option<bool>,
//     pub m_HumanoidOversampling: Option<i32>,
//     pub m_ResampleRotations: Option<bool>,
//     pub m_ResampleCurves: Option<bool>,
//     pub m_RigImportErrors: Option<String>,
//     pub m_RigImportWarnings: Option<String>,
//     pub m_ImportVisibility: Option<bool>,
//     pub m_ImportCameras: Option<bool>,
//     pub m_ImportLights: Option<bool>,
//     pub normalCalculationMode: Option<i32>,
//     pub m_ExtraUserProperties: Option<Vec<String>>,
//     pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
//     pub m_AutoMapExternalMaterials: Option<bool>,
//     pub m_MaterialLocation: Option<i32>,
//     pub m_Materials: Option<Vec<SourceAssetIdentifier>>,
//     pub m_ImportAnimatedCustomProperties: Option<bool>,
//     pub m_HasEmbeddedTextures: Option<bool>,
//     pub m_SupportsEmbeddedMaterials: Option<bool>,
//     pub m_PreserveHierarchy: Option<bool>,
//     pub indexFormat: Option<i32>,
//     pub m_ImportConstraints: Option<bool>,
//     pub m_PreviousCalculatedGlobalScale: Option<f32>,
//     pub m_HasPreviousCalculatedGlobalScale: Option<bool>,
//     pub m_UseSRGBMaterialColor: Option<bool>,
//     pub m_FileScaleUnit: Option<String>,
//     pub m_FileScaleFactor: Option<f32>,
//     pub legacyComputeAllNormalsFromSmoothingGroupsWhenMeshHasBlendShapes: Option<bool>,
//     pub blendShapeNormalImportMode: Option<i32>,
//     pub normalSmoothingSource: Option<i32>,
//     pub m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
//     pub m_UsedFileIDs: Option<Vec<i64>>,
//     pub skinWeightsMode: Option<i32>,
//     pub maxBonesPerVertex: Option<i32>,
//     pub minBoneWeight: Option<f32>,
//     pub meshOptimizationFlags: Option<i32>,
//     pub m_SortHierarchyByName: Option<bool>,
//     pub m_AvatarSetup: Option<i32>,
//     pub m_MaterialImportMode: Option<i32>,
//     pub m_AutoGenerateAvatarMappingIfUnspecified: Option<bool>,
//     pub m_FileIdsGeneration: Option<i32>,
//     pub secondaryUVMarginMethod: Option<i32>,
//     pub secondaryUVMinLightmapResolution: Option<f32>,
//     pub secondaryUVMinObjectScale: Option<f32>,
//     pub bakeAxisConversion: Option<bool>,
//     pub m_AddHumanoidExtraRootOnlyWhenUsingAvatar: Option<bool>,
//     pub optimizeBones: Option<bool>,
//     pub m_RemoveConstantScaleCurves: Option<bool>,
//     pub m_NodeNameCollisionStrategy: Option<i32>,
//     pub m_StrictVertexDataChecks: Option<bool>,
//     pub m_ImportBlendShapeDeformPercent: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FakeComponent {
    pub m_GameObject: PPtr,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct FalloffTable {
//     pub m_Table: (
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
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FixedJoint {
    pub m_GameObject: PPtr,
    pub m_ConnectedBody: PPtr,
    pub m_BreakForce: f32,
    pub m_BreakTorque: f32,
    pub m_EnableCollision: Option<bool>,
    pub m_EnablePreprocessing: Option<bool>,
    pub m_MassScale: Option<f32>,
    pub m_ConnectedMassScale: Option<f32>,
    pub m_Enabled: Option<bool>,
    pub m_ConnectedArticulationBody: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FixedJoint2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_EnableCollision: bool,
    pub m_ConnectedRigidBody: PPtr,
    pub m_BreakForce: f32,
    pub m_BreakTorque: f32,
    pub m_AutoConfigureConnectedAnchor: bool,
    pub m_Anchor: Vector2f,
    pub m_ConnectedAnchor: Vector2f,
    pub m_DampingRatio: f32,
    pub m_Frequency: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flare {
    pub m_Name: String,
    pub m_FlareTexture: PPtr,
    pub m_TextureLayout: i32,
    pub m_Elements: Vec<FlareElement>,
    pub m_UseFog: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlareElement {
    pub m_ImageIndex: u32,
    pub m_Position: f32,
    pub m_Size: f32,
    pub m_Color: ColorRGBA,
    pub m_UseLightColor: bool,
    pub m_Rotate: bool,
    pub m_Zoom: bool,
    pub m_Fade: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlareLayer {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FloatCurve {
    pub curve: AnimationCurve,
    pub attribute: String,
    pub path: String,
    pub classID: i32,
    pub script: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Font {
    pub m_Name: String,
    pub m_AsciiStartOffset: i32,
    pub m_FontCountX: Option<i32>,
    pub m_FontCountY: Option<i32>,
    pub m_Kerning: Option<f32>,
    pub m_LineSpacing: f32,
    pub m_PerCharacterKerning: Option<Vec<(i32, f32)>>,
    pub m_ConvertCase: i32,
    pub m_DefaultMaterial: PPtr,
    pub m_CharacterRects: Vec<CharacterInfo>,
    pub m_Texture: PPtr,
    pub m_KerningValues: Vec<((u16, u16), f32)>,
    pub m_GridFont: Option<bool>,
    pub m_FontData: Vec<char>,
    pub m_FontSize: f32,
    pub m_Ascent: f32,
    pub m_DefaultStyle: u32,
    pub m_FontNames: Vec<String>,
    pub m_CharacterSpacing: Option<i32>,
    pub m_CharacterPadding: Option<i32>,
    pub m_PixelScale: Option<f32>,
    pub m_FallbackFonts: Option<Vec<PPtr>>,
    pub m_FontRenderingMode: Option<i32>,
    pub m_Tracking: Option<f32>,
    pub m_Descent: Option<f32>,
    pub m_UseLegacyBoundsCalculation: Option<bool>,
    pub m_ShouldRoundAdvanceValue: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForceModule {
    pub enabled: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    pub inWorldSpace: bool,
    pub randomizePerFrame: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrictionJoint2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_EnableCollision: bool,
    pub m_ConnectedRigidBody: PPtr,
    pub m_BreakForce: f32,
    pub m_BreakTorque: f32,
    pub m_AutoConfigureConnectedAnchor: bool,
    pub m_Anchor: Vector2f,
    pub m_ConnectedAnchor: Vector2f,
    pub m_MaxForce: f32,
    pub m_MaxTorque: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GISettings {
    pub m_BounceScale: f32,
    pub m_IndirectOutputScale: f32,
    pub m_AlbedoBoost: f32,
    pub m_TemporalCoherenceThreshold: Option<f32>,
    pub m_EnvironmentLightingMode: u32,
    pub m_EnableBakedLightmaps: bool,
    pub m_EnableRealtimeLightmaps: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GLTextureSettings {
    pub m_FilterMode: i32,
    pub m_Aniso: i32,
    pub m_MipBias: f32,
    pub m_WrapMode: Option<i32>,
    pub m_WrapU: Option<i32>,
    pub m_WrapV: Option<i32>,
    pub m_WrapW: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GUID {
    pub data: (u32, u32, u32, u32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GUIDSerializer {
    pub guidToPath: Vec<(GUID, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GUIElement {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GUILayer {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GUIText {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Text: String,
    pub m_Anchor: i16,
    pub m_Alignment: i16,
    pub m_PixelOffset: Vector2f,
    pub m_LineSpacing: f32,
    pub m_TabSize: f32,
    pub m_Font: PPtr,
    pub m_Material: PPtr,
    pub m_FontSize: i32,
    pub m_FontStyle: i32,
    pub m_PixelCorrect: bool,
    pub m_RichText: Option<bool>,
    pub m_Color: Option<ColorRGBA>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GUITexture {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Texture: PPtr,
    pub m_Color: ColorRGBA,
    pub m_PixelInset: Rectf,
    pub m_LeftBorder: i32,
    pub m_RightBorder: i32,
    pub m_TopBorder: i32,
    pub m_BottomBorder: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameObject {
    pub m_Component: (Option<Vec<ComponentPair>>, Option<Vec<(i32, PPtr)>>),
    pub m_Layer: u32,
    pub m_Name: String,
    pub m_Tag: u16,
    pub m_IsActive: (Option<u8>, Option<bool>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameObjectRecorder {
    pub m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericBinding {
    pub path: u32,
    pub attribute: u32,
    pub script: PPtr,
    pub classID: Option<u16>,
    pub customType: u8,
    pub isPPtrCurve: u8,
    pub typeID: Option<i32>,
    pub isIntCurve: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalGameManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Google {
    pub depthFormat: i32,
    pub enableTransitionView: Option<bool>,
    pub useSustainedPerformanceMode: Option<bool>,
    pub enableVideoLayer: Option<bool>,
    pub useProtectedVideoMemory: Option<bool>,
    pub minimumSupportedHeadTracking: Option<i32>,
    pub maximumSupportedHeadTracking: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Gradient {
    pub m_Color: Option<(ColorRGBA, ColorRGBA, ColorRGBA, ColorRGBA, ColorRGBA)>,
    pub key0: Option<ColorRGBA>,
    pub key1: Option<ColorRGBA>,
    pub key2: Option<ColorRGBA>,
    pub key3: Option<ColorRGBA>,
    pub key4: Option<ColorRGBA>,
    pub key5: Option<ColorRGBA>,
    pub key6: Option<ColorRGBA>,
    pub key7: Option<ColorRGBA>,
    pub ctime0: Option<u16>,
    pub ctime1: Option<u16>,
    pub ctime2: Option<u16>,
    pub ctime3: Option<u16>,
    pub ctime4: Option<u16>,
    pub ctime5: Option<u16>,
    pub ctime6: Option<u16>,
    pub ctime7: Option<u16>,
    pub atime0: Option<u16>,
    pub atime1: Option<u16>,
    pub atime2: Option<u16>,
    pub atime3: Option<u16>,
    pub atime4: Option<u16>,
    pub atime5: Option<u16>,
    pub atime6: Option<u16>,
    pub atime7: Option<u16>,
    pub m_Mode: Option<i32>,
    pub m_NumColorKeys: Option<u8>,
    pub m_NumAlphaKeys: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GradientNEW {
    pub key0: ColorRGBA,
    pub key1: ColorRGBA,
    pub key2: ColorRGBA,
    pub key3: ColorRGBA,
    pub key4: ColorRGBA,
    pub key5: ColorRGBA,
    pub key6: ColorRGBA,
    pub key7: ColorRGBA,
    pub ctime0: u16,
    pub ctime1: u16,
    pub ctime2: u16,
    pub ctime3: u16,
    pub ctime4: u16,
    pub ctime5: u16,
    pub ctime6: u16,
    pub ctime7: u16,
    pub atime0: u16,
    pub atime1: u16,
    pub atime2: u16,
    pub atime3: u16,
    pub atime4: u16,
    pub atime5: u16,
    pub atime6: u16,
    pub atime7: u16,
    pub m_NumColorKeys: u8,
    pub m_NumAlphaKeys: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grid {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_CellSize: Vector3f,
    pub m_CellGap: Vector3f,
    pub m_CellLayout: i32,
    pub m_CellSwizzle: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GridLayout {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupConnection {
    pub sourceGroupIndex: u32,
    pub targetGroupIndex: u32,
    pub sendEffectIndex: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupConstant {
    pub parentConstantIndex: i32,
    pub volumeIndex: u32,
    pub pitchIndex: u32,
    pub mute: bool,
    pub solo: bool,
    pub bypassEffects: bool,
    pub sendIndex: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Halo {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Color: ColorRGBA,
    pub m_Size: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HaloLayer {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HaloManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hand {
    pub m_HandBoneIndex: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandPose {
    pub m_GrabX: xform,
    pub m_DoFArray: Vec<f32>,
    pub m_Override: f32,
    pub m_CloseOpen: f32,
    pub m_InOut: f32,
    pub m_Grab: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Handle {
    pub m_X: xform,
    pub m_ParentHumanIndex: u32,
    pub m_ID: u32,
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
    pub bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeightMeshBVNode {
    pub min: Vector3f,
    pub max: Vector3f,
    pub i: i32,
    pub n: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeightMeshData {
    pub m_Vertices: Vec<Vector3f>,
    pub m_Indices: Vec<i32>,
    pub m_Bounds: AABB,
    pub m_Nodes: Vec<HeightMeshBVNode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heightmap {
    pub m_Heights: Vec<i16>,
    pub m_PrecomputedError: Vec<f32>,
    pub m_MinMaxPatchHeights: Vec<f32>,
    pub m_DefaultPhysicMaterial: Option<PPtr>,
    pub m_Width: Option<i32>,
    pub m_Height: Option<i32>,
    pub m_Levels: i32,
    pub m_Scale: Vector3f,
    pub m_Thickness: Option<f32>,
    pub m_SurfaceMask: Option<Vec<u8>>,
    pub m_SurfaceMaskLOD: Option<Vec<u8>>,
    pub m_EnableSurfaceMaskTextureCompression: Option<bool>,
    pub m_Resolution: Option<i32>,
    pub m_Holes: Option<Vec<u8>>,
    pub m_HolesLOD: Option<Vec<u8>>,
    pub m_EnableHolesTextureCompression: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeightmapData {
    pub position: Vector3f,
    pub terrainData: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HierarchicalSceneData {
    pub m_SceneGUID: GUID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HierarchyState {
    pub m_ObjectHideFlags: Option<u32>,
    pub expanded: Vec<PPtr>,
    pub selection: Vec<PPtr>,
    pub scrollposition_x: f32,
    pub scrollposition_y: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HingeJoint {
    pub m_GameObject: PPtr,
    pub m_ConnectedBody: PPtr,
    pub m_Anchor: Vector3f,
    pub m_Axis: Vector3f,
    pub m_UseSpring: bool,
    pub m_Spring: JointSpring,
    pub m_UseMotor: bool,
    pub m_Motor: JointMotor,
    pub m_UseLimits: bool,
    pub m_Limits: JointLimits,
    pub m_BreakForce: f32,
    pub m_BreakTorque: f32,
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    pub m_ConnectedAnchor: Option<Vector3f>,
    pub m_EnableCollision: Option<bool>,
    pub m_EnablePreprocessing: Option<bool>,
    pub m_MassScale: Option<f32>,
    pub m_ConnectedMassScale: Option<f32>,
    pub m_Enabled: Option<bool>,
    pub m_ConnectedArticulationBody: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HingeJoint2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_CollideConnected: Option<bool>,
    pub m_ConnectedRigidBody: PPtr,
    pub m_Anchor: Vector2f,
    pub m_ConnectedAnchor: Vector2f,
    pub m_UseMotor: bool,
    pub m_Motor: JointMotor2D,
    pub m_UseLimits: bool,
    pub m_AngleLimits: (Option<JointAngleLimit2D>, Option<JointAngleLimits2D>),
    pub m_EnableCollision: Option<bool>,
    pub m_BreakForce: Option<f32>,
    pub m_BreakTorque: Option<f32>,
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HoloLens {
    pub depthFormat: i32,
    pub depthBufferSharingEnabled: Option<bool>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Human {
//     pub m_RootX: xform,
//     pub m_Skeleton: OffsetPtr,
//     pub m_SkeletonPose: OffsetPtr,
//     pub m_LeftHand: OffsetPtr,
//     pub m_RightHand: OffsetPtr,
//     pub m_Handles: Option<Vec<Handle>>,
//     pub m_ColliderArray: Option<Vec<Collider>>,
//     pub m_HumanBoneIndex: Vec<i32>,
//     pub m_HumanBoneMass: Vec<f32>,
//     pub m_ColliderIndex: Option<Vec<i32>>,
//     pub m_Scale: f32,
//     pub m_ArmTwist: f32,
//     pub m_ForeArmTwist: f32,
//     pub m_UpperLegTwist: f32,
//     pub m_LegTwist: f32,
//     pub m_ArmStretch: f32,
//     pub m_LegStretch: f32,
//     pub m_FeetSpacing: f32,
//     pub m_HasLeftHand: bool,
//     pub m_HasRightHand: bool,
//     pub m_HasTDoF: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanBone {
    pub m_BoneName: String,
    pub m_HumanName: String,
    pub m_Limit: SkeletonBoneLimit,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanDescription {
    pub m_Human: Vec<HumanBone>,
    pub m_Skeleton: Vec<SkeletonBone>,
    pub m_Handles: Option<Vec<HumanHandle>>,
    pub m_ArmTwist: f32,
    pub m_ForeArmTwist: f32,
    pub m_UpperLegTwist: f32,
    pub m_LegTwist: f32,
    pub m_ArmStretch: f32,
    pub m_LegStretch: f32,
    pub m_FeetSpacing: f32,
    pub m_RootMotionBoneName: String,
    pub m_HasTranslationDoF: Option<bool>,
    pub m_RootMotionBoneRotation: Option<Quaternionf>,
    pub m_HasExtraRoot: Option<bool>,
    pub m_SkeletonHasParents: Option<bool>,
    pub m_GlobalScale: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanGoal {
    pub m_X: xform,
    pub m_WeightT: f32,
    pub m_WeightR: f32,
    pub m_HintT: Option<(Option<float3>, Option<float4>)>,
    pub m_HintWeightT: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanHandle {
    pub m_Name: String,
    pub m_BoneName: String,
    pub m_Position: Vector3f,
    pub m_Rotation: Quaternionf,
    pub m_Scale: Vector3f,
    pub m_LookAt: bool,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct HumanLayerConstant {
//     pub m_StateMachineIndex: u32,
//     pub m_StateMachineMotionSetIndex: u32,
//     pub m_BodyMask: HumanPoseMask,
//     pub m_SkeletonMask: OffsetPtr,
//     pub m_Binding: u32,
//     pub m_LayerBlendingMode: i32,
//     pub m_IKPass: bool,
//     pub m_DefaultWeight: Option<f32>,
//     pub m_SyncedLayerAffectsTiming: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanPose {
    pub m_RootX: xform,
    pub m_LookAtPosition: (Option<float3>, Option<float4>),
    pub m_LookAtWeight: float4,
    pub m_GoalArray: Vec<HumanGoal>,
    pub m_LeftHandPose: HandPose,
    pub m_RightHandPose: HandPose,
    pub m_DoFArray: Vec<f32>,
    pub m_TDoFArray: Option<(Option<Vec<float4>>, Option<Vec<float3>>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanPoseMask {
    pub word0: u32,
    pub word1: u32,
    pub word2: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanTemplate {
    pub m_Name: String,
    pub m_BoneTemplate: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IConstraint {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IHVImageFormatImporter {
    pub m_Name: String,
    pub m_TextureSettings: GLTextureSettings,
    pub m_IsReadable: bool,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_sRGBTexture: Option<bool>,
    pub m_StreamingMipmaps: Option<bool>,
    pub m_StreamingMipmapsPriority: Option<i32>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub m_Format: (Option<u32>, Option<i32>),
    pub m_Width: i32,
    pub m_Height: i32,
    pub m_RowBytes: i32,
    pub image_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InheritVelocityModule {
    pub enabled: bool,
    pub m_Mode: i32,
    pub m_Curve: MinMaxCurve,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitialModule {
    pub enabled: bool,
    pub startLifetime: MinMaxCurve,
    pub startSpeed: MinMaxCurve,
    pub startColor: MinMaxGradient,
    pub startSize: MinMaxCurve,
    pub startRotation: MinMaxCurve,
    pub gravityModifier: (Option<f32>, Option<MinMaxCurve>),
    pub inheritVelocity: Option<f32>,
    pub maxNumParticles: i32,
    pub startRotationX: Option<MinMaxCurve>,
    pub startRotationY: Option<MinMaxCurve>,
    pub randomizeRotationDirection: Option<f32>,
    pub rotation3D: Option<bool>,
    pub startSizeY: Option<MinMaxCurve>,
    pub startSizeZ: Option<MinMaxCurve>,
    pub size3D: Option<bool>,
    pub customEmitterVelocity: Option<Vector3f>,
    pub gravitySource: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputAxis {
    pub m_Name: String,
    pub descriptiveName: String,
    pub descriptiveNegativeName: String,
    pub negativeButton: String,
    pub positiveButton: String,
    pub altNegativeButton: String,
    pub altPositiveButton: String,
    pub gravity: f32,
    pub dead: f32,
    pub sensitivity: f32,
    pub snap: bool,
    pub invert: bool,
    pub __type: i32,
    pub axis: i32,
    pub joyNum: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputImportSettings {
    pub name: String,
    pub value: Option<SubstanceValue>,
    pub alphaSource: Option<i32>,
    pub filterMode: Option<i32>,
    pub aniso: Option<i32>,
    pub wrapMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputManager {
    pub m_Axes: Vec<InputAxis>,
    pub m_UsePhysicalKeys: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InspectorExpandedState {
    pub m_ObjectHideFlags: Option<u32>,
    pub m_ExpandedData: Vec<ExpandedData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntPoint {
    pub X: i64,
    pub Y: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveCloth {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_BendingStiffness: f32,
    pub m_StretchingStiffness: f32,
    pub m_Damping: f32,
    pub m_Thickness: f32,
    pub m_UseGravity: bool,
    pub m_SelfCollision: bool,
    pub m_ExternalAcceleration: Vector3f,
    pub m_RandomAcceleration: Vector3f,
    pub m_Mesh: PPtr,
    pub m_Friction: f32,
    pub m_Density: f32,
    pub m_Pressure: f32,
    pub m_CollisionResponse: f32,
    pub m_AttachmentTearFactor: f32,
    pub m_AttachmentResponse: f32,
    pub m_TearFactor: f32,
    pub m_AttachedColliders: Vec<ClothAttachment>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub markedForRemoval: Option<bool>,
    pub downloadResolution: Option<i32>,
    pub nameConflictResolution: Option<i32>,
    pub changeset: Option<i32>,
    pub guid: Option<GUID>,
    pub name: Option<String>,
    pub parent: Option<GUID>,
    pub __type: Option<i32>,
    pub digest: Option<(Option<MdFour>, Option<Hash128>)>,
    pub origin: Option<i32>,
    pub oldVersion: Option<i32>,
    pub parentFolderID: Option<i32>,
    pub changeFlags: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Joint {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Joint2D {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointAngleLimit2D {
    pub m_LowerAngle: f32,
    pub m_UpperAngle: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointAngleLimits2D {
    pub m_LowerAngle: f32,
    pub m_UpperAngle: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointDrive {
    pub mode: Option<i32>,
    pub positionSpring: f32,
    pub positionDamper: f32,
    pub maximumForce: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointLimits {
    pub min: f32,
    pub max: f32,
    pub minBounce: Option<f32>,
    pub maxBounce: Option<f32>,
    pub contactDistance: Option<f32>,
    pub bounciness: Option<f32>,
    pub bounceMinVelocity: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointMotor {
    pub targetVelocity: f32,
    pub force: f32,
    pub freeSpin: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointMotor2D {
    pub m_MotorSpeed: f32,
    pub m_MaximumMotorForce: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointSpring {
    pub spring: f32,
    pub damper: f32,
    pub targetPosition: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointSuspension2D {
    pub m_DampingRatio: f32,
    pub m_Frequency: f32,
    pub m_Angle: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JointTranslationLimits2D {
    pub m_LowerTranslation: f32,
    pub m_UpperTranslation: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KTXImporter {
    pub m_Name: String,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keyframe {
    pub time: f32,
    pub value: (Option<f32>, Option<Quaternionf>, Option<Vector3f>),
    pub inSlope: (Option<f32>, Option<Quaternionf>, Option<Vector3f>),
    pub outSlope: (Option<f32>, Option<Quaternionf>, Option<Vector3f>),
    pub weightedMode: Option<i32>,
    pub inWeight: Option<(Option<f32>, Option<Quaternionf>, Option<Vector3f>)>,
    pub outWeight: Option<(Option<f32>, Option<Quaternionf>, Option<Vector3f>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LOD {
    pub screenRelativeHeight: f32,
    pub renderers: Vec<LODRenderer>,
    pub fadeMode: Option<i32>,
    pub fadeTransitionWidth: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LODGroup {
    pub m_GameObject: PPtr,
    pub m_LocalReferencePoint: Vector3f,
    pub m_Size: f32,
    pub m_ScreenRelativeTransitionHeight: Option<f32>,
    pub m_LODs: Vec<LOD>,
    pub m_Enabled: bool,
    pub m_FadeMode: Option<i32>,
    pub m_AnimateCrossFading: Option<bool>,
    pub m_LastLODIsBillboard: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LODRenderer {
    pub renderer: PPtr,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct LayerConstant {
//     pub m_StateMachineIndex: u32,
//     pub m_StateMachineMotionSetIndex: Option<u32>,
//     pub m_BodyMask: HumanPoseMask,
//     pub m_SkeletonMask: OffsetPtr,
//     pub m_Binding: u32,
//     pub m_LayerBlendingMode: i32,
//     pub m_DefaultWeight: f32,
//     pub m_IKPass: bool,
//     pub m_SyncedLayerAffectsTiming: bool,
//     pub m_StateMachineSynchronizedLayerIndex: Option<u32>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayoutDataOne {
    pub m_FloatArray: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayoutDataThree {
    pub m_AnotherFloatArray: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayoutDataTwo {
    pub m_IntegerValue: i32,
    pub m_FloatValue: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeafInfoConstant {
    pub m_IDArray: Vec<u32>,
    pub m_IndexOffset: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LensFlare {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Flare: PPtr,
    pub m_Color: ColorRGBA,
    pub m_Brightness: f32,
    pub m_IgnoreLayers: BitField,
    pub m_Directional: bool,
    pub m_FadeSpeed: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelGameManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryAssetImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_UserData: Option<String>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryRepresentation {
    pub name: String,
    pub thumbnail: Image,
    pub object: Option<(Option<PPtr>, Option<PPtr>)>,
    pub thumbnailClassID: (Option<i16>, Option<i32>),
    pub scriptClassName: String,
    pub flags: Option<u16>,
    pub guid: Option<GUID>,
    pub path: Option<String>,
    pub localIdentifier: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LifetimeByEmitterSpeedModule {
    pub enabled: bool,
    pub m_Curve: MinMaxCurve,
    pub m_Range: Vector2f,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Light {
//     pub m_GameObject: PPtr,
//     pub m_Enabled: u8,
//     pub m_Type: i32,
//     pub m_Color: ColorRGBA,
//     pub m_Intensity: f32,
//     pub m_Range: f32,
//     pub m_SpotAngle: f32,
//     pub m_CookieSize: f32,
//     pub m_Shadows: ShadowSettings,
//     pub m_Cookie: PPtr,
//     pub m_DrawHalo: bool,
//     pub m_ActuallyLightmapped: Option<bool>,
//     pub m_Flare: PPtr,
//     pub m_RenderMode: i32,
//     pub m_CullingMask: BitField,
//     pub m_Lightmapping: i32,
//     pub m_BounceIntensity: Option<f32>,
//     pub m_BakedIndex: Option<i32>,
//     pub m_AreaSize: Option<Vector2f>,
//     pub m_CCT: Option<f32>,
//     pub m_BakingOutput: Option<LightBakingOutput>,
//     pub m_ColorTemperature: Option<f32>,
//     pub m_UseColorTemperature: Option<bool>,
//     pub m_FalloffTable: Option<FalloffTable>,
//     pub m_LightShadowCasterMode: Option<i32>,
//     pub m_InnerSpotAngle: Option<f32>,
//     pub m_RenderingLayerMask: Option<u32>,
//     pub m_BoundingSphereOverride: Option<Vector4f>,
//     pub m_UseBoundingSphereOverride: Option<bool>,
//     pub m_Shape: Option<i32>,
//     pub m_UseViewFrustumForShadowCasterCull: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightBakingOutput {
    pub probeOcclusionLightIndex: i32,
    pub shadowMaskChannel: Option<i32>,
    pub lightmappingMask: Option<i32>,
    pub occlusionMaskChannel: Option<i32>,
    pub lightmapBakeMode: Option<LightmapBakeMode>,
    pub isBaked: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightProbeData {
    pub m_Tetrahedralization: ProbeSetTetrahedralization,
    pub m_ProbeSets: Vec<ProbeSetIndex>,
    pub m_Positions: Vec<Vector3f>,
    pub m_NonTetrahedralizedProbeSetIndexMap: Vec<(Hash128, i32)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightProbeGroup {
    pub m_GameObject: PPtr,
    pub m_Enabled: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightProbeOcclusion {
    pub m_BakedLightIndex: Option<Vec<i32>>,
    pub m_Occlusion: Vec<f32>,
    pub m_ProbeOcclusionLightIndex: Option<Vec<i32>>,
    pub m_ShadowMaskChannel: Option<Vec<i8>>,
    pub m_OcclusionMaskChannel: Option<Vec<i8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightProbeProxyVolume {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_BoundingBoxMode: i32,
    pub m_ResolutionX: u32,
    pub m_ResolutionY: u32,
    pub m_ResolutionZ: u32,
    pub m_ResolutionProbesPerUnit: f32,
    pub m_BoundingBoxSize: Vector3f,
    pub m_BoundingBoxOrigin: Vector3f,
    pub m_ResolutionMode: i32,
    pub m_ProbePositionMode: i32,
    pub m_RefreshMode: i32,
    pub m_QualityMode: Option<i32>,
    pub m_DataFormat: Option<i32>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct LightProbes {
//     pub m_Name: String,
//     pub bakedPositions: Option<Vec<Vector3f>>,
//     pub bakedCoefficients: Option<Vec<LightmapData>>,
//     pub tetrahedra: Option<Vec<Tetrahedron>>,
//     pub hullRays: Option<Vec<Vector3f>>,
//     pub m_Data: Option<LightProbeData>,
//     pub m_BakedCoefficients: Option<Vec<SphericalHarmonicsL2>>,
//     pub m_BakedLightOcclusion: Option<Vec<LightProbeOcclusion>>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightingDataAssetParent {
    pub m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightingSettings {
    pub m_Name: String,
    pub m_GIWorkflowMode: i32,
    pub m_EnableBakedLightmaps: bool,
    pub m_EnableRealtimeLightmaps: bool,
    pub m_RealtimeEnvironmentLighting: bool,
    pub m_BounceScale: f32,
    pub m_AlbedoBoost: f32,
    pub m_UsingShadowmask: bool,
    pub m_BakeBackend: Option<i32>,
    pub m_LightmapMaxSize: Option<i32>,
    pub m_BakeResolution: Option<f32>,
    pub m_Padding: Option<i32>,
    pub m_TextureCompression: Option<bool>,
    pub m_AO: Option<bool>,
    pub m_AOMaxDistance: Option<f32>,
    pub m_CompAOExponent: Option<f32>,
    pub m_CompAOExponentDirect: Option<f32>,
    pub m_ExtractAO: Option<bool>,
    pub m_MixedBakeMode: Option<i32>,
    pub m_LightmapsBakeMode: Option<i32>,
    pub m_FilterMode: Option<i32>,
    pub m_LightmapParameters: Option<PPtr>,
    pub m_ExportTrainingData: Option<bool>,
    pub m_TrainingDataDestination: Option<String>,
    pub m_RealtimeResolution: Option<f32>,
    pub m_ForceWhiteAlbedo: Option<bool>,
    pub m_ForceUpdates: Option<bool>,
    pub m_FinalGather: Option<bool>,
    pub m_FinalGatherRayCount: Option<i32>,
    pub m_FinalGatherFiltering: Option<bool>,
    pub m_PVRCulling: Option<bool>,
    pub m_PVRSampling: Option<i32>,
    pub m_PVRDirectSampleCount: Option<i32>,
    pub m_PVRSampleCount: Option<i32>,
    pub m_PVREnvironmentSampleCount: Option<i32>,
    pub m_PVRBounces: Option<i32>,
    pub m_PVREnvironmentMIS: Option<i32>,
    pub m_PVREnvironmentReferencePointCount: Option<i32>,
    pub m_PVRFilteringMode: Option<i32>,
    pub m_PVRDenoiserTypeDirect: Option<i32>,
    pub m_PVRDenoiserTypeIndirect: Option<i32>,
    pub m_PVRDenoiserTypeAO: Option<i32>,
    pub m_PVRFilterTypeDirect: Option<i32>,
    pub m_PVRFilterTypeIndirect: Option<i32>,
    pub m_PVRFilterTypeAO: Option<i32>,
    pub m_PVRFilteringGaussRadiusDirect: Option<i32>,
    pub m_PVRFilteringGaussRadiusIndirect: Option<i32>,
    pub m_PVRFilteringGaussRadiusAO: Option<i32>,
    pub m_PVRFilteringAtrousPositionSigmaDirect: Option<f32>,
    pub m_PVRFilteringAtrousPositionSigmaIndirect: Option<f32>,
    pub m_PVRFilteringAtrousPositionSigmaAO: Option<f32>,
    pub m_IndirectOutputScale: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightmapBakeMode {
    pub lightmapBakeType: i32,
    pub mixedLightingMode: i32,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct LightmapData {
//     pub m_Lightmap: Option<PPtr>,
//     pub m_IndirectLightmap: Option<PPtr>,
//     pub m_DirLightmap: Option<PPtr>,
//     pub m_ShadowMask: Option<PPtr>,
//     pub sh: Option<(
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
    pub m_Name: String,
    pub resolution: f32,
    pub clusterResolution: f32,
    pub irradianceBudget: i32,
    pub irradianceQuality: i32,
    pub backFaceTolerance: f32,
    pub isTransparent: i32,
    pub modellingTolerance: f32,
    pub systemTag: i32,
    pub edgeStitching: i32,
    pub blurRadius: i32,
    pub directLightQuality: i32,
    pub antiAliasingSamples: i32,
    pub AOQuality: i32,
    pub AOAntiAliasingSamples: i32,
    pub bakedLightmapTag: i32,
    pub pushoff: Option<f32>,
    pub limitLightmapCount: Option<bool>,
    pub maxLightmapCount: Option<i32>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct LightmapSettings {
//     pub m_Lightmaps: Vec<LightmapData>,
//     pub m_LightmapsMode: i32,
//     pub m_UseDualLightmapsInForward: Option<bool>,
//     pub m_LightProbes: Option<PPtr>,
//     pub m_BakedColorSpace: Option<i32>,
//     pub m_EnlightenSceneMapping: Option<EnlightenSceneMapping>,
//     pub m_GISettings: Option<GISettings>,
//     pub m_RuntimeCPUUsage: Option<i32>,
//     pub m_ShadowMaskMode: Option<i32>,
//     pub m_UseShadowmask: Option<bool>,
//     pub m_LightingSettings: Option<PPtr>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct LightmapSnapshot {
//     pub m_Name: String,
//     pub m_Lightmaps: Vec<LightmapData>,
//     pub m_LightProbes: PPtr,
//     pub m_BakedAmbientProbesInLinear: Option<Vec<SphericalHarmonicsL2>>,
//     pub m_BakedAmbientProbesInGamma: Option<Vec<SphericalHarmonicsL2>>,
//     pub m_LightmappedRendererData: Vec<RendererData>,
//     pub m_LightmappedRendererDataIDs: Vec<SceneObjectIdentifier>,
//     pub m_EnlightenSceneMapping: EnlightenSceneMapping,
//     pub m_EnlightenSceneMappingRendererIDs: Vec<SceneObjectIdentifier>,
//     pub m_Lights: Vec<SceneObjectIdentifier>,
//     pub m_BakedReflectionProbeCubemaps: Vec<PPtr>,
//     pub m_BakedSkyboxProbeCubemaps: Option<Vec<PPtr>>,
//     pub m_BakedReflectionProbes: Vec<SceneObjectIdentifier>,
//     pub m_EnlightenData: Vec<u8>,
//     pub m_SceneGUID: Option<GUID>,
//     pub m_BakedAmbientProbeInLinear: Option<SphericalHarmonicsL2>,
//     pub m_BakedAmbientProbeInGamma: Option<SphericalHarmonicsL2>,
//     pub m_EnlightenDataVersion: Option<i32>,
//     pub m_LightmapsMode: Option<i32>,
//     pub m_BakedLightIndices: Option<Vec<i32>>,
//     pub m_LightBakingOutputs: Option<Vec<LightBakingOutput>>,
//     pub m_Scene: Option<PPtr>,
//     pub m_LightmapsCacheFiles: Option<Vec<String>>,
//     pub m_BakedReflectionProbeCubemapCacheFiles: Option<Vec<String>>,
//     pub m_AOTextures: Option<Vec<PPtr>>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightsModule {
    pub enabled: bool,
    pub ratio: f32,
    pub light: PPtr,
    pub randomDistribution: bool,
    pub color: bool,
    pub range: bool,
    pub intensity: bool,
    pub rangeCurve: MinMaxCurve,
    pub intensityCurve: MinMaxCurve,
    pub maxLights: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Limit {
    pub m_Min: (Option<float3>, Option<float4>),
    pub m_Max: (Option<float3>, Option<float4>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineParameters {
    pub widthMultiplier: Option<f32>,
    pub widthCurve: Option<AnimationCurve>,
    pub colorGradient: Option<Gradient>,
    pub numCornerVertices: Option<i32>,
    pub numCapVertices: Option<i32>,
    pub alignment: Option<i32>,
    pub textureMode: Option<i32>,
    pub generateLightingData: Option<bool>,
    pub shadowBias: Option<f32>,
    pub textureScale: Option<Vector2f>,
    pub startWidth: Option<f32>,
    pub endWidth: Option<f32>,
    pub m_StartColor: Option<ColorRGBA>,
    pub m_EndColor: Option<ColorRGBA>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineRenderer {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: (Option<u8>, Option<bool>),
    pub m_ReceiveShadows: (Option<u8>, Option<bool>),
    pub m_LightmapIndex: (Option<u8>, Option<u16>),
    pub m_LightmapTilingOffset: Vector4f,
    pub m_Materials: Vec<PPtr>,
    pub m_SubsetIndices: Option<Vec<u32>>,
    pub m_StaticBatchRoot: PPtr,
    pub m_Positions: Vec<Vector3f>,
    pub m_Parameters: LineParameters,
    pub m_UseWorldSpace: bool,
    pub m_UseLightProbes: Option<bool>,
    pub m_LightProbeAnchor: Option<PPtr>,
    pub m_SortingLayer: Option<i16>,
    pub m_SortingOrder: Option<i16>,
    pub m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    pub m_LightmapIndexDynamic: Option<u16>,
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    pub m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    pub m_ProbeAnchor: Option<PPtr>,
    pub m_MotionVectors: Option<u8>,
    pub m_LightProbeUsage: Option<u8>,
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    pub m_Loop: Option<bool>,
    pub m_DynamicOccludee: Option<u8>,
    pub m_RenderingLayerMask: Option<u32>,
    pub m_RendererPriority: Option<i32>,
    pub m_RayTracingMode: Option<u8>,
    pub m_RayTraceProcedural: Option<u8>,
    pub m_StaticShadowCaster: Option<u8>,
    pub m_MaskInteraction: Option<i32>,
    pub m_ApplyActiveColorSpace: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalizationAsset {
    pub m_Name: String,
    pub Locale_ISO_Code: String,
    pub Editor_Asset: bool,
    pub String_Table: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalizationImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LookAtConstraint {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Weight: f32,
    pub m_RotationAtRest: Vector3f,
    pub m_RotationOffset: Vector3f,
    pub m_Roll: f32,
    pub m_WorldUpObject: PPtr,
    pub m_UseUpObject: bool,
    pub m_IsContraintActive: Option<bool>,
    pub m_Sources: Vec<ConstraintSource>,
    pub m_Active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LowerResBlitTexture {
    pub m_Name: String,
    pub m_ForcedFallbackFormat: Option<i32>,
    pub m_DownscaleFallback: Option<bool>,
    pub m_IsAlphaChannelOptional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lumin {
    pub depthFormat: i32,
    pub frameTiming: i32,
    pub enableGLCache: bool,
    pub glCacheMaxBlobSize: u32,
    pub glCacheMaxFileSize: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MasterServerInterface {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Material {
    pub m_Name: String,
    pub m_Shader: PPtr,
    pub m_SavedProperties: UnityPropertySheet,
    pub m_ShaderKeywords: Option<(Option<Vec<String>>, Option<String>)>,
    pub m_CustomRenderQueue: Option<i32>,
    pub m_LightmapFlags: Option<u32>,
    pub stringTagMap: Option<Vec<(String, String)>>,
    pub m_EnableInstancingVariants: Option<bool>,
    pub disabledShaderPasses: Option<Vec<String>>,
    pub m_DoubleSidedGI: Option<bool>,
    pub m_BuildTextureStacks: Option<Vec<BuildTextureStackReference>>,
    pub m_ValidKeywords: Option<Vec<String>>,
    pub m_InvalidKeywords: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaterialImportOutput {
    pub currentSettings: BuildTargetSettings,
    pub baked: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaterialInstanceSettings {
    pub name: String,
    pub prototypeName: String,
    pub shaderName: Option<String>,
    pub inputs: Vec<InputImportSettings>,
    pub materialInformation: ProceduralMaterialInformation,
    pub materialProperties: UnityPropertySheet,
    pub textureParameters: Vec<InputImportSettings>,
    pub buildTargetSettings: Vec<BuildTargetSettings>,
    pub shader: Option<PPtr>,
    pub textureAssignments: Option<Vec<ProceduralTextureAssignment>>,
    pub shaderKeywords: Option<String>,
    pub renderQueue: Option<i32>,
    pub lightmapFlags: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Matrix3x4f {
    pub e00: f32,
    pub e01: f32,
    pub e02: f32,
    pub e03: f32,
    pub e10: f32,
    pub e11: f32,
    pub e12: f32,
    pub e13: f32,
    pub e20: f32,
    pub e21: f32,
    pub e22: f32,
    pub e23: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Matrix4x4f {
    pub e00: f32,
    pub e01: f32,
    pub e02: f32,
    pub e03: f32,
    pub e10: f32,
    pub e11: f32,
    pub e12: f32,
    pub e13: f32,
    pub e20: f32,
    pub e21: f32,
    pub e22: f32,
    pub e23: f32,
    pub e30: f32,
    pub e31: f32,
    pub e32: f32,
    pub e33: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatrixParameter {
    pub m_NameIndex: i32,
    pub m_Index: i32,
    pub m_ArraySize: i32,
    pub m_Type: i8,
    pub m_RowCount: i8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MdFour {
    pub md4_hash: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemorySettings {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mesh {
    pub m_Name: String,
    pub m_Use16BitIndices: Option<i32>,
    pub m_SubMeshes: Vec<SubMesh>,
    pub m_MeshCompression: u8,
    pub m_IndexBuffer: Vec<u8>,
    pub m_Vertices: Option<Vec<Vector3f>>,
    pub m_Skin: Option<(Option<Vec<BoneInfluence>>, Option<Vec<BoneWeights4>>)>,
    pub m_BindPose: Vec<Matrix4x4f>,
    pub m_UV: Option<Vec<Vector2f>>,
    pub m_UV1: Option<Vec<Vector2f>>,
    pub m_Tangents: Option<Vec<Vector4f>>,
    pub m_Normals: Option<Vec<Vector3f>>,
    pub m_CompressedMesh: CompressedMesh,
    pub m_LocalAABB: AABB,
    pub m_Colors: Option<Vec<ColorRGBA>>,
    pub m_CollisionTriangles: Option<Vec<u32>>,
    pub m_CollisionVertexCount: Option<i32>,
    pub m_MeshUsageFlags: i32,
    pub m_VertexData: Option<VertexData>,
    pub m_StreamCompression: Option<u8>,
    pub m_IsReadable: Option<bool>,
    pub m_KeepVertices: Option<bool>,
    pub m_KeepIndices: Option<bool>,
    pub m_Shapes: Option<(Option<Vec<MeshBlendShape>>, Option<BlendShapeData>)>,
    pub m_ShapeVertices: Option<Vec<MeshBlendShapeVertex>>,
    pub m_BoneNameHashes: Option<Vec<u32>>,
    pub m_RootBoneNameHash: Option<u32>,
    pub m_BakedConvexCollisionMesh: Option<Vec<u8>>,
    pub m_BakedTriangleCollisionMesh: Option<Vec<u8>>,
    pub m_IndexFormat: Option<i32>,
    pub m_MeshMetrics: Option<(f32, f32)>,
    pub m_StreamData: Option<StreamingInfo>,
    pub m_BonesAABB: Option<Vec<MinMaxAABB>>,
    pub m_VariableBoneCountWeights: Option<VariableBoneCountWeights>,
    pub m_CookingOptions: Option<i32>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Mesh3DSImporter {
//     pub m_Name: String,
//     pub m_FileIDToRecycleName: Option<(Option<Vec<(i64, String)>>, Option<Vec<(i32, String)>>)>,
//     pub m_OldHashIdentity: Option<MdFour>,
//     pub m_NewHashIdentity: Option<MdFour>,
//     pub m_GenerateMaterials: Option<i32>,
//     pub m_GenerateAnimations: Option<i32>,
//     pub m_GlobalScale: f32,
//     pub m_BakeSimulation: bool,
//     pub m_SplitAnimations: Option<bool>,
//     pub m_AddColliders: bool,
//     pub m_UseFileUnits: bool,
//     pub m_AnimationCompression: i32,
//     pub m_AnimationRotationError: f32,
//     pub m_AnimationPositionError: f32,
//     pub m_AnimationScaleError: f32,
//     pub m_MeshCompression: i32,
//     pub m_MeshSettings_swapUVChannels: Option<bool>,
//     pub m_MeshSettings_generateSecondaryUV: Option<bool>,
//     pub m_MeshSettings_secondaryUVAngleDistortion: Option<f32>,
//     pub m_MeshSettings_secondaryUVAreaDistortion: Option<f32>,
//     pub m_MeshSettings_secondaryUVHardAngle: Option<f32>,
//     pub m_MeshSettings_secondaryUVPackMargin: Option<f32>,
//     pub m_MeshSettings_normalImportMode: Option<i32>,
//     pub m_MeshSettings_tangentImportMode: Option<i32>,
//     pub m_AnimationWrapMode: i32,
//     pub m_ClipAnimations: Vec<ClipAnimationInfo>,
//     pub m_FirstImportVersion: Option<i32>,
//     pub normalSmoothAngle: f32,
//     pub splitTangentsAcrossUV: Option<bool>,
//     pub m_ImportedRoots: Vec<PPtr>,
//     pub m_HasExtraRoot: bool,
//     pub m_ImportMaterials: Option<bool>,
//     pub m_MaterialName: Option<i32>,
//     pub m_MaterialSearch: Option<i32>,
//     pub m_LODScreenPercentages: Option<Vec<f32>>,
//     pub swapUVChannels: Option<bool>,
//     pub generateSecondaryUV: Option<bool>,
//     pub optimizeMesh: Option<bool>,
//     pub secondaryUVAngleDistortion: Option<f32>,
//     pub secondaryUVAreaDistortion: Option<f32>,
//     pub secondaryUVHardAngle: Option<f32>,
//     pub secondaryUVPackMargin: Option<f32>,
//     pub normalImportMode: Option<i32>,
//     pub tangentImportMode: Option<i32>,
//     pub m_LegacyGenerateAnimations: Option<i32>,
//     pub m_IsReadable: Option<bool>,
//     pub optimizeMeshForGPU: Option<bool>,
//     pub m_ImportedTakeInfos: Option<Vec<TakeInfo>>,
//     pub m_ReferencedClips: Option<Vec<GUID>>,
//     pub m_ImportAnimation: Option<bool>,
//     pub m_CopyAvatar: Option<bool>,
//     pub m_HumanDescription: Option<HumanDescription>,
//     pub m_LastHumanDescriptionAvatarSource: Option<PPtr>,
//     pub m_AdditionalBone: Option<bool>,
//     pub m_AnimationType: Option<i32>,
//     pub m_UserData: Option<String>,
//     pub m_ImportBlendShapes: Option<bool>,
//     pub weldVertices: Option<bool>,
//     pub m_OptimizeGameObjects: Option<bool>,
//     pub m_ExtraExposedTransformPaths: Option<Vec<String>>,
//     pub m_MotionNodeName: Option<String>,
//     pub keepQuads: Option<bool>,
//     pub m_UseFileScale: Option<bool>,
//     pub m_FileScale: Option<f32>,
//     pub m_AssetBundleName: Option<String>,
//     pub m_AssetBundleVariant: Option<String>,
//     pub m_AnimationImportErrors: Option<String>,
//     pub m_AnimationImportWarnings: Option<String>,
//     pub m_AnimationRetargetingWarnings: Option<String>,
//     pub m_AnimationDoRetargetingWarnings: Option<bool>,
//     pub m_HumanoidOversampling: Option<i32>,
//     pub m_ResampleRotations: Option<bool>,
//     pub m_ResampleCurves: Option<bool>,
//     pub m_RigImportErrors: Option<String>,
//     pub m_RigImportWarnings: Option<String>,
//     pub m_ImportVisibility: Option<bool>,
//     pub m_ImportCameras: Option<bool>,
//     pub m_ImportLights: Option<bool>,
//     pub normalCalculationMode: Option<i32>,
//     pub m_ExtraUserProperties: Option<Vec<String>>,
//     pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
//     pub m_AutoMapExternalMaterials: Option<bool>,
//     pub m_MaterialLocation: Option<i32>,
//     pub m_Materials: Option<Vec<SourceAssetIdentifier>>,
//     pub m_ImportAnimatedCustomProperties: Option<bool>,
//     pub m_HasEmbeddedTextures: Option<bool>,
//     pub m_SupportsEmbeddedMaterials: Option<bool>,
//     pub m_PreserveHierarchy: Option<bool>,
//     pub indexFormat: Option<i32>,
//     pub m_ImportConstraints: Option<bool>,
//     pub m_PreviousCalculatedGlobalScale: Option<f32>,
//     pub m_HasPreviousCalculatedGlobalScale: Option<bool>,
//     pub m_UseSRGBMaterialColor: Option<bool>,
//     pub m_FileScaleUnit: Option<String>,
//     pub m_FileScaleFactor: Option<f32>,
//     pub legacyComputeAllNormalsFromSmoothingGroupsWhenMeshHasBlendShapes: Option<bool>,
//     pub blendShapeNormalImportMode: Option<i32>,
//     pub normalSmoothingSource: Option<i32>,
//     pub m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
//     pub m_UsedFileIDs: Option<Vec<i64>>,
//     pub skinWeightsMode: Option<i32>,
//     pub maxBonesPerVertex: Option<i32>,
//     pub minBoneWeight: Option<f32>,
//     pub meshOptimizationFlags: Option<i32>,
//     pub m_SortHierarchyByName: Option<bool>,
//     pub m_AvatarSetup: Option<i32>,
//     pub m_MaterialImportMode: Option<i32>,
//     pub m_AutoGenerateAvatarMappingIfUnspecified: Option<bool>,
//     pub m_FileIdsGeneration: Option<i32>,
//     pub secondaryUVMarginMethod: Option<i32>,
//     pub secondaryUVMinLightmapResolution: Option<f32>,
//     pub secondaryUVMinObjectScale: Option<f32>,
//     pub bakeAxisConversion: Option<bool>,
//     pub m_AddHumanoidExtraRootOnlyWhenUsingAvatar: Option<bool>,
//     pub optimizeBones: Option<bool>,
//     pub m_RemoveConstantScaleCurves: Option<bool>,
//     pub m_NodeNameCollisionStrategy: Option<i32>,
//     pub m_StrictVertexDataChecks: Option<bool>,
//     pub m_ImportBlendShapeDeformPercent: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshBlendShape {
    pub name: Option<String>,
    pub firstVertex: u32,
    pub vertexCount: u32,
    pub aabbMinDelta: Option<Vector3f>,
    pub aabbMaxDelta: Option<Vector3f>,
    pub hasNormals: bool,
    pub hasTangents: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshBlendShapeChannel {
    pub name: String,
    pub nameHash: u32,
    pub frameIndex: i32,
    pub frameCount: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshBlendShapeVertex {
    pub vertex: Vector3f,
    pub normal: Vector3f,
    pub tangent: Vector3f,
    pub index: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshCollider {
    pub m_GameObject: PPtr,
    pub m_Material: PPtr,
    pub m_IsTrigger: bool,
    pub m_SmoothSphereCollisions: Option<bool>,
    pub m_Convex: bool,
    pub m_Mesh: PPtr,
    pub m_Enabled: Option<bool>,
    pub m_InflateMesh: Option<bool>,
    pub m_SkinWidth: Option<f32>,
    pub m_CookingOptions: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshFilter {
    pub m_GameObject: PPtr,
    pub m_Mesh: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshParticleEmitter {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_Emit: bool,
    pub minSize: f32,
    pub maxSize: f32,
    pub minEnergy: f32,
    pub maxEnergy: f32,
    pub minEmission: f32,
    pub maxEmission: f32,
    pub worldVelocity: Vector3f,
    pub localVelocity: Vector3f,
    pub rndVelocity: Vector3f,
    pub emitterVelocityScale: f32,
    pub tangentVelocity: Vector3f,
    pub angularVelocity: f32,
    pub rndAngularVelocity: f32,
    pub rndRotation: bool,
    pub Simulate_in_Worldspace: bool,
    pub m_OneShot: bool,
    pub m_InterpolateTriangles: bool,
    pub m_Systematic: bool,
    pub m_MinNormalVelocity: f32,
    pub m_MaxNormalVelocity: f32,
    pub m_Mesh: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshRenderer {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: (Option<u8>, Option<bool>),
    pub m_ReceiveShadows: (Option<u8>, Option<bool>),
    pub m_LightmapIndex: (Option<u8>, Option<u16>),
    pub m_LightmapTilingOffset: Vector4f,
    pub m_Materials: Vec<PPtr>,
    pub m_SubsetIndices: Option<Vec<u32>>,
    pub m_StaticBatchRoot: PPtr,
    pub m_UseLightProbes: Option<bool>,
    pub m_LightProbeAnchor: Option<PPtr>,
    pub m_SortingLayer: Option<i16>,
    pub m_SortingOrder: Option<i16>,
    pub m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    pub m_LightmapIndexDynamic: Option<u16>,
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    pub m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    pub m_ProbeAnchor: Option<PPtr>,
    pub m_AdditionalVertexStreams: Option<PPtr>,
    pub m_MotionVectors: Option<u8>,
    pub m_LightProbeUsage: Option<u8>,
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    pub m_DynamicOccludee: Option<u8>,
    pub m_RenderingLayerMask: Option<u32>,
    pub m_RendererPriority: Option<i32>,
    pub m_RayTracingMode: Option<u8>,
    pub m_EnlightenVertexStream: Option<PPtr>,
    pub m_RayTraceProcedural: Option<u8>,
    pub m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinMaxAABB {
    pub m_Min: Vector3f,
    pub m_Max: Vector3f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinMaxCurve {
    pub scalar: f32,
    pub maxCurve: AnimationCurve,
    pub minCurve: AnimationCurve,
    pub minMaxState: (Option<i16>, Option<u16>),
    pub minScalar: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinMaxGradient {
    pub maxGradient: (Option<Gradient>, Option<GradientNEW>),
    pub minGradient: (Option<Gradient>, Option<GradientNEW>),
    pub minColor: ColorRGBA,
    pub maxColor: ColorRGBA,
    pub minMaxState: (Option<i16>, Option<u16>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelImporter {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Module {
    pub name: String,
    pub dependencies: Vec<String>,
    pub strippable: bool,
    pub controlledByBuiltinPackage: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonoAssemblyImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_ExecutionOrder: Vec<(String, i32)>,
    pub m_IconMap: Vec<(String, PPtr)>,
    pub m_UserData: Option<String>,
    pub m_IsPreloaded: Option<bool>,
    pub m_PlatformData: Option<(
        Option<Vec<(String, PlatformSettingsData)>>,
        Option<Vec<((String, String), PlatformSettingsData)>>,
    )>,
    pub m_Output: Option<PluginImportOutput>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_IsOverridable: Option<bool>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_DefineConstraints: Option<Vec<String>>,
    pub m_IsExplicitlyReferenced: Option<bool>,
    pub m_ValidateReferences: Option<bool>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonoBehaviour {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Script: PPtr,
    pub m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonoImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_DefaultReferences: Vec<(String, PPtr)>,
    pub executionOrder: i16,
    pub icon: PPtr,
    pub m_UserData: Option<String>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonoManager {
    pub m_Scripts: Vec<PPtr>,
    pub m_AssemblyNames: Option<Vec<String>>,
    pub m_AssemblyTypes: Option<Vec<i32>>,
    pub m_ScriptHashes: Option<Vec<(Hash128, Hash128)>>,
    pub m_RuntimeClassHashes: Option<Vec<(i32, Hash128)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonoObject {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonoScript {
    pub m_Name: String,
    pub m_ClassName: String,
    pub m_Namespace: String,
    pub m_AssemblyName: String,
    pub m_IsEditorScript: Option<bool>,
    pub m_ExecutionOrder: Option<i32>,
    pub m_PropertiesHash: Option<(Option<u32>, Option<Hash128>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Motion {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MotionNeighborList {
    pub m_NeighborArray: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MovieImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_Quality: f32,
    pub m_LinearTexture: Option<bool>,
    pub m_UserData: Option<String>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MovieTexture {
    pub m_Name: String,
    pub m_Loop: Option<bool>,
    pub m_AudioClip: Option<PPtr>,
    pub m_MovieData: Option<Vec<u8>>,
    pub m_ColorSpace: Option<i32>,
    pub m_ForcedFallbackFormat: Option<i32>,
    pub m_DownscaleFallback: Option<bool>,
    pub m_IsAlphaChannelOptional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiArtifactTestImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiModeParameter {
    pub value: Option<f32>,
    pub mode: i32,
    pub spread: f32,
    pub speed: MinMaxCurve,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NScreenBridge {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameToObjectMap {
    pub m_ObjectToName: Vec<(PPtr, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NamedObject {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NativeFormatImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_UserData: Option<String>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_MainObjectFileID: Option<i64>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NativeObjectType {
    pub m_Inner: NativeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NativeType {
    pub a: i32,
    pub b: f32,
    pub embedded: EmbeddedNativeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMesh {
    pub m_Name: String,
    pub m_MeshData: Option<Vec<u8>>,
    pub m_Heightmaps: Option<Vec<HeightmapData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshAgent {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Radius: f32,
    pub m_Speed: f32,
    pub m_Acceleration: f32,
    pub m_AngularSpeed: f32,
    pub m_StoppingDistance: f32,
    pub m_AutoTraverseOffMeshLink: bool,
    pub m_AutoRepath: bool,
    pub m_Height: f32,
    pub m_BaseOffset: f32,
    pub m_WalkableMask: u32,
    pub m_ObstacleAvoidanceType: i32,
    pub avoidancePriority: Option<i32>,
    pub m_AutoBraking: Option<bool>,
    pub m_AgentTypeID: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshAreaData {
    pub name: String,
    pub cost: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshBuildDebugSettings {
    pub m_Flags: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshBuildSettings {
    pub agentTypeID: i32,
    pub agentRadius: f32,
    pub agentHeight: f32,
    pub agentSlope: f32,
    pub agentClimb: f32,
    pub ledgeDropHeight: f32,
    pub maxJumpAcrossDistance: f32,
    pub minRegionArea: f32,
    pub manualCellSize: (Option<bool>, Option<i32>),
    pub cellSize: f32,
    pub manualTileSize: (Option<bool>, Option<i32>),
    pub tileSize: i32,
    pub accuratePlacement: (Option<bool>, Option<i32>),
    pub debug: Option<NavMeshBuildDebugSettings>,
    pub maxJobWorkers: Option<u32>,
    pub keepTiles: Option<i32>,
    pub preserveTilesOutsideBounds: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshData {
    pub m_Name: String,
    pub m_NavMeshTiles: Vec<NavMeshTileData>,
    pub m_NavMeshParams: Option<NavMeshParams>,
    pub m_Heightmaps: Vec<HeightmapData>,
    pub m_HeightMeshes: Vec<HeightMeshData>,
    pub m_OffMeshLinks: Vec<AutoOffMeshLinkData>,
    pub m_NavMeshBuildSettings: Option<NavMeshBuildSettings>,
    pub m_SourceBounds: Option<AABB>,
    pub m_Rotation: Option<Quaternionf>,
    pub m_Position: Option<Vector3f>,
    pub m_AgentTypeID: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshLayerData {
    pub name: String,
    pub cost: f32,
    pub editType: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshLayers {
    pub Built_in_Layer_0: Option<NavMeshLayerData>,
    pub Built_in_Layer_1: Option<NavMeshLayerData>,
    pub Built_in_Layer_2: Option<NavMeshLayerData>,
    pub User_Layer_0: Option<NavMeshLayerData>,
    pub User_Layer_1: Option<NavMeshLayerData>,
    pub User_Layer_2: Option<NavMeshLayerData>,
    pub User_Layer_3: Option<NavMeshLayerData>,
    pub User_Layer_4: Option<NavMeshLayerData>,
    pub User_Layer_5: Option<NavMeshLayerData>,
    pub User_Layer_6: Option<NavMeshLayerData>,
    pub User_Layer_7: Option<NavMeshLayerData>,
    pub User_Layer_8: Option<NavMeshLayerData>,
    pub User_Layer_9: Option<NavMeshLayerData>,
    pub User_Layer_10: Option<NavMeshLayerData>,
    pub User_Layer_11: Option<NavMeshLayerData>,
    pub User_Layer_12: Option<NavMeshLayerData>,
    pub User_Layer_13: Option<NavMeshLayerData>,
    pub User_Layer_14: Option<NavMeshLayerData>,
    pub User_Layer_15: Option<NavMeshLayerData>,
    pub User_Layer_16: Option<NavMeshLayerData>,
    pub User_Layer_17: Option<NavMeshLayerData>,
    pub User_Layer_18: Option<NavMeshLayerData>,
    pub User_Layer_19: Option<NavMeshLayerData>,
    pub User_Layer_20: Option<NavMeshLayerData>,
    pub User_Layer_21: Option<NavMeshLayerData>,
    pub User_Layer_22: Option<NavMeshLayerData>,
    pub User_Layer_23: Option<NavMeshLayerData>,
    pub User_Layer_24: Option<NavMeshLayerData>,
    pub User_Layer_25: Option<NavMeshLayerData>,
    pub User_Layer_26: Option<NavMeshLayerData>,
    pub User_Layer_27: Option<NavMeshLayerData>,
    pub User_Layer_28: Option<NavMeshLayerData>,
    pub areas: Option<Vec<NavMeshAreaData>>,
    pub m_LastAgentTypeID: Option<i32>,
    pub m_Settings: Option<Vec<NavMeshBuildSettings>>,
    pub m_SettingNames: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshObstacle {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Radius: Option<f32>,
    pub m_Height: Option<f32>,
    pub m_MoveThreshold: Option<f32>,
    pub m_Carve: Option<bool>,
    pub m_Shape: Option<i32>,
    pub m_Extents: Option<Vector3f>,
    pub m_CarveOnlyStationary: Option<bool>,
    pub m_Center: Option<Vector3f>,
    pub m_TimeToStationary: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshParams {
    pub tileSize: f32,
    pub walkableHeight: f32,
    pub walkableRadius: f32,
    pub walkableClimb: f32,
    pub cellSize: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshSettings {
    pub m_NavMesh: Option<PPtr>,
    pub m_NavMeshData: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMeshTileData {
    pub m_MeshData: Vec<u8>,
    pub m_Hash: Option<Hash128>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkManager {
    pub m_DebugLevel: i32,
    pub m_Sendrate: f32,
    pub m_AssetToPrefab: Vec<(GUID, PPtr)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkView {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_StateSynchronization: i32,
    pub m_Observed: PPtr,
    pub m_ViewID: NetworkViewID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkViewID {
    pub m_ID: u32,
    pub m_Type: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewAnimationTrack {
    pub m_ObjectHideFlags: Option<u32>,
    pub m_ExtensionPtr: Option<PPtr>,
    pub m_Name: String,
    pub m_Curves: Vec<Channel>,
    pub m_ClassID: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub m_ParentId: i32,
    pub m_AxesId: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoiseModule {
    pub enabled: bool,
    pub strength: MinMaxCurve,
    pub strengthY: MinMaxCurve,
    pub strengthZ: MinMaxCurve,
    pub separateAxes: bool,
    pub frequency: f32,
    pub damping: bool,
    pub octaves: i32,
    pub octaveMultiplier: f32,
    pub octaveScale: f32,
    pub quality: i32,
    pub scrollSpeed: MinMaxCurve,
    pub remap: MinMaxCurve,
    pub remapY: MinMaxCurve,
    pub remapZ: MinMaxCurve,
    pub remapEnabled: bool,
    pub positionAmount: Option<MinMaxCurve>,
    pub rotationAmount: Option<MinMaxCurve>,
    pub sizeAmount: Option<MinMaxCurve>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NonAlignedStruct {
    pub m_Bool: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcclusionArea {
    pub m_GameObject: PPtr,
    pub m_Size: Vector3f,
    pub m_Center: Vector3f,
    pub m_IsViewVolume: bool,
    pub m_IsTargetVolume: Option<bool>,
    pub m_TargetResolution: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcclusionCullingData {
    pub m_Name: String,
    pub m_PVSData: Vec<u8>,
    pub m_Scenes: Vec<OcclusionScene>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcclusionPortal {
    pub m_GameObject: PPtr,
    pub m_Open: bool,
    pub m_Center: Vector3f,
    pub m_Size: Vector3f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcclusionScene {
    pub indexRenderers: i32,
    pub sizeRenderers: i32,
    pub indexPortals: i32,
    pub sizePortals: i32,
    pub scene: GUID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Oculus {
    pub sharedDepthBuffer: bool,
    pub dashSupport: bool,
    pub lowOverheadMode: Option<bool>,
    pub protectedContext: Option<bool>,
    pub v2Signing: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OffMeshLink {
    pub m_GameObject: PPtr,
    pub m_Start: PPtr,
    pub m_End: PPtr,
    pub m_DtPolyRef: Option<u32>,
    pub m_CostOverride: f32,
    pub m_BiDirectional: bool,
    pub m_Activated: bool,
    pub m_NavMeshLayer: Option<u32>,
    pub m_Enabled: Option<u8>,
    pub m_AutoUpdatePositions: Option<bool>,
    pub m_AreaIndex: Option<u32>,
    pub m_AgentTypeID: Option<i32>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct OffsetPtr {
//     pub data: Box<(
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
    pub previewData: Option<Vec<f32>>,
    pub importedType: Option<i32>,
    pub hasEmptyFontData: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PPtrCurve {
    pub curve: Vec<PPtrKeyframe>,
    pub attribute: String,
    pub path: String,
    pub classID: i32,
    pub script: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PPtrKeyframe {
    pub time: f32,
    pub value: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PVRImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_UserData: Option<String>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageManifest {
    pub m_Name: String,
    pub m_Script: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageManifestImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackedAssets {
    pub m_File: Option<u32>,
    pub m_ShortPath: String,
    pub m_Overhead: u64,
    pub m_Contents: Vec<BuildReportPackedAssetInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackedBitVector {
    pub m_NumItems: u32,
    pub m_Range: Option<f32>,
    pub m_Start: Option<f32>,
    pub m_Data: Vec<u8>,
    pub m_BitSize: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackingSettings {
    pub padding: i32,
    pub blockOffset: i32,
    pub allowAlphaSplitting: bool,
    pub enableRotation: bool,
    pub enableTightPacking: bool,
    pub enableAlphaDilation: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parameter {
    pub m_ParameterName: String,
    pub m_GUID: GUID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParentConstraint {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Weight: f32,
    pub m_TranslationAtRest: Vector3f,
    pub m_RotationAtRest: Vector3f,
    pub m_TranslationOffsets: Vec<Vector3f>,
    pub m_RotationOffsets: Vec<Vector3f>,
    pub m_AffectTranslationX: bool,
    pub m_AffectTranslationY: bool,
    pub m_AffectTranslationZ: bool,
    pub m_AffectRotationX: bool,
    pub m_AffectRotationY: bool,
    pub m_AffectRotationZ: bool,
    pub m_IsContraintActive: Option<bool>,
    pub m_Sources: Vec<ConstraintSource>,
    pub m_Active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParserBindChannels {
    pub m_Channels: Vec<ShaderBindChannel>,
    pub m_SourceMap: (Option<u32>, Option<i32>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleAnimator {
    pub m_GameObject: PPtr,
    pub Does_Animate_Color: bool,
    pub worldRotationAxis: Vector3f,
    pub localRotationAxis: Vector3f,
    pub sizeGrow: f32,
    pub rndForce: Vector3f,
    pub force: Vector3f,
    pub damping: f32,
    pub stopSimulation: bool,
    pub autodestruct: bool,
    pub colorAnimation: (ColorRGBA, ColorRGBA, ColorRGBA, ColorRGBA, ColorRGBA),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleEmitter {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleRenderer {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: (Option<u8>, Option<bool>),
    pub m_ReceiveShadows: (Option<u8>, Option<bool>),
    pub m_LightmapIndex: (Option<u8>, Option<u16>),
    pub m_LightmapTilingOffset: Vector4f,
    pub m_Materials: Vec<PPtr>,
    pub m_SubsetIndices: Option<Vec<u32>>,
    pub m_StaticBatchRoot: PPtr,
    pub m_CameraVelocityScale: f32,
    pub m_StretchParticles: i32,
    pub m_LengthScale: f32,
    pub m_VelocityScale: f32,
    pub m_MaxParticleSize: f32,
    pub UV_Animation: UVAnimation,
    pub m_UseLightProbes: Option<bool>,
    pub m_LightProbeAnchor: Option<PPtr>,
    pub m_SortingLayer: Option<i16>,
    pub m_SortingOrder: Option<i16>,
    pub m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    pub m_LightmapIndexDynamic: Option<u16>,
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    pub m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    pub m_ProbeAnchor: Option<PPtr>,
    pub m_MotionVectors: Option<u8>,
    pub m_LightProbeUsage: Option<u8>,
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    pub m_DynamicOccludee: Option<u8>,
    pub m_RenderingLayerMask: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleSystem {
    pub m_GameObject: PPtr,
    pub lengthInSec: f32,
    pub startDelay: (Option<f32>, Option<MinMaxCurve>),
    pub speed: Option<f32>,
    pub randomSeed: (Option<u32>, Option<i32>),
    pub looping: bool,
    pub prewarm: bool,
    pub playOnAwake: bool,
    pub moveWithTransform: (Option<bool>, Option<i32>),
    pub InitialModule: InitialModule,
    pub ShapeModule: ShapeModule,
    pub EmissionModule: EmissionModule,
    pub SizeModule: SizeModule,
    pub RotationModule: RotationModule,
    pub ColorModule: ColorModule,
    pub UVModule: UVModule,
    pub VelocityModule: VelocityModule,
    pub ForceModule: ForceModule,
    pub ClampVelocityModule: ClampVelocityModule,
    pub SizeBySpeedModule: SizeBySpeedModule,
    pub RotationBySpeedModule: RotationBySpeedModule,
    pub ColorBySpeedModule: ColorBySpeedModule,
    pub CollisionModule: CollisionModule,
    pub SubModule: SubModule,
    pub ExternalForcesModule: Option<ExternalForcesModule>,
    pub scalingMode: Option<i32>,
    pub InheritVelocityModule: Option<InheritVelocityModule>,
    pub TriggerModule: Option<TriggerModule>,
    pub autoRandomSeed: Option<bool>,
    pub moveWithCustomTransform: Option<PPtr>,
    pub NoiseModule: Option<NoiseModule>,
    pub LightsModule: Option<LightsModule>,
    pub TrailModule: Option<TrailModule>,
    pub simulationSpeed: Option<f32>,
    pub CustomDataModule: Option<CustomDataModule>,
    pub useUnscaledTime: Option<bool>,
    pub useRigidbodyForVelocity: Option<bool>,
    pub stopAction: Option<i32>,
    pub cullingMode: Option<i32>,
    pub ringBufferMode: Option<i32>,
    pub ringBufferLoopRange: Option<Vector2f>,
    pub LifetimeByEmitterSpeedModule: Option<LifetimeByEmitterSpeedModule>,
    pub emitterVelocityMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleSystemEmissionBurst {
    pub time: f32,
    pub minCount: Option<u32>,
    pub maxCount: Option<u32>,
    pub cycleCount: (Option<u32>, Option<i32>),
    pub repeatInterval: f32,
    pub countCurve: Option<MinMaxCurve>,
    pub probability: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleSystemForceField {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Parameters: ParticleSystemForceFieldParameters,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleSystemForceFieldParameters {
    pub m_Shape: i32,
    pub m_StartRange: f32,
    pub m_EndRange: f32,
    pub m_Length: f32,
    pub m_GravityFocus: f32,
    pub m_RotationRandomness: Vector2f,
    pub m_DirectionCurveX: MinMaxCurve,
    pub m_DirectionCurveY: MinMaxCurve,
    pub m_DirectionCurveZ: MinMaxCurve,
    pub m_GravityCurve: MinMaxCurve,
    pub m_RotationSpeedCurve: MinMaxCurve,
    pub m_RotationAttractionCurve: MinMaxCurve,
    pub m_DragCurve: MinMaxCurve,
    pub m_VectorField: PPtr,
    pub m_VectorFieldSpeedCurve: MinMaxCurve,
    pub m_VectorFieldAttractionCurve: MinMaxCurve,
    pub m_MultiplyDragByParticleSize: bool,
    pub m_MultiplyDragByParticleVelocity: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleSystemRenderer {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: (Option<u8>, Option<bool>),
    pub m_ReceiveShadows: (Option<u8>, Option<bool>),
    pub m_LightmapIndex: (Option<u8>, Option<u16>),
    pub m_LightmapTilingOffset: Vector4f,
    pub m_Materials: Vec<PPtr>,
    pub m_SubsetIndices: Option<Vec<u32>>,
    pub m_StaticBatchRoot: PPtr,
    pub m_UseLightProbes: Option<bool>,
    pub m_LightProbeAnchor: Option<PPtr>,
    pub m_RenderMode: (Option<u16>, Option<i32>),
    pub m_MaxParticleSize: f32,
    pub m_CameraVelocityScale: f32,
    pub m_VelocityScale: f32,
    pub m_LengthScale: f32,
    pub m_SortingFudge: f32,
    pub m_SortMode: (Option<u8>, Option<u16>, Option<i32>),
    pub m_Mesh: PPtr,
    pub m_NormalDirection: Option<f32>,
    pub m_Mesh1: Option<PPtr>,
    pub m_Mesh2: Option<PPtr>,
    pub m_Mesh3: Option<PPtr>,
    pub m_SortingLayer: Option<i16>,
    pub m_SortingOrder: Option<i16>,
    pub m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    pub m_LightmapIndexDynamic: Option<u16>,
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    pub m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    pub m_ProbeAnchor: Option<PPtr>,
    pub m_MinParticleSize: Option<f32>,
    pub m_RenderAlignment: Option<i32>,
    pub m_Pivot: Option<Vector3f>,
    pub m_MotionVectors: Option<u8>,
    pub m_LightProbeUsage: Option<u8>,
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    pub m_UseCustomVertexStreams: Option<bool>,
    pub m_VertexStreamMask: Option<i32>,
    pub m_VertexStreams: Option<Vec<u8>>,
    pub m_MaskInteraction: Option<i32>,
    pub m_DynamicOccludee: Option<u8>,
    pub m_RenderingLayerMask: Option<u32>,
    pub m_EnableGPUInstancing: Option<bool>,
    pub m_ApplyActiveColorSpace: Option<bool>,
    pub m_RendererPriority: Option<i32>,
    pub m_ShadowBias: Option<f32>,
    pub m_Flip: Option<Vector3f>,
    pub m_AllowRoll: Option<bool>,
    pub m_RayTracingMode: Option<u8>,
    pub m_FreeformStretching: Option<bool>,
    pub m_RotateWithStretchDirection: Option<bool>,
    pub m_RayTraceProcedural: Option<u8>,
    pub m_StaticShadowCaster: Option<u8>,
    pub m_MeshDistribution: Option<u8>,
    pub m_MeshWeighting: Option<f32>,
    pub m_MeshWeighting1: Option<f32>,
    pub m_MeshWeighting2: Option<f32>,
    pub m_MeshWeighting3: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerLODSettings {
    pub height: f32,
    pub castShadows: bool,
    pub receiveShadows: bool,
    pub useLightProbes: bool,
    pub reflectionProbeUsage: i32,
    pub enableBump: bool,
    pub enableHue: bool,
    pub windQuality: i32,
    pub enableSubsurface: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerformanceReportingManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerformanceReportingSettings {
    pub m_Enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicMaterial {
    pub m_Name: String,
    pub dynamicFriction: f32,
    pub staticFriction: f32,
    pub bounciness: f32,
    pub frictionCombine: i32,
    pub bounceCombine: i32,
    pub frictionDirection2: Option<Vector3f>,
    pub dynamicFriction2: Option<f32>,
    pub staticFriction2: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Physics2DSettings {
    pub m_Gravity: Vector2f,
    pub m_DefaultMaterial: PPtr,
    pub m_VelocityIterations: i32,
    pub m_PositionIterations: i32,
    pub m_RaycastsHitTriggers: Option<bool>,
    pub m_LayerCollisionMatrix: Vec<u32>,
    pub m_VelocityThreshold: Option<f32>,
    pub m_MaxLinearCorrection: Option<f32>,
    pub m_MaxAngularCorrection: Option<f32>,
    pub m_MaxTranslationSpeed: Option<f32>,
    pub m_MaxRotationSpeed: Option<f32>,
    pub m_BaumgarteScale: Option<f32>,
    pub m_BaumgarteTimeOfImpactScale: Option<f32>,
    pub m_TimeToSleep: Option<f32>,
    pub m_LinearSleepTolerance: Option<f32>,
    pub m_AngularSleepTolerance: Option<f32>,
    pub m_DeleteStopsCallbacks: Option<bool>,
    pub m_MinPenetrationForPenalty: Option<f32>,
    pub m_RaycastsStartInColliders: Option<bool>,
    pub m_ChangeStopsCallbacks: Option<bool>,
    pub m_QueriesHitTriggers: Option<bool>,
    pub m_QueriesStartInColliders: Option<bool>,
    pub m_DefaultContactOffset: Option<f32>,
    pub m_CallbacksOnDisable: Option<bool>,
    pub m_AutoSimulation: Option<bool>,
    pub m_AutoSyncTransforms: Option<bool>,
    pub m_JobOptions: Option<PhysicsJobOptions2D>,
    pub m_ReuseCollisionCallbacks: Option<bool>,
    pub m_SimulationMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsJobOptions2D {
    pub m_UseMultithreading: Option<bool>,
    pub m_UseConsistencySorting: Option<bool>,
    pub m_InterpolationPosesPerJob: i32,
    pub m_NewContactsPerJob: i32,
    pub m_CollideContactsPerJob: i32,
    pub m_ClearFlagsPerJob: i32,
    pub m_ClearBodyForcesPerJob: i32,
    pub m_SyncDiscreteFixturesPerJob: i32,
    pub m_SyncContinuousFixturesPerJob: i32,
    pub m_FindNearestContactsPerJob: i32,
    pub m_UpdateTriggerContactsPerJob: i32,
    pub m_IslandSolverCostThreshold: i32,
    pub m_IslandSolverBodyCostScale: i32,
    pub m_IslandSolverContactCostScale: i32,
    pub m_IslandSolverJointCostScale: i32,
    pub m_IslandSolverBodiesPerJob: i32,
    pub m_IslandSolverContactsPerJob: i32,
    pub useMultithreading: Option<bool>,
    pub useConsistencySorting: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsManager {
    pub m_Gravity: Vector3f,
    pub m_DefaultMaterial: PPtr,
    pub m_BounceThreshold: f32,
    pub m_SleepVelocity: Option<f32>,
    pub m_SleepAngularVelocity: Option<f32>,
    pub m_MaxAngularVelocity: Option<f32>,
    pub m_MinPenetrationForPenalty: Option<f32>,
    pub m_SolverIterationCount: Option<i32>,
    pub m_RaycastsHitTriggers: Option<bool>,
    pub m_LayerCollisionMatrix: Vec<u32>,
    pub m_SleepThreshold: Option<f32>,
    pub m_DefaultContactOffset: Option<f32>,
    pub m_EnableAdaptiveForce: Option<bool>,
    pub m_QueriesHitTriggers: Option<bool>,
    pub m_SolverVelocityIterations: Option<i32>,
    pub m_DefaultSolverIterations: Option<i32>,
    pub m_DefaultSolverVelocityIterations: Option<i32>,
    pub m_QueriesHitBackfaces: Option<bool>,
    pub m_EnablePCM: Option<bool>,
    pub m_AutoSimulation: Option<bool>,
    pub m_AutoSyncTransforms: Option<bool>,
    pub m_ClothInterCollisionDistance: Option<f32>,
    pub m_ClothInterCollisionStiffness: Option<f32>,
    pub m_ContactsGeneration: Option<i32>,
    pub m_ClothInterCollisionSettingsToggle: Option<bool>,
    pub m_ContactPairsMode: Option<i32>,
    pub m_BroadphaseType: Option<i32>,
    pub m_WorldBounds: Option<AABB>,
    pub m_WorldSubdivisions: Option<i32>,
    pub m_FrictionType: Option<i32>,
    pub m_EnableEnhancedDeterminism: Option<bool>,
    pub m_EnableUnifiedHeightmaps: Option<bool>,
    pub m_ReuseCollisionCallbacks: Option<bool>,
    pub m_DefaultMaxAngluarSpeed: Option<f32>,
    pub m_ClothGravity: Option<Vector3f>,
    pub m_DefaultMaxAngularSpeed: Option<f32>,
    pub m_SolverType: Option<i32>,
    pub m_DefaultMaxDepenetrationVelocity: Option<f32>,
    pub m_ImprovedPatchFriction: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsMaterial2D {
    pub m_Name: String,
    pub friction: f32,
    pub bounciness: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsShape {
    pub m_ShapeType: i32,
    pub m_Radius: f32,
    pub m_VertexStartIndex: i32,
    pub m_VertexCount: i32,
    pub m_UseAdjacentStart: i32,
    pub m_UseAdjacentEnd: i32,
    pub m_AdjacentStart: Vector2f,
    pub m_AdjacentEnd: Vector2f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsShapeGroup2D {
    pub m_Shapes: Vec<PhysicsShape>,
    pub m_Vertices: Vec<Vector2f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsUpdateBehaviour2D {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pipeline {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformEffector2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_ColliderMask: BitField,
    pub m_OneWay: Option<bool>,
    pub m_SideFriction: Option<bool>,
    pub m_SideBounce: Option<bool>,
    pub m_SideAngleVariance: Option<f32>,
    pub m_UseOneWay: Option<bool>,
    pub m_UseSideFriction: Option<bool>,
    pub m_UseSideBounce: Option<bool>,
    pub m_UseColliderMask: Option<bool>,
    pub m_SurfaceArc: Option<f32>,
    pub m_SideArc: Option<f32>,
    pub m_UseOneWayGrouping: Option<bool>,
    pub m_RotationalOffset: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformModuleSetup {
    pub modules: Vec<Module>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformSettings {
    pub m_BuildTarget: String,
    pub m_MaxTextureSize: i32,
    pub m_TextureFormat: i32,
    pub m_TextureCompression: i32,
    pub m_CompressionQuality: i32,
    pub m_CrunchedCompression: bool,
    pub m_AllowsAlphaSplitting: bool,
    pub m_Overridden: bool,
    pub m_ResizeAlgorithm: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformSettingsData {
    pub enabled: Option<bool>,
    pub settings: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformShaderDefines {
    pub shaderPlatform: i32,
    pub defines_Tier1: Vec<u32>,
    pub defines_Tier2: Vec<u32>,
    pub defines_Tier3: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformShaderSettings {
    pub useScreenSpaceShadows: Option<bool>,
    pub useCascadedShadowMaps: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerSettings {
    pub AndroidLicensePublicKey: Option<String>,
    pub defaultScreenOrientation: i32,
    pub targetDevice: i32,
    pub targetPlatform: Option<i32>,
    pub targetResolution: Option<i32>,
    pub Override_IPod_Music: Option<bool>,
    pub companyName: String,
    pub productName: String,
    pub defaultScreenWidth: i32,
    pub defaultScreenHeight: i32,
    pub defaultScreenWidthWeb: i32,
    pub defaultScreenHeightWeb: i32,
    pub m_RenderingPath: Option<i32>,
    pub displayResolutionDialog: Option<i32>,
    pub defaultIsFullScreen: Option<bool>,
    pub useAlphaInDashboard: Option<bool>,
    pub runInBackground: bool,
    pub captureSingleScreen: bool,
    pub alwaysDisplayWatermark: Option<bool>,
    pub m_SupportedAspectRatios: AspectRatios,
    pub firstStreamedLevelWithResources: Option<i32>,
    pub unityRebuildLibraryVersion: Option<i32>,
    pub unityForwardCompatibleVersion: Option<i32>,
    pub unityStandardAssetsVersion: Option<i32>,
    pub AndroidProfiler: Option<bool>,
    pub usePlayerLog: Option<bool>,
    pub allowedAutorotateToPortrait: Option<bool>,
    pub allowedAutorotateToPortraitUpsideDown: Option<bool>,
    pub allowedAutorotateToLandscapeRight: Option<bool>,
    pub allowedAutorotateToLandscapeLeft: Option<bool>,
    pub useOSAutorotation: Option<bool>,
    pub use32BitDisplayBuffer: Option<bool>,
    pub useMacAppStoreValidation: Option<bool>,
    pub xboxSkinOnGPU: Option<bool>,
    pub accelerometerFrequency: Option<i32>,
    pub m_ActiveColorSpace: Option<i32>,
    pub m_MTRendering: Option<bool>,
    pub iosShowActivityIndicatorOnLoading: Option<i32>,
    pub androidShowActivityIndicatorOnLoading: Option<i32>,
    pub use24BitDepthBuffer: Option<bool>,
    pub xboxEnableAvatar: Option<bool>,
    pub xboxEnableKinect: Option<bool>,
    pub xboxEnableKinectAutoTracking: Option<bool>,
    pub xboxEnableSpeech: Option<bool>,
    pub wiiHio2Usage: Option<i32>,
    pub wiiLoadingScreenRectPlacement: Option<i32>,
    pub wiiLoadingScreenBackground: Option<ColorRGBA>,
    pub wiiLoadingScreenPeriod: Option<i32>,
    pub wiiLoadingScreenFileName: Option<String>,
    pub wiiLoadingScreenRect: Option<Rectf>,
    pub debugUnloadMode: Option<i32>,
    pub targetGlesGraphics: Option<i32>,
    pub defaultCursor: Option<PPtr>,
    pub cursorHotspot: Option<Vector2f>,
    pub m_UseDX11: Option<bool>,
    pub stripPhysics: Option<bool>,
    pub resizableWindow: Option<bool>,
    pub xboxEnableFitness: Option<bool>,
    pub macFullscreenMode: Option<i32>,
    pub xboxSpeechDB: Option<u32>,
    pub enableHWStatistics: Option<bool>,
    pub xboxEnableHeadOrientation: Option<bool>,
    pub iPhoneBundleIdentifier: Option<String>,
    pub defaultIsNativeResolution: Option<bool>,
    pub Prepare_IOS_For_Recording: Option<bool>,
    pub forceSingleInstance: Option<bool>,
    pub gpuSkinning: Option<bool>,
    pub m_MobileRenderingPath: Option<i32>,
    pub m_MobileMTRendering: Option<bool>,
    pub xboxPIXTextureCapture: Option<bool>,
    pub xboxEnableGuest: Option<bool>,
    pub metroEnableIndependentInputSource: Option<bool>,
    pub metroEnableLowLatencyPresentationAPI: Option<bool>,
    pub m_Stereoscopic3D: Option<bool>,
    pub videoMemoryForVertexBuffers: Option<i32>,
    pub d3d9FullscreenMode: Option<i32>,
    pub visibleInBackground: Option<bool>,
    pub d3d11ForceExclusiveMode: Option<bool>,
    pub targetIOSGraphics: Option<i32>,
    pub cloudProjectId: Option<String>,
    pub iosAppInBackgroundBehavior: Option<i32>,
    pub disableDepthAndStencilBuffers: Option<bool>,
    pub submitAnalytics: Option<bool>,
    pub bakeCollisionMeshes: Option<bool>,
    pub d3d11FullscreenMode: Option<i32>,
    pub xboxOneResolution: Option<i32>,
    pub ps3SplashScreen: Option<PPtr>,
    pub psp2PowerMode: Option<i32>,
    pub psp2AcquireBGM: Option<bool>,
    pub bundleIdentifier: Option<String>,
    pub bundleVersion: Option<String>,
    pub preloadedAssets: Option<Vec<PPtr>>,
    pub xboxOneDisableKinectGpuReservation: Option<bool>,
    pub m_ShowUnitySplashScreen: Option<bool>,
    pub projectId: Option<String>,
    pub projectName: Option<String>,
    pub organizationId: Option<String>,
    pub cloudEnabled: Option<bool>,
    pub virtualRealitySupported: Option<bool>,
    pub n3dsDisableStereoscopicView: Option<bool>,
    pub n3dsEnableSharedListOpt: Option<bool>,
    pub n3dsEnableVSync: Option<bool>,
    pub wiiUTVResolution: Option<i32>,
    pub wiiUGamePadMSAA: Option<i32>,
    pub wiiUSupportsNunchuk: Option<bool>,
    pub wiiUSupportsClassicController: Option<bool>,
    pub wiiUSupportsBalanceBoard: Option<bool>,
    pub wiiUSupportsMotionPlus: Option<bool>,
    pub wiiUSupportsProController: Option<bool>,
    pub wiiUAllowScreenCapture: Option<bool>,
    pub wiiUControllerCount: Option<i32>,
    pub useOnDemandResources: Option<bool>,
    pub iosAllowHTTPDownload: Option<bool>,
    pub m_VirtualRealitySplashScreen: Option<PPtr>,
    pub allowFullscreenSwitch: Option<bool>,
    pub uiUse16BitDepthBuffer: Option<bool>,
    pub xboxEnablePIXSampling: Option<bool>,
    pub ignoreAlphaClear: Option<bool>,
    pub xboxEnableEnableRenderThreadRunsJobs: Option<bool>,
    pub xboxOneMonoLoggingLevel: Option<i32>,
    pub xboxOneLoggingLevel: Option<i32>,
    pub muteOtherAudioSources: Option<bool>,
    pub m_SplashScreenStyle: Option<i32>,
    pub m_StackTraceTypes: Option<Vec<i32>>,
    pub graphicsJobs: Option<bool>,
    pub singlePassStereoRendering: Option<bool>,
    pub protectGraphicsMemory: Option<bool>,
    pub productGUID: Option<GUID>,
    pub tizenShowActivityIndicatorOnLoading: Option<i32>,
    pub m_SplashScreenBackgroundColor: Option<ColorRGBA>,
    pub m_ShowUnitySplashLogo: Option<bool>,
    pub m_SplashScreenOverlayOpacity: Option<f32>,
    pub m_SplashScreenAnimation: Option<i32>,
    pub m_SplashScreenLogoStyle: Option<i32>,
    pub m_SplashScreenDrawMode: Option<i32>,
    pub m_SplashScreenBackgroundAnimationZoom: Option<f32>,
    pub m_SplashScreenLogoAnimationZoom: Option<f32>,
    pub m_SplashScreenBackgroundLandscapeAspect: Option<f32>,
    pub m_SplashScreenBackgroundPortraitAspect: Option<f32>,
    pub m_SplashScreenBackgroundLandscapeUvs: Option<Rectf>,
    pub m_SplashScreenBackgroundPortraitUvs: Option<Rectf>,
    pub m_SplashScreenLogos: Option<Vec<SplashScreenLogo>>,
    pub m_SplashScreenBackgroundLandscape: Option<PPtr>,
    pub m_SplashScreenBackgroundPortrait: Option<PPtr>,
    pub m_HolographicTrackingLossScreen: Option<PPtr>,
    pub m_StereoRenderingPath: Option<i32>,
    pub metroInputSource: Option<i32>,
    pub m_HolographicPauseOnTrackingLoss: Option<bool>,
    pub graphicsJobMode: Option<i32>,
    pub useHDRDisplay: Option<bool>,
    pub xboxOneSResolution: Option<i32>,
    pub xboxOneXResolution: Option<i32>,
    pub xboxOneEnable7thCore: Option<bool>,
    pub vrSettings: Option<VRSettings>,
    pub enableNewInputSystem: Option<bool>,
    pub macAppStoreCategory: Option<String>,
    pub deferSystemGesturesMode: Option<i32>,
    pub hideHomeButton: Option<bool>,
    pub Force_IOS_Speakers_When_Recording: Option<bool>,
    pub targetPixelDensity: Option<i32>,
    pub resolutionScalingMode: Option<i32>,
    pub enableNativePlatformBackendsForNewInputSystem: Option<bool>,
    pub disableOldInputManagerSupport: Option<bool>,
    pub xboxOneDisableEsram: Option<bool>,
    pub AndroidFilterTouchesWhenObscured: Option<bool>,
    pub mobileMTRenderingBaked: Option<bool>,
    pub androidBlitType: Option<i32>,
    pub macRetinaSupport: Option<bool>,
    pub metalFramebufferOnly: Option<bool>,
    pub xboxOnePresentImmediateThreshold: Option<u32>,
    pub m_ColorGamuts: Option<Vec<i32>>,
    pub androidSupportedAspectRatio: Option<i32>,
    pub androidMaxAspectRatio: Option<f32>,
    pub wsaTransparentSwapchain: Option<bool>,
    pub preserveFramebufferAlpha: Option<bool>,
    pub iosUseCustomAppBackgroundBehavior: Option<bool>,
    pub AndroidEnableSustainedPerformanceMode: Option<bool>,
    pub fullscreenMode: Option<i32>,
    pub switchQueueCommandMemory: Option<i32>,
    pub vulkanEnableSetSRGBWrite: Option<bool>,
    pub vulkanUseSWCommandBuffers: Option<bool>,
    pub isWsaHolographicRemotingEnabled: Option<bool>,
    pub androidStartInFullscreen: Option<bool>,
    pub androidRenderOutsideSafeArea: Option<bool>,
    pub enableFrameTimingStats: Option<bool>,
    pub framebufferDepthMemorylessMode: Option<i32>,
    pub legacyClampBlendShapeWeights: Option<bool>,
    pub switchQueueControlMemory: Option<i32>,
    pub switchQueueComputeMemory: Option<i32>,
    pub switchNVNShaderPoolsGranularity: Option<i32>,
    pub switchNVNDefaultPoolsGranularity: Option<i32>,
    pub switchNVNOtherPoolsGranularity: Option<i32>,
    pub xboxOneEnableTypeOptimization: Option<bool>,
    pub switchNVNMaxPublicTextureIDCount: Option<i32>,
    pub switchNVNMaxPublicSamplerIDCount: Option<i32>,
    pub useFlipModelSwapchain: Option<bool>,
    pub androidUseSwappy: Option<bool>,
    pub D3DHDRBitDepth: Option<i32>,
    pub vulkanNumSwapchainBuffers: Option<u32>,
    pub stadiaPresentMode: Option<i32>,
    pub stadiaTargetFramerate: Option<i32>,
    pub vulkanEnableLateAcquireNextImage: Option<bool>,
    pub androidResizableWindow: Option<bool>,
    pub androidDefaultWindowWidth: Option<i32>,
    pub androidDefaultWindowHeight: Option<i32>,
    pub androidMinimumWindowWidth: Option<i32>,
    pub androidMinimumWindowHeight: Option<i32>,
    pub androidFullscreenMode: Option<i32>,
    pub virtualTexturingSupportEnabled: Option<bool>,
    pub mipStripping: Option<bool>,
    pub numberOfMipsStripped: Option<i32>,
    pub vulkanEnablePreTransform: Option<bool>,
    pub AID: Option<Hash128>,
    pub qualitySettingsNames: Option<Vec<String>>,
    pub activeInputHandler: Option<i32>,
    pub playerMinOpenGLESVersion: Option<i32>,
    pub vulkanEnableCommandBufferRecycling: Option<bool>,
    pub resetResolutionOnWindowResize: Option<bool>,
    pub playerDataPath: Option<String>,
    pub forceSRGBBlit: Option<bool>,
    pub uploadClearedTextureDataAfterCreationFromScript: Option<bool>,
    pub enableOpenGLProfilerGPURecorders: Option<bool>,
    pub switchNVNGraphicsFirmwareMemory: Option<i32>,
    pub insecureHttpOption: Option<i32>,
    pub m_SpriteBatchVertexThreshold: Option<i32>,
    pub switchGpuScratchPoolGranularity: Option<i32>,
    pub switchAllowGpuScratchShrinking: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginBuildInfo {
    pub m_RuntimePlugins: Vec<String>,
    pub m_EditorPlugins: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginImportOutput {
    pub pluginType: Option<i32>,
    pub dllType: Option<i32>,
    pub scriptingRuntimeVersion: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PointEffector2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_ColliderMask: BitField,
    pub m_ForceMagnitude: f32,
    pub m_ForceVariation: f32,
    pub m_DistanceScale: f32,
    pub m_Drag: f32,
    pub m_AngularDrag: f32,
    pub m_ForceSource: (Option<u8>, Option<i32>),
    pub m_ForceTarget: (Option<u8>, Option<i32>),
    pub m_ForceMode: (Option<u8>, Option<i32>),
    pub m_UseColliderMask: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Polygon2D {
    pub m_Paths: Vec<Vec<Vector2f>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolygonCollider2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Material: PPtr,
    pub m_IsTrigger: bool,
    pub m_Poly: Option<Polygon2D>,
    pub m_UsedByEffector: Option<bool>,
    pub m_Offset: Option<Vector2f>,
    pub m_Points: Option<Polygon2D>,
    pub m_Density: Option<f32>,
    pub m_UsedByComposite: Option<bool>,
    pub m_AutoTiling: Option<bool>,
    pub m_SpriteTilingProperty: Option<SpriteTilingProperty>,
    pub m_UseDelaunayMesh: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolygonColliderBase2D {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PositionConstraint {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Weight: f32,
    pub m_TranslationAtRest: Vector3f,
    pub m_TranslationOffset: Vector3f,
    pub m_AffectTranslationX: bool,
    pub m_AffectTranslationY: bool,
    pub m_AffectTranslationZ: bool,
    pub m_IsContraintActive: Option<bool>,
    pub m_Sources: Vec<ConstraintSource>,
    pub m_Active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prefab {
    pub m_RootGameObject: PPtr,
    pub m_HideFlagsBehaviour: Option<i32>,
    pub m_ContainsMissingSerializeReferenceTypes: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrefabImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_AddedObjectFileIDs: Vec<i64>,
    pub m_IsPrefabVariant: bool,
    pub m_UnableToImportOnPreviousDomainReload: Option<bool>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrefabModification {
    pub m_TransformParent: PPtr,
    pub m_Modifications: Vec<PropertyModification>,
    pub m_RemovedComponents: (Option<Vec<PPtr>>, Option<Vec<PPtr>>),
    pub m_AddedGameObjects: Option<Vec<AddedGameObject>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreloadData {
    pub m_Name: String,
    pub m_Assets: Vec<PPtr>,
    pub m_Dependencies: Option<Vec<String>>,
    pub m_ExplicitDataLayout: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preset {
    pub m_Name: String,
    pub m_TargetType: PresetType,
    pub m_Properties: Vec<PropertyModification>,
    pub m_ExcludedProperties: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresetManager {
    pub m_DefaultList: Option<Vec<DefaultPresetList>>,
    pub m_DefaultPresets: Option<Vec<(PresetType, Vec<DefaultPreset>)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresetType {
    pub m_NativeTypeID: i32,
    pub m_ManagedTypePPtr: PPtr,
    pub m_ManagedTypeFallback: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct PreviewAnimationClip {
//     pub m_Name: String,
//     pub m_Legacy: bool,
//     pub m_Compressed: bool,
//     pub m_UseHighQualityCurve: bool,
//     pub m_RotationCurves: Vec<QuaternionCurve>,
//     pub m_CompressedRotationCurves: Vec<CompressedAnimationCurve>,
//     pub m_EulerCurves: Vec<Vector3Curve>,
//     pub m_PositionCurves: Vec<Vector3Curve>,
//     pub m_ScaleCurves: Vec<Vector3Curve>,
//     pub m_FloatCurves: Vec<FloatCurve>,
//     pub m_PPtrCurves: Vec<PPtrCurve>,
//     pub m_SampleRate: f32,
//     pub m_WrapMode: i32,
//     pub m_Bounds: AABB,
//     pub m_MuscleClipSize: u32,
//     pub m_MuscleClip: ClipMuscleConstant,
//     pub m_ClipBindingConstant: AnimationClipBindingConstant,
//     pub m_Events: Vec<AnimationEvent>,
//     pub m_HasGenericRootTransform: Option<bool>,
//     pub m_HasMotionFloatCurves: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreviewData {
    pub m_PreviewData: Vec<f32>,
    pub m_OrigSize: i32,
    pub m_CompSize: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreviewImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProbeSetIndex {
    pub m_Hash: Hash128,
    pub m_Offset: i32,
    pub m_Size: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProbeSetTetrahedralization {
    pub m_Tetrahedra: Vec<Tetrahedron>,
    pub m_HullRays: Vec<Vector3f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProceduralMaterial {
    pub m_Name: String,
    pub m_Shader: PPtr,
    pub m_SavedProperties: UnityPropertySheet,
    pub m_SubstancePackage: Option<PPtr>,
    pub m_Width: Option<i32>,
    pub m_Height: Option<i32>,
    pub m_Textures: Option<Vec<PPtr>>,
    pub m_Inputs: Option<Vec<SubstanceInput>>,
    pub m_Flags: Option<u32>,
    pub m_AnimationUpdateRate: Option<i32>,
    pub m_MaximumSize: Option<i32>,
    pub m_CacheSize: Option<i32>,
    pub m_LoadingBehavior: Option<i32>,
    pub m_ShaderKeywords: Option<(Option<Vec<String>>, Option<String>)>,
    pub m_Hash: Option<Hash128>,
    pub m_PrototypeName: Option<String>,
    pub m_CustomRenderQueue: Option<i32>,
    pub m_GenerateMipmaps: Option<bool>,
    pub m_LightmapFlags: Option<u32>,
    pub stringTagMap: Option<Vec<(String, String)>>,
    pub m_EnableInstancingVariants: Option<bool>,
    pub disabledShaderPasses: Option<Vec<String>>,
    pub m_DoubleSidedGI: Option<bool>,
    pub m_BuildTextureStacks: Option<Vec<BuildTextureStackReference>>,
    pub m_ValidKeywords: Option<Vec<String>>,
    pub m_InvalidKeywords: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProceduralMaterialInformation {
    pub m_Offset: Vector2f,
    pub m_Scale: Vector2f,
    pub m_GeneratedAtLoading: Option<i32>,
    pub m_GenerateAllOutputs: Option<i32>,
    pub m_AnimationUpdateRate: Option<i32>,
    pub m_GenerateMipmaps: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProceduralTexture {
    pub m_Name: String,
    pub m_SubstanceMaterial: Option<PPtr>,
    pub m_SubstanceTextureUID: Option<u64>,
    pub Type: Option<i32>,
    pub AlphaSource: Option<i32>,
    pub Format: Option<i32>,
    pub m_TextureParameters: Option<TextureParameters>,
    pub m_TextureSettings: Option<GLTextureSettings>,
    pub m_BakedData: Option<Vec<u8>>,
    pub m_BakedParameters: Option<TextureParameters>,
    pub m_LightmapFormat: Option<i32>,
    pub m_ColorSpace: Option<i32>,
    pub m_AlphaSourceUID: Option<u64>,
    pub AlphaSourceIsGrayscale: Option<bool>,
    pub m_Mipmaps: Option<i32>,
    pub m_AlphaSourceIsInverted: Option<bool>,
    pub m_ForcedFallbackFormat: Option<i32>,
    pub m_DownscaleFallback: Option<bool>,
    pub m_IsAlphaChannelOptional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProceduralTextureAssignment {
    pub shaderProp: (Option<String>, Option<FastPropertyName>),
    pub material: PPtr,
    pub baseUID: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Projector {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_NearClipPlane: f32,
    pub m_FarClipPlane: f32,
    pub m_FieldOfView: f32,
    pub m_AspectRatio: f32,
    pub m_Orthographic: bool,
    pub m_OrthographicSize: f32,
    pub m_Material: PPtr,
    pub m_IgnoreLayers: BitField,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropertyModification {
    pub target: PPtr,
    pub propertyPath: String,
    pub value: String,
    pub objectReference: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropertyModificationsTargetTestNativeObject {
    pub m_IntegerValue: i32,
    pub m_FloatValue: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropertyModificationsTargetTestObject {
    pub m_Data: PropertyModificationsTargetTestNativeObject,
    pub m_Array: Vec<PropertyModificationsTargetTestNativeObject>,
    pub m_FloatTestValue: f32,
    pub m_Bytes: Option<Vec<u8>>,
    pub m_Floats: Option<Vec<f32>>,
    pub m_BytesSize: Option<u32>,
    pub byte_data: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualitySetting {
    pub pixelLightCount: i32,
    pub shadows: i32,
    pub shadowResolution: i32,
    pub shadowCascades: i32,
    pub shadowDistance: f32,
    pub blendWeights: Option<i32>,
    pub textureQuality: i32,
    pub anisotropicTextures: i32,
    pub antiAliasing: i32,
    pub softParticles: bool,
    pub softVegetation: bool,
    pub syncToVBL: Option<bool>,
    pub shadowProjection: Option<i32>,
    pub vSyncCount: Option<i32>,
    pub name: Option<String>,
    pub lodBias: Option<f32>,
    pub maximumLODLevel: Option<i32>,
    pub particleRaycastBudget: Option<i32>,
    pub shadowCascade2Split: Option<f32>,
    pub shadowCascade4Split: Option<Vector3f>,
    pub realtimeReflectionProbes: Option<bool>,
    pub billboardsFaceCameraPosition: Option<bool>,
    pub shadowNearPlaneOffset: Option<f32>,
    pub asyncUploadTimeSlice: Option<i32>,
    pub asyncUploadBufferSize: Option<i32>,
    pub shadowmaskMode: Option<i32>,
    pub resolutionScalingFixedDPIFactor: Option<f32>,
    pub streamingMipmapsActive: Option<bool>,
    pub streamingMipmapsAddAllCameras: Option<bool>,
    pub streamingMipmapsMemoryBudget: Option<f32>,
    pub streamingMipmapsRenderersPerFrame: Option<i32>,
    pub streamingMipmapsMaxLevelReduction: Option<i32>,
    pub streamingMipmapsMaxFileIORequests: Option<i32>,
    pub asyncUploadPersistentBuffer: Option<bool>,
    pub skinWeights: Option<i32>,
    pub customRenderPipeline: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualitySettings {
    pub m_DefaultStandaloneQuality: Option<i32>,
    pub m_DefaultWebPlayerQuality: Option<i32>,
    pub m_EditorQuality: Option<i32>,
    pub Fastest: Option<QualitySetting>,
    pub Fast: Option<QualitySetting>,
    pub Simple: Option<QualitySetting>,
    pub Good: Option<QualitySetting>,
    pub Beautiful: Option<QualitySetting>,
    pub Fantastic: Option<QualitySetting>,
    pub m_DefaultMobileQuality: Option<i32>,
    pub m_CurrentQuality: Option<i32>,
    pub m_QualitySettings: Option<Vec<QualitySetting>>,
    pub m_StrippedMaximumLODLevel: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuaternionCurve {
    pub curve: AnimationCurve,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quaternionf {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShader {
    pub m_Name: String,
    pub variants: Vec<RayTracingShaderVariant>,
    pub m_MaxRecursionDepth: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderBuiltinSampler {
    pub sampler: u32,
    pub bindPoint: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderConstantBuffer {
    pub name: String,
    pub byteSize: i32,
    pub params: Vec<RayTracingShaderParam>,
    pub hash: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderFunctionDesc {
    pub identifier: RayTracingShaderID,
    pub payloadSizeInBytes: u32,
    pub attributeSizeInBytes: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderID {
    pub __type: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_CurrentAPIMask: Option<u32>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderParam {
    pub name: String,
    pub __type: Option<i32>,
    pub offset: (Option<u32>, Option<i32>),
    pub arraySize: (Option<u32>, Option<i32>),
    pub rowCount: (Option<u8>, Option<i32>),
    pub colCount: (Option<u8>, Option<i32>),
    pub dataType: Option<i32>,
    pub dataSize: Option<u32>,
    pub propertySheetType: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderReflectionData {
    pub functions: Vec<RayTracingShaderFunctionDesc>,
    pub localResources: RayTracingShaderResources,
    pub globalResources: RayTracingShaderResources,
    pub hasErrors: bool,
    pub code: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderResource {
    pub name: String,
    pub bindPoint: i32,
    pub samplerBindPoint: i32,
    pub texDimension: i32,
    pub rayGenMask: u64,
    pub arraySize: Option<i32>,
    pub multisampled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderResources {
    pub textures: Vec<RayTracingShaderResource>,
    pub builtinSamplers: Vec<RayTracingShaderBuiltinSampler>,
    pub inputBuffers: Vec<RayTracingShaderResource>,
    pub outputBuffers: Vec<RayTracingShaderResource>,
    pub constantBuffersDesc: Vec<RayTracingShaderConstantBuffer>,
    pub constantBuffers: Vec<RayTracingShaderResource>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RayTracingShaderVariant {
    pub targetRenderer: i32,
    pub resourceReflectionData: RayTracingShaderReflectionData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaycastCollider {
    pub m_GameObject: PPtr,
    pub m_Material: PPtr,
    pub m_IsTrigger: bool,
    pub m_Center: Vector3f,
    pub m_Length: f32,
    pub m_Enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RectTransform {
    pub m_GameObject: PPtr,
    pub m_AnchorMin: Vector2f,
    pub m_AnchorMax: Vector2f,
    pub m_Position: Option<Vector2f>,
    pub m_SizeDelta: Vector2f,
    pub m_Pivot: Vector2f,
    pub m_LocalRotation: Option<Quaternionf>,
    pub m_LocalPosition: Option<Vector3f>,
    pub m_LocalScale: Option<Vector3f>,
    pub m_Children: Option<Vec<PPtr>>,
    pub m_Father: Option<PPtr>,
    pub m_AnchoredPosition: Option<Vector2f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rectf {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReferencesArtifactGenerator {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReflectionProbe {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Type: i32,
    pub m_Mode: i32,
    pub m_RefreshMode: i32,
    pub m_TimeSlicingMode: i32,
    pub m_Resolution: i32,
    pub m_UpdateFrequency: i32,
    pub m_Importance: (Option<i16>, Option<i32>),
    pub m_BoxSize: Vector3f,
    pub m_BoxOffset: Vector3f,
    pub m_NearClip: f32,
    pub m_FarClip: f32,
    pub m_ShadowDistance: f32,
    pub m_ClearFlags: u32,
    pub m_BackGroundColor: ColorRGBA,
    pub m_CullingMask: BitField,
    pub m_IntensityMultiplier: f32,
    pub m_HDR: bool,
    pub m_BoxProjection: bool,
    pub m_RenderDynamicObjects: bool,
    pub m_UseOcclusionCulling: bool,
    pub m_CustomBakedTexture: PPtr,
    pub m_BakedTexture: PPtr,
    pub m_BlendDistance: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelativeJoint2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_EnableCollision: bool,
    pub m_ConnectedRigidBody: PPtr,
    pub m_BreakForce: f32,
    pub m_BreakTorque: f32,
    pub m_MaxForce: f32,
    pub m_MaxTorque: f32,
    pub m_CorrectionScale: f32,
    pub m_AutoConfigureOffset: bool,
    pub m_LinearOffset: Vector2f,
    pub m_AngularOffset: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderManager {
    pub m_AlwaysIncludedShaders: Option<Vec<PPtr>>,
    pub m_Deferred: Option<BuiltinShaderSettings>,
    pub m_LegacyDeferred: Option<BuiltinShaderSettings>,
    pub m_PreloadedShaders: Option<Vec<PPtr>>,
    pub m_DeferredReflections: Option<BuiltinShaderSettings>,
    pub m_ShaderSettings: Option<PlatformShaderSettings>,
    pub m_ScreenSpaceShadows: Option<BuiltinShaderSettings>,
    pub m_DepthNormals: Option<BuiltinShaderSettings>,
    pub m_MotionVectors: Option<BuiltinShaderSettings>,
    pub m_LightHalo: Option<BuiltinShaderSettings>,
    pub m_LensFlare: Option<BuiltinShaderSettings>,
    pub m_ShaderSettings_Tier1: Option<PlatformShaderSettings>,
    pub m_ShaderSettings_Tier2: Option<PlatformShaderSettings>,
    pub m_ShaderSettings_Tier3: Option<PlatformShaderSettings>,
    pub m_SpritesDefaultMaterial: Option<PPtr>,
    pub m_TierSettings_Tier1: Option<TierGraphicsSettings>,
    pub m_TierSettings_Tier2: Option<TierGraphicsSettings>,
    pub m_TierSettings_Tier3: Option<TierGraphicsSettings>,
    pub m_TransparencySortMode: Option<i32>,
    pub m_TransparencySortAxis: Option<Vector3f>,
    pub m_LightsUseLinearIntensity: Option<bool>,
    pub m_LightsUseCCT: Option<bool>,
    pub m_CustomRenderPipeline: Option<PPtr>,
    pub m_LightsUseColorTemperature: Option<bool>,
    pub m_ShaderDefinesPerShaderCompiler: Option<Vec<PlatformShaderDefines>>,
    pub m_LogWhenShaderIsCompiled: Option<bool>,
    pub m_AllowEnlightenSupportForUpgradedProject: Option<bool>,
    pub m_VideoShadersIncludeMode: Option<i32>,
    pub m_DefaultRenderingLayerMask: Option<u32>,
    pub m_SRPDefaultSettings: Option<Vec<(String, PPtr)>>,
    pub m_PreloadShadersBatchTimeLimit: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderPassAttachment {}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct RenderSettings {
//     pub m_Fog: bool,
//     pub m_FogColor: ColorRGBA,
//     pub m_FogDensity: f32,
//     pub m_AmbientLight: Option<ColorRGBA>,
//     pub m_SkyboxMaterial: PPtr,
//     pub m_HaloStrength: f32,
//     pub m_FlareStrength: f32,
//     pub m_HaloTexture: PPtr,
//     pub m_SpotCookie: PPtr,
//     pub m_FogMode: Option<i32>,
//     pub m_LinearFogStart: Option<f32>,
//     pub m_LinearFogEnd: Option<f32>,
//     pub m_FlareFadeSpeed: Option<f32>,
//     pub m_AmbientSkyColor: Option<ColorRGBA>,
//     pub m_AmbientEquatorColor: Option<ColorRGBA>,
//     pub m_AmbientGroundColor: Option<ColorRGBA>,
//     pub m_AmbientIntensity: Option<f32>,
//     pub m_AmbientMode: Option<i32>,
//     pub m_DefaultReflectionMode: Option<i32>,
//     pub m_DefaultReflectionResolution: Option<i32>,
//     pub m_ReflectionBounces: Option<i32>,
//     pub m_ReflectionIntensity: Option<f32>,
//     pub m_CustomReflection: Option<(Option<PPtr>, Option<PPtr>)>,
//     pub m_AmbientProbe: Option<SphericalHarmonicsL2>,
//     pub m_AmbientProbeInGamma: Option<SphericalHarmonicsL2>,
//     pub m_GeneratedSkyboxReflection: Option<PPtr>,
//     pub m_Sun: Option<PPtr>,
//     pub m_IndirectSpecularColor: Option<ColorRGBA>,
//     pub m_SubtractiveShadowColor: Option<ColorRGBA>,
//     pub m_UseRadianceAmbientProbe: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderTexture {
    pub m_Name: String,
    pub m_Width: i32,
    pub m_Height: i32,
    pub m_DepthFormat: Option<i32>,
    pub m_ColorFormat: i32,
    pub m_IsPowerOfTwo: Option<bool>,
    pub m_IsCubemap: Option<bool>,
    pub m_MipMap: bool,
    pub m_TextureSettings: GLTextureSettings,
    pub m_SRGB: Option<bool>,
    pub m_AntiAliasing: Option<i32>,
    pub m_GenerateMips: Option<bool>,
    pub m_Dimension: Option<i32>,
    pub m_VolumeDepth: Option<i32>,
    pub m_ForcedFallbackFormat: Option<i32>,
    pub m_DownscaleFallback: Option<bool>,
    pub m_UseDynamicScale: Option<bool>,
    pub m_BindMS: Option<bool>,
    pub m_EnableCompatibleFormat: Option<bool>,
    pub m_MipCount: Option<i32>,
    pub m_IsAlphaChannelOptional: Option<bool>,
    pub m_DepthStencilFormat: Option<i32>,
    pub m_ShadowSamplingMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Renderer {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RendererData {
    pub uvMesh: PPtr,
    pub terrainDynamicUVST: Vector4f,
    pub terrainChunkDynamicUVST: Vector4f,
    pub lightmapIndex: u16,
    pub lightmapIndexDynamic: u16,
    pub lightmapST: Vector4f,
    pub lightmapSTDynamic: Vector4f,
    pub explicitProbeSetHash: Option<Hash128>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RendererFake {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: u8,
    pub m_ReceiveShadows: u8,
    pub m_DynamicOccludee: u8,
    pub m_MotionVectors: u8,
    pub m_LightProbeUsage: u8,
    pub m_ReflectionProbeUsage: u8,
    pub m_RenderingLayerMask: u32,
    pub m_RendererPriority: i32,
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    pub m_Materials: Vec<PPtr>,
    pub m_StaticBatchInfo: StaticBatchInfo,
    pub m_StaticBatchRoot: PPtr,
    pub m_ProbeAnchor: PPtr,
    pub m_LightProbeVolumeOverride: PPtr,
    pub m_SortingLayerID: i32,
    pub m_SortingLayer: i16,
    pub m_SortingOrder: i16,
    pub m_Positions: Vec<Vector3f>,
    pub m_Parameters: LineParameters,
    pub m_UseWorldSpace: bool,
    pub m_Loop: bool,
    pub m_RayTracingMode: Option<u8>,
    pub m_RayTraceProcedural: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceManager {
    pub m_Container: Vec<(String, PPtr)>,
    pub m_DependentAssets: Option<Vec<ResourceManager_Dependency>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceManager_Dependency {
    pub m_Object: PPtr,
    pub m_Dependencies: Vec<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rigidbody {
    pub m_GameObject: PPtr,
    pub m_Mass: f32,
    pub m_Drag: f32,
    pub m_AngularDrag: f32,
    pub m_UseGravity: bool,
    pub m_IsKinematic: bool,
    pub m_Interpolate: u8,
    pub m_FreezeRotation: Option<bool>,
    pub m_CollisionDetection: i32,
    pub m_Constraints: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rigidbody2D {
    pub m_GameObject: PPtr,
    pub m_Mass: f32,
    pub m_LinearDrag: f32,
    pub m_AngularDrag: f32,
    pub m_GravityScale: f32,
    pub m_FixedAngle: Option<bool>,
    pub m_IsKinematic: Option<bool>,
    pub m_Interpolate: (Option<u8>, Option<i32>),
    pub m_SleepingMode: (Option<u8>, Option<i32>),
    pub m_CollisionDetection: (Option<u8>, Option<i32>),
    pub m_Constraints: Option<i32>,
    pub m_UseAutoMass: Option<bool>,
    pub m_BodyType: Option<i32>,
    pub m_Simulated: Option<bool>,
    pub m_UseFullKinematicContacts: Option<bool>,
    pub m_Material: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RootMotionData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoslynAdditionalFileAsset {
    pub m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoslynAdditionalFileImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoslynAnalyzerConfigAsset {
    pub m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoslynAnalyzerConfigImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotationBySpeedModule {
    pub enabled: bool,
    pub curve: MinMaxCurve,
    pub range: Vector2f,
    pub x: Option<MinMaxCurve>,
    pub y: Option<MinMaxCurve>,
    pub separateAxes: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotationConstraint {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Weight: f32,
    pub m_RotationAtRest: Vector3f,
    pub m_RotationOffset: Vector3f,
    pub m_AffectRotationX: bool,
    pub m_AffectRotationY: bool,
    pub m_AffectRotationZ: bool,
    pub m_IsContraintActive: Option<bool>,
    pub m_Sources: Vec<ConstraintSource>,
    pub m_Active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotationModule {
    pub enabled: bool,
    pub curve: MinMaxCurve,
    pub x: Option<MinMaxCurve>,
    pub y: Option<MinMaxCurve>,
    pub separateAxes: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleSetFileAsset {
    pub m_Name: String,
    pub m_Script: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleSetFileImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct RuntimeAnimatorController {
//     pub m_Name: String,
//     pub m_ControllerSize: u32,
//     pub m_Controller: ControllerConstant,
//     pub m_TOS: Vec<(u32, String)>,
//     pub m_AnimationClips: Vec<PPtr>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuntimeInitializeOnLoadManager {
    pub m_AssemblyNames: Option<Vec<String>>,
    pub m_NamespaceNames: Option<Vec<String>>,
    pub m_ClassInfos: Option<Vec<ClassInfo>>,
    pub m_ClassMethodInfos: Option<Vec<ClassMethodInfo>>,
    pub m_UnityMethodExecutionOrders: Option<Vec<i32>>,
    pub m_MethodExecutionOrders: Option<Vec<i32>>,
    pub m_BeforeUnityMethodExecutionOrders: Option<Vec<i32>>,
    pub m_AfterUnityMethodExecutionOrders: Option<Vec<i32>>,
    pub m_BeforeMethodExecutionOrders: Option<Vec<i32>>,
    pub m_AfterMethodExecutionOrders: Option<Vec<i32>>,
    pub m_AfterAssembliesLoadedUnityMethodExecutionOrders: Option<Vec<i32>>,
    pub m_AfterAssembliesLoadedMethodExecutionOrders: Option<Vec<i32>>,
    pub m_BeforeSplashScreenUnityMethodExecutionOrders: Option<Vec<i32>>,
    pub m_BeforeSplashScreenMethodExecutionOrders: Option<Vec<i32>>,
    pub m_SubsystemRegistrationUnityMethodExecutionOrders: Option<Vec<i32>>,
    pub m_SubsystemRegistrationMethodExecutionOrders: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SBranchWindLevel {
    pub m_afDistance_0: f32,
    pub m_afDistance_1: f32,
    pub m_afDistance_2: f32,
    pub m_afDistance_3: f32,
    pub m_afDistance_4: f32,
    pub m_afDistance_5: f32,
    pub m_afDistance_6: f32,
    pub m_afDistance_7: f32,
    pub m_afDistance_8: f32,
    pub m_afDistance_9: f32,
    pub m_afDirectionAdherence_0: f32,
    pub m_afDirectionAdherence_1: f32,
    pub m_afDirectionAdherence_2: f32,
    pub m_afDirectionAdherence_3: f32,
    pub m_afDirectionAdherence_4: f32,
    pub m_afDirectionAdherence_5: f32,
    pub m_afDirectionAdherence_6: f32,
    pub m_afDirectionAdherence_7: f32,
    pub m_afDirectionAdherence_8: f32,
    pub m_afDirectionAdherence_9: f32,
    pub m_afWhip_0: f32,
    pub m_afWhip_1: f32,
    pub m_afWhip_2: f32,
    pub m_afWhip_3: f32,
    pub m_afWhip_4: f32,
    pub m_afWhip_5: f32,
    pub m_afWhip_6: f32,
    pub m_afWhip_7: f32,
    pub m_afWhip_8: f32,
    pub m_afWhip_9: f32,
    pub m_fTurbulence: f32,
    pub m_fTwitch: f32,
    pub m_fTwitchFreqScale: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SParams {
    pub m_fStrengthResponse: f32,
    pub m_fDirectionResponse: f32,
    pub m_fAnchorOffset: f32,
    pub m_fAnchorDistanceScale: f32,
    pub Oscillation0_0: f32,
    pub Oscillation0_1: f32,
    pub Oscillation0_2: f32,
    pub Oscillation0_3: f32,
    pub Oscillation0_4: f32,
    pub Oscillation0_5: f32,
    pub Oscillation0_6: f32,
    pub Oscillation0_7: f32,
    pub Oscillation0_8: f32,
    pub Oscillation0_9: f32,
    pub Oscillation1_0: f32,
    pub Oscillation1_1: f32,
    pub Oscillation1_2: f32,
    pub Oscillation1_3: f32,
    pub Oscillation1_4: f32,
    pub Oscillation1_5: f32,
    pub Oscillation1_6: f32,
    pub Oscillation1_7: f32,
    pub Oscillation1_8: f32,
    pub Oscillation1_9: f32,
    pub Oscillation2_0: f32,
    pub Oscillation2_1: f32,
    pub Oscillation2_2: f32,
    pub Oscillation2_3: f32,
    pub Oscillation2_4: f32,
    pub Oscillation2_5: f32,
    pub Oscillation2_6: f32,
    pub Oscillation2_7: f32,
    pub Oscillation2_8: f32,
    pub Oscillation2_9: f32,
    pub Oscillation3_0: f32,
    pub Oscillation3_1: f32,
    pub Oscillation3_2: f32,
    pub Oscillation3_3: f32,
    pub Oscillation3_4: f32,
    pub Oscillation3_5: f32,
    pub Oscillation3_6: f32,
    pub Oscillation3_7: f32,
    pub Oscillation3_8: f32,
    pub Oscillation3_9: f32,
    pub Oscillation4_0: f32,
    pub Oscillation4_1: f32,
    pub Oscillation4_2: f32,
    pub Oscillation4_3: f32,
    pub Oscillation4_4: f32,
    pub Oscillation4_5: f32,
    pub Oscillation4_6: f32,
    pub Oscillation4_7: f32,
    pub Oscillation4_8: f32,
    pub Oscillation4_9: f32,
    pub Oscillation5_0: f32,
    pub Oscillation5_1: f32,
    pub Oscillation5_2: f32,
    pub Oscillation5_3: f32,
    pub Oscillation5_4: f32,
    pub Oscillation5_5: f32,
    pub Oscillation5_6: f32,
    pub Oscillation5_7: f32,
    pub Oscillation5_8: f32,
    pub Oscillation5_9: f32,
    pub Oscillation6_0: f32,
    pub Oscillation6_1: f32,
    pub Oscillation6_2: f32,
    pub Oscillation6_3: f32,
    pub Oscillation6_4: f32,
    pub Oscillation6_5: f32,
    pub Oscillation6_6: f32,
    pub Oscillation6_7: f32,
    pub Oscillation6_8: f32,
    pub Oscillation6_9: f32,
    pub Oscillation7_0: f32,
    pub Oscillation7_1: f32,
    pub Oscillation7_2: f32,
    pub Oscillation7_3: f32,
    pub Oscillation7_4: f32,
    pub Oscillation7_5: f32,
    pub Oscillation7_6: f32,
    pub Oscillation7_7: f32,
    pub Oscillation7_8: f32,
    pub Oscillation7_9: f32,
    pub Oscillation8_0: f32,
    pub Oscillation8_1: f32,
    pub Oscillation8_2: f32,
    pub Oscillation8_3: f32,
    pub Oscillation8_4: f32,
    pub Oscillation8_5: f32,
    pub Oscillation8_6: f32,
    pub Oscillation8_7: f32,
    pub Oscillation8_8: f32,
    pub Oscillation8_9: f32,
    pub Oscillation9_0: f32,
    pub Oscillation9_1: f32,
    pub Oscillation9_2: f32,
    pub Oscillation9_3: f32,
    pub Oscillation9_4: f32,
    pub Oscillation9_5: f32,
    pub Oscillation9_6: f32,
    pub Oscillation9_7: f32,
    pub Oscillation9_8: f32,
    pub Oscillation9_9: f32,
    pub m_fGlobalHeight: f32,
    pub m_fGlobalHeightExponent: f32,
    pub m_afGlobalDistance_0: f32,
    pub m_afGlobalDistance_1: f32,
    pub m_afGlobalDistance_2: f32,
    pub m_afGlobalDistance_3: f32,
    pub m_afGlobalDistance_4: f32,
    pub m_afGlobalDistance_5: f32,
    pub m_afGlobalDistance_6: f32,
    pub m_afGlobalDistance_7: f32,
    pub m_afGlobalDistance_8: f32,
    pub m_afGlobalDistance_9: f32,
    pub m_afGlobalDirectionAdherence_0: f32,
    pub m_afGlobalDirectionAdherence_1: f32,
    pub m_afGlobalDirectionAdherence_2: f32,
    pub m_afGlobalDirectionAdherence_3: f32,
    pub m_afGlobalDirectionAdherence_4: f32,
    pub m_afGlobalDirectionAdherence_5: f32,
    pub m_afGlobalDirectionAdherence_6: f32,
    pub m_afGlobalDirectionAdherence_7: f32,
    pub m_afGlobalDirectionAdherence_8: f32,
    pub m_afGlobalDirectionAdherence_9: f32,
    pub BranchLevel1: SBranchWindLevel,
    pub BranchLevel2: SBranchWindLevel,
    pub LeafGroup1: SWindGroup,
    pub LeafGroup2: SWindGroup,
    pub m_afFrondRippleDistance_0: f32,
    pub m_afFrondRippleDistance_1: f32,
    pub m_afFrondRippleDistance_2: f32,
    pub m_afFrondRippleDistance_3: f32,
    pub m_afFrondRippleDistance_4: f32,
    pub m_afFrondRippleDistance_5: f32,
    pub m_afFrondRippleDistance_6: f32,
    pub m_afFrondRippleDistance_7: f32,
    pub m_afFrondRippleDistance_8: f32,
    pub m_afFrondRippleDistance_9: f32,
    pub m_fFrondRippleTile: f32,
    pub m_fFrondRippleLightingScalar: f32,
    pub m_fRollingNoiseSize: f32,
    pub m_fRollingNoiseTwist: f32,
    pub m_fRollingNoiseTurbulence: f32,
    pub m_fRollingNoisePeriod: f32,
    pub m_fRollingNoiseSpeed: f32,
    pub m_fRollingBranchFieldMin: f32,
    pub m_fRollingBranchLightingAdjust: f32,
    pub m_fRollingBranchVerticalOffset: f32,
    pub m_fRollingLeafRippleMin: f32,
    pub m_fRollingLeafTumbleMin: f32,
    pub m_fGustFrequency: f32,
    pub m_fGustStrengthMin: f32,
    pub m_fGustStrengthMax: f32,
    pub m_fGustDurationMin: f32,
    pub m_fGustDurationMax: f32,
    pub m_fGustRiseScalar: f32,
    pub m_fGustFallScalar: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SWindGroup {
    pub m_afRippleDistance_0: f32,
    pub m_afRippleDistance_1: f32,
    pub m_afRippleDistance_2: f32,
    pub m_afRippleDistance_3: f32,
    pub m_afRippleDistance_4: f32,
    pub m_afRippleDistance_5: f32,
    pub m_afRippleDistance_6: f32,
    pub m_afRippleDistance_7: f32,
    pub m_afRippleDistance_8: f32,
    pub m_afRippleDistance_9: f32,
    pub m_afTumbleFlip_0: f32,
    pub m_afTumbleFlip_1: f32,
    pub m_afTumbleFlip_2: f32,
    pub m_afTumbleFlip_3: f32,
    pub m_afTumbleFlip_4: f32,
    pub m_afTumbleFlip_5: f32,
    pub m_afTumbleFlip_6: f32,
    pub m_afTumbleFlip_7: f32,
    pub m_afTumbleFlip_8: f32,
    pub m_afTumbleFlip_9: f32,
    pub m_afTumbleTwist_0: f32,
    pub m_afTumbleTwist_1: f32,
    pub m_afTumbleTwist_2: f32,
    pub m_afTumbleTwist_3: f32,
    pub m_afTumbleTwist_4: f32,
    pub m_afTumbleTwist_5: f32,
    pub m_afTumbleTwist_6: f32,
    pub m_afTumbleTwist_7: f32,
    pub m_afTumbleTwist_8: f32,
    pub m_afTumbleTwist_9: f32,
    pub m_afTumbleDirectionAdherence_0: f32,
    pub m_afTumbleDirectionAdherence_1: f32,
    pub m_afTumbleDirectionAdherence_2: f32,
    pub m_afTumbleDirectionAdherence_3: f32,
    pub m_afTumbleDirectionAdherence_4: f32,
    pub m_afTumbleDirectionAdherence_5: f32,
    pub m_afTumbleDirectionAdherence_6: f32,
    pub m_afTumbleDirectionAdherence_7: f32,
    pub m_afTumbleDirectionAdherence_8: f32,
    pub m_afTumbleDirectionAdherence_9: f32,
    pub m_afTwitchThrow_0: f32,
    pub m_afTwitchThrow_1: f32,
    pub m_afTwitchThrow_2: f32,
    pub m_afTwitchThrow_3: f32,
    pub m_afTwitchThrow_4: f32,
    pub m_afTwitchThrow_5: f32,
    pub m_afTwitchThrow_6: f32,
    pub m_afTwitchThrow_7: f32,
    pub m_afTwitchThrow_8: f32,
    pub m_afTwitchThrow_9: f32,
    pub m_fTwitchSharpness: f32,
    pub m_fRollMaxScale: f32,
    pub m_fRollMinScale: f32,
    pub m_fRollSpeed: f32,
    pub m_fRollSeparation: f32,
    pub m_fLeewardScalar: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SampleClip {
    pub m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SampleSettings {
    pub loadType: i32,
    pub sampleRateSetting: i32,
    pub sampleRateOverride: u32,
    pub compressionFormat: i32,
    pub quality: f32,
    pub conversionMode: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SamplerParameter {
    pub sampler: u32,
    pub bindPoint: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScaleConstraint {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Weight: f32,
    pub m_ScaleAtRest: Vector3f,
    pub m_ScaleOffset: Vector3f,
    pub m_AffectScalingX: bool,
    pub m_AffectScalingY: bool,
    pub m_AffectScalingZ: bool,
    pub m_IsContraintActive: Option<bool>,
    pub m_Sources: Vec<ConstraintSource>,
    pub m_Active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scene {
    pub m_PVSData: Option<Vec<u8>>,
    pub m_PVSObjectsArray: Option<Vec<PPtr>>,
    pub m_QueryMode: Option<u32>,
    pub m_PVSPortalsArray: Option<Vec<PPtr>>,
    pub m_SceneGUID: Option<GUID>,
    pub m_OcclusionCullingData: Option<PPtr>,
    pub m_StaticRenderers: Option<Vec<PPtr>>,
    pub m_Portals: Option<Vec<PPtr>>,
    pub enabled: Option<bool>,
    pub path: Option<String>,
    pub guid: Option<GUID>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneAsset {
    pub m_Name: String,
    pub m_Message: Option<String>,
    pub m_IsWarning: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneDataContainer {
    pub m_SceneData: Vec<(SceneIdentifier, HierarchicalSceneData)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneIdentifier {
    pub guid: GUID,
    pub handle: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneObjectIdentifier {
    pub targetObject: i64,
    pub targetPrefab: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneVisibilityData {
    pub m_SceneGUID: GUID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneVisibilityState {
    pub m_SceneData: Option<Vec<(SceneIdentifier, SceneVisibilityData)>>,
    pub m_MainStageIsolated: Option<bool>,
    pub m_PrefabStageIsolated: Option<bool>,
    pub m_SceneVisibilityData: Option<SceneDataContainer>,
    pub m_SceneVisibilityDataIsolated: Option<SceneDataContainer>,
    pub m_ScenePickingData: Option<SceneDataContainer>,
    pub m_IsolationMode: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScenesUsingAssets {
    pub m_ScenesUsingAssets: Vec<BuildReportScenesUsingAsset>,
    pub m_ListOfScenesUsingEachAsset: Vec<(String, Vec<String>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScriptMapper {
    pub m_Shaders: NameToObjectMap,
    pub m_PreloadShaders: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScriptableCamera {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_ClearFlags: u32,
    pub m_BackGroundColor: ColorRGBA,
    pub m_projectionMatrixMode: i32,
    pub m_GateFitMode: i32,
    pub m_SensorSize: Vector2f,
    pub m_LensShift: Vector2f,
    pub m_FocalLength: f32,
    pub m_NormalizedViewPortRect: Rectf,
    pub near_clip_plane: f32,
    pub far_clip_plane: f32,
    pub field_of_view: f32,
    pub orthographic: bool,
    pub orthographic_size: f32,
    pub m_Depth: f32,
    pub m_CullingMask: BitField,
    pub m_RenderingPath: i32,
    pub m_TargetTexture: PPtr,
    pub m_TargetDisplay: i32,
    pub m_TargetEye: i32,
    pub m_HDR: bool,
    pub m_AllowMSAA: bool,
    pub m_AllowDynamicResolution: bool,
    pub m_ForceIntoRT: bool,
    pub m_OcclusionCulling: bool,
    pub m_StereoConvergence: f32,
    pub m_StereoSeparation: f32,
    pub m_Script: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScriptedImporter {
    pub m_Name: String,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_Script: PPtr,
    pub m_FileIDToRecycleName: Option<Vec<(i64, String)>>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecondarySpriteTexture {
    pub texture: PPtr,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecondaryTextureSettings {
    pub platformSettings: Vec<TextureImporterPlatformSettings>,
    pub sRGB: Option<bool>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct SelectorStateConstant {
//     pub m_TransitionConstantArray: Vec<OffsetPtr>,
//     pub m_FullPathID: u32,
//     pub m_IsEntry: bool,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct SelectorTransitionConstant {
//     pub m_Destination: u32,
//     pub m_ConditionConstantArray: Vec<OffsetPtr>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializableManagedHost {
    pub m_Script: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializableManagedRefTestClass {
    pub m_Script: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedCustomEditorForRenderPipeline {
    pub customEditorName: String,
    pub renderPipelineType: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedPass {
    pub m_NameIndices: Vec<(String, i32)>,
    pub m_Type: i32,
    pub m_State: SerializedShaderState,
    pub m_ProgramMask: u32,
    pub progVertex: SerializedProgram,
    pub progFragment: SerializedProgram,
    pub progGeometry: SerializedProgram,
    pub progHull: SerializedProgram,
    pub progDomain: SerializedProgram,
    pub m_HasInstancingVariant: bool,
    pub m_UseName: String,
    pub m_Name: String,
    pub m_TextureName: String,
    pub m_Tags: SerializedTagMap,
    pub m_HasProceduralInstancingVariant: Option<bool>,
    pub progRayTracing: Option<SerializedProgram>,
    pub m_EditorDataHash: Option<Vec<Hash128>>,
    pub m_Platforms: Option<Vec<u8>>,
    pub m_LocalKeywordMask: Option<Vec<u16>>,
    pub m_GlobalKeywordMask: Option<Vec<u16>>,
    pub m_SerializedKeywordStateMask: Option<Vec<u16>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedProgram {
    pub m_SubPrograms: Vec<SerializedSubProgram>,
    pub m_CommonParameters: Option<SerializedProgramParameters>,
    pub m_SerializedKeywordStateMask: Option<Vec<u16>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedProgramParameters {
    pub m_VectorParams: Vec<VectorParameter>,
    pub m_MatrixParams: Vec<MatrixParameter>,
    pub m_TextureParams: Vec<TextureParameter>,
    pub m_BufferParams: Vec<BufferBinding>,
    pub m_ConstantBuffers: Vec<ConstantBuffer>,
    pub m_ConstantBufferBindings: Vec<BufferBinding>,
    pub m_UAVParams: Vec<UAVParameter>,
    pub m_Samplers: Vec<SamplerParameter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedProperties {
    pub m_Props: Vec<SerializedProperty>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedProperty {
    pub m_Name: String,
    pub m_Description: String,
    pub m_Attributes: Vec<String>,
    pub m_Type: i32,
    pub m_Flags: u32,
    pub m_DefTexture: SerializedTextureProperty,
    pub m_DefValue: (f32, f32, f32, f32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedShader {
    pub m_PropInfo: SerializedProperties,
    pub m_SubShaders: Vec<SerializedSubShader>,
    pub m_Name: String,
    pub m_CustomEditorName: String,
    pub m_FallbackName: String,
    pub m_Dependencies: Vec<SerializedShaderDependency>,
    pub m_DisableNoSubshadersMessage: bool,
    pub m_CustomEditorForRenderPipelines: Option<Vec<SerializedCustomEditorForRenderPipeline>>,
    pub m_KeywordNames: Option<Vec<String>>,
    pub m_KeywordFlags: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedShaderDependency {
    pub from: String,
    pub to: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedShaderFloatValue {
    pub val: f32,
    pub name: (Option<String>, Option<FastPropertyName>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedShaderRTBlendState {
    pub srcBlend: SerializedShaderFloatValue,
    pub destBlend: SerializedShaderFloatValue,
    pub srcBlendAlpha: SerializedShaderFloatValue,
    pub destBlendAlpha: SerializedShaderFloatValue,
    pub blendOp: SerializedShaderFloatValue,
    pub blendOpAlpha: SerializedShaderFloatValue,
    pub colMask: SerializedShaderFloatValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedShaderState {
    pub m_Name: String,
    pub rtBlend0: SerializedShaderRTBlendState,
    pub rtBlend1: SerializedShaderRTBlendState,
    pub rtBlend2: SerializedShaderRTBlendState,
    pub rtBlend3: SerializedShaderRTBlendState,
    pub rtBlend4: SerializedShaderRTBlendState,
    pub rtBlend5: SerializedShaderRTBlendState,
    pub rtBlend6: SerializedShaderRTBlendState,
    pub rtBlend7: SerializedShaderRTBlendState,
    pub rtSeparateBlend: bool,
    pub zTest: SerializedShaderFloatValue,
    pub zWrite: SerializedShaderFloatValue,
    pub culling: SerializedShaderFloatValue,
    pub offsetFactor: SerializedShaderFloatValue,
    pub offsetUnits: SerializedShaderFloatValue,
    pub alphaToMask: SerializedShaderFloatValue,
    pub stencilOp: SerializedStencilOp,
    pub stencilOpFront: SerializedStencilOp,
    pub stencilOpBack: SerializedStencilOp,
    pub stencilReadMask: SerializedShaderFloatValue,
    pub stencilWriteMask: SerializedShaderFloatValue,
    pub stencilRef: SerializedShaderFloatValue,
    pub fogStart: SerializedShaderFloatValue,
    pub fogEnd: SerializedShaderFloatValue,
    pub fogDensity: SerializedShaderFloatValue,
    pub fogColor: SerializedShaderVectorValue,
    pub fogMode: i32,
    pub gpuProgramID: i32,
    pub m_Tags: SerializedTagMap,
    pub m_LOD: i32,
    pub lighting: bool,
    pub zClip: Option<SerializedShaderFloatValue>,
    pub conservative: Option<SerializedShaderFloatValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedShaderVectorValue {
    pub x: SerializedShaderFloatValue,
    pub y: SerializedShaderFloatValue,
    pub z: SerializedShaderFloatValue,
    pub w: SerializedShaderFloatValue,
    pub name: (Option<String>, Option<FastPropertyName>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedStencilOp {
    pub pass: SerializedShaderFloatValue,
    pub fail: SerializedShaderFloatValue,
    pub zFail: SerializedShaderFloatValue,
    pub comp: SerializedShaderFloatValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedSubProgram {
    pub m_BlobIndex: u32,
    pub m_Channels: ParserBindChannels,
    pub m_KeywordIndices: Option<Vec<u16>>,
    pub m_ShaderHardwareTier: i8,
    pub m_GpuProgramType: i8,
    pub m_VectorParams: Option<Vec<VectorParameter>>,
    pub m_MatrixParams: Option<Vec<MatrixParameter>>,
    pub m_TextureParams: Option<Vec<TextureParameter>>,
    pub m_BufferParams: Option<Vec<BufferBinding>>,
    pub m_ConstantBuffers: Option<Vec<ConstantBuffer>>,
    pub m_ConstantBufferBindings: Option<Vec<BufferBinding>>,
    pub m_UAVParams: Option<Vec<UAVParameter>>,
    pub m_Samplers: Option<Vec<SamplerParameter>>,
    pub m_ShaderRequirements: Option<(Option<i64>, Option<i32>)>,
    pub m_GlobalKeywordIndices: Option<Vec<u16>>,
    pub m_LocalKeywordIndices: Option<Vec<u16>>,
    pub m_Parameters: Option<SerializedProgramParameters>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedSubShader {
    pub m_Passes: Vec<SerializedPass>,
    pub m_Tags: SerializedTagMap,
    pub m_LOD: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedTagMap {
    pub tags: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedTextureProperty {
    pub m_DefaultName: String,
    pub m_TexDim: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shader {
    pub m_Name: String,
    pub m_Script: Option<String>,
    pub m_PathName: Option<String>,
    pub m_DefaultProperties: Option<UnityPropertySheet>,
    pub m_Dependencies: Option<Vec<PPtr>>,
    pub m_ShaderIsBaked: Option<bool>,
    pub decompressedSize: Option<u32>,
    pub m_SubProgramBlob: Option<Vec<u8>>,
    pub m_ParsedForm: Option<SerializedShader>,
    pub platforms: Option<Vec<u32>>,
    pub offsets: Option<(Option<Vec<u32>>, Option<Vec<Vec<u32>>>)>,
    pub compressedLengths: Option<(Option<Vec<u32>>, Option<Vec<Vec<u32>>>)>,
    pub decompressedLengths: Option<(Option<Vec<u32>>, Option<Vec<Vec<u32>>>)>,
    pub compressedBlob: Option<Vec<u8>>,
    pub m_NonModifiableTextures: Option<Vec<(String, PPtr)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShaderBindChannel {
    pub source: i8,
    pub target: i8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShaderImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_UserData: Option<String>,
    pub m_DefaultTextures: Option<Vec<(String, PPtr)>>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_NonModifiableTextures: Option<Vec<(String, PPtr)>>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
    pub m_PreprocessorOverride: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShaderIncludeImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShaderInfo {
    pub variants: Vec<VariantInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShaderVariantCollection {
    pub m_Name: String,
    pub m_Shaders: Vec<(PPtr, ShaderInfo)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShadowSettings {
    pub m_Type: i32,
    pub m_Resolution: i32,
    pub m_Strength: f32,
    pub m_Projection: Option<i32>,
    pub m_Bias: f32,
    pub m_Softness: Option<f32>,
    pub m_SoftnessFade: Option<f32>,
    pub m_NormalBias: Option<f32>,
    pub m_NearPlane: Option<f32>,
    pub m_CustomResolution: Option<i32>,
    pub m_CullingMatrixOverride: Option<Matrix4x4f>,
    pub m_UseCullingMatrixOverride: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShapeModule {
    pub enabled: bool,
    pub __type: i32,
    pub radius: (Option<MultiModeParameter>, Option<f32>),
    pub angle: f32,
    pub boxX: Option<f32>,
    pub boxY: Option<f32>,
    pub boxZ: Option<f32>,
    pub placementMode: i32,
    pub m_Mesh: PPtr,
    pub randomDirection: Option<bool>,
    pub length: Option<f32>,
    pub arc: Option<(Option<MultiModeParameter>, Option<f32>)>,
    pub m_MeshRenderer: Option<PPtr>,
    pub m_SkinnedMeshRenderer: Option<PPtr>,
    pub m_MeshMaterialIndex: Option<i32>,
    pub m_MeshNormalOffset: Option<f32>,
    pub m_UseMeshMaterialIndex: Option<bool>,
    pub m_UseMeshColors: Option<bool>,
    pub m_MeshScale: Option<f32>,
    pub alignToDirection: Option<bool>,
    pub randomDirectionAmount: Option<f32>,
    pub sphericalDirectionAmount: Option<f32>,
    pub boxThickness: Option<Vector3f>,
    pub radiusThickness: Option<f32>,
    pub donutRadius: Option<f32>,
    pub m_Position: Option<Vector3f>,
    pub m_Rotation: Option<Vector3f>,
    pub m_Scale: Option<Vector3f>,
    pub randomPositionAmount: Option<f32>,
    pub m_Texture: Option<PPtr>,
    pub m_TextureClipChannel: Option<i32>,
    pub m_TextureClipThreshold: Option<f32>,
    pub m_TextureUVChannel: Option<i32>,
    pub m_TextureColorAffectsParticles: Option<bool>,
    pub m_TextureAlphaAffectsParticles: Option<bool>,
    pub m_TextureBilinearFiltering: Option<bool>,
    pub m_Sprite: Option<PPtr>,
    pub m_SpriteRenderer: Option<PPtr>,
    pub m_MeshSpawn: Option<MultiModeParameter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiblingDerived {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SizeBySpeedModule {
    pub enabled: bool,
    pub curve: MinMaxCurve,
    pub range: Vector2f,
    pub y: Option<MinMaxCurve>,
    pub z: Option<MinMaxCurve>,
    pub separateAxes: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SizeModule {
    pub enabled: bool,
    pub curve: MinMaxCurve,
    pub y: Option<MinMaxCurve>,
    pub z: Option<MinMaxCurve>,
    pub separateAxes: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skeleton {
    pub m_Node: Vec<Node>,
    pub m_ID: Vec<u32>,
    pub m_AxesArray: Vec<Axes>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkeletonBone {
    pub m_Name: String,
    pub m_Position: Vector3f,
    pub m_Rotation: Quaternionf,
    pub m_Scale: Vector3f,
    pub m_TransformModified: Option<bool>,
    pub m_ParentName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkeletonBoneLimit {
    pub m_Min: Vector3f,
    pub m_Max: Vector3f,
    pub m_Value: Vector3f,
    pub m_PreQ: Option<Quaternionf>,
    pub m_PostQ: Option<Quaternionf>,
    pub m_Length: f32,
    pub m_Modified: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkeletonMask {
    pub m_Data: Vec<SkeletonMaskElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkeletonMaskElement {
    pub m_Index: Option<u32>,
    pub m_Weight: f32,
    pub m_PathHash: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkeletonPose {
    pub m_X: Vec<xform>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SketchUpImportCamera {
    pub position: Vector3f,
    pub lookAt: Vector3f,
    pub up: Vector3f,
    pub fov: f32,
    pub aspectRatio: f32,
    pub orthoSize: f32,
    pub isPerspective: i32,
    pub nearPlane: Option<f32>,
    pub farPlane: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SketchUpImportData {
    pub defaultCamera: SketchUpImportCamera,
    pub scenes: Vec<SketchUpImportScene>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SketchUpImportScene {
    pub camera: SketchUpImportCamera,
    pub name: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct SketchUpImporter {
//     pub m_GenerateBackFace: bool,
//     pub m_MergeCoplanarFaces: bool,
//     pub m_SelectedNodes: Vec<i32>,
//     pub m_AssetHash: Hash128,
//     pub m_Longitude: f64,
//     pub m_Latitude: f64,
//     pub m_NorthCorrection: f64,
//     pub m_FileUnit: i32,
//     pub m_SketchUpImportData: SketchUpImportData,
//     pub m_Name: String,
//     pub m_FileIDToRecycleName: Option<Vec<(i64, String)>>,
//     pub m_ImportMaterials: Option<bool>,
//     pub m_MaterialName: i32,
//     pub m_MaterialSearch: i32,
//     pub m_LegacyGenerateAnimations: i32,
//     pub m_BakeSimulation: bool,
//     pub m_OptimizeGameObjects: bool,
//     pub m_MotionNodeName: String,
//     pub m_AnimationImportErrors: String,
//     pub m_AnimationImportWarnings: String,
//     pub m_AnimationRetargetingWarnings: String,
//     pub m_AnimationDoRetargetingWarnings: bool,
//     pub m_AnimationCompression: i32,
//     pub m_AnimationRotationError: f32,
//     pub m_AnimationPositionError: f32,
//     pub m_AnimationScaleError: f32,
//     pub m_AnimationWrapMode: i32,
//     pub m_ExtraExposedTransformPaths: Vec<String>,
//     pub m_ClipAnimations: Vec<ClipAnimationInfo>,
//     pub m_IsReadable: bool,
//     pub m_LODScreenPercentages: Vec<f32>,
//     pub m_GlobalScale: f32,
//     pub m_MeshCompression: i32,
//     pub m_AddColliders: bool,
//     pub m_ImportBlendShapes: bool,
//     pub swapUVChannels: bool,
//     pub generateSecondaryUV: bool,
//     pub m_UseFileUnits: bool,
//     pub optimizeMeshForGPU: Option<bool>,
//     pub keepQuads: bool,
//     pub weldVertices: bool,
//     pub secondaryUVAngleDistortion: f32,
//     pub secondaryUVAreaDistortion: f32,
//     pub secondaryUVHardAngle: f32,
//     pub secondaryUVPackMargin: f32,
//     pub m_UseFileScale: bool,
//     pub m_FileScale: f32,
//     pub normalSmoothAngle: f32,
//     pub splitTangentsAcrossUV: Option<bool>,
//     pub normalImportMode: i32,
//     pub tangentImportMode: i32,
//     pub m_ImportedTakeInfos: Vec<TakeInfo>,
//     pub m_ReferencedClips: Vec<GUID>,
//     pub m_ImportedRoots: Vec<PPtr>,
//     pub m_HasExtraRoot: bool,
//     pub m_ImportAnimation: bool,
//     pub m_CopyAvatar: Option<bool>,
//     pub m_HumanDescription: HumanDescription,
//     pub m_LastHumanDescriptionAvatarSource: PPtr,
//     pub m_AnimationType: i32,
//     pub m_AdditionalBone: bool,
//     pub m_UserData: String,
//     pub m_AssetBundleName: String,
//     pub m_AssetBundleVariant: String,
//     pub m_HumanoidOversampling: Option<i32>,
//     pub m_ResampleRotations: Option<bool>,
//     pub m_ResampleCurves: Option<bool>,
//     pub m_RigImportErrors: Option<String>,
//     pub m_RigImportWarnings: Option<String>,
//     pub m_ImportVisibility: Option<bool>,
//     pub m_ImportCameras: Option<bool>,
//     pub m_ImportLights: Option<bool>,
//     pub normalCalculationMode: Option<i32>,
//     pub m_ExtraUserProperties: Option<Vec<String>>,
//     pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
//     pub m_AutoMapExternalMaterials: Option<bool>,
//     pub m_MaterialLocation: Option<i32>,
//     pub m_Materials: Option<Vec<SourceAssetIdentifier>>,
//     pub m_ImportAnimatedCustomProperties: Option<bool>,
//     pub m_HasEmbeddedTextures: Option<bool>,
//     pub m_SupportsEmbeddedMaterials: Option<bool>,
//     pub m_PreserveHierarchy: Option<bool>,
//     pub indexFormat: Option<i32>,
//     pub m_ImportConstraints: Option<bool>,
//     pub m_PreviousCalculatedGlobalScale: Option<f32>,
//     pub m_HasPreviousCalculatedGlobalScale: Option<bool>,
//     pub m_UseSRGBMaterialColor: Option<bool>,
//     pub m_FileScaleUnit: Option<String>,
//     pub m_FileScaleFactor: Option<f32>,
//     pub legacyComputeAllNormalsFromSmoothingGroupsWhenMeshHasBlendShapes: Option<bool>,
//     pub blendShapeNormalImportMode: Option<i32>,
//     pub normalSmoothingSource: Option<i32>,
//     pub m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
//     pub m_UsedFileIDs: Option<Vec<i64>>,
//     pub skinWeightsMode: Option<i32>,
//     pub maxBonesPerVertex: Option<i32>,
//     pub minBoneWeight: Option<f32>,
//     pub meshOptimizationFlags: Option<i32>,
//     pub m_SortHierarchyByName: Option<bool>,
//     pub m_AvatarSetup: Option<i32>,
//     pub m_MaterialImportMode: Option<i32>,
//     pub m_AutoGenerateAvatarMappingIfUnspecified: Option<bool>,
//     pub m_FileIdsGeneration: Option<i32>,
//     pub secondaryUVMarginMethod: Option<i32>,
//     pub secondaryUVMinLightmapResolution: Option<f32>,
//     pub secondaryUVMinObjectScale: Option<f32>,
//     pub bakeAxisConversion: Option<bool>,
//     pub m_AddHumanoidExtraRootOnlyWhenUsingAvatar: Option<bool>,
//     pub optimizeBones: Option<bool>,
//     pub m_RemoveConstantScaleCurves: Option<bool>,
//     pub m_NodeNameCollisionStrategy: Option<i32>,
//     pub m_StrictVertexDataChecks: Option<bool>,
//     pub m_ImportBlendShapeDeformPercent: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkinnedCloth {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_BendingStiffness: f32,
    pub m_StretchingStiffness: f32,
    pub m_Damping: f32,
    pub m_Thickness: f32,
    pub m_UseGravity: bool,
    pub m_SelfCollision: bool,
    pub m_ExternalAcceleration: Vector3f,
    pub m_RandomAcceleration: Vector3f,
    pub m_WorldVelocityScale: f32,
    pub m_WorldAccelerationScale: f32,
    pub m_Coefficients: Vec<ClothConstrainCoefficients>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkinnedMeshRenderer {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: (Option<u8>, Option<bool>),
    pub m_ReceiveShadows: (Option<u8>, Option<bool>),
    pub m_LightmapIndex: (Option<u8>, Option<u16>),
    pub m_LightmapTilingOffset: Vector4f,
    pub m_Materials: Vec<PPtr>,
    pub m_SubsetIndices: Option<Vec<u32>>,
    pub m_StaticBatchRoot: PPtr,
    pub m_Quality: i32,
    pub m_UpdateWhenOffscreen: bool,
    pub m_SkinNormals: Option<bool>,
    pub m_Mesh: PPtr,
    pub m_Bones: Vec<PPtr>,
    pub m_AABB: Option<AABB>,
    pub m_DirtyAABB: Option<bool>,
    pub m_UseLightProbes: Option<bool>,
    pub m_LightProbeAnchor: Option<PPtr>,
    pub m_RootBone: Option<PPtr>,
    pub m_SortingLayer: Option<i16>,
    pub m_SortingOrder: Option<i16>,
    pub m_BlendShapeWeights: Option<Vec<f32>>,
    pub m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    pub m_LightmapIndexDynamic: Option<u16>,
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    pub m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    pub m_ProbeAnchor: Option<PPtr>,
    pub m_MotionVectors: Option<u8>,
    pub m_LightProbeUsage: Option<u8>,
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    pub m_SkinnedMotionVectors: Option<bool>,
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    pub m_DynamicOccludee: Option<u8>,
    pub m_RenderingLayerMask: Option<u32>,
    pub m_RendererPriority: Option<i32>,
    pub m_RayTracingMode: Option<u8>,
    pub m_RayTraceProcedural: Option<u8>,
    pub m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skybox {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_CustomSkybox: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SliderJoint2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_CollideConnected: Option<bool>,
    pub m_ConnectedRigidBody: PPtr,
    pub m_Anchor: Vector2f,
    pub m_ConnectedAnchor: Vector2f,
    pub m_Angle: f32,
    pub m_UseMotor: bool,
    pub m_Motor: JointMotor2D,
    pub m_UseLimits: bool,
    pub m_TranslationLimits: JointTranslationLimits2D,
    pub m_EnableCollision: Option<bool>,
    pub m_BreakForce: Option<f32>,
    pub m_BreakTorque: Option<f32>,
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    pub m_AutoConfigureAngle: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnapshotConstant {
    pub nameHash: u32,
    pub values: Vec<f32>,
    pub transitionTypes: Vec<u32>,
    pub transitionIndices: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoftJointLimit {
    pub limit: f32,
    pub bounciness: f32,
    pub spring: Option<f32>,
    pub damper: Option<f32>,
    pub contactDistance: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoftJointLimitSpring {
    pub spring: f32,
    pub damper: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SortingGroup {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_SortingLayer: i16,
    pub m_SortingOrder: i16,
    pub m_SortingLayerID: Option<i32>,
    pub m_SortAtRoot: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SortingLayerEntry {
    pub name: String,
    pub userID: Option<u32>,
    pub uniqueID: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceAssetIdentifier {
    pub __type: String,
    pub assembly: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTextureInformation {
    pub width: i32,
    pub height: i32,
    pub doesTextureContainAlpha: bool,
    pub doesTextureContainColor: Option<bool>,
    pub sourceWasHDR: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SparseTexture {
    pub m_Name: String,
    pub m_Width: i32,
    pub m_Height: i32,
    pub m_MipCount: i32,
    pub m_Format: (Option<u32>, Option<i32>),
    pub m_ColorSpace: i32,
    pub m_TextureSettings: GLTextureSettings,
    pub m_ForcedFallbackFormat: Option<i32>,
    pub m_DownscaleFallback: Option<bool>,
    pub m_IsAlphaChannelOptional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpeedTreeImporter {
    pub m_Name: String,
    pub m_MainColor: ColorRGBA,
    pub m_SpecColor: Option<ColorRGBA>,
    pub m_HueVariation: ColorRGBA,
    pub m_Shininess: Option<f32>,
    pub m_AlphaTestRef: f32,
    pub m_BestWindQuality: i32,
    pub m_HasBillboard: bool,
    pub m_LODSettings: Vec<PerLODSettings>,
    pub m_EnableSmoothLODTransition: bool,
    pub m_BillboardTransitionCrossFadeWidth: f32,
    pub m_FadeOutWidth: f32,
    pub m_ScaleFactor: f32,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_AnimateCrossFading: Option<bool>,
    pub m_MaterialVersion: Option<i32>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_MaterialLocation: Option<i32>,
    pub m_Materials: Option<Vec<SourceAssetIdentifier>>,
    pub m_SupportsEmbeddedMaterials: Option<bool>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
    pub m_FileIDType: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpeedTreeWind {
    pub m_sParams: SParams,
    pub BranchWindAnchor0: f32,
    pub BranchWindAnchor1: f32,
    pub BranchWindAnchor2: f32,
    pub m_fMaxBranchLevel1Length: f32,
    pub GLOBAL_WIND: bool,
    pub GLOBAL_PRESERVE_SHAPE: bool,
    pub BRANCH_SIMPLE_1: bool,
    pub BRANCH_DIRECTIONAL_1: bool,
    pub BRANCH_DIRECTIONAL_FROND_1: bool,
    pub BRANCH_TURBULENCE_1: bool,
    pub BRANCH_WHIP_1: bool,
    pub BRANCH_OSC_COMPLEX_1: bool,
    pub BRANCH_SIMPLE_2: bool,
    pub BRANCH_DIRECTIONAL_2: bool,
    pub BRANCH_DIRECTIONAL_FROND_2: bool,
    pub BRANCH_TURBULENCE_2: bool,
    pub BRANCH_WHIP_2: bool,
    pub BRANCH_OSC_COMPLEX_2: bool,
    pub LEAF_RIPPLE_VERTEX_NORMAL_1: bool,
    pub LEAF_RIPPLE_COMPUTED_1: bool,
    pub LEAF_TUMBLE_1: bool,
    pub LEAF_TWITCH_1: bool,
    pub LEAF_OCCLUSION_1: bool,
    pub LEAF_RIPPLE_VERTEX_NORMAL_2: bool,
    pub LEAF_RIPPLE_COMPUTED_2: bool,
    pub LEAF_TUMBLE_2: bool,
    pub LEAF_TWITCH_2: bool,
    pub LEAF_OCCLUSION_2: bool,
    pub FROND_RIPPLE_ONE_SIDED: bool,
    pub FROND_RIPPLE_TWO_SIDED: bool,
    pub FROND_RIPPLE_ADJUST_LIGHTING: bool,
    pub ROLLING: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpeedTreeWindAsset {
    pub m_Name: String,
    pub m_Wind: SpeedTreeWind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SphereCollider {
    pub m_GameObject: PPtr,
    pub m_Material: PPtr,
    pub m_IsTrigger: bool,
    pub m_Radius: f32,
    pub m_Center: Vector3f,
    pub m_Enabled: Option<bool>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct SphericalHarmonicsL2 {
//     pub sh: (
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
    pub logo: PPtr,
    pub duration: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SplatDatabase {
    pub m_Splats: Option<Vec<SplatPrototype>>,
    pub m_AlphaTextures: Vec<PPtr>,
    pub m_AlphamapResolution: i32,
    pub m_BaseMapResolution: i32,
    pub m_ColorSpace: Option<i32>,
    pub m_MaterialRequiresMetallic: Option<bool>,
    pub m_MaterialRequiresSmoothness: Option<bool>,
    pub m_TerrainLayers: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SplatPrototype {
    pub texture: PPtr,
    pub tileSize: Vector2f,
    pub tileOffset: Vector2f,
    pub normalMap: Option<PPtr>,
    pub specularMetallic: Option<Vector4f>,
    pub smoothness: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpringJoint {
    pub m_GameObject: PPtr,
    pub m_ConnectedBody: PPtr,
    pub m_Anchor: Vector3f,
    pub m_Spring: f32,
    pub m_Damper: f32,
    pub m_MinDistance: f32,
    pub m_MaxDistance: f32,
    pub m_BreakForce: f32,
    pub m_BreakTorque: f32,
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    pub m_ConnectedAnchor: Option<Vector3f>,
    pub m_EnableCollision: Option<bool>,
    pub m_EnablePreprocessing: Option<bool>,
    pub m_Tolerance: Option<f32>,
    pub m_MassScale: Option<f32>,
    pub m_ConnectedMassScale: Option<f32>,
    pub m_Enabled: Option<bool>,
    pub m_ConnectedArticulationBody: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpringJoint2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_CollideConnected: Option<bool>,
    pub m_ConnectedRigidBody: PPtr,
    pub m_Anchor: Vector2f,
    pub m_ConnectedAnchor: Vector2f,
    pub m_Distance: f32,
    pub m_DampingRatio: f32,
    pub m_Frequency: f32,
    pub m_EnableCollision: Option<bool>,
    pub m_BreakForce: Option<f32>,
    pub m_BreakTorque: Option<f32>,
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    pub m_AutoConfigureDistance: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sprite {
    pub m_Name: String,
    pub m_Rect: Rectf,
    pub m_Offset: Vector2f,
    pub m_PixelsToUnits: f32,
    pub m_Extrude: u32,
    pub m_RD: SpriteRenderData,
    pub m_Border: Option<Vector4f>,
    pub m_IsPolygon: Option<bool>,
    pub m_Pivot: Option<Vector2f>,
    pub m_RenderDataKey: Option<(GUID, i64)>,
    pub m_AtlasTags: Option<Vec<String>>,
    pub m_SpriteAtlas: Option<PPtr>,
    pub m_PhysicsShape: Option<Vec<Vec<Vector2f>>>,
    pub m_Bones: Option<Vec<SpriteBone>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlas {
    pub m_Name: String,
    pub m_PackedSprites: Vec<PPtr>,
    pub m_PackedSpriteNamesToIndex: Vec<String>,
    pub m_RenderDataMap: Vec<((GUID, i64), SpriteAtlasData)>,
    pub m_Tag: String,
    pub m_IsVariant: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlasAsset {
    pub m_Name: String,
    pub m_MasterAtlas: PPtr,
    pub m_ImporterData: (Option<SpriteAtlasAssetData>, Option<SpriteAtlasEditorData>),
    pub m_IsVariant: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlasAssetData {
    pub packables: Vec<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlasData {
    pub texture: PPtr,
    pub alphaTexture: PPtr,
    pub textureRect: Rectf,
    pub textureRectOffset: Vector2f,
    pub uvTransform: Vector4f,
    pub downscaleMultiplier: f32,
    pub settingsRaw: u32,
    pub atlasRectOffset: Option<Vector2f>,
    pub secondaryTextures: Option<Vec<SecondarySpriteTexture>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlasDatabase {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlasEditorData {
    pub textureSettings: TextureSettings,
    pub platformSettings: Vec<TextureImporterPlatformSettings>,
    pub packingSettings: PackingSettings,
    pub variantMultiplier: f32,
    pub packables: Vec<PPtr>,
    pub totalSpriteSurfaceArea: Option<u32>,
    pub bindAsDefault: bool,
    pub storedHash: Option<Hash128>,
    pub isAtlasV2: bool,
    pub cachedData: PPtr,
    pub secondaryTextureSettings: Option<Vec<(String, SecondaryTextureSettings)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteAtlasImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_TextureSettings: Option<TextureSettings>,
    pub m_PlatformSettings: Option<Vec<TextureImporterPlatformSettings>>,
    pub m_PackingSettings: Option<PackingSettings>,
    pub m_SecondaryTextureSettings: Option<Vec<(String, SecondaryTextureSettings)>>,
    pub m_VariantMultiplier: Option<f32>,
    pub m_BindAsDefault: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteBone {
    pub name: String,
    pub position: Vector3f,
    pub rotation: Quaternionf,
    pub length: f32,
    pub parentId: i32,
    pub guid: Option<String>,
    pub color: Option<ColorRGBA>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteData {
    pub sprite: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteMask {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: u8,
    pub m_ReceiveShadows: u8,
    pub m_MotionVectors: u8,
    pub m_LightProbeUsage: u8,
    pub m_ReflectionProbeUsage: u8,
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    pub m_Materials: Vec<PPtr>,
    pub m_StaticBatchInfo: StaticBatchInfo,
    pub m_StaticBatchRoot: PPtr,
    pub m_ProbeAnchor: PPtr,
    pub m_LightProbeVolumeOverride: PPtr,
    pub m_SortingLayerID: i32,
    pub m_SortingLayer: i16,
    pub m_SortingOrder: i16,
    pub m_Sprite: PPtr,
    pub m_MaskAlphaCutoff: f32,
    pub m_FrontSortingLayer: i16,
    pub m_BackSortingLayer: i16,
    pub m_FrontSortingOrder: i16,
    pub m_BackSortingOrder: i16,
    pub m_IsCustomRangeActive: bool,
    pub m_DynamicOccludee: Option<u8>,
    pub m_RenderingLayerMask: Option<u32>,
    pub m_SpriteSortPoint: Option<i32>,
    pub m_RendererPriority: Option<i32>,
    pub m_FrontSortingLayerID: Option<i32>,
    pub m_BackSortingLayerID: Option<i32>,
    pub m_RayTracingMode: Option<u8>,
    pub m_RayTraceProcedural: Option<u8>,
    pub m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteMetaData {
    pub m_Name: String,
    pub m_Rect: Rectf,
    pub m_Alignment: i32,
    pub m_Pivot: Vector2f,
    pub m_Border: Option<Vector4f>,
    pub m_Outline: Option<Vec<Vec<Vector2f>>>,
    pub m_TessellationDetail: Option<f32>,
    pub m_PhysicsShape: Option<Vec<Vec<Vector2f>>>,
    pub m_Bones: Option<Vec<SpriteBone>>,
    pub m_SpriteID: Option<String>,
    pub m_Vertices: Option<Vec<Vector2f>>,
    pub m_Indices: Option<Vec<i32>>,
    pub m_Edges: Option<Vec<int2_storage>>,
    pub m_Weights: Option<Vec<BoneWeights4>>,
    pub m_InternalID: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteRenderData {
    pub texture: PPtr,
    pub vertices: Option<Vec<SpriteVertex>>,
    pub indices: Option<Vec<u16>>,
    pub textureRect: Rectf,
    pub textureRectOffset: Vector2f,
    pub settingsRaw: u32,
    pub uvTransform: Option<Vector4f>,
    pub alphaTexture: Option<PPtr>,
    pub atlasRectOffset: Option<Vector2f>,
    pub m_SubMeshes: Option<Vec<SubMesh>>,
    pub m_IndexBuffer: Option<Vec<u8>>,
    pub m_VertexData: Option<VertexData>,
    pub downscaleMultiplier: Option<f32>,
    pub m_Bindpose: Option<Vec<Matrix4x4f>>,
    pub m_SourceSkin: Option<Vec<BoneWeights4>>,
    pub secondaryTextures: Option<Vec<SecondarySpriteTexture>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteRenderer {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: (Option<u8>, Option<bool>),
    pub m_ReceiveShadows: (Option<u8>, Option<bool>),
    pub m_LightmapIndex: (Option<u8>, Option<u16>),
    pub m_LightmapTilingOffset: Vector4f,
    pub m_Materials: Vec<PPtr>,
    pub m_SubsetIndices: Option<Vec<u32>>,
    pub m_StaticBatchRoot: PPtr,
    pub m_UseLightProbes: Option<bool>,
    pub m_LightProbeAnchor: Option<PPtr>,
    pub m_SortingLayer: Option<i16>,
    pub m_SortingOrder: i16,
    pub m_Sprite: PPtr,
    pub m_Color: ColorRGBA,
    pub m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    pub m_LightmapIndexDynamic: Option<u16>,
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    pub m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    pub m_ProbeAnchor: Option<PPtr>,
    pub m_FlipX: Option<bool>,
    pub m_FlipY: Option<bool>,
    pub m_MotionVectors: Option<u8>,
    pub m_LightProbeUsage: Option<u8>,
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    pub m_DrawMode: Option<i32>,
    pub m_Size: Option<Vector2f>,
    pub m_AdaptiveModeThreshold: Option<f32>,
    pub m_SpriteTileMode: Option<i32>,
    pub m_WasSpriteAssigned: Option<bool>,
    pub m_MaskInteraction: Option<i32>,
    pub m_DynamicOccludee: Option<u8>,
    pub m_RenderingLayerMask: Option<u32>,
    pub m_SpriteSortPoint: Option<i32>,
    pub m_RendererPriority: Option<i32>,
    pub m_RayTracingMode: Option<u8>,
    pub m_RayTraceProcedural: Option<u8>,
    pub m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteShapeRenderer {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: u8,
    pub m_ReceiveShadows: u8,
    pub m_DynamicOccludee: u8,
    pub m_MotionVectors: u8,
    pub m_LightProbeUsage: u8,
    pub m_ReflectionProbeUsage: u8,
    pub m_RenderingLayerMask: u32,
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    pub m_Materials: Vec<PPtr>,
    pub m_StaticBatchInfo: StaticBatchInfo,
    pub m_StaticBatchRoot: PPtr,
    pub m_ProbeAnchor: PPtr,
    pub m_LightProbeVolumeOverride: PPtr,
    pub m_SortingLayerID: i32,
    pub m_SortingLayer: i16,
    pub m_SortingOrder: i16,
    pub m_Color: ColorRGBA,
    pub m_MaskInteraction: i32,
    pub m_ShapeTexture: PPtr,
    pub m_Sprites: Vec<PPtr>,
    pub m_LocalAABB: AABB,
    pub m_RendererPriority: Option<i32>,
    pub m_RayTracingMode: Option<u8>,
    pub m_RayTraceProcedural: Option<u8>,
    pub m_StaticShadowCaster: Option<u8>,
    pub m_SpriteSortPoint: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteSheetMetaData {
    pub m_Sprites: Vec<SpriteMetaData>,
    pub m_Outline: Option<Vec<Vec<Vector2f>>>,
    pub m_PhysicsShape: Option<Vec<Vec<Vector2f>>>,
    pub m_Bones: Option<Vec<SpriteBone>>,
    pub m_SpriteID: Option<String>,
    pub m_Vertices: Option<Vec<Vector2f>>,
    pub m_Indices: Option<Vec<i32>>,
    pub m_Edges: Option<Vec<int2_storage>>,
    pub m_Weights: Option<Vec<BoneWeights4>>,
    pub m_InternalID: Option<i64>,
    pub m_SecondaryTextures: Option<Vec<SecondarySpriteTexture>>,
    pub m_NameFileIdTable: Option<Vec<(String, i64)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteTilingProperty {
    pub border: Vector4f,
    pub pivot: Vector2f,
    pub oldSize: Vector2f,
    pub newSize: Vector2f,
    pub adaptiveTilingThreshold: f32,
    pub drawMode: i32,
    pub adaptiveTiling: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteVertex {
    pub pos: Vector3f,
    pub uv: Option<Vector2f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub m_Name: String,
    pub m_Speed: f32,
    pub m_Motions: Option<Vec<PPtr>>,
    pub m_ParentStateMachine: Option<PPtr>,
    pub m_Position: Vector3f,
    pub m_IKOnFeet: bool,
    pub m_Tag: String,
    pub m_CycleOffset: Option<f32>,
    pub m_Mirror: Option<bool>,
    pub m_Transitions: Option<Vec<PPtr>>,
    pub m_StateMachineBehaviours: Option<Vec<PPtr>>,
    pub m_WriteDefaultValues: Option<bool>,
    pub m_Motion: Option<PPtr>,
    pub m_SpeedParameterActive: Option<bool>,
    pub m_MirrorParameterActive: Option<bool>,
    pub m_CycleOffsetParameterActive: Option<bool>,
    pub m_SpeedParameter: Option<String>,
    pub m_MirrorParameter: Option<String>,
    pub m_CycleOffsetParameter: Option<String>,
    pub m_TimeParameterActive: Option<bool>,
    pub m_TimeParameter: Option<String>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct StateConstant {
//     pub m_TransitionConstantArray: Vec<OffsetPtr>,
//     pub m_BlendTreeConstantIndexArray: Vec<i32>,
//     pub m_LeafInfoArray: Option<Vec<LeafInfoConstant>>,
//     pub m_BlendTreeConstantArray: Vec<OffsetPtr>,
//     pub m_ID: Option<u32>,
//     pub m_TagID: u32,
//     pub m_Speed: f32,
//     pub m_IKOnFeet: bool,
//     pub m_Loop: bool,
//     pub m_CycleOffset: Option<f32>,
//     pub m_Mirror: Option<bool>,
//     pub m_NameID: Option<u32>,
//     pub m_PathID: Option<u32>,
//     pub m_FullPathID: Option<u32>,
//     pub m_WriteDefaultValues: Option<bool>,
//     pub m_SpeedParamID: Option<u32>,
//     pub m_MirrorParamID: Option<u32>,
//     pub m_CycleOffsetParamID: Option<u32>,
//     pub m_TimeParamID: Option<u32>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateKey {
    pub m_StateID: u32,
    pub m_LayerIndex: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateMachine {
    pub m_Name: String,
    pub m_DefaultState: (Option<PPtr>, Option<PPtr>),
    pub m_States: Option<Vec<PPtr>>,
    pub m_ChildStateMachine: Option<Vec<PPtr>>,
    pub m_ChildStateMachinePosition: Option<Vec<Vector3f>>,
    pub m_LocalTransitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    pub m_OrderedTransitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    pub m_MotionSetCount: Option<i32>,
    pub m_AnyStatePosition: Vector3f,
    pub m_ParentStateMachinePosition: Vector3f,
    pub m_ChildStates: Option<Vec<ChildAnimatorState>>,
    pub m_ChildStateMachines: Option<Vec<ChildAnimatorStateMachine>>,
    pub m_AnyStateTransitions: Option<Vec<PPtr>>,
    pub m_EntryTransitions: Option<Vec<PPtr>>,
    pub m_StateMachineTransitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
    pub m_StateMachineBehaviours: Option<Vec<PPtr>>,
    pub m_EntryPosition: Option<Vector3f>,
    pub m_ExitPosition: Option<Vector3f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateMachineBehaviourVectorDescription {
    pub m_StateMachineBehaviourRanges: Vec<(StateKey, StateRange)>,
    pub m_StateMachineBehaviourIndices: Vec<u32>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct StateMachineConstant {
//     pub m_StateConstantArray: Vec<OffsetPtr>,
//     pub m_AnyStateTransitionConstantArray: Vec<OffsetPtr>,
//     pub m_DefaultState: u32,
//     pub m_MotionSetCount: Option<u32>,
//     pub m_SelectorStateConstantArray: Option<Vec<OffsetPtr>>,
//     pub m_SynchronizedLayerCount: Option<u32>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateRange {
    pub m_StartIndex: u32,
    pub m_Count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaticBatchInfo {
    pub firstSubMesh: u16,
    pub subMeshCount: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamInfo {
    pub channelMask: u32,
    pub offset: u32,
    pub stride: (Option<u8>, Option<u32>),
    pub align: Option<u32>,
    pub dividerOp: Option<u8>,
    pub frequency: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamedClip {
    pub data: Vec<u32>,
    pub curveCount: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamedResource {
    pub m_Source: String,
    pub m_Offset: (Option<u64>, Option<i32>),
    pub m_Size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamingController {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_StreamingMipmapBias: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamingInfo {
    pub offset: (Option<u64>, Option<u32>),
    pub size: u32,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamingManager {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructParameter {
    pub m_NameIndex: i32,
    pub m_Index: i32,
    pub m_ArraySize: i32,
    pub m_StructSize: i32,
    pub m_VectorMembers: Vec<VectorParameter>,
    pub m_MatrixMembers: Vec<MatrixParameter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StyleSheetImporter {
    pub m_Name: String,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubCollider {
    pub m_Collider: PPtr,
    pub m_ColliderPaths: Vec<Vec<IntPoint>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubDerived {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubEmitterData {
    pub emitter: PPtr,
    pub __type: i32,
    pub properties: i32,
    pub emitProbability: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubMesh {
    pub firstByte: u32,
    pub indexCount: u32,
    pub isTriStrip: Option<u32>,
    pub triangleCount: Option<u32>,
    pub firstVertex: u32,
    pub vertexCount: u32,
    pub localAABB: AABB,
    pub topology: Option<i32>,
    pub baseVertex: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubModule {
    pub enabled: bool,
    pub subEmitterBirth: Option<PPtr>,
    pub subEmitterDeath: Option<PPtr>,
    pub subEmitterCollision: Option<PPtr>,
    pub subEmitterBirth1: Option<PPtr>,
    pub subEmitterCollision1: Option<PPtr>,
    pub subEmitterDeath1: Option<PPtr>,
    pub subEmitters: Option<Vec<SubEmitterData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubstanceArchive {
    pub m_Name: String,
    pub m_PackageData: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubstanceEnumItem {
    pub value: i32,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubstanceImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_MaterialInstances: Option<Vec<MaterialInstanceSettings>>,
    pub m_IsFirstImport: Option<i32>,
    pub m_DeletedPrototypes: Option<Vec<String>>,
    pub m_MaterialImportOutputs: Option<Vec<MaterialImportOutput>>,
    pub m_UserData: Option<String>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubstanceInput {
    pub name: String,
    pub group: Option<String>,
    pub __type: i32,
    pub value: SubstanceValue,
    pub internalType: i32,
    pub internalIndex: u32,
    pub internalIdentifier: Option<u32>,
    pub minimum: f32,
    pub maximum: f32,
    pub step: f32,
    pub flags: u32,
    pub alteredTexturesUID: Vec<u32>,
    pub enumValues: Vec<SubstanceEnumItem>,
    pub label: Option<String>,
    pub componentLabels: Option<Vec<String>>,
    pub visibleIf: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubstanceValue {
    pub texture: PPtr,
    pub scalar: (f32, f32, f32, f32),
    pub stringvalue: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SurfaceEffector2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_ColliderMask: BitField,
    pub m_Speed: f32,
    pub m_SpeedVariation: f32,
    pub m_ForceScale: Option<f32>,
    pub m_UseContactForce: Option<bool>,
    pub m_UseFriction: Option<bool>,
    pub m_UseBounce: Option<bool>,
    pub m_UseColliderMask: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagManager {
    pub tags: Vec<String>,
    pub Builtin_Layer_0: Option<String>,
    pub Builtin_Layer_1: Option<String>,
    pub Builtin_Layer_2: Option<String>,
    pub Builtin_Layer_3: Option<String>,
    pub Builtin_Layer_4: Option<String>,
    pub Builtin_Layer_5: Option<String>,
    pub Builtin_Layer_6: Option<String>,
    pub Builtin_Layer_7: Option<String>,
    pub User_Layer_8: Option<String>,
    pub User_Layer_9: Option<String>,
    pub User_Layer_10: Option<String>,
    pub User_Layer_11: Option<String>,
    pub User_Layer_12: Option<String>,
    pub User_Layer_13: Option<String>,
    pub User_Layer_14: Option<String>,
    pub User_Layer_15: Option<String>,
    pub User_Layer_16: Option<String>,
    pub User_Layer_17: Option<String>,
    pub User_Layer_18: Option<String>,
    pub User_Layer_19: Option<String>,
    pub User_Layer_20: Option<String>,
    pub User_Layer_21: Option<String>,
    pub User_Layer_22: Option<String>,
    pub User_Layer_23: Option<String>,
    pub User_Layer_24: Option<String>,
    pub User_Layer_25: Option<String>,
    pub User_Layer_26: Option<String>,
    pub User_Layer_27: Option<String>,
    pub User_Layer_28: Option<String>,
    pub User_Layer_29: Option<String>,
    pub User_Layer_30: Option<String>,
    pub User_Layer_31: Option<String>,
    pub m_SortingLayers: Option<Vec<SortingLayerEntry>>,
    pub layers: Option<Vec<String>>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct TakeInfo {
//     pub name: String,
//     pub defaultClipName: String,
//     pub startTime: f32,
//     pub stopTime: f32,
//     pub bakeStartTime: f32,
//     pub bakeStopTime: f32,
//     pub sampleRate: f32,
//     pub clip: PPtr,
//     pub internalID: Option<i64>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TargetJoint2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_EnableCollision: bool,
    pub m_ConnectedRigidBody: PPtr,
    pub m_BreakForce: f32,
    pub m_BreakTorque: f32,
    pub m_Anchor: Vector2f,
    pub m_Target: Vector2f,
    pub m_AutoConfigureTarget: bool,
    pub m_MaxForce: f32,
    pub m_DampingRatio: f32,
    pub m_Frequency: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Terrain {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_TerrainData: PPtr,
    pub m_TreeDistance: f32,
    pub m_TreeBillboardDistance: f32,
    pub m_TreeCrossFadeLength: f32,
    pub m_TreeMaximumFullLODCount: i32,
    pub m_DetailObjectDistance: f32,
    pub m_DetailObjectDensity: f32,
    pub m_HeightmapPixelError: f32,
    pub m_SplatMapDistance: f32,
    pub m_HeightmapMaximumLOD: i32,
    pub m_CastShadows: Option<bool>,
    pub m_DrawHeightmap: bool,
    pub m_DrawTreesAndFoliage: bool,
    pub m_ReflectionProbeUsage: i32,
    pub m_MaterialType: Option<i32>,
    pub m_LegacySpecular: Option<ColorRGBA>,
    pub m_LegacyShininess: Option<f32>,
    pub m_UseDefaultSmoothness: Option<bool>,
    pub m_DefaultSmoothness: Option<f32>,
    pub m_MaterialTemplate: PPtr,
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    pub m_BakeLightProbesForTrees: bool,
    pub m_DynamicUVST: Vector4f,
    pub m_ChunkDynamicUVST: Vector4f,
    pub m_ExplicitProbeSetHash: Option<Hash128>,
    pub m_PreserveTreePrototypeLayers: Option<bool>,
    pub m_DrawInstanced: Option<bool>,
    pub m_GroupingID: Option<i32>,
    pub m_AllowAutoConnect: Option<bool>,
    pub m_ShadowCastingMode: Option<i32>,
    pub m_RenderingLayerMask: Option<u32>,
    pub m_StaticShadowCaster: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerrainCollider {
    pub m_GameObject: PPtr,
    pub m_Material: Option<PPtr>,
    pub m_IsTrigger: Option<bool>,
    pub m_TerrainData: PPtr,
    pub m_CreateTreeColliders: Option<bool>,
    pub m_Enabled: Option<bool>,
    pub m_EnableTreeColliders: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerrainData {
    pub m_Name: String,
    pub m_SplatDatabase: SplatDatabase,
    pub m_DetailDatabase: DetailDatabase,
    pub m_Heightmap: Heightmap,
    pub m_PreloadShaders: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerrainLayer {
    pub m_Name: String,
    pub m_DiffuseTexture: PPtr,
    pub m_NormalMapTexture: PPtr,
    pub m_MaskMapTexture: PPtr,
    pub m_TileSize: Vector2f,
    pub m_TileOffset: Vector2f,
    pub m_Specular: ColorRGBA,
    pub m_Metallic: f32,
    pub m_Smoothness: f32,
    pub m_NormalScale: f32,
    pub m_DiffuseRemapMin: Vector4f,
    pub m_DiffuseRemapMax: Vector4f,
    pub m_MaskMapRemapMin: Vector4f,
    pub m_MaskMapRemapMax: Vector4f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectVectorPairStringBool {
    pub m_Map: Vec<(String, bool)>,
    pub m_String: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectWithSerializedAnimationCurve {
    pub m_Curve: AnimationCurve,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectWithSerializedArray {
    pub m_IntegerArray: Vec<i32>,
    pub m_ClampTestValue: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectWithSerializedMapStringBool {
    pub m_Map: Vec<(String, bool)>,
    pub m_String: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectWithSerializedMapStringNonAlignedStruct {
    pub m_Map: Vec<(String, NonAlignedStruct)>,
    pub m_String: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectWithSpecialLayoutOne {
    pub differentLayout: LayoutDataOne,
    pub sameLayout: LayoutDataOne,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestObjectWithSpecialLayoutTwo {
    pub differentLayout: LayoutDataTwo,
    pub sameLayout: LayoutDataThree,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tetrahedron {
    pub matrix: Matrix3x4f,
    pub indices: (i32, i32, i32, i32),
    pub neighbors: (i32, i32, i32, i32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextAsset {
    pub m_Name: String,
    pub m_Script: String,
    pub m_PathName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextMesh {
    pub m_GameObject: PPtr,
    pub m_Text: String,
    pub m_OffsetZ: f32,
    pub m_CharacterSize: f32,
    pub m_LineSpacing: f32,
    pub m_Anchor: i16,
    pub m_Alignment: i16,
    pub m_TabSize: f32,
    pub m_FontSize: i32,
    pub m_FontStyle: i32,
    pub m_Font: PPtr,
    pub m_RichText: Option<bool>,
    pub m_Color: Option<ColorRGBA>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextScriptImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_UserData: Option<String>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Texture {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Texture2D {
    pub m_Name: String,
    pub m_Width: i32,
    pub m_Height: i32,
    pub m_CompleteImageSize: (Option<u32>, Option<i32>),
    pub m_TextureFormat: i32,
    pub m_MipMap: Option<bool>,
    pub m_IsReadable: bool,
    pub m_ReadAllowed: Option<bool>,
    pub m_ImageCount: i32,
    pub m_TextureDimension: i32,
    pub m_TextureSettings: GLTextureSettings,
    pub m_LightmapFormat: i32,
    pub image_data: Vec<u8>,
    pub m_ColorSpace: Option<i32>,
    pub m_MipCount: Option<i32>,
    pub m_StreamData: Option<StreamingInfo>,
    pub m_ForcedFallbackFormat: Option<i32>,
    pub m_DownscaleFallback: Option<bool>,
    pub m_StreamingMipmaps: Option<bool>,
    pub m_StreamingMipmapsPriority: Option<i32>,
    pub m_IgnoreMasterTextureLimit: Option<bool>,
    pub m_IsPreProcessed: Option<bool>,
    pub m_MipsStripped: Option<i32>,
    pub m_IsAlphaChannelOptional: Option<bool>,
    pub m_PlatformBlob: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Texture2DArray {
    pub m_Name: String,
    pub m_Width: i32,
    pub m_Height: i32,
    pub m_Depth: i32,
    pub m_Format: i32,
    pub m_MipCount: i32,
    pub m_DataSize: u32,
    pub m_TextureSettings: GLTextureSettings,
    pub m_ColorSpace: i32,
    pub m_IsReadable: bool,
    pub image_data: Vec<u8>,
    pub m_StreamData: Option<StreamingInfo>,
    pub m_ForcedFallbackFormat: Option<i32>,
    pub m_DownscaleFallback: Option<bool>,
    pub m_IsAlphaChannelOptional: Option<bool>,
    pub m_UsageMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Texture3D {
    pub m_Name: String,
    pub m_Width: i32,
    pub m_Height: i32,
    pub m_Depth: Option<i32>,
    pub m_Format: Option<(Option<u32>, Option<i32>)>,
    pub m_MipMap: Option<bool>,
    pub m_DataSize: Option<u32>,
    pub m_TextureSettings: GLTextureSettings,
    pub image_data: Vec<u8>,
    pub m_CompleteImageSize: Option<i32>,
    pub m_TextureFormat: Option<i32>,
    pub m_IsReadable: Option<bool>,
    pub m_ReadAllowed: Option<bool>,
    pub m_ImageCount: Option<i32>,
    pub m_TextureDimension: Option<i32>,
    pub m_LightmapFormat: Option<i32>,
    pub m_ColorSpace: Option<i32>,
    pub m_MipCount: Option<i32>,
    pub m_StreamData: Option<StreamingInfo>,
    pub m_ForcedFallbackFormat: Option<i32>,
    pub m_DownscaleFallback: Option<bool>,
    pub m_IsAlphaChannelOptional: Option<bool>,
    pub m_UsageMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureImportInstructions {
    pub compressedFormat: i32,
    pub uncompressedFormat: i32,
    pub recommendedFormat: Option<i32>,
    pub usageMode: i32,
    pub colorSpace: i32,
    pub width: i32,
    pub height: i32,
    pub compressionQuality: i32,
    pub desiredFormat: Option<i32>,
    pub androidETC2FallbackFormat: Option<i32>,
    pub androidETC2FallbackDownscale: Option<bool>,
    pub vtOnly: Option<bool>,
    pub depth: Option<i32>,
    pub cubeIntermediateSize: Option<i32>,
    pub cubeMode: Option<i32>,
    pub cubeLayout: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureImportOutput {
    pub textureImportInstructions: TextureImportInstructions,
    pub sourceTextureInformation: SourceTextureInformation,
    pub importInspectorWarnings: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<(Option<Vec<(i64, String)>>, Option<Vec<(i32, String)>>)>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_MipMapMode: i32,
    pub m_EnableMipMap: i32,
    pub m_CorrectGamma: Option<i32>,
    pub m_FadeOut: i32,
    pub m_BorderMipMap: i32,
    pub m_MipMapFadeDistanceStart: i32,
    pub m_MipMapFadeDistanceEnd: i32,
    pub m_ConvertToNormalMap: i32,
    pub m_ExternalNormalMap: i32,
    pub m_IsReadable: i32,
    pub m_HeightScale: f32,
    pub m_NormalMapFilter: i32,
    pub m_GrayScaleToAlpha: i32,
    pub m_GenerateCubemap: i32,
    pub m_TextureFormat: i32,
    pub m_MaxTextureSize: i32,
    pub m_TextureSettings: GLTextureSettings,
    pub m_NPOTScale: i32,
    pub m_Lightmap: i32,
    pub m_TextureType: i32,
    pub m_RecommendedTextureFormat: Option<i32>,
    pub m_SourceTextureInformation: Option<SourceTextureInformation>,
    pub m_BuildTargetSettings: Option<Vec<BuildTargetSettings>>,
    pub m_LinearTexture: Option<i32>,
    pub correctGamma: Option<i32>,
    pub m_CompressionQuality: Option<i32>,
    pub m_SeamlessCubemap: Option<i32>,
    pub m_Output: Option<TextureImportOutput>,
    pub m_UserData: Option<String>,
    pub m_AlphaIsTransparency: Option<i32>,
    pub m_SpriteMode: Option<i32>,
    pub m_SpriteExtrude: Option<u32>,
    pub m_SpriteMeshType: Option<i32>,
    pub m_Alignment: Option<i32>,
    pub m_SpritePivot: Option<Vector2f>,
    pub m_SpritePixelsToUnits: Option<f32>,
    pub m_SpriteSheet: Option<SpriteSheetMetaData>,
    pub m_SpritePackingTag: Option<String>,
    pub m_SpriteBorder: Option<Vector4f>,
    pub m_CubemapConvolution: Option<i32>,
    pub m_CubemapConvolutionSteps: Option<i32>,
    pub m_CubemapConvolutionExponent: Option<f32>,
    pub m_RGBM: Option<i32>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_AllowsAlphaSplitting: Option<i32>,
    pub m_SpriteTessellationDetail: Option<f32>,
    pub m_sRGBTexture: Option<i32>,
    pub m_AlphaUsage: Option<i32>,
    pub m_TextureShape: Option<i32>,
    pub m_MaxTextureSizeSet: Option<i32>,
    pub m_CompressionQualitySet: Option<i32>,
    pub m_TextureFormatSet: Option<i32>,
    pub m_PlatformSettings: Option<(
        Option<Vec<PlatformSettings>>,
        Option<Vec<TextureImporterPlatformSettings>>,
    )>,
    pub m_MipMapsPreserveCoverage: Option<i32>,
    pub m_AlphaTestReferenceValue: Option<f32>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_SpriteGenerateFallbackPhysicsShape: Option<i32>,
    pub m_SingleChannelComponent: Option<i32>,
    pub m_StreamingMipmaps: Option<i32>,
    pub m_StreamingMipmapsPriority: Option<i32>,
    pub m_PushPullDilation: Option<i32>,
    pub m_PSDRemoveMatte: Option<bool>,
    pub m_PSDShowRemoveMatteOption: Option<bool>,
    pub m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
    pub m_ApplyGammaDecoding: Option<i32>,
    pub m_IgnorePngGamma: Option<(Option<bool>, Option<i32>)>,
    pub m_VTOnly: Option<i32>,
    pub m_FlipbookRows: Option<i32>,
    pub m_FlipbookColumns: Option<i32>,
    pub m_IgnoreMasterTextureLimit: Option<i32>,
    pub m_FlipGreenChannel: Option<i32>,
    pub m_Swizzle: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureImporterPlatformSettings {
    pub m_BuildTarget: String,
    pub m_MaxTextureSize: i32,
    pub m_ResizeAlgorithm: i32,
    pub m_TextureFormat: i32,
    pub m_TextureCompression: i32,
    pub m_CompressionQuality: i32,
    pub m_CrunchedCompression: bool,
    pub m_AllowsAlphaSplitting: bool,
    pub m_Overridden: bool,
    pub m_AndroidETC2FallbackOverride: i32,
    pub m_ForceMaximumCompressionQuality_BC6H_BC7: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureParameter {
    pub m_NameIndex: i32,
    pub m_Index: i32,
    pub m_SamplerIndex: i32,
    pub m_Dim: i8,
    pub m_MultiSampled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureParameters {
    pub width: i32,
    pub height: i32,
    pub mipLevels: i32,
    pub textureFormat: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextureSettings {
    pub anisoLevel: i32,
    pub compressionQuality: i32,
    pub maxTextureSize: i32,
    pub textureCompression: i32,
    pub filterMode: i32,
    pub generateMipMaps: bool,
    pub readable: bool,
    pub crunchedCompression: bool,
    pub sRGB: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TierGraphicsSettings {
    pub renderingPath: i32,
    pub useCascadedShadowMaps: bool,
    pub hdrMode: Option<i32>,
    pub useHDR: Option<bool>,
    pub realtimeGICPUUsage: Option<i32>,
    pub enableLPPV: Option<bool>,
    pub prefer32BitShadowMaps: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tile {
    pub m_TileIndex: u32,
    pub m_TileSpriteIndex: u32,
    pub m_TileMatrixIndex: (Option<u32>, Option<u16>),
    pub m_TileColorIndex: (Option<u32>, Option<u16>),
    pub m_ObjectToInstantiate: Option<PPtr>,
    pub m_TileFlags: Option<(Option<u32>, Option<i32>)>,
    pub m_ColliderType: Option<i32>,
    pub m_TileObjectToInstantiateIndex: Option<u16>,
    pub m_AllTileFlags: Option<u32>,
    pub dummyAlignment: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TileAnimationData {
    pub m_AnimatedSprites: Vec<PPtr>,
    pub m_AnimationSpeed: f32,
    pub m_AnimationTimeOffset: f32,
    pub m_IsLooping: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tilemap {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Tiles: Vec<(int3_storage, Tile)>,
    pub m_AnimatedTiles: Vec<(int3_storage, TileAnimationData)>,
    pub m_TileAssetArray: Vec<TilemapRefCountedData>,
    pub m_TileSpriteArray: Vec<TilemapRefCountedData>,
    pub m_TileMatrixArray: Vec<TilemapRefCountedData>,
    pub m_TileColorArray: Vec<TilemapRefCountedData>,
    pub m_AnimationFrameRate: f32,
    pub m_Color: ColorRGBA,
    pub m_Origin: int3_storage,
    pub m_Size: int3_storage,
    pub m_TileAnchor: Vector3f,
    pub m_TileOrientation: i32,
    pub m_TileOrientationMatrix: Matrix4x4f,
    pub m_TileObjectToInstantiateArray: Option<Vec<TilemapRefCountedData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TilemapCollider2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Density: f32,
    pub m_Material: PPtr,
    pub m_IsTrigger: bool,
    pub m_UsedByEffector: bool,
    pub m_UsedByComposite: bool,
    pub m_Offset: Vector2f,
    pub m_MaximumTileChangeCount: Option<u32>,
    pub m_ExtrusionFactor: Option<f32>,
    pub m_UseDelaunayMesh: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TilemapEditorUserSettings {
    pub m_LastUsedPalette: PPtr,
    pub m_FocusMode: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TilemapRefCountedData {
    pub m_RefCount: u32,
    pub m_Data: (
        Option<PPtr>,
        Option<PPtr>,
        Option<Matrix4x4f>,
        Option<ColorRGBA>,
        Option<PPtr>,
    ),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TilemapRenderer {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: u8,
    pub m_ReceiveShadows: u8,
    pub m_DynamicOccludee: u8,
    pub m_MotionVectors: u8,
    pub m_LightProbeUsage: u8,
    pub m_ReflectionProbeUsage: u8,
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    pub m_Materials: Vec<PPtr>,
    pub m_StaticBatchInfo: StaticBatchInfo,
    pub m_StaticBatchRoot: PPtr,
    pub m_ProbeAnchor: PPtr,
    pub m_LightProbeVolumeOverride: PPtr,
    pub m_SortingLayerID: i32,
    pub m_SortingLayer: i16,
    pub m_SortingOrder: i16,
    pub m_ChunkSize: int3_storage,
    pub m_MaxChunkCount: u32,
    pub m_MaxFrameAge: u32,
    pub m_SortOrder: i32,
    pub m_MaskInteraction: i32,
    pub m_ChunkCullingBounds: Option<Vector3f>,
    pub m_RenderingLayerMask: Option<u32>,
    pub m_DetectChunkCullingBounds: Option<i32>,
    pub m_RendererPriority: Option<i32>,
    pub m_Mode: Option<i32>,
    pub m_RayTracingMode: Option<u8>,
    pub m_RayTraceProcedural: Option<u8>,
    pub m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeManager {
    pub Fixed_Timestep: f32,
    pub Maximum_Allowed_Timestep: f32,
    pub m_TimeScale: f32,
    pub Maximum_Particle_Timestep: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrailModule {
    pub enabled: bool,
    pub ratio: f32,
    pub lifetime: MinMaxCurve,
    pub minVertexDistance: f32,
    pub textureMode: i32,
    pub worldSpace: bool,
    pub dieWithParticles: bool,
    pub sizeAffectsWidth: bool,
    pub sizeAffectsLifetime: bool,
    pub inheritParticleColor: bool,
    pub colorOverLifetime: MinMaxGradient,
    pub widthOverTrail: MinMaxCurve,
    pub colorOverTrail: MinMaxGradient,
    pub generateLightingData: Option<bool>,
    pub mode: Option<i32>,
    pub ribbonCount: Option<i32>,
    pub splitSubEmitterRibbons: Option<bool>,
    pub shadowBias: Option<f32>,
    pub attachRibbonsToTransform: Option<bool>,
    pub textureScale: Option<Vector2f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrailRenderer {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: (Option<u8>, Option<bool>),
    pub m_ReceiveShadows: (Option<u8>, Option<bool>),
    pub m_LightmapIndex: (Option<u8>, Option<u16>),
    pub m_LightmapTilingOffset: Vector4f,
    pub m_Materials: Vec<PPtr>,
    pub m_SubsetIndices: Option<Vec<u32>>,
    pub m_StaticBatchRoot: PPtr,
    pub m_Time: f32,
    pub m_StartWidth: Option<f32>,
    pub m_EndWidth: Option<f32>,
    pub m_Colors: Option<Gradient>,
    pub m_MinVertexDistance: f32,
    pub m_Autodestruct: bool,
    pub m_UseLightProbes: Option<bool>,
    pub m_LightProbeAnchor: Option<PPtr>,
    pub m_SortingLayer: Option<i16>,
    pub m_SortingOrder: Option<i16>,
    pub m_SortingLayerID: Option<(Option<u32>, Option<i32>)>,
    pub m_LightmapIndexDynamic: Option<u16>,
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    pub m_ReflectionProbeUsage: Option<(Option<u8>, Option<i32>)>,
    pub m_ProbeAnchor: Option<PPtr>,
    pub m_MotionVectors: Option<u8>,
    pub m_LightProbeUsage: Option<u8>,
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    pub m_Parameters: Option<LineParameters>,
    pub m_DynamicOccludee: Option<u8>,
    pub m_RenderingLayerMask: Option<u32>,
    pub m_Emitting: Option<bool>,
    pub m_RendererPriority: Option<i32>,
    pub m_RayTracingMode: Option<u8>,
    pub m_RayTraceProcedural: Option<u8>,
    pub m_StaticShadowCaster: Option<u8>,
    pub m_MaskInteraction: Option<i32>,
    pub m_ApplyActiveColorSpace: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transform {
    pub m_GameObject: PPtr,
    pub m_LocalRotation: Quaternionf,
    pub m_LocalPosition: Vector3f,
    pub m_LocalScale: Vector3f,
    pub m_Children: Vec<PPtr>,
    pub m_Father: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransformMaskElement {
    pub m_Path: String,
    pub m_Weight: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transition {
    pub m_Name: String,
    pub m_SrcState: Option<PPtr>,
    pub m_DstState: (Option<PPtr>, Option<PPtr>),
    pub m_TransitionDuration: f32,
    pub m_TransitionOffset: f32,
    pub m_Conditions: (Option<Vec<AnimatorCondition>>, Option<Vec<Condition>>),
    pub m_Atomic: Option<bool>,
    pub m_Solo: bool,
    pub m_Mute: bool,
    pub m_CanTransitionToSelf: Option<bool>,
    pub m_DstStateMachine: Option<PPtr>,
    pub m_IsExit: Option<bool>,
    pub m_ExitTime: Option<f32>,
    pub m_HasExitTime: Option<bool>,
    pub m_InterruptionSource: Option<i32>,
    pub m_OrderedInterruption: Option<bool>,
    pub m_HasFixedDuration: Option<bool>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct TransitionConstant {
//     pub m_ConditionConstantArray: Vec<OffsetPtr>,
//     pub m_DestinationState: u32,
//     pub m_ID: u32,
//     pub m_UserID: u32,
//     pub m_TransitionDuration: f32,
//     pub m_TransitionOffset: f32,
//     pub m_Atomic: Option<bool>,
//     pub m_CanTransitionToSelf: Option<bool>,
//     pub m_FullPathID: Option<u32>,
//     pub m_ExitTime: Option<f32>,
//     pub m_HasExitTime: Option<bool>,
//     pub m_InterruptionSource: Option<i32>,
//     pub m_OrderedInterruption: Option<bool>,
//     pub m_HasFixedDuration: Option<bool>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tree {
    pub m_GameObject: PPtr,
    pub m_SpeedTreeWindAsset: Option<PPtr>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TreeInstance {
    pub position: Vector3f,
    pub widthScale: f32,
    pub heightScale: f32,
    pub color: ColorRGBA,
    pub lightmapColor: ColorRGBA,
    pub index: i32,
    pub rotation: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TreePrototype {
    pub prefab: PPtr,
    pub bendFactor: f32,
    pub navMeshLod: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerModule {
    pub enabled: bool,
    pub collisionShape0: Option<PPtr>,
    pub collisionShape1: Option<PPtr>,
    pub collisionShape2: Option<PPtr>,
    pub collisionShape3: Option<PPtr>,
    pub collisionShape4: Option<PPtr>,
    pub collisionShape5: Option<PPtr>,
    pub inside: i32,
    pub outside: i32,
    pub enter: i32,
    pub exit: i32,
    pub radiusScale: f32,
    pub colliderQueryMode: Option<i32>,
    pub primitives: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrueTypeFontImporter {
    pub m_Name: String,
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    pub m_OldHashIdentity: Option<MdFour>,
    pub m_NewHashIdentity: Option<MdFour>,
    pub m_FontSize: i32,
    pub m_ForceTextureCase: i32,
    pub m_RenderMode: Option<i32>,
    pub m_Style: Option<i32>,
    pub m_IncludeFontData: bool,
    pub m_Use2xBehaviour: Option<bool>,
    pub m_FontNames: Vec<String>,
    pub m_FontColor: Option<ColorRGBA>,
    pub m_CustomCharacters: Option<String>,
    pub m_CharacterSpacing: Option<i32>,
    pub m_CharacterPadding: Option<i32>,
    pub m_FontRenderingMode: Option<i32>,
    pub m_Output: Option<Output>,
    pub m_UserData: Option<String>,
    pub m_AssetBundleName: Option<String>,
    pub m_AssetBundleVariant: Option<String>,
    pub m_FallbackFontReferences: Option<Vec<PPtr>>,
    pub m_AscentCalculationMode: Option<i32>,
    pub m_FontName: Option<String>,
    pub m_UseLegacyBoundsCalculation: Option<bool>,
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_ShouldRoundAdvanceValue: Option<bool>,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UAVParameter {
    pub m_NameIndex: i32,
    pub m_Index: i32,
    pub m_OriginalIndex: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UIRenderer {
    pub m_GameObject: PPtr,
    pub m_CullTransparentMesh: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UVAnimation {
    pub x_Tile: i32,
    pub y_Tile: i32,
    pub cycles: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UVModule {
    pub enabled: bool,
    pub frameOverTime: MinMaxCurve,
    pub tilesX: i32,
    pub tilesY: i32,
    pub animationType: i32,
    pub rowIndex: i32,
    pub cycles: f32,
    pub randomRow: Option<bool>,
    pub startFrame: Option<MinMaxCurve>,
    pub uvChannelMask: Option<i32>,
    pub flipU: Option<f32>,
    pub flipV: Option<f32>,
    pub mode: Option<i32>,
    pub sprites: Option<Vec<SpriteData>>,
    pub timeMode: Option<i32>,
    pub fps: Option<f32>,
    pub speedRange: Option<Vector2f>,
    pub rowMode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityAdsSettings {
    pub m_Enabled: Option<bool>,
    pub m_InitializeOnStartup: Option<bool>,
    pub m_TestMode: Option<bool>,
    pub m_EnabledPlatforms: Option<u32>,
    pub m_IosGameId: Option<String>,
    pub m_AndroidGameId: Option<String>,
    pub m_GameId: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityAnalyticsManager {
    pub m_Enabled: Option<bool>,
    pub m_InitializeOnStartup: Option<bool>,
    pub m_TestMode: Option<bool>,
    pub m_TestEventUrl: Option<String>,
    pub m_TestConfigUrl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityAnalyticsSettings {
    pub m_Enabled: bool,
    pub m_InitializeOnStartup: bool,
    pub m_TestMode: bool,
    pub m_TestEventUrl: Option<String>,
    pub m_TestConfigUrl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityConnectSettings {
    pub UnityPurchasingSettings: UnityPurchasingSettings,
    pub UnityAnalyticsSettings: UnityAnalyticsSettings,
    pub m_Enabled: Option<bool>,
    pub m_TestMode: Option<bool>,
    pub m_TestEventUrl: Option<String>,
    pub m_TestConfigUrl: Option<String>,
    pub CrashReportingSettings: Option<CrashReportingSettings>,
    pub UnityAdsSettings: Option<UnityAdsSettings>,
    pub PerformanceReportingSettings: Option<PerformanceReportingSettings>,
    pub m_TestInitMode: Option<i32>,
    pub m_EventOldUrl: Option<String>,
    pub m_EventUrl: Option<String>,
    pub m_ConfigUrl: Option<String>,
    pub m_DashboardUrl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityPropertySheet {
    pub m_TexEnvs: (
        Option<Vec<(String, UnityTexEnv)>>,
        Option<Vec<(FastPropertyName, UnityTexEnv)>>,
    ),
    pub m_Floats: (
        Option<Vec<(FastPropertyName, f32)>>,
        Option<Vec<(String, f32)>>,
    ),
    pub m_Colors: (
        Option<Vec<(String, ColorRGBA)>>,
        Option<Vec<(FastPropertyName, ColorRGBA)>>,
    ),
    pub m_Ints: Option<Vec<(String, i32)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityPurchasingSettings {
    pub m_Enabled: bool,
    pub m_TestMode: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnityTexEnv {
    pub m_Texture: PPtr,
    pub m_Scale: Vector2f,
    pub m_Offset: Vector2f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateZoneInfo {
    pub updateZoneCenter: Vector3f,
    pub updateZoneSize: Vector3f,
    pub rotation: f32,
    pub passIndex: i32,
    pub needSwap: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXCPUBufferData {
    pub data: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXCPUBufferDesc {
    pub capacity: u32,
    pub stride: u32,
    pub layout: Vec<VFXLayoutElementDesc>,
    pub initialData: VFXCPUBufferData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXEditorSystemDesc {
    pub __type: i32,
    pub flags: i32,
    pub capacity: u32,
    pub layer: u32,
    pub buffers: Vec<VFXMapping>,
    pub values: Vec<VFXMapping>,
    pub tasks: Vec<VFXEditorTaskDesc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXEditorTaskDesc {
    pub __type: i32,
    pub buffers: Vec<VFXMapping>,
    pub values: Vec<VFXMapping>,
    pub params: Vec<VFXMapping>,
    pub processor: PPtr,
    pub shaderSourceIndex: i32,
    pub temporaryBuffers: Option<Vec<VFXMappingTemporary>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXEntryExposed {
    pub m_Value: (
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
    pub m_Name: String,
    pub m_Overridden: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXEntryExpressionValue {
    pub m_ExpressionIndex: u32,
    pub m_Value: (
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
    pub name: String,
    pub playSystems: Vec<u32>,
    pub stopSystems: Vec<u32>,
    pub initSystems: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXExpressionContainer {
    pub m_Expressions: Vec<Expression>,
    pub m_NeedsLocalToWorld: bool,
    pub m_NeedsWorldToLocal: bool,
    pub m_NeededMainCameraBuffers: Option<i32>,
    pub m_MaxCommonExpressionsIndex: Option<u32>,
    pub m_NeedsMainCamera: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXField {
    pub m_Array: (
        Option<Vec<VFXEntryExpressionValue>>,
        Option<Vec<VFXEntryExposed>>,
    ),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXGPUBufferDesc {
    pub __type: i32,
    pub size: u32,
    pub layout: Vec<VFXLayoutElementDesc>,
    pub capacity: u32,
    pub stride: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXLayoutElementDesc {
    pub name: String,
    pub __type: i32,
    pub offset: VFXLayoutOffset,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXLayoutOffset {
    pub bucket: u32,
    pub structure: u32,
    pub element: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXManager {
    pub m_IndirectShader: PPtr,
    pub m_CopyBufferShader: PPtr,
    pub m_SortShader: PPtr,
    pub m_RenderPipeSettingsPath: String,
    pub m_FixedTimeStep: f32,
    pub m_MaxDeltaTime: f32,
    pub m_StripUpdateShader: Option<PPtr>,
    pub m_CompiledVersion: Option<u32>,
    pub m_RuntimeVersion: Option<u32>,
    pub m_RuntimeResources: Option<PPtr>,
    pub m_MaxScrubTime: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXMapping {
    pub nameId: String,
    pub index: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXMappingTemporary {
    pub mapping: VFXMapping,
    pub pastFrameIndex: u32,
    pub perCameraBuffer: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXPropertySheetSerializedBase {
    pub m_Float: VFXField,
    pub m_Vector2f: VFXField,
    pub m_Vector3f: VFXField,
    pub m_Vector4f: VFXField,
    pub m_Uint: VFXField,
    pub m_Int: VFXField,
    pub m_Matrix4x4f: VFXField,
    pub m_AnimationCurve: VFXField,
    pub m_Gradient: VFXField,
    pub m_NamedObject: VFXField,
    pub m_Bool: VFXField,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXRenderer {
    pub m_GameObject: PPtr,
    pub m_Enabled: bool,
    pub m_CastShadows: u8,
    pub m_ReceiveShadows: u8,
    pub m_DynamicOccludee: u8,
    pub m_MotionVectors: u8,
    pub m_LightProbeUsage: u8,
    pub m_ReflectionProbeUsage: u8,
    pub m_RenderingLayerMask: u32,
    pub m_RendererPriority: i32,
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    pub m_Materials: Option<Vec<PPtr>>,
    pub m_StaticBatchInfo: StaticBatchInfo,
    pub m_StaticBatchRoot: PPtr,
    pub m_ProbeAnchor: PPtr,
    pub m_LightProbeVolumeOverride: PPtr,
    pub m_SortingLayerID: i32,
    pub m_SortingLayer: i16,
    pub m_SortingOrder: i16,
    pub m_RayTracingMode: Option<u8>,
    pub m_RayTraceProcedural: Option<u8>,
    pub m_StaticShadowCaster: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXRendererSettings {
    pub motionVectorGenerationMode: i32,
    pub shadowCastingMode: i32,
    pub receiveShadows: bool,
    pub reflectionProbeUsage: i32,
    pub lightProbeUsage: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXShaderSourceDesc {
    pub compute: bool,
    pub name: String,
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXSystemDesc {
    pub __type: i32,
    pub flags: i32,
    pub capacity: u32,
    pub layer: u32,
    pub buffers: Vec<VFXMapping>,
    pub values: Vec<VFXMapping>,
    pub tasks: Vec<VFXTaskDesc>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXTaskDesc {
    pub __type: i32,
    pub buffers: Vec<VFXMapping>,
    pub values: Vec<VFXMapping>,
    pub params: Vec<VFXMapping>,
    pub processor: PPtr,
    pub temporaryBuffers: Option<Vec<VFXMappingTemporary>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VFXTemporaryGPUBufferDesc {
    pub desc: VFXGPUBufferDesc,
    pub frameCount: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VRSettings {
    pub none: Option<DeviceNone>,
    pub cardboard: Option<Google>,
    pub daydream: Option<Google>,
    pub hololens: Option<HoloLens>,
    pub oculus: Option<Oculus>,
    pub enable360StereoCapture: Option<bool>,
    pub lumin: Option<Lumin>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueArray {
    pub m_BoolValues: Vec<bool>,
    pub m_IntValues: Vec<i32>,
    pub m_FloatValues: Vec<f32>,
    pub m_VectorValues: Option<Vec<float4>>,
    pub m_PositionValues: Option<(Option<Vec<float4>>, Option<Vec<float3>>)>,
    pub m_QuaternionValues: Option<Vec<float4>>,
    pub m_ScaleValues: Option<(Option<Vec<float4>>, Option<Vec<float3>>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueArrayConstant {
    pub m_ValueArray: Vec<ValueConstant>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueConstant {
    pub m_ID: u32,
    pub m_TypeID: Option<u32>,
    pub m_Type: u32,
    pub m_Index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueDelta {
    pub m_Start: f32,
    pub m_Stop: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariableBoneCountWeights {
    pub m_Data: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariantInfo {
    pub keywords: String,
    pub passType: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector3Curve {
    pub curve: AnimationCurve,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VectorParameter {
    pub m_NameIndex: i32,
    pub m_Index: i32,
    pub m_ArraySize: i32,
    pub m_Type: i8,
    pub m_Dim: i8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VelocityModule {
    pub enabled: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    pub inWorldSpace: bool,
    pub speedModifier: Option<MinMaxCurve>,
    pub orbitalX: Option<MinMaxCurve>,
    pub orbitalY: Option<MinMaxCurve>,
    pub orbitalZ: Option<MinMaxCurve>,
    pub orbitalOffsetX: Option<MinMaxCurve>,
    pub orbitalOffsetY: Option<MinMaxCurve>,
    pub orbitalOffsetZ: Option<MinMaxCurve>,
    pub radial: Option<MinMaxCurve>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionControlSettings {
    pub m_Mode: String,
    pub m_CollabEditorSettings: CollabEditorSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VertexData {
    pub m_CurrentChannels: Option<(Option<u32>, Option<i32>)>,
    pub m_VertexCount: u32,
    pub m_DataSize: Vec<u8>,
    pub m_Streams: Option<(
        Option<(StreamInfo, StreamInfo, StreamInfo, StreamInfo)>,
        Option<Vec<StreamInfo>>,
    )>,
    pub m_Channels: Option<Vec<ChannelInfo>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoBuildInfo {
    pub m_VideoClipCount: i32,
    pub m_IsVideoModuleDisabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoClip {
    pub m_Name: String,
    pub m_OriginalPath: String,
    pub m_ProxyWidth: u32,
    pub m_ProxyHeight: u32,
    pub Width: u32,
    pub Height: u32,
    pub m_FrameRate: f64,
    pub m_FrameCount: u64,
    pub m_Format: i32,
    pub m_AudioChannelCount: Vec<u16>,
    pub m_AudioSampleRate: Vec<u32>,
    pub m_AudioLanguage: Vec<String>,
    pub m_ExternalResources: StreamedResource,
    pub m_HasSplitAlpha: bool,
    pub m_PixelAspecRatioNum: Option<u32>,
    pub m_PixelAspecRatioDen: Option<u32>,
    pub m_sRGB: Option<bool>,
    pub m_VideoShaders: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoClipImporterOutput {
    pub format: Option<i32>,
    pub settings: Option<VideoClipImporterTargetSettings>,
    pub encodedWidth: Option<i32>,
    pub encodedHeight: Option<i32>,
    pub encodedStartFrame: Option<i32>,
    pub encodedEndFrame: Option<i32>,
    pub streamedResource: Option<StreamedResource>,
    pub sourceFrameRate: Option<f64>,
    pub sourceFileSize: Option<u64>,
    pub sourceAudioChannelCount: Option<Vec<u16>>,
    pub sourceHasAlpha: Option<bool>,
    pub sourceAudioSampleRate: Option<Vec<u32>>,
    pub originalWidth: Option<i32>,
    pub originalHeight: Option<i32>,
    pub originalFrameCount: Option<i32>,
    pub encodedSettings: Option<VideoClipImporterTargetSettings>,
    pub sourcePixelAspectRatioNumerator: Option<u32>,
    pub sourcePixelAspectRatioDenominator: Option<u32>,
    pub transcodeSkipped: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoClipImporterTargetSettings {
    pub enableTranscoding: bool,
    pub codec: i32,
    pub resizeFormat: i32,
    pub aspectRatio: i32,
    pub customWidth: i32,
    pub customHeight: i32,
    pub bitrateMode: i32,
    pub spatialQuality: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoPlayer {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_VideoClip: PPtr,
    pub m_TargetCameraAlpha: f32,
    pub m_TargetCamera: PPtr,
    pub m_TargetTexture: PPtr,
    pub m_TargetMaterialRenderer: PPtr,
    pub m_RenderMode: i32,
    pub m_AspectRatio: i32,
    pub m_DataSource: i32,
    pub m_PlaybackSpeed: f32,
    pub m_AudioOutputMode: i32,
    pub m_TargetAudioSources: Vec<PPtr>,
    pub m_DirectAudioVolumes: Vec<f32>,
    pub m_Url: String,
    pub m_TargetMaterialName: Option<String>,
    pub m_TargetMaterialProperty: String,
    pub m_EnabledAudioTracks: Vec<bool>,
    pub m_DirectAudioMutes: Vec<bool>,
    pub m_ControlledAudioTrackCount: u16,
    pub m_PlayOnAwake: bool,
    pub m_SkipOnDrop: bool,
    pub m_Looping: bool,
    pub m_WaitForFirstFrame: bool,
    pub m_FrameReadyEventEnabled: bool,
    pub m_TimeReference: Option<i32>,
    pub m_TargetCamera3DLayout: Option<i32>,
    pub m_VideoShaders: Option<Vec<PPtr>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffect {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Asset: PPtr,
    pub m_StartSeed: u32,
    pub m_ResetSeedOnPlay: (Option<u8>, Option<bool>),
    pub m_PropertySheet: VFXPropertySheetSerializedBase,
    pub m_InitialEventName: Option<String>,
    pub m_InitialEventNameOverriden: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectAsset {
    pub m_Name: String,
    pub m_Infos: VisualEffectInfo,
    pub m_Systems: Vec<VFXSystemDesc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectImporter {
    pub m_Name: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_UserData: String,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectInfo {
    pub m_Expressions: VFXExpressionContainer,
    pub m_PropertySheet: VFXPropertySheetSerializedBase,
    pub m_ExposedExpressions: Vec<VFXMapping>,
    pub m_Buffers: Vec<VFXGPUBufferDesc>,
    pub m_CPUBuffers: Vec<VFXCPUBufferDesc>,
    pub m_Events: Vec<VFXEventDesc>,
    pub m_RendererSettings: VFXRendererSettings,
    pub m_CullingFlags: i32,
    pub m_UpdateMode: i32,
    pub m_RuntimeVersion: Option<u32>,
    pub m_PreWarmDeltaTime: Option<f32>,
    pub m_PreWarmStepCount: Option<u32>,
    pub m_TemporaryBuffers: Option<Vec<VFXTemporaryGPUBufferDesc>>,
    pub m_InitialEventName: Option<String>,
    pub m_CompilationVersion: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectObject {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectResource {
    pub m_Name: String,
    pub m_Graph: PPtr,
    pub m_ShaderSources: Option<Vec<VFXShaderSourceDesc>>,
    pub m_Infos: (Option<VisualEffectInfo>, Option<VisualEffectSettings>),
    pub m_Systems: Option<Vec<VFXEditorSystemDesc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectSettings {
    pub m_RendererSettings: VFXRendererSettings,
    pub m_CullingFlags: i32,
    pub m_UpdateMode: i32,
    pub m_PreWarmDeltaTime: f32,
    pub m_PreWarmStepCount: u32,
    pub m_InitialEventName: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectSubgraph {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectSubgraphBlock {
    pub m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualEffectSubgraphOperator {
    pub m_Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebCamTexture {
    pub m_Name: String,
    pub m_ForcedFallbackFormat: Option<i32>,
    pub m_DownscaleFallback: Option<bool>,
    pub m_IsAlphaChannelOptional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WheelCollider {
    pub m_GameObject: PPtr,
    pub m_Center: Vector3f,
    pub m_Radius: f32,
    pub m_SuspensionDistance: f32,
    pub m_SuspensionSpring: JointSpring,
    pub m_Mass: f32,
    pub m_ForwardFriction: WheelFrictionCurve,
    pub m_SidewaysFriction: WheelFrictionCurve,
    pub m_Enabled: Option<bool>,
    pub m_ForceAppPointDistance: Option<f32>,
    pub m_WheelDampingRate: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WheelFrictionCurve {
    pub extremumSlip: Option<f32>,
    pub extremumValue: Option<f32>,
    pub asymptoteSlip: Option<f32>,
    pub asymptoteValue: Option<f32>,
    pub stiffnessFactor: Option<f32>,
    pub m_ExtremumSlip: Option<f32>,
    pub m_ExtremumValue: Option<f32>,
    pub m_AsymptoteSlip: Option<f32>,
    pub m_AsymptoteValue: Option<f32>,
    pub m_Stiffness: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WheelJoint2D {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_CollideConnected: Option<bool>,
    pub m_ConnectedRigidBody: PPtr,
    pub m_Anchor: Vector2f,
    pub m_ConnectedAnchor: Vector2f,
    pub m_Suspension: JointSuspension2D,
    pub m_UseMotor: bool,
    pub m_Motor: JointMotor2D,
    pub m_EnableCollision: Option<bool>,
    pub m_BreakForce: Option<f32>,
    pub m_BreakTorque: Option<f32>,
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindZone {
    pub m_GameObject: PPtr,
    pub m_Enabled: u8,
    pub m_Mode: i32,
    pub m_Radius: f32,
    pub m_WindMain: f32,
    pub m_WindTurbulence: f32,
    pub m_WindPulseMagnitude: f32,
    pub m_WindPulseFrequency: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldAnchor {
    pub m_GameObject: PPtr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldParticleCollider {
    pub m_GameObject: PPtr,
    pub m_BounceFactor: f32,
    pub m_CollisionEnergyLoss: f32,
    pub m_CollidesWith: BitField,
    pub m_SendCollisionMessage: bool,
    pub m_MinKillVelocity: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct bitset {
    pub bitCount: i32,
    pub bitblocks: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct int2_storage {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct int3_storage {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct void {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct xform {
    pub t: (Option<float3>, Option<float4>),
    pub q: float4,
    pub s: (Option<float3>, Option<float4>),
}

use AnimatorTransition as AnimatorStateTransition;
use State as AnimatorState;
use StateMachine as AnimatorStateMachine;
