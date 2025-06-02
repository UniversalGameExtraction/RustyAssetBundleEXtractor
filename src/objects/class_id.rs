use std::collections::BTreeMap;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassId(pub i32);
impl Default for ClassId {
    fn default() -> Self {
        ClassId::UnknownType
    }
}

impl ClassId {
    pub fn name(&self) -> Option<&str> {
        CLASS_ID_NAME.get(&self).map(|name| *name)
    }
}

impl std::fmt::Debug for ClassId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.name() {
            Some(name) => f.write_str(name),
            None => write!(f, "ClassId({})", self.0),
        }
    }
}

#[allow(non_upper_case_globals)]
impl ClassId {
    pub const UnknownType: ClassId = ClassId(-1);
    pub const Object: ClassId = ClassId(0);
    pub const GameObject: ClassId = ClassId(1);
    pub const Component: ClassId = ClassId(2);
    pub const LevelGameManager: ClassId = ClassId(3);
    pub const Transform: ClassId = ClassId(4);
    pub const TimeManager: ClassId = ClassId(5);
    pub const GlobalGameManager: ClassId = ClassId(6);
    pub const Behaviour: ClassId = ClassId(8);
    pub const GameManager: ClassId = ClassId(9);
    pub const AudioManager: ClassId = ClassId(11);
    pub const ParticleAnimator: ClassId = ClassId(12);
    pub const InputManager: ClassId = ClassId(13);
    pub const EllipsoidParticleEmitter: ClassId = ClassId(15);
    pub const Pipeline: ClassId = ClassId(17);
    pub const EditorExtension: ClassId = ClassId(18);
    pub const Physics2DSettings: ClassId = ClassId(19);
    pub const Camera: ClassId = ClassId(20);
    pub const Material: ClassId = ClassId(21);
    pub const MeshRenderer: ClassId = ClassId(23);
    pub const Renderer: ClassId = ClassId(25);
    pub const ParticleRenderer: ClassId = ClassId(26);
    pub const Texture: ClassId = ClassId(27);
    pub const Texture2D: ClassId = ClassId(28);
    pub const OcclusionCullingSettings: ClassId = ClassId(29);
    pub const GraphicsSettings: ClassId = ClassId(30);
    pub const MeshFilter: ClassId = ClassId(33);
    pub const OcclusionPortal: ClassId = ClassId(41);
    pub const Mesh: ClassId = ClassId(43);
    pub const Skybox: ClassId = ClassId(45);
    pub const QualitySettings: ClassId = ClassId(47);
    pub const Shader: ClassId = ClassId(48);
    pub const TextAsset: ClassId = ClassId(49);
    pub const Rigidbody2D: ClassId = ClassId(50);
    pub const Physics2DManager: ClassId = ClassId(51);
    pub const Collider2D: ClassId = ClassId(53);
    pub const Rigidbody: ClassId = ClassId(54);
    pub const PhysicsManager: ClassId = ClassId(55);
    pub const Collider: ClassId = ClassId(56);
    pub const Joint: ClassId = ClassId(57);
    pub const CircleCollider2D: ClassId = ClassId(58);
    pub const HingeJoint: ClassId = ClassId(59);
    pub const PolygonCollider2D: ClassId = ClassId(60);
    pub const BoxCollider2D: ClassId = ClassId(61);
    pub const PhysicsMaterial2D: ClassId = ClassId(62);
    pub const MeshCollider: ClassId = ClassId(64);
    pub const BoxCollider: ClassId = ClassId(65);
    pub const CompositeCollider2D: ClassId = ClassId(66);
    pub const EdgeCollider2D: ClassId = ClassId(68);
    pub const CapsuleCollider2D: ClassId = ClassId(70);
    pub const ComputeShader: ClassId = ClassId(72);
    pub const AnimationClip: ClassId = ClassId(74);
    pub const ConstantForce: ClassId = ClassId(75);
    pub const WorldParticleCollider: ClassId = ClassId(76);
    pub const TagManager: ClassId = ClassId(78);
    pub const AudioListener: ClassId = ClassId(81);
    pub const AudioSource: ClassId = ClassId(82);
    pub const AudioClip: ClassId = ClassId(83);
    pub const RenderTexture: ClassId = ClassId(84);
    pub const CustomRenderTexture: ClassId = ClassId(86);
    pub const MeshParticleEmitter: ClassId = ClassId(87);
    pub const ParticleEmitter: ClassId = ClassId(88);
    pub const Cubemap: ClassId = ClassId(89);
    pub const Avatar: ClassId = ClassId(90);
    pub const AnimatorController: ClassId = ClassId(91);
    pub const GUILayer: ClassId = ClassId(92);
    pub const RuntimeAnimatorController: ClassId = ClassId(93);
    pub const ScriptMapper: ClassId = ClassId(94);
    pub const Animator: ClassId = ClassId(95);
    pub const TrailRenderer: ClassId = ClassId(96);
    pub const DelayedCallManager: ClassId = ClassId(98);
    pub const TextMesh: ClassId = ClassId(102);
    pub const RenderSettings: ClassId = ClassId(104);
    pub const Light: ClassId = ClassId(108);
    pub const CGProgram: ClassId = ClassId(109);
    pub const BaseAnimationTrack: ClassId = ClassId(110);
    pub const Animation: ClassId = ClassId(111);
    pub const MonoBehaviour: ClassId = ClassId(114);
    pub const MonoScript: ClassId = ClassId(115);
    pub const MonoManager: ClassId = ClassId(116);
    pub const Texture3D: ClassId = ClassId(117);
    pub const NewAnimationTrack: ClassId = ClassId(118);
    pub const Projector: ClassId = ClassId(119);
    pub const LineRenderer: ClassId = ClassId(120);
    pub const Flare: ClassId = ClassId(121);
    pub const Halo: ClassId = ClassId(122);
    pub const LensFlare: ClassId = ClassId(123);
    pub const FlareLayer: ClassId = ClassId(124);
    pub const HaloLayer: ClassId = ClassId(125);
    pub const NavMeshProjectSettings: ClassId = ClassId(126);
    pub const HaloManager: ClassId = ClassId(127);
    pub const Font: ClassId = ClassId(128);
    pub const PlayerSettings: ClassId = ClassId(129);
    pub const NamedObject: ClassId = ClassId(130);
    pub const GUITexture: ClassId = ClassId(131);
    pub const GUIText: ClassId = ClassId(132);
    pub const GUIElement: ClassId = ClassId(133);
    pub const PhysicMaterial: ClassId = ClassId(134);
    pub const SphereCollider: ClassId = ClassId(135);
    pub const CapsuleCollider: ClassId = ClassId(136);
    pub const SkinnedMeshRenderer: ClassId = ClassId(137);
    pub const FixedJoint: ClassId = ClassId(138);
    pub const RaycastCollider: ClassId = ClassId(140);
    pub const BuildSettings: ClassId = ClassId(141);
    pub const AssetBundle: ClassId = ClassId(142);
    pub const CharacterController: ClassId = ClassId(143);
    pub const CharacterJoint: ClassId = ClassId(144);
    pub const SpringJoint: ClassId = ClassId(145);
    pub const WheelCollider: ClassId = ClassId(146);
    pub const ResourceManager: ClassId = ClassId(147);
    pub const NetworkView: ClassId = ClassId(148);
    pub const NetworkManager: ClassId = ClassId(149);
    pub const PreloadData: ClassId = ClassId(150);
    pub const MovieTexture: ClassId = ClassId(152);
    pub const ConfigurableJoint: ClassId = ClassId(153);
    pub const TerrainCollider: ClassId = ClassId(154);
    pub const MasterServerInterface: ClassId = ClassId(155);
    pub const TerrainData: ClassId = ClassId(156);
    pub const LightmapSettings: ClassId = ClassId(157);
    pub const WebCamTexture: ClassId = ClassId(158);
    pub const EditorSettings: ClassId = ClassId(159);
    pub const InteractiveCloth: ClassId = ClassId(160);
    pub const ClothRenderer: ClassId = ClassId(161);
    pub const EditorUserSettings: ClassId = ClassId(162);
    pub const SkinnedCloth: ClassId = ClassId(163);
    pub const AudioReverbFilter: ClassId = ClassId(164);
    pub const AudioHighPassFilter: ClassId = ClassId(165);
    pub const AudioChorusFilter: ClassId = ClassId(166);
    pub const AudioReverbZone: ClassId = ClassId(167);
    pub const AudioEchoFilter: ClassId = ClassId(168);
    pub const AudioLowPassFilter: ClassId = ClassId(169);
    pub const AudioDistortionFilter: ClassId = ClassId(170);
    pub const SparseTexture: ClassId = ClassId(171);
    pub const AudioBehaviour: ClassId = ClassId(180);
    pub const AudioFilter: ClassId = ClassId(181);
    pub const WindZone: ClassId = ClassId(182);
    pub const Cloth: ClassId = ClassId(183);
    pub const SubstanceArchive: ClassId = ClassId(184);
    pub const ProceduralMaterial: ClassId = ClassId(185);
    pub const ProceduralTexture: ClassId = ClassId(186);
    pub const Texture2DArray: ClassId = ClassId(187);
    pub const CubemapArray: ClassId = ClassId(188);
    pub const OffMeshLink: ClassId = ClassId(191);
    pub const OcclusionArea: ClassId = ClassId(192);
    pub const Tree: ClassId = ClassId(193);
    pub const NavMeshObsolete: ClassId = ClassId(194);
    pub const NavMeshAgent: ClassId = ClassId(195);
    pub const NavMeshSettings: ClassId = ClassId(196);
    pub const LightProbesLegacy: ClassId = ClassId(197);
    pub const ParticleSystem: ClassId = ClassId(198);
    pub const ParticleSystemRenderer: ClassId = ClassId(199);
    pub const ShaderVariantCollection: ClassId = ClassId(200);
    pub const LODGroup: ClassId = ClassId(205);
    pub const BlendTree: ClassId = ClassId(206);
    pub const Motion: ClassId = ClassId(207);
    pub const NavMeshObstacle: ClassId = ClassId(208);
    pub const SortingGroup: ClassId = ClassId(210);
    pub const SpriteRenderer: ClassId = ClassId(212);
    pub const Sprite: ClassId = ClassId(213);
    pub const CachedSpriteAtlas: ClassId = ClassId(214);
    pub const ReflectionProbe: ClassId = ClassId(215);
    pub const ReflectionProbes: ClassId = ClassId(216);
    pub const Terrain: ClassId = ClassId(218);
    pub const LightProbeGroup: ClassId = ClassId(220);
    pub const AnimatorOverrideController: ClassId = ClassId(221);
    pub const CanvasRenderer: ClassId = ClassId(222);
    pub const Canvas: ClassId = ClassId(223);
    pub const RectTransform: ClassId = ClassId(224);
    pub const CanvasGroup: ClassId = ClassId(225);
    pub const BillboardAsset: ClassId = ClassId(226);
    pub const BillboardRenderer: ClassId = ClassId(227);
    pub const SpeedTreeWindAsset: ClassId = ClassId(228);
    pub const AnchoredJoint2D: ClassId = ClassId(229);
    pub const Joint2D: ClassId = ClassId(230);
    pub const SpringJoint2D: ClassId = ClassId(231);
    pub const DistanceJoint2D: ClassId = ClassId(232);
    pub const HingeJoint2D: ClassId = ClassId(233);
    pub const SliderJoint2D: ClassId = ClassId(234);
    pub const WheelJoint2D: ClassId = ClassId(235);
    pub const ClusterInputManager: ClassId = ClassId(236);
    pub const BaseVideoTexture: ClassId = ClassId(237);
    pub const NavMeshData: ClassId = ClassId(238);
    pub const AudioMixer: ClassId = ClassId(240);
    pub const AudioMixerController: ClassId = ClassId(241);
    pub const AudioMixerGroupController: ClassId = ClassId(243);
    pub const AudioMixerEffectController: ClassId = ClassId(244);
    pub const AudioMixerSnapshotController: ClassId = ClassId(245);
    pub const PhysicsUpdateBehaviour2D: ClassId = ClassId(246);
    pub const ConstantForce2D: ClassId = ClassId(247);
    pub const Effector2D: ClassId = ClassId(248);
    pub const AreaEffector2D: ClassId = ClassId(249);
    pub const PointEffector2D: ClassId = ClassId(250);
    pub const PlatformEffector2D: ClassId = ClassId(251);
    pub const SurfaceEffector2D: ClassId = ClassId(252);
    pub const BuoyancyEffector2D: ClassId = ClassId(253);
    pub const RelativeJoint2D: ClassId = ClassId(254);
    pub const FixedJoint2D: ClassId = ClassId(255);
    pub const FrictionJoint2D: ClassId = ClassId(256);
    pub const TargetJoint2D: ClassId = ClassId(257);
    pub const LightProbes: ClassId = ClassId(258);
    pub const LightProbeProxyVolume: ClassId = ClassId(259);
    pub const SampleClip: ClassId = ClassId(271);
    pub const AudioMixerSnapshot: ClassId = ClassId(272);
    pub const AudioMixerGroup: ClassId = ClassId(273);
    pub const NScreenBridge: ClassId = ClassId(280);
    pub const AssetBundleManifest: ClassId = ClassId(290);
    pub const UnityAdsManager: ClassId = ClassId(292);
    pub const RuntimeInitializeOnLoadManager: ClassId = ClassId(300);
    pub const CloudWebServicesManager: ClassId = ClassId(301);
    pub const UnityAnalyticsManager: ClassId = ClassId(303);
    pub const CrashReportManager: ClassId = ClassId(304);
    pub const PerformanceReportingManager: ClassId = ClassId(305);
    pub const UnityConnectSettings: ClassId = ClassId(310);
    pub const AvatarMask: ClassId = ClassId(319);
    pub const PlayableDirector: ClassId = ClassId(320);
    pub const VideoPlayer: ClassId = ClassId(328);
    pub const VideoClip: ClassId = ClassId(329);
    pub const ParticleSystemForceField: ClassId = ClassId(330);
    pub const SpriteMask: ClassId = ClassId(331);
    pub const WorldAnchor: ClassId = ClassId(362);
    pub const OcclusionCullingData: ClassId = ClassId(363);
    pub const SmallestEditorClassId: ClassId = ClassId(1000);
    pub const PrefabInstance: ClassId = ClassId(1001);
    pub const EditorExtensionImpl: ClassId = ClassId(1002);
    pub const AssetImporter: ClassId = ClassId(1003);
    pub const AssetDatabaseV1: ClassId = ClassId(1004);
    pub const Mesh3DSImporter: ClassId = ClassId(1005);
    pub const TextureImporter: ClassId = ClassId(1006);
    pub const ShaderImporter: ClassId = ClassId(1007);
    pub const ComputeShaderImporter: ClassId = ClassId(1008);
    pub const AudioImporter: ClassId = ClassId(1020);
    pub const HierarchyState: ClassId = ClassId(1026);
    pub const GUIDSerializer: ClassId = ClassId(1027);
    pub const AssetMetaData: ClassId = ClassId(1028);
    pub const DefaultAsset: ClassId = ClassId(1029);
    pub const DefaultImporter: ClassId = ClassId(1030);
    pub const TextScriptImporter: ClassId = ClassId(1031);
    pub const SceneAsset: ClassId = ClassId(1032);
    pub const NativeFormatImporter: ClassId = ClassId(1034);
    pub const MonoImporter: ClassId = ClassId(1035);
    pub const AssetServerCache: ClassId = ClassId(1037);
    pub const LibraryAssetImporter: ClassId = ClassId(1038);
    pub const ModelImporter: ClassId = ClassId(1040);
    pub const FBXImporter: ClassId = ClassId(1041);
    pub const TrueTypeFontImporter: ClassId = ClassId(1042);
    pub const MovieImporter: ClassId = ClassId(1044);
    pub const EditorBuildSettings: ClassId = ClassId(1045);
    pub const DDSImporter: ClassId = ClassId(1046);
    pub const InspectorExpandedState: ClassId = ClassId(1048);
    pub const AnnotationManager: ClassId = ClassId(1049);
    pub const PluginImporter: ClassId = ClassId(1050);
    pub const EditorUserBuildSettings: ClassId = ClassId(1051);
    pub const PVRImporter: ClassId = ClassId(1052);
    pub const ASTCImporter: ClassId = ClassId(1053);
    pub const KTXImporter: ClassId = ClassId(1054);
    pub const IHVImageFormatImporter: ClassId = ClassId(1055);
    pub const AnimatorStateTransition: ClassId = ClassId(1101);
    pub const AnimatorState: ClassId = ClassId(1102);
    pub const HumanTemplate: ClassId = ClassId(1105);
    pub const AnimatorStateMachine: ClassId = ClassId(1107);
    pub const PreviewAnimationClip: ClassId = ClassId(1108);
    pub const AnimatorTransition: ClassId = ClassId(1109);
    pub const SpeedTreeImporter: ClassId = ClassId(1110);
    pub const AnimatorTransitionBase: ClassId = ClassId(1111);
    pub const SubstanceImporter: ClassId = ClassId(1112);
    pub const LightmapParameters: ClassId = ClassId(1113);
    pub const LightingDataAsset: ClassId = ClassId(1120);
    pub const GISRaster: ClassId = ClassId(1121);
    pub const GISRasterImporter: ClassId = ClassId(1122);
    pub const CadImporter: ClassId = ClassId(1123);
    pub const SketchUpImporter: ClassId = ClassId(1124);
    pub const BuildReport: ClassId = ClassId(1125);
    pub const PackedAssets: ClassId = ClassId(1126);
    pub const VideoClipImporter: ClassId = ClassId(1127);
    pub const ActivationLogComponent: ClassId = ClassId(2000);
    pub const int: ClassId = ClassId(100000);
    pub const bool: ClassId = ClassId(100001);
    pub const float: ClassId = ClassId(100002);
    pub const MonoObject: ClassId = ClassId(100003);
    pub const Collision: ClassId = ClassId(100004);
    pub const Vector3f: ClassId = ClassId(100005);
    pub const RootMotionData: ClassId = ClassId(100006);
    pub const Collision2D: ClassId = ClassId(100007);
    pub const AudioMixerLiveUpdateFloat: ClassId = ClassId(100008);
    pub const AudioMixerLiveUpdateBool: ClassId = ClassId(100009);
    pub const Polygon2D: ClassId = ClassId(100010);
    pub const void: ClassId = ClassId(100011);
    pub const TilemapCollider2D: ClassId = ClassId(19719996);
    pub const AssetImporterLog: ClassId = ClassId(41386430);
    pub const VFXRenderer: ClassId = ClassId(73398921);
    pub const SerializableManagedRefTestClass: ClassId = ClassId(76251197);
    pub const Grid: ClassId = ClassId(156049354);
    pub const ScenesUsingAssets: ClassId = ClassId(156483287);
    pub const ArticulationBody: ClassId = ClassId(171741748);
    pub const Preset: ClassId = ClassId(181963792);
    pub const EmptyObject: ClassId = ClassId(277625683);
    pub const IConstraint: ClassId = ClassId(285090594);
    pub const TestObjectWithSpecialLayoutOne: ClassId = ClassId(293259124);
    pub const AssemblyDefinitionReferenceImporter: ClassId = ClassId(294290339);
    pub const SiblingDerived: ClassId = ClassId(334799969);
    pub const TestObjectWithSerializedMapStringNonAlignedStruct: ClassId = ClassId(342846651);
    pub const SubDerived: ClassId = ClassId(367388927);
    pub const AssetImportInProgressProxy: ClassId = ClassId(369655926);
    pub const PluginBuildInfo: ClassId = ClassId(382020655);
    pub const EditorProjectAccess: ClassId = ClassId(426301858);
    pub const PrefabImporter: ClassId = ClassId(468431735);
    pub const TestObjectWithSerializedArray: ClassId = ClassId(478637458);
    pub const TestObjectWithSerializedAnimationCurve: ClassId = ClassId(478637459);
    pub const TilemapRenderer: ClassId = ClassId(483693784);
    pub const ScriptableCamera: ClassId = ClassId(488575907);
    pub const SpriteAtlasAsset: ClassId = ClassId(612988286);
    pub const SpriteAtlasDatabase: ClassId = ClassId(638013454);
    pub const AudioBuildInfo: ClassId = ClassId(641289076);
    pub const CachedSpriteAtlasRuntimeData: ClassId = ClassId(644342135);
    pub const RendererFake: ClassId = ClassId(646504946);
    pub const AssemblyDefinitionReferenceAsset: ClassId = ClassId(662584278);
    pub const BuiltAssetBundleInfoSet: ClassId = ClassId(668709126);
    pub const SpriteAtlas: ClassId = ClassId(687078895);
    pub const RayTracingShaderImporter: ClassId = ClassId(747330370);
    pub const RayTracingShader: ClassId = ClassId(825902497);
    pub const LightingSettings: ClassId = ClassId(850595691);
    pub const PlatformModuleSetup: ClassId = ClassId(877146078);
    pub const VersionControlSettings: ClassId = ClassId(890905787);
    pub const AimConstraint: ClassId = ClassId(895512359);
    pub const VFXManager: ClassId = ClassId(937362698);
    pub const VisualEffectSubgraph: ClassId = ClassId(994735392);
    pub const VisualEffectSubgraphOperator: ClassId = ClassId(994735403);
    pub const VisualEffectSubgraphBlock: ClassId = ClassId(994735404);
    pub const LocalizationImporter: ClassId = ClassId(1027052791);
    pub const Derived: ClassId = ClassId(1091556383);
    pub const PropertyModificationsTargetTestObject: ClassId = ClassId(1111377672);
    pub const ReferencesArtifactGenerator: ClassId = ClassId(1114811875);
    pub const AssemblyDefinitionAsset: ClassId = ClassId(1152215463);
    pub const SceneVisibilityState: ClassId = ClassId(1154873562);
    pub const LookAtConstraint: ClassId = ClassId(1183024399);
    pub const SpriteAtlasImporter: ClassId = ClassId(1210832254);
    pub const MultiArtifactTestImporter: ClassId = ClassId(1223240404);
    pub const GameObjectRecorder: ClassId = ClassId(1268269756);
    pub const LightingDataAssetParent: ClassId = ClassId(1325145578);
    pub const PresetManager: ClassId = ClassId(1386491679);
    pub const TestObjectWithSpecialLayoutTwo: ClassId = ClassId(1392443030);
    pub const StreamingManager: ClassId = ClassId(1403656975);
    pub const LowerResBlitTexture: ClassId = ClassId(1480428607);
    pub const StreamingController: ClassId = ClassId(1542919678);
    pub const RenderPassAttachment: ClassId = ClassId(1571458007);
    pub const TestObjectVectorPairStringBool: ClassId = ClassId(1628831178);
    pub const GridLayout: ClassId = ClassId(1742807556);
    pub const AssemblyDefinitionImporter: ClassId = ClassId(1766753193);
    pub const ParentConstraint: ClassId = ClassId(1773428102);
    pub const FakeComponent: ClassId = ClassId(1803986026);
    pub const PositionConstraint: ClassId = ClassId(1818360608);
    pub const RotationConstraint: ClassId = ClassId(1818360609);
    pub const ScaleConstraint: ClassId = ClassId(1818360610);
    pub const Tilemap: ClassId = ClassId(1839735485);
    pub const PackageManifest: ClassId = ClassId(1896753125);
    pub const PackageManifestImporter: ClassId = ClassId(1896753126);
    pub const TerrainLayer: ClassId = ClassId(1953259897);
    pub const SpriteShapeRenderer: ClassId = ClassId(1971053207);
    pub const NativeObjectType: ClassId = ClassId(1977754360);
    pub const TestObjectWithSerializedMapStringBool: ClassId = ClassId(1981279845);
    pub const SerializableManagedHost: ClassId = ClassId(1995898324);
    pub const VisualEffectAsset: ClassId = ClassId(2058629509);
    pub const VisualEffectImporter: ClassId = ClassId(2058629510);
    pub const VisualEffectResource: ClassId = ClassId(2058629511);
    pub const VisualEffectObject: ClassId = ClassId(2059678085);
    pub const VisualEffect: ClassId = ClassId(2083052967);
    pub const LocalizationAsset: ClassId = ClassId(2083778819);
    pub const ScriptedImporter: ClassId = ClassId(2089858483);
}

use std::sync::LazyLock;
#[rustfmt::skip]
pub static CLASS_ID_NAME: LazyLock<BTreeMap<ClassId, &'static str>> = LazyLock::new(|| [
    (ClassId::UnknownType, "UnknownType"),
    (ClassId::Object, "Object"),
    (ClassId::GameObject, "GameObject"),
    (ClassId::Component, "Component"),
    (ClassId::LevelGameManager, "LevelGameManager"),
    (ClassId::Transform, "Transform"),
    (ClassId::TimeManager, "TimeManager"),
    (ClassId::GlobalGameManager, "GlobalGameManager"),
    (ClassId::Behaviour, "Behaviour"),
    (ClassId::GameManager, "GameManager"),
    (ClassId::AudioManager, "AudioManager"),
    (ClassId::ParticleAnimator, "ParticleAnimator"),
    (ClassId::InputManager, "InputManager"),
    (ClassId::EllipsoidParticleEmitter, "EllipsoidParticleEmitter"),
    (ClassId::Pipeline, "Pipeline"),
    (ClassId::EditorExtension, "EditorExtension"),
    (ClassId::Physics2DSettings, "Physics2DSettings"),
    (ClassId::Camera, "Camera"),
    (ClassId::Material, "Material"),
    (ClassId::MeshRenderer, "MeshRenderer"),
    (ClassId::Renderer, "Renderer"),
    (ClassId::ParticleRenderer, "ParticleRenderer"),
    (ClassId::Texture, "Texture"),
    (ClassId::Texture2D, "Texture2D"),
    (ClassId::OcclusionCullingSettings, "OcclusionCullingSettings"),
    (ClassId::GraphicsSettings, "GraphicsSettings"),
    (ClassId::MeshFilter, "MeshFilter"),
    (ClassId::OcclusionPortal, "OcclusionPortal"),
    (ClassId::Mesh, "Mesh"),
    (ClassId::Skybox, "Skybox"),
    (ClassId::QualitySettings, "QualitySettings"),
    (ClassId::Shader, "Shader"),
    (ClassId::TextAsset, "TextAsset"),
    (ClassId::Rigidbody2D, "Rigidbody2D"),
    (ClassId::Physics2DManager, "Physics2DManager"),
    (ClassId::Collider2D, "Collider2D"),
    (ClassId::Rigidbody, "Rigidbody"),
    (ClassId::PhysicsManager, "PhysicsManager"),
    (ClassId::Collider, "Collider"),
    (ClassId::Joint, "Joint"),
    (ClassId::CircleCollider2D, "CircleCollider2D"),
    (ClassId::HingeJoint, "HingeJoint"),
    (ClassId::PolygonCollider2D, "PolygonCollider2D"),
    (ClassId::BoxCollider2D, "BoxCollider2D"),
    (ClassId::PhysicsMaterial2D, "PhysicsMaterial2D"),
    (ClassId::MeshCollider, "MeshCollider"),
    (ClassId::BoxCollider, "BoxCollider"),
    (ClassId::CompositeCollider2D, "CompositeCollider2D"),
    (ClassId::EdgeCollider2D, "EdgeCollider2D"),
    (ClassId::CapsuleCollider2D, "CapsuleCollider2D"),
    (ClassId::ComputeShader, "ComputeShader"),
    (ClassId::AnimationClip, "AnimationClip"),
    (ClassId::ConstantForce, "ConstantForce"),
    (ClassId::WorldParticleCollider, "WorldParticleCollider"),
    (ClassId::TagManager, "TagManager"),
    (ClassId::AudioListener, "AudioListener"),
    (ClassId::AudioSource, "AudioSource"),
    (ClassId::AudioClip, "AudioClip"),
    (ClassId::RenderTexture, "RenderTexture"),
    (ClassId::CustomRenderTexture, "CustomRenderTexture"),
    (ClassId::MeshParticleEmitter, "MeshParticleEmitter"),
    (ClassId::ParticleEmitter, "ParticleEmitter"),
    (ClassId::Cubemap, "Cubemap"),
    (ClassId::Avatar, "Avatar"),
    (ClassId::AnimatorController, "AnimatorController"),
    (ClassId::GUILayer, "GUILayer"),
    (ClassId::RuntimeAnimatorController, "RuntimeAnimatorController"),
    (ClassId::ScriptMapper, "ScriptMapper"),
    (ClassId::Animator, "Animator"),
    (ClassId::TrailRenderer, "TrailRenderer"),
    (ClassId::DelayedCallManager, "DelayedCallManager"),
    (ClassId::TextMesh, "TextMesh"),
    (ClassId::RenderSettings, "RenderSettings"),
    (ClassId::Light, "Light"),
    (ClassId::CGProgram, "CGProgram"),
    (ClassId::BaseAnimationTrack, "BaseAnimationTrack"),
    (ClassId::Animation, "Animation"),
    (ClassId::MonoBehaviour, "MonoBehaviour"),
    (ClassId::MonoScript, "MonoScript"),
    (ClassId::MonoManager, "MonoManager"),
    (ClassId::Texture3D, "Texture3D"),
    (ClassId::NewAnimationTrack, "NewAnimationTrack"),
    (ClassId::Projector, "Projector"),
    (ClassId::LineRenderer, "LineRenderer"),
    (ClassId::Flare, "Flare"),
    (ClassId::Halo, "Halo"),
    (ClassId::LensFlare, "LensFlare"),
    (ClassId::FlareLayer, "FlareLayer"),
    (ClassId::HaloLayer, "HaloLayer"),
    (ClassId::NavMeshProjectSettings, "NavMeshProjectSettings"),
    (ClassId::HaloManager, "HaloManager"),
    (ClassId::Font, "Font"),
    (ClassId::PlayerSettings, "PlayerSettings"),
    (ClassId::NamedObject, "NamedObject"),
    (ClassId::GUITexture, "GUITexture"),
    (ClassId::GUIText, "GUIText"),
    (ClassId::GUIElement, "GUIElement"),
    (ClassId::PhysicMaterial, "PhysicMaterial"),
    (ClassId::SphereCollider, "SphereCollider"),
    (ClassId::CapsuleCollider, "CapsuleCollider"),
    (ClassId::SkinnedMeshRenderer, "SkinnedMeshRenderer"),
    (ClassId::FixedJoint, "FixedJoint"),
    (ClassId::RaycastCollider, "RaycastCollider"),
    (ClassId::BuildSettings, "BuildSettings"),
    (ClassId::AssetBundle, "AssetBundle"),
    (ClassId::CharacterController, "CharacterController"),
    (ClassId::CharacterJoint, "CharacterJoint"),
    (ClassId::SpringJoint, "SpringJoint"),
    (ClassId::WheelCollider, "WheelCollider"),
    (ClassId::ResourceManager, "ResourceManager"),
    (ClassId::NetworkView, "NetworkView"),
    (ClassId::NetworkManager, "NetworkManager"),
    (ClassId::PreloadData, "PreloadData"),
    (ClassId::MovieTexture, "MovieTexture"),
    (ClassId::ConfigurableJoint, "ConfigurableJoint"),
    (ClassId::TerrainCollider, "TerrainCollider"),
    (ClassId::MasterServerInterface, "MasterServerInterface"),
    (ClassId::TerrainData, "TerrainData"),
    (ClassId::LightmapSettings, "LightmapSettings"),
    (ClassId::WebCamTexture, "WebCamTexture"),
    (ClassId::EditorSettings, "EditorSettings"),
    (ClassId::InteractiveCloth, "InteractiveCloth"),
    (ClassId::ClothRenderer, "ClothRenderer"),
    (ClassId::EditorUserSettings, "EditorUserSettings"),
    (ClassId::SkinnedCloth, "SkinnedCloth"),
    (ClassId::AudioReverbFilter, "AudioReverbFilter"),
    (ClassId::AudioHighPassFilter, "AudioHighPassFilter"),
    (ClassId::AudioChorusFilter, "AudioChorusFilter"),
    (ClassId::AudioReverbZone, "AudioReverbZone"),
    (ClassId::AudioEchoFilter, "AudioEchoFilter"),
    (ClassId::AudioLowPassFilter, "AudioLowPassFilter"),
    (ClassId::AudioDistortionFilter, "AudioDistortionFilter"),
    (ClassId::SparseTexture, "SparseTexture"),
    (ClassId::AudioBehaviour, "AudioBehaviour"),
    (ClassId::AudioFilter, "AudioFilter"),
    (ClassId::WindZone, "WindZone"),
    (ClassId::Cloth, "Cloth"),
    (ClassId::SubstanceArchive, "SubstanceArchive"),
    (ClassId::ProceduralMaterial, "ProceduralMaterial"),
    (ClassId::ProceduralTexture, "ProceduralTexture"),
    (ClassId::Texture2DArray, "Texture2DArray"),
    (ClassId::CubemapArray, "CubemapArray"),
    (ClassId::OffMeshLink, "OffMeshLink"),
    (ClassId::OcclusionArea, "OcclusionArea"),
    (ClassId::Tree, "Tree"),
    (ClassId::NavMeshObsolete, "NavMeshObsolete"),
    (ClassId::NavMeshAgent, "NavMeshAgent"),
    (ClassId::NavMeshSettings, "NavMeshSettings"),
    (ClassId::LightProbesLegacy, "LightProbesLegacy"),
    (ClassId::ParticleSystem, "ParticleSystem"),
    (ClassId::ParticleSystemRenderer, "ParticleSystemRenderer"),
    (ClassId::ShaderVariantCollection, "ShaderVariantCollection"),
    (ClassId::LODGroup, "LODGroup"),
    (ClassId::BlendTree, "BlendTree"),
    (ClassId::Motion, "Motion"),
    (ClassId::NavMeshObstacle, "NavMeshObstacle"),
    (ClassId::SortingGroup, "SortingGroup"),
    (ClassId::SpriteRenderer, "SpriteRenderer"),
    (ClassId::Sprite, "Sprite"),
    (ClassId::CachedSpriteAtlas, "CachedSpriteAtlas"),
    (ClassId::ReflectionProbe, "ReflectionProbe"),
    (ClassId::ReflectionProbes, "ReflectionProbes"),
    (ClassId::Terrain, "Terrain"),
    (ClassId::LightProbeGroup, "LightProbeGroup"),
    (ClassId::AnimatorOverrideController, "AnimatorOverrideController"),
    (ClassId::CanvasRenderer, "CanvasRenderer"),
    (ClassId::Canvas, "Canvas"),
    (ClassId::RectTransform, "RectTransform"),
    (ClassId::CanvasGroup, "CanvasGroup"),
    (ClassId::BillboardAsset, "BillboardAsset"),
    (ClassId::BillboardRenderer, "BillboardRenderer"),
    (ClassId::SpeedTreeWindAsset, "SpeedTreeWindAsset"),
    (ClassId::AnchoredJoint2D, "AnchoredJoint2D"),
    (ClassId::Joint2D, "Joint2D"),
    (ClassId::SpringJoint2D, "SpringJoint2D"),
    (ClassId::DistanceJoint2D, "DistanceJoint2D"),
    (ClassId::HingeJoint2D, "HingeJoint2D"),
    (ClassId::SliderJoint2D, "SliderJoint2D"),
    (ClassId::WheelJoint2D, "WheelJoint2D"),
    (ClassId::ClusterInputManager, "ClusterInputManager"),
    (ClassId::BaseVideoTexture, "BaseVideoTexture"),
    (ClassId::NavMeshData, "NavMeshData"),
    (ClassId::AudioMixer, "AudioMixer"),
    (ClassId::AudioMixerController, "AudioMixerController"),
    (ClassId::AudioMixerGroupController, "AudioMixerGroupController"),
    (ClassId::AudioMixerEffectController, "AudioMixerEffectController"),
    (ClassId::AudioMixerSnapshotController, "AudioMixerSnapshotController"),
    (ClassId::PhysicsUpdateBehaviour2D, "PhysicsUpdateBehaviour2D"),
    (ClassId::ConstantForce2D, "ConstantForce2D"),
    (ClassId::Effector2D, "Effector2D"),
    (ClassId::AreaEffector2D, "AreaEffector2D"),
    (ClassId::PointEffector2D, "PointEffector2D"),
    (ClassId::PlatformEffector2D, "PlatformEffector2D"),
    (ClassId::SurfaceEffector2D, "SurfaceEffector2D"),
    (ClassId::BuoyancyEffector2D, "BuoyancyEffector2D"),
    (ClassId::RelativeJoint2D, "RelativeJoint2D"),
    (ClassId::FixedJoint2D, "FixedJoint2D"),
    (ClassId::FrictionJoint2D, "FrictionJoint2D"),
    (ClassId::TargetJoint2D, "TargetJoint2D"),
    (ClassId::LightProbes, "LightProbes"),
    (ClassId::LightProbeProxyVolume, "LightProbeProxyVolume"),
    (ClassId::SampleClip, "SampleClip"),
    (ClassId::AudioMixerSnapshot, "AudioMixerSnapshot"),
    (ClassId::AudioMixerGroup, "AudioMixerGroup"),
    (ClassId::NScreenBridge, "NScreenBridge"),
    (ClassId::AssetBundleManifest, "AssetBundleManifest"),
    (ClassId::UnityAdsManager, "UnityAdsManager"),
    (ClassId::RuntimeInitializeOnLoadManager, "RuntimeInitializeOnLoadManager"),
    (ClassId::CloudWebServicesManager, "CloudWebServicesManager"),
    (ClassId::UnityAnalyticsManager, "UnityAnalyticsManager"),
    (ClassId::CrashReportManager, "CrashReportManager"),
    (ClassId::PerformanceReportingManager, "PerformanceReportingManager"),
    (ClassId::UnityConnectSettings, "UnityConnectSettings"),
    (ClassId::AvatarMask, "AvatarMask"),
    (ClassId::PlayableDirector, "PlayableDirector"),
    (ClassId::VideoPlayer, "VideoPlayer"),
    (ClassId::VideoClip, "VideoClip"),
    (ClassId::ParticleSystemForceField, "ParticleSystemForceField"),
    (ClassId::SpriteMask, "SpriteMask"),
    (ClassId::WorldAnchor, "WorldAnchor"),
    (ClassId::OcclusionCullingData, "OcclusionCullingData"),
    (ClassId::SmallestEditorClassId, "SmallestEditorClassId"),
    (ClassId::PrefabInstance, "PrefabInstance"),
    (ClassId::EditorExtensionImpl, "EditorExtensionImpl"),
    (ClassId::AssetImporter, "AssetImporter"),
    (ClassId::AssetDatabaseV1, "AssetDatabaseV1"),
    (ClassId::Mesh3DSImporter, "Mesh3DSImporter"),
    (ClassId::TextureImporter, "TextureImporter"),
    (ClassId::ShaderImporter, "ShaderImporter"),
    (ClassId::ComputeShaderImporter, "ComputeShaderImporter"),
    (ClassId::AudioImporter, "AudioImporter"),
    (ClassId::HierarchyState, "HierarchyState"),
    (ClassId::GUIDSerializer, "GUIDSerializer"),
    (ClassId::AssetMetaData, "AssetMetaData"),
    (ClassId::DefaultAsset, "DefaultAsset"),
    (ClassId::DefaultImporter, "DefaultImporter"),
    (ClassId::TextScriptImporter, "TextScriptImporter"),
    (ClassId::SceneAsset, "SceneAsset"),
    (ClassId::NativeFormatImporter, "NativeFormatImporter"),
    (ClassId::MonoImporter, "MonoImporter"),
    (ClassId::AssetServerCache, "AssetServerCache"),
    (ClassId::LibraryAssetImporter, "LibraryAssetImporter"),
    (ClassId::ModelImporter, "ModelImporter"),
    (ClassId::FBXImporter, "FBXImporter"),
    (ClassId::TrueTypeFontImporter, "TrueTypeFontImporter"),
    (ClassId::MovieImporter, "MovieImporter"),
    (ClassId::EditorBuildSettings, "EditorBuildSettings"),
    (ClassId::DDSImporter, "DDSImporter"),
    (ClassId::InspectorExpandedState, "InspectorExpandedState"),
    (ClassId::AnnotationManager, "AnnotationManager"),
    (ClassId::PluginImporter, "PluginImporter"),
    (ClassId::EditorUserBuildSettings, "EditorUserBuildSettings"),
    (ClassId::PVRImporter, "PVRImporter"),
    (ClassId::ASTCImporter, "ASTCImporter"),
    (ClassId::KTXImporter, "KTXImporter"),
    (ClassId::IHVImageFormatImporter, "IHVImageFormatImporter"),
    (ClassId::AnimatorStateTransition, "AnimatorStateTransition"),
    (ClassId::AnimatorState, "AnimatorState"),
    (ClassId::HumanTemplate, "HumanTemplate"),
    (ClassId::AnimatorStateMachine, "AnimatorStateMachine"),
    (ClassId::PreviewAnimationClip, "PreviewAnimationClip"),
    (ClassId::AnimatorTransition, "AnimatorTransition"),
    (ClassId::SpeedTreeImporter, "SpeedTreeImporter"),
    (ClassId::AnimatorTransitionBase, "AnimatorTransitionBase"),
    (ClassId::SubstanceImporter, "SubstanceImporter"),
    (ClassId::LightmapParameters, "LightmapParameters"),
    (ClassId::LightingDataAsset, "LightingDataAsset"),
    (ClassId::GISRaster, "GISRaster"),
    (ClassId::GISRasterImporter, "GISRasterImporter"),
    (ClassId::CadImporter, "CadImporter"),
    (ClassId::SketchUpImporter, "SketchUpImporter"),
    (ClassId::BuildReport, "BuildReport"),
    (ClassId::PackedAssets, "PackedAssets"),
    (ClassId::VideoClipImporter, "VideoClipImporter"),
    (ClassId::ActivationLogComponent, "ActivationLogComponent"),
    (ClassId::int, "int"),
    (ClassId::bool, "bool"),
    (ClassId::float, "float"),
    (ClassId::MonoObject, "MonoObject"),
    (ClassId::Collision, "Collision"),
    (ClassId::Vector3f, "Vector3f"),
    (ClassId::RootMotionData, "RootMotionData"),
    (ClassId::Collision2D, "Collision2D"),
    (ClassId::AudioMixerLiveUpdateFloat, "AudioMixerLiveUpdateFloat"),
    (ClassId::AudioMixerLiveUpdateBool, "AudioMixerLiveUpdateBool"),
    (ClassId::Polygon2D, "Polygon2D"),
    (ClassId::void, "void"),
    (ClassId::TilemapCollider2D, "TilemapCollider2D"),
    (ClassId::AssetImporterLog, "AssetImporterLog"),
    (ClassId::VFXRenderer, "VFXRenderer"),
    (ClassId::SerializableManagedRefTestClass, "SerializableManagedRefTestClass"),
    (ClassId::Grid, "Grid"),
    (ClassId::ScenesUsingAssets, "ScenesUsingAssets"),
    (ClassId::ArticulationBody, "ArticulationBody"),
    (ClassId::Preset, "Preset"),
    (ClassId::EmptyObject, "EmptyObject"),
    (ClassId::IConstraint, "IConstraint"),
    (ClassId::TestObjectWithSpecialLayoutOne, "TestObjectWithSpecialLayoutOne"),
    (ClassId::AssemblyDefinitionReferenceImporter, "AssemblyDefinitionReferenceImporter"),
    (ClassId::SiblingDerived, "SiblingDerived"),
    (ClassId::TestObjectWithSerializedMapStringNonAlignedStruct, "TestObjectWithSerializedMapStringNonAlignedStruct"),
    (ClassId::SubDerived, "SubDerived"),
    (ClassId::AssetImportInProgressProxy, "AssetImportInProgressProxy"),
    (ClassId::PluginBuildInfo, "PluginBuildInfo"),
    (ClassId::EditorProjectAccess, "EditorProjectAccess"),
    (ClassId::PrefabImporter, "PrefabImporter"),
    (ClassId::TestObjectWithSerializedArray, "TestObjectWithSerializedArray"),
    (ClassId::TestObjectWithSerializedAnimationCurve, "TestObjectWithSerializedAnimationCurve"),
    (ClassId::TilemapRenderer, "TilemapRenderer"),
    (ClassId::ScriptableCamera, "ScriptableCamera"),
    (ClassId::SpriteAtlasAsset, "SpriteAtlasAsset"),
    (ClassId::SpriteAtlasDatabase, "SpriteAtlasDatabase"),
    (ClassId::AudioBuildInfo, "AudioBuildInfo"),
    (ClassId::CachedSpriteAtlasRuntimeData, "CachedSpriteAtlasRuntimeData"),
    (ClassId::RendererFake, "RendererFake"),
    (ClassId::AssemblyDefinitionReferenceAsset, "AssemblyDefinitionReferenceAsset"),
    (ClassId::BuiltAssetBundleInfoSet, "BuiltAssetBundleInfoSet"),
    (ClassId::SpriteAtlas, "SpriteAtlas"),
    (ClassId::RayTracingShaderImporter, "RayTracingShaderImporter"),
    (ClassId::RayTracingShader, "RayTracingShader"),
    (ClassId::LightingSettings, "LightingSettings"),
    (ClassId::PlatformModuleSetup, "PlatformModuleSetup"),
    (ClassId::VersionControlSettings, "VersionControlSettings"),
    (ClassId::AimConstraint, "AimConstraint"),
    (ClassId::VFXManager, "VFXManager"),
    (ClassId::VisualEffectSubgraph, "VisualEffectSubgraph"),
    (ClassId::VisualEffectSubgraphOperator, "VisualEffectSubgraphOperator"),
    (ClassId::VisualEffectSubgraphBlock, "VisualEffectSubgraphBlock"),
    (ClassId::LocalizationImporter, "LocalizationImporter"),
    (ClassId::Derived, "Derived"),
    (ClassId::PropertyModificationsTargetTestObject, "PropertyModificationsTargetTestObject"),
    (ClassId::ReferencesArtifactGenerator, "ReferencesArtifactGenerator"),
    (ClassId::AssemblyDefinitionAsset, "AssemblyDefinitionAsset"),
    (ClassId::SceneVisibilityState, "SceneVisibilityState"),
    (ClassId::LookAtConstraint, "LookAtConstraint"),
    (ClassId::SpriteAtlasImporter, "SpriteAtlasImporter"),
    (ClassId::MultiArtifactTestImporter, "MultiArtifactTestImporter"),
    (ClassId::GameObjectRecorder, "GameObjectRecorder"),
    (ClassId::LightingDataAssetParent, "LightingDataAssetParent"),
    (ClassId::PresetManager, "PresetManager"),
    (ClassId::TestObjectWithSpecialLayoutTwo, "TestObjectWithSpecialLayoutTwo"),
    (ClassId::StreamingManager, "StreamingManager"),
    (ClassId::LowerResBlitTexture, "LowerResBlitTexture"),
    (ClassId::StreamingController, "StreamingController"),
    (ClassId::RenderPassAttachment, "RenderPassAttachment"),
    (ClassId::TestObjectVectorPairStringBool, "TestObjectVectorPairStringBool"),
    (ClassId::GridLayout, "GridLayout"),
    (ClassId::AssemblyDefinitionImporter, "AssemblyDefinitionImporter"),
    (ClassId::ParentConstraint, "ParentConstraint"),
    (ClassId::FakeComponent, "FakeComponent"),
    (ClassId::PositionConstraint, "PositionConstraint"),
    (ClassId::RotationConstraint, "RotationConstraint"),
    (ClassId::ScaleConstraint, "ScaleConstraint"),
    (ClassId::Tilemap, "Tilemap"),
    (ClassId::PackageManifest, "PackageManifest"),
    (ClassId::PackageManifestImporter, "PackageManifestImporter"),
    (ClassId::TerrainLayer, "TerrainLayer"),
    (ClassId::SpriteShapeRenderer, "SpriteShapeRenderer"),
    (ClassId::NativeObjectType, "NativeObjectType"),
    (ClassId::TestObjectWithSerializedMapStringBool, "TestObjectWithSerializedMapStringBool"),
    (ClassId::SerializableManagedHost, "SerializableManagedHost"),
    (ClassId::VisualEffectAsset, "VisualEffectAsset"),
    (ClassId::VisualEffectImporter, "VisualEffectImporter"),
    (ClassId::VisualEffectResource, "VisualEffectResource"),
    (ClassId::VisualEffectObject, "VisualEffectObject"),
    (ClassId::VisualEffect, "VisualEffect"),
    (ClassId::LocalizationAsset, "LocalizationAsset"),
    (ClassId::ScriptedImporter, "ScriptedImporter")
]
    .iter()
    .copied()
    .collect());
