#![allow(warnings)]
use lazy_static::lazy_static;
use std::collections::BTreeMap;

pub const UnknownType: i32 = -1;
pub const Object: i32 = 0;
pub const GameObject: i32 = 1;
pub const Component: i32 = 2;
pub const LevelGameManager: i32 = 3;
pub const Transform: i32 = 4;
pub const TimeManager: i32 = 5;
pub const GlobalGameManager: i32 = 6;
pub const Behaviour: i32 = 8;
pub const GameManager: i32 = 9;
pub const AudioManager: i32 = 11;
pub const ParticleAnimator: i32 = 12;
pub const InputManager: i32 = 13;
pub const EllipsoidParticleEmitter: i32 = 15;
pub const Pipeline: i32 = 17;
pub const EditorExtension: i32 = 18;
pub const Physics2DSettings: i32 = 19;
pub const Camera: i32 = 20;
pub const Material: i32 = 21;
pub const MeshRenderer: i32 = 23;
pub const Renderer: i32 = 25;
pub const ParticleRenderer: i32 = 26;
pub const Texture: i32 = 27;
pub const Texture2D: i32 = 28;
pub const OcclusionCullingSettings: i32 = 29;
pub const GraphicsSettings: i32 = 30;
pub const MeshFilter: i32 = 33;
pub const OcclusionPortal: i32 = 41;
pub const Mesh: i32 = 43;
pub const Skybox: i32 = 45;
pub const QualitySettings: i32 = 47;
pub const Shader: i32 = 48;
pub const TextAsset: i32 = 49;
pub const Rigidbody2D: i32 = 50;
pub const Physics2DManager: i32 = 51;
pub const Collider2D: i32 = 53;
pub const Rigidbody: i32 = 54;
pub const PhysicsManager: i32 = 55;
pub const Collider: i32 = 56;
pub const Joint: i32 = 57;
pub const CircleCollider2D: i32 = 58;
pub const HingeJoint: i32 = 59;
pub const PolygonCollider2D: i32 = 60;
pub const BoxCollider2D: i32 = 61;
pub const PhysicsMaterial2D: i32 = 62;
pub const MeshCollider: i32 = 64;
pub const BoxCollider: i32 = 65;
pub const CompositeCollider2D: i32 = 66;
pub const EdgeCollider2D: i32 = 68;
pub const CapsuleCollider2D: i32 = 70;
pub const ComputeShader: i32 = 72;
pub const AnimationClip: i32 = 74;
pub const ConstantForce: i32 = 75;
pub const WorldParticleCollider: i32 = 76;
pub const TagManager: i32 = 78;
pub const AudioListener: i32 = 81;
pub const AudioSource: i32 = 82;
pub const AudioClip: i32 = 83;
pub const RenderTexture: i32 = 84;
pub const CustomRenderTexture: i32 = 86;
pub const MeshParticleEmitter: i32 = 87;
pub const ParticleEmitter: i32 = 88;
pub const Cubemap: i32 = 89;
pub const Avatar: i32 = 90;
pub const AnimatorController: i32 = 91;
pub const GUILayer: i32 = 92;
pub const RuntimeAnimatorController: i32 = 93;
pub const ScriptMapper: i32 = 94;
pub const Animator: i32 = 95;
pub const TrailRenderer: i32 = 96;
pub const DelayedCallManager: i32 = 98;
pub const TextMesh: i32 = 102;
pub const RenderSettings: i32 = 104;
pub const Light: i32 = 108;
pub const CGProgram: i32 = 109;
pub const BaseAnimationTrack: i32 = 110;
pub const Animation: i32 = 111;
pub const MonoBehaviour: i32 = 114;
pub const MonoScript: i32 = 115;
pub const MonoManager: i32 = 116;
pub const Texture3D: i32 = 117;
pub const NewAnimationTrack: i32 = 118;
pub const Projector: i32 = 119;
pub const LineRenderer: i32 = 120;
pub const Flare: i32 = 121;
pub const Halo: i32 = 122;
pub const LensFlare: i32 = 123;
pub const FlareLayer: i32 = 124;
pub const HaloLayer: i32 = 125;
pub const NavMeshProjectSettings: i32 = 126;
pub const HaloManager: i32 = 127;
pub const Font: i32 = 128;
pub const PlayerSettings: i32 = 129;
pub const NamedObject: i32 = 130;
pub const GUITexture: i32 = 131;
pub const GUIText: i32 = 132;
pub const GUIElement: i32 = 133;
pub const PhysicMaterial: i32 = 134;
pub const SphereCollider: i32 = 135;
pub const CapsuleCollider: i32 = 136;
pub const SkinnedMeshRenderer: i32 = 137;
pub const FixedJoint: i32 = 138;
pub const RaycastCollider: i32 = 140;
pub const BuildSettings: i32 = 141;
pub const AssetBundle: i32 = 142;
pub const CharacterController: i32 = 143;
pub const CharacterJoint: i32 = 144;
pub const SpringJoint: i32 = 145;
pub const WheelCollider: i32 = 146;
pub const ResourceManager: i32 = 147;
pub const NetworkView: i32 = 148;
pub const NetworkManager: i32 = 149;
pub const PreloadData: i32 = 150;
pub const MovieTexture: i32 = 152;
pub const ConfigurableJoint: i32 = 153;
pub const TerrainCollider: i32 = 154;
pub const MasterServerInterface: i32 = 155;
pub const TerrainData: i32 = 156;
pub const LightmapSettings: i32 = 157;
pub const WebCamTexture: i32 = 158;
pub const EditorSettings: i32 = 159;
pub const InteractiveCloth: i32 = 160;
pub const ClothRenderer: i32 = 161;
pub const EditorUserSettings: i32 = 162;
pub const SkinnedCloth: i32 = 163;
pub const AudioReverbFilter: i32 = 164;
pub const AudioHighPassFilter: i32 = 165;
pub const AudioChorusFilter: i32 = 166;
pub const AudioReverbZone: i32 = 167;
pub const AudioEchoFilter: i32 = 168;
pub const AudioLowPassFilter: i32 = 169;
pub const AudioDistortionFilter: i32 = 170;
pub const SparseTexture: i32 = 171;
pub const AudioBehaviour: i32 = 180;
pub const AudioFilter: i32 = 181;
pub const WindZone: i32 = 182;
pub const Cloth: i32 = 183;
pub const SubstanceArchive: i32 = 184;
pub const ProceduralMaterial: i32 = 185;
pub const ProceduralTexture: i32 = 186;
pub const Texture2DArray: i32 = 187;
pub const CubemapArray: i32 = 188;
pub const OffMeshLink: i32 = 191;
pub const OcclusionArea: i32 = 192;
pub const Tree: i32 = 193;
pub const NavMeshObsolete: i32 = 194;
pub const NavMeshAgent: i32 = 195;
pub const NavMeshSettings: i32 = 196;
pub const LightProbesLegacy: i32 = 197;
pub const ParticleSystem: i32 = 198;
pub const ParticleSystemRenderer: i32 = 199;
pub const ShaderVariantCollection: i32 = 200;
pub const LODGroup: i32 = 205;
pub const BlendTree: i32 = 206;
pub const Motion: i32 = 207;
pub const NavMeshObstacle: i32 = 208;
pub const SortingGroup: i32 = 210;
pub const SpriteRenderer: i32 = 212;
pub const Sprite: i32 = 213;
pub const CachedSpriteAtlas: i32 = 214;
pub const ReflectionProbe: i32 = 215;
pub const ReflectionProbes: i32 = 216;
pub const Terrain: i32 = 218;
pub const LightProbeGroup: i32 = 220;
pub const AnimatorOverrideController: i32 = 221;
pub const CanvasRenderer: i32 = 222;
pub const Canvas: i32 = 223;
pub const RectTransform: i32 = 224;
pub const CanvasGroup: i32 = 225;
pub const BillboardAsset: i32 = 226;
pub const BillboardRenderer: i32 = 227;
pub const SpeedTreeWindAsset: i32 = 228;
pub const AnchoredJoint2D: i32 = 229;
pub const Joint2D: i32 = 230;
pub const SpringJoint2D: i32 = 231;
pub const DistanceJoint2D: i32 = 232;
pub const HingeJoint2D: i32 = 233;
pub const SliderJoint2D: i32 = 234;
pub const WheelJoint2D: i32 = 235;
pub const ClusterInputManager: i32 = 236;
pub const BaseVideoTexture: i32 = 237;
pub const NavMeshData: i32 = 238;
pub const AudioMixer: i32 = 240;
pub const AudioMixerController: i32 = 241;
pub const AudioMixerGroupController: i32 = 243;
pub const AudioMixerEffectController: i32 = 244;
pub const AudioMixerSnapshotController: i32 = 245;
pub const PhysicsUpdateBehaviour2D: i32 = 246;
pub const ConstantForce2D: i32 = 247;
pub const Effector2D: i32 = 248;
pub const AreaEffector2D: i32 = 249;
pub const PointEffector2D: i32 = 250;
pub const PlatformEffector2D: i32 = 251;
pub const SurfaceEffector2D: i32 = 252;
pub const BuoyancyEffector2D: i32 = 253;
pub const RelativeJoint2D: i32 = 254;
pub const FixedJoint2D: i32 = 255;
pub const FrictionJoint2D: i32 = 256;
pub const TargetJoint2D: i32 = 257;
pub const LightProbes: i32 = 258;
pub const LightProbeProxyVolume: i32 = 259;
pub const SampleClip: i32 = 271;
pub const AudioMixerSnapshot: i32 = 272;
pub const AudioMixerGroup: i32 = 273;
pub const NScreenBridge: i32 = 280;
pub const AssetBundleManifest: i32 = 290;
pub const UnityAdsManager: i32 = 292;
pub const RuntimeInitializeOnLoadManager: i32 = 300;
pub const CloudWebServicesManager: i32 = 301;
pub const UnityAnalyticsManager: i32 = 303;
pub const CrashReportManager: i32 = 304;
pub const PerformanceReportingManager: i32 = 305;
pub const UnityConnectSettings: i32 = 310;
pub const AvatarMask: i32 = 319;
pub const PlayableDirector: i32 = 320;
pub const VideoPlayer: i32 = 328;
pub const VideoClip: i32 = 329;
pub const ParticleSystemForceField: i32 = 330;
pub const SpriteMask: i32 = 331;
pub const WorldAnchor: i32 = 362;
pub const OcclusionCullingData: i32 = 363;
pub const SmallestEditorClassID: i32 = 1000;
pub const PrefabInstance: i32 = 1001;
pub const EditorExtensionImpl: i32 = 1002;
pub const AssetImporter: i32 = 1003;
pub const AssetDatabaseV1: i32 = 1004;
pub const Mesh3DSImporter: i32 = 1005;
pub const TextureImporter: i32 = 1006;
pub const ShaderImporter: i32 = 1007;
pub const ComputeShaderImporter: i32 = 1008;
pub const AudioImporter: i32 = 1020;
pub const HierarchyState: i32 = 1026;
pub const GUIDSerializer: i32 = 1027;
pub const AssetMetaData: i32 = 1028;
pub const DefaultAsset: i32 = 1029;
pub const DefaultImporter: i32 = 1030;
pub const TextScriptImporter: i32 = 1031;
pub const SceneAsset: i32 = 1032;
pub const NativeFormatImporter: i32 = 1034;
pub const MonoImporter: i32 = 1035;
pub const AssetServerCache: i32 = 1037;
pub const LibraryAssetImporter: i32 = 1038;
pub const ModelImporter: i32 = 1040;
pub const FBXImporter: i32 = 1041;
pub const TrueTypeFontImporter: i32 = 1042;
pub const MovieImporter: i32 = 1044;
pub const EditorBuildSettings: i32 = 1045;
pub const DDSImporter: i32 = 1046;
pub const InspectorExpandedState: i32 = 1048;
pub const AnnotationManager: i32 = 1049;
pub const PluginImporter: i32 = 1050;
pub const EditorUserBuildSettings: i32 = 1051;
pub const PVRImporter: i32 = 1052;
pub const ASTCImporter: i32 = 1053;
pub const KTXImporter: i32 = 1054;
pub const IHVImageFormatImporter: i32 = 1055;
pub const AnimatorStateTransition: i32 = 1101;
pub const AnimatorState: i32 = 1102;
pub const HumanTemplate: i32 = 1105;
pub const AnimatorStateMachine: i32 = 1107;
pub const PreviewAnimationClip: i32 = 1108;
pub const AnimatorTransition: i32 = 1109;
pub const SpeedTreeImporter: i32 = 1110;
pub const AnimatorTransitionBase: i32 = 1111;
pub const SubstanceImporter: i32 = 1112;
pub const LightmapParameters: i32 = 1113;
pub const LightingDataAsset: i32 = 1120;
pub const GISRaster: i32 = 1121;
pub const GISRasterImporter: i32 = 1122;
pub const CadImporter: i32 = 1123;
pub const SketchUpImporter: i32 = 1124;
pub const BuildReport: i32 = 1125;
pub const PackedAssets: i32 = 1126;
pub const VideoClipImporter: i32 = 1127;
pub const ActivationLogComponent: i32 = 2000;
pub const int: i32 = 100000;
pub const bool: i32 = 100001;
pub const float: i32 = 100002;
pub const MonoObject: i32 = 100003;
pub const Collision: i32 = 100004;
pub const Vector3f: i32 = 100005;
pub const RootMotionData: i32 = 100006;
pub const Collision2D: i32 = 100007;
pub const AudioMixerLiveUpdateFloat: i32 = 100008;
pub const AudioMixerLiveUpdateBool: i32 = 100009;
pub const Polygon2D: i32 = 100010;
pub const void: i32 = 100011;
pub const TilemapCollider2D: i32 = 19719996;
pub const AssetImporterLog: i32 = 41386430;
pub const VFXRenderer: i32 = 73398921;
pub const SerializableManagedRefTestClass: i32 = 76251197;
pub const Grid: i32 = 156049354;
pub const ScenesUsingAssets: i32 = 156483287;
pub const ArticulationBody: i32 = 171741748;
pub const Preset: i32 = 181963792;
pub const EmptyObject: i32 = 277625683;
pub const IConstraint: i32 = 285090594;
pub const TestObjectWithSpecialLayoutOne: i32 = 293259124;
pub const AssemblyDefinitionReferenceImporter: i32 = 294290339;
pub const SiblingDerived: i32 = 334799969;
pub const TestObjectWithSerializedMapStringNonAlignedStruct: i32 = 342846651;
pub const SubDerived: i32 = 367388927;
pub const AssetImportInProgressProxy: i32 = 369655926;
pub const PluginBuildInfo: i32 = 382020655;
pub const EditorProjectAccess: i32 = 426301858;
pub const PrefabImporter: i32 = 468431735;
pub const TestObjectWithSerializedArray: i32 = 478637458;
pub const TestObjectWithSerializedAnimationCurve: i32 = 478637459;
pub const TilemapRenderer: i32 = 483693784;
pub const ScriptableCamera: i32 = 488575907;
pub const SpriteAtlasAsset: i32 = 612988286;
pub const SpriteAtlasDatabase: i32 = 638013454;
pub const AudioBuildInfo: i32 = 641289076;
pub const CachedSpriteAtlasRuntimeData: i32 = 644342135;
pub const RendererFake: i32 = 646504946;
pub const AssemblyDefinitionReferenceAsset: i32 = 662584278;
pub const BuiltAssetBundleInfoSet: i32 = 668709126;
pub const SpriteAtlas: i32 = 687078895;
pub const RayTracingShaderImporter: i32 = 747330370;
pub const RayTracingShader: i32 = 825902497;
pub const LightingSettings: i32 = 850595691;
pub const PlatformModuleSetup: i32 = 877146078;
pub const VersionControlSettings: i32 = 890905787;
pub const AimConstraint: i32 = 895512359;
pub const VFXManager: i32 = 937362698;
pub const VisualEffectSubgraph: i32 = 994735392;
pub const VisualEffectSubgraphOperator: i32 = 994735403;
pub const VisualEffectSubgraphBlock: i32 = 994735404;
pub const LocalizationImporter: i32 = 1027052791;
pub const Derived: i32 = 1091556383;
pub const PropertyModificationsTargetTestObject: i32 = 1111377672;
pub const ReferencesArtifactGenerator: i32 = 1114811875;
pub const AssemblyDefinitionAsset: i32 = 1152215463;
pub const SceneVisibilityState: i32 = 1154873562;
pub const LookAtConstraint: i32 = 1183024399;
pub const SpriteAtlasImporter: i32 = 1210832254;
pub const MultiArtifactTestImporter: i32 = 1223240404;
pub const GameObjectRecorder: i32 = 1268269756;
pub const LightingDataAssetParent: i32 = 1325145578;
pub const PresetManager: i32 = 1386491679;
pub const TestObjectWithSpecialLayoutTwo: i32 = 1392443030;
pub const StreamingManager: i32 = 1403656975;
pub const LowerResBlitTexture: i32 = 1480428607;
pub const StreamingController: i32 = 1542919678;
pub const RenderPassAttachment: i32 = 1571458007;
pub const TestObjectVectorPairStringBool: i32 = 1628831178;
pub const GridLayout: i32 = 1742807556;
pub const AssemblyDefinitionImporter: i32 = 1766753193;
pub const ParentConstraint: i32 = 1773428102;
pub const FakeComponent: i32 = 1803986026;
pub const PositionConstraint: i32 = 1818360608;
pub const RotationConstraint: i32 = 1818360609;
pub const ScaleConstraint: i32 = 1818360610;
pub const Tilemap: i32 = 1839735485;
pub const PackageManifest: i32 = 1896753125;
pub const PackageManifestImporter: i32 = 1896753126;
pub const TerrainLayer: i32 = 1953259897;
pub const SpriteShapeRenderer: i32 = 1971053207;
pub const NativeObjectType: i32 = 1977754360;
pub const TestObjectWithSerializedMapStringBool: i32 = 1981279845;
pub const SerializableManagedHost: i32 = 1995898324;
pub const VisualEffectAsset: i32 = 2058629509;
pub const VisualEffectImporter: i32 = 2058629510;
pub const VisualEffectResource: i32 = 2058629511;
pub const VisualEffectObject: i32 = 2059678085;
pub const VisualEffect: i32 = 2083052967;
pub const LocalizationAsset: i32 = 2083778819;
pub const ScriptedImporter: i32 = 2089858483;

lazy_static! {
    pub static ref CLASS_ID_NAME: BTreeMap<i32, &'static str> = [
        (UnknownType, "UnknownType"),
        (Object, "Object"),
        (GameObject, "GameObject"),
        (Component, "Component"),
        (LevelGameManager, "LevelGameManager"),
        (Transform, "Transform"),
        (TimeManager, "TimeManager"),
        (GlobalGameManager, "GlobalGameManager"),
        (Behaviour, "Behaviour"),
        (GameManager, "GameManager"),
        (AudioManager, "AudioManager"),
        (ParticleAnimator, "ParticleAnimator"),
        (InputManager, "InputManager"),
        (EllipsoidParticleEmitter, "EllipsoidParticleEmitter"),
        (Pipeline, "Pipeline"),
        (EditorExtension, "EditorExtension"),
        (Physics2DSettings, "Physics2DSettings"),
        (Camera, "Camera"),
        (Material, "Material"),
        (MeshRenderer, "MeshRenderer"),
        (Renderer, "Renderer"),
        (ParticleRenderer, "ParticleRenderer"),
        (Texture, "Texture"),
        (Texture2D, "Texture2D"),
        (OcclusionCullingSettings, "OcclusionCullingSettings"),
        (GraphicsSettings, "GraphicsSettings"),
        (MeshFilter, "MeshFilter"),
        (OcclusionPortal, "OcclusionPortal"),
        (Mesh, "Mesh"),
        (Skybox, "Skybox"),
        (QualitySettings, "QualitySettings"),
        (Shader, "Shader"),
        (TextAsset, "TextAsset"),
        (Rigidbody2D, "Rigidbody2D"),
        (Physics2DManager, "Physics2DManager"),
        (Collider2D, "Collider2D"),
        (Rigidbody, "Rigidbody"),
        (PhysicsManager, "PhysicsManager"),
        (Collider, "Collider"),
        (Joint, "Joint"),
        (CircleCollider2D, "CircleCollider2D"),
        (HingeJoint, "HingeJoint"),
        (PolygonCollider2D, "PolygonCollider2D"),
        (BoxCollider2D, "BoxCollider2D"),
        (PhysicsMaterial2D, "PhysicsMaterial2D"),
        (MeshCollider, "MeshCollider"),
        (BoxCollider, "BoxCollider"),
        (CompositeCollider2D, "CompositeCollider2D"),
        (EdgeCollider2D, "EdgeCollider2D"),
        (CapsuleCollider2D, "CapsuleCollider2D"),
        (ComputeShader, "ComputeShader"),
        (AnimationClip, "AnimationClip"),
        (ConstantForce, "ConstantForce"),
        (WorldParticleCollider, "WorldParticleCollider"),
        (TagManager, "TagManager"),
        (AudioListener, "AudioListener"),
        (AudioSource, "AudioSource"),
        (AudioClip, "AudioClip"),
        (RenderTexture, "RenderTexture"),
        (CustomRenderTexture, "CustomRenderTexture"),
        (MeshParticleEmitter, "MeshParticleEmitter"),
        (ParticleEmitter, "ParticleEmitter"),
        (Cubemap, "Cubemap"),
        (Avatar, "Avatar"),
        (AnimatorController, "AnimatorController"),
        (GUILayer, "GUILayer"),
        (RuntimeAnimatorController, "RuntimeAnimatorController"),
        (ScriptMapper, "ScriptMapper"),
        (Animator, "Animator"),
        (TrailRenderer, "TrailRenderer"),
        (DelayedCallManager, "DelayedCallManager"),
        (TextMesh, "TextMesh"),
        (RenderSettings, "RenderSettings"),
        (Light, "Light"),
        (CGProgram, "CGProgram"),
        (BaseAnimationTrack, "BaseAnimationTrack"),
        (Animation, "Animation"),
        (MonoBehaviour, "MonoBehaviour"),
        (MonoScript, "MonoScript"),
        (MonoManager, "MonoManager"),
        (Texture3D, "Texture3D"),
        (NewAnimationTrack, "NewAnimationTrack"),
        (Projector, "Projector"),
        (LineRenderer, "LineRenderer"),
        (Flare, "Flare"),
        (Halo, "Halo"),
        (LensFlare, "LensFlare"),
        (FlareLayer, "FlareLayer"),
        (HaloLayer, "HaloLayer"),
        (NavMeshProjectSettings, "NavMeshProjectSettings"),
        (HaloManager, "HaloManager"),
        (Font, "Font"),
        (PlayerSettings, "PlayerSettings"),
        (NamedObject, "NamedObject"),
        (GUITexture, "GUITexture"),
        (GUIText, "GUIText"),
        (GUIElement, "GUIElement"),
        (PhysicMaterial, "PhysicMaterial"),
        (SphereCollider, "SphereCollider"),
        (CapsuleCollider, "CapsuleCollider"),
        (SkinnedMeshRenderer, "SkinnedMeshRenderer"),
        (FixedJoint, "FixedJoint"),
        (RaycastCollider, "RaycastCollider"),
        (BuildSettings, "BuildSettings"),
        (AssetBundle, "AssetBundle"),
        (CharacterController, "CharacterController"),
        (CharacterJoint, "CharacterJoint"),
        (SpringJoint, "SpringJoint"),
        (WheelCollider, "WheelCollider"),
        (ResourceManager, "ResourceManager"),
        (NetworkView, "NetworkView"),
        (NetworkManager, "NetworkManager"),
        (PreloadData, "PreloadData"),
        (MovieTexture, "MovieTexture"),
        (ConfigurableJoint, "ConfigurableJoint"),
        (TerrainCollider, "TerrainCollider"),
        (MasterServerInterface, "MasterServerInterface"),
        (TerrainData, "TerrainData"),
        (LightmapSettings, "LightmapSettings"),
        (WebCamTexture, "WebCamTexture"),
        (EditorSettings, "EditorSettings"),
        (InteractiveCloth, "InteractiveCloth"),
        (ClothRenderer, "ClothRenderer"),
        (EditorUserSettings, "EditorUserSettings"),
        (SkinnedCloth, "SkinnedCloth"),
        (AudioReverbFilter, "AudioReverbFilter"),
        (AudioHighPassFilter, "AudioHighPassFilter"),
        (AudioChorusFilter, "AudioChorusFilter"),
        (AudioReverbZone, "AudioReverbZone"),
        (AudioEchoFilter, "AudioEchoFilter"),
        (AudioLowPassFilter, "AudioLowPassFilter"),
        (AudioDistortionFilter, "AudioDistortionFilter"),
        (SparseTexture, "SparseTexture"),
        (AudioBehaviour, "AudioBehaviour"),
        (AudioFilter, "AudioFilter"),
        (WindZone, "WindZone"),
        (Cloth, "Cloth"),
        (SubstanceArchive, "SubstanceArchive"),
        (ProceduralMaterial, "ProceduralMaterial"),
        (ProceduralTexture, "ProceduralTexture"),
        (Texture2DArray, "Texture2DArray"),
        (CubemapArray, "CubemapArray"),
        (OffMeshLink, "OffMeshLink"),
        (OcclusionArea, "OcclusionArea"),
        (Tree, "Tree"),
        (NavMeshObsolete, "NavMeshObsolete"),
        (NavMeshAgent, "NavMeshAgent"),
        (NavMeshSettings, "NavMeshSettings"),
        (LightProbesLegacy, "LightProbesLegacy"),
        (ParticleSystem, "ParticleSystem"),
        (ParticleSystemRenderer, "ParticleSystemRenderer"),
        (ShaderVariantCollection, "ShaderVariantCollection"),
        (LODGroup, "LODGroup"),
        (BlendTree, "BlendTree"),
        (Motion, "Motion"),
        (NavMeshObstacle, "NavMeshObstacle"),
        (SortingGroup, "SortingGroup"),
        (SpriteRenderer, "SpriteRenderer"),
        (Sprite, "Sprite"),
        (CachedSpriteAtlas, "CachedSpriteAtlas"),
        (ReflectionProbe, "ReflectionProbe"),
        (ReflectionProbes, "ReflectionProbes"),
        (Terrain, "Terrain"),
        (LightProbeGroup, "LightProbeGroup"),
        (AnimatorOverrideController, "AnimatorOverrideController"),
        (CanvasRenderer, "CanvasRenderer"),
        (Canvas, "Canvas"),
        (RectTransform, "RectTransform"),
        (CanvasGroup, "CanvasGroup"),
        (BillboardAsset, "BillboardAsset"),
        (BillboardRenderer, "BillboardRenderer"),
        (SpeedTreeWindAsset, "SpeedTreeWindAsset"),
        (AnchoredJoint2D, "AnchoredJoint2D"),
        (Joint2D, "Joint2D"),
        (SpringJoint2D, "SpringJoint2D"),
        (DistanceJoint2D, "DistanceJoint2D"),
        (HingeJoint2D, "HingeJoint2D"),
        (SliderJoint2D, "SliderJoint2D"),
        (WheelJoint2D, "WheelJoint2D"),
        (ClusterInputManager, "ClusterInputManager"),
        (BaseVideoTexture, "BaseVideoTexture"),
        (NavMeshData, "NavMeshData"),
        (AudioMixer, "AudioMixer"),
        (AudioMixerController, "AudioMixerController"),
        (AudioMixerGroupController, "AudioMixerGroupController"),
        (AudioMixerEffectController, "AudioMixerEffectController"),
        (AudioMixerSnapshotController, "AudioMixerSnapshotController"),
        (PhysicsUpdateBehaviour2D, "PhysicsUpdateBehaviour2D"),
        (ConstantForce2D, "ConstantForce2D"),
        (Effector2D, "Effector2D"),
        (AreaEffector2D, "AreaEffector2D"),
        (PointEffector2D, "PointEffector2D"),
        (PlatformEffector2D, "PlatformEffector2D"),
        (SurfaceEffector2D, "SurfaceEffector2D"),
        (BuoyancyEffector2D, "BuoyancyEffector2D"),
        (RelativeJoint2D, "RelativeJoint2D"),
        (FixedJoint2D, "FixedJoint2D"),
        (FrictionJoint2D, "FrictionJoint2D"),
        (TargetJoint2D, "TargetJoint2D"),
        (LightProbes, "LightProbes"),
        (LightProbeProxyVolume, "LightProbeProxyVolume"),
        (SampleClip, "SampleClip"),
        (AudioMixerSnapshot, "AudioMixerSnapshot"),
        (AudioMixerGroup, "AudioMixerGroup"),
        (NScreenBridge, "NScreenBridge"),
        (AssetBundleManifest, "AssetBundleManifest"),
        (UnityAdsManager, "UnityAdsManager"),
        (
            RuntimeInitializeOnLoadManager,
            "RuntimeInitializeOnLoadManager"
        ),
        (CloudWebServicesManager, "CloudWebServicesManager"),
        (UnityAnalyticsManager, "UnityAnalyticsManager"),
        (CrashReportManager, "CrashReportManager"),
        (PerformanceReportingManager, "PerformanceReportingManager"),
        (UnityConnectSettings, "UnityConnectSettings"),
        (AvatarMask, "AvatarMask"),
        (PlayableDirector, "PlayableDirector"),
        (VideoPlayer, "VideoPlayer"),
        (VideoClip, "VideoClip"),
        (ParticleSystemForceField, "ParticleSystemForceField"),
        (SpriteMask, "SpriteMask"),
        (WorldAnchor, "WorldAnchor"),
        (OcclusionCullingData, "OcclusionCullingData"),
        (SmallestEditorClassID, "SmallestEditorClassID"),
        (PrefabInstance, "PrefabInstance"),
        (EditorExtensionImpl, "EditorExtensionImpl"),
        (AssetImporter, "AssetImporter"),
        (AssetDatabaseV1, "AssetDatabaseV1"),
        (Mesh3DSImporter, "Mesh3DSImporter"),
        (TextureImporter, "TextureImporter"),
        (ShaderImporter, "ShaderImporter"),
        (ComputeShaderImporter, "ComputeShaderImporter"),
        (AudioImporter, "AudioImporter"),
        (HierarchyState, "HierarchyState"),
        (GUIDSerializer, "GUIDSerializer"),
        (AssetMetaData, "AssetMetaData"),
        (DefaultAsset, "DefaultAsset"),
        (DefaultImporter, "DefaultImporter"),
        (TextScriptImporter, "TextScriptImporter"),
        (SceneAsset, "SceneAsset"),
        (NativeFormatImporter, "NativeFormatImporter"),
        (MonoImporter, "MonoImporter"),
        (AssetServerCache, "AssetServerCache"),
        (LibraryAssetImporter, "LibraryAssetImporter"),
        (ModelImporter, "ModelImporter"),
        (FBXImporter, "FBXImporter"),
        (TrueTypeFontImporter, "TrueTypeFontImporter"),
        (MovieImporter, "MovieImporter"),
        (EditorBuildSettings, "EditorBuildSettings"),
        (DDSImporter, "DDSImporter"),
        (InspectorExpandedState, "InspectorExpandedState"),
        (AnnotationManager, "AnnotationManager"),
        (PluginImporter, "PluginImporter"),
        (EditorUserBuildSettings, "EditorUserBuildSettings"),
        (PVRImporter, "PVRImporter"),
        (ASTCImporter, "ASTCImporter"),
        (KTXImporter, "KTXImporter"),
        (IHVImageFormatImporter, "IHVImageFormatImporter"),
        (AnimatorStateTransition, "AnimatorStateTransition"),
        (AnimatorState, "AnimatorState"),
        (HumanTemplate, "HumanTemplate"),
        (AnimatorStateMachine, "AnimatorStateMachine"),
        (PreviewAnimationClip, "PreviewAnimationClip"),
        (AnimatorTransition, "AnimatorTransition"),
        (SpeedTreeImporter, "SpeedTreeImporter"),
        (AnimatorTransitionBase, "AnimatorTransitionBase"),
        (SubstanceImporter, "SubstanceImporter"),
        (LightmapParameters, "LightmapParameters"),
        (LightingDataAsset, "LightingDataAsset"),
        (GISRaster, "GISRaster"),
        (GISRasterImporter, "GISRasterImporter"),
        (CadImporter, "CadImporter"),
        (SketchUpImporter, "SketchUpImporter"),
        (BuildReport, "BuildReport"),
        (PackedAssets, "PackedAssets"),
        (VideoClipImporter, "VideoClipImporter"),
        (ActivationLogComponent, "ActivationLogComponent"),
        (int, "int"),
        (bool, "bool"),
        (float, "float"),
        (MonoObject, "MonoObject"),
        (Collision, "Collision"),
        (Vector3f, "Vector3f"),
        (RootMotionData, "RootMotionData"),
        (Collision2D, "Collision2D"),
        (AudioMixerLiveUpdateFloat, "AudioMixerLiveUpdateFloat"),
        (AudioMixerLiveUpdateBool, "AudioMixerLiveUpdateBool"),
        (Polygon2D, "Polygon2D"),
        (void, "void"),
        (TilemapCollider2D, "TilemapCollider2D"),
        (AssetImporterLog, "AssetImporterLog"),
        (VFXRenderer, "VFXRenderer"),
        (
            SerializableManagedRefTestClass,
            "SerializableManagedRefTestClass"
        ),
        (Grid, "Grid"),
        (ScenesUsingAssets, "ScenesUsingAssets"),
        (ArticulationBody, "ArticulationBody"),
        (Preset, "Preset"),
        (EmptyObject, "EmptyObject"),
        (IConstraint, "IConstraint"),
        (
            TestObjectWithSpecialLayoutOne,
            "TestObjectWithSpecialLayoutOne"
        ),
        (
            AssemblyDefinitionReferenceImporter,
            "AssemblyDefinitionReferenceImporter"
        ),
        (SiblingDerived, "SiblingDerived"),
        (
            TestObjectWithSerializedMapStringNonAlignedStruct,
            "TestObjectWithSerializedMapStringNonAlignedStruct"
        ),
        (SubDerived, "SubDerived"),
        (AssetImportInProgressProxy, "AssetImportInProgressProxy"),
        (PluginBuildInfo, "PluginBuildInfo"),
        (EditorProjectAccess, "EditorProjectAccess"),
        (PrefabImporter, "PrefabImporter"),
        (
            TestObjectWithSerializedArray,
            "TestObjectWithSerializedArray"
        ),
        (
            TestObjectWithSerializedAnimationCurve,
            "TestObjectWithSerializedAnimationCurve"
        ),
        (TilemapRenderer, "TilemapRenderer"),
        (ScriptableCamera, "ScriptableCamera"),
        (SpriteAtlasAsset, "SpriteAtlasAsset"),
        (SpriteAtlasDatabase, "SpriteAtlasDatabase"),
        (AudioBuildInfo, "AudioBuildInfo"),
        (CachedSpriteAtlasRuntimeData, "CachedSpriteAtlasRuntimeData"),
        (RendererFake, "RendererFake"),
        (
            AssemblyDefinitionReferenceAsset,
            "AssemblyDefinitionReferenceAsset"
        ),
        (BuiltAssetBundleInfoSet, "BuiltAssetBundleInfoSet"),
        (SpriteAtlas, "SpriteAtlas"),
        (RayTracingShaderImporter, "RayTracingShaderImporter"),
        (RayTracingShader, "RayTracingShader"),
        (LightingSettings, "LightingSettings"),
        (PlatformModuleSetup, "PlatformModuleSetup"),
        (VersionControlSettings, "VersionControlSettings"),
        (AimConstraint, "AimConstraint"),
        (VFXManager, "VFXManager"),
        (VisualEffectSubgraph, "VisualEffectSubgraph"),
        (VisualEffectSubgraphOperator, "VisualEffectSubgraphOperator"),
        (VisualEffectSubgraphBlock, "VisualEffectSubgraphBlock"),
        (LocalizationImporter, "LocalizationImporter"),
        (Derived, "Derived"),
        (
            PropertyModificationsTargetTestObject,
            "PropertyModificationsTargetTestObject"
        ),
        (ReferencesArtifactGenerator, "ReferencesArtifactGenerator"),
        (AssemblyDefinitionAsset, "AssemblyDefinitionAsset"),
        (SceneVisibilityState, "SceneVisibilityState"),
        (LookAtConstraint, "LookAtConstraint"),
        (SpriteAtlasImporter, "SpriteAtlasImporter"),
        (MultiArtifactTestImporter, "MultiArtifactTestImporter"),
        (GameObjectRecorder, "GameObjectRecorder"),
        (LightingDataAssetParent, "LightingDataAssetParent"),
        (PresetManager, "PresetManager"),
        (
            TestObjectWithSpecialLayoutTwo,
            "TestObjectWithSpecialLayoutTwo"
        ),
        (StreamingManager, "StreamingManager"),
        (LowerResBlitTexture, "LowerResBlitTexture"),
        (StreamingController, "StreamingController"),
        (RenderPassAttachment, "RenderPassAttachment"),
        (
            TestObjectVectorPairStringBool,
            "TestObjectVectorPairStringBool"
        ),
        (GridLayout, "GridLayout"),
        (AssemblyDefinitionImporter, "AssemblyDefinitionImporter"),
        (ParentConstraint, "ParentConstraint"),
        (FakeComponent, "FakeComponent"),
        (PositionConstraint, "PositionConstraint"),
        (RotationConstraint, "RotationConstraint"),
        (ScaleConstraint, "ScaleConstraint"),
        (Tilemap, "Tilemap"),
        (PackageManifest, "PackageManifest"),
        (PackageManifestImporter, "PackageManifestImporter"),
        (TerrainLayer, "TerrainLayer"),
        (SpriteShapeRenderer, "SpriteShapeRenderer"),
        (NativeObjectType, "NativeObjectType"),
        (
            TestObjectWithSerializedMapStringBool,
            "TestObjectWithSerializedMapStringBool"
        ),
        (SerializableManagedHost, "SerializableManagedHost"),
        (VisualEffectAsset, "VisualEffectAsset"),
        (VisualEffectImporter, "VisualEffectImporter"),
        (VisualEffectResource, "VisualEffectResource"),
        (VisualEffectObject, "VisualEffectObject"),
        (VisualEffect, "VisualEffect"),
        (LocalizationAsset, "LocalizationAsset"),
        (ScriptedImporter, "ScriptedImporter")
    ]
    .iter()
    .copied()
    .collect();
}
