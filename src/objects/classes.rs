#![allow(warnings)]
use crate::objects::PPtr;
use serde::{Deserialize, Serialize};

/// AABB is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AABB {
    pub m_Center: Vector3f,
    pub m_Extent: Vector3f,
}

/// AddedComponent is a sub class of the Unity engine since version 2022.2.0b16.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SceneManagement.AddedComponent.html):
/**
Class with information about a component that has been added to a Prefab instance.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AddedComponent {
    pub addedObject: PPtr, /*<Component>*/
    pub insertIndex: i32,
    pub targetCorrespondingSourceObject: PPtr, /*<GameObject>*/
}

/// AddedGameObject is a sub class of the Unity engine since version 2022.2.0b16.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SceneManagement.AddedGameObject.html):
/**
Class with information about a GameObject that has been added as a child under a Prefab instance.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AddedGameObject {
    pub addedObject: PPtr, /*<Transform>*/
    pub insertIndex: i32,
    pub targetCorrespondingSourceObject: PPtr, /*<Transform>*/
}

/// AimConstraint is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AimConstraint.html):
/**
Constrains the orientation of an object relative to the position of one or more source objects, such that the object is facing the average position of the sources.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AimConstraint {
    pub m_AffectRotationX: bool,
    pub m_AffectRotationY: bool,
    pub m_AffectRotationZ: bool,
    /**The axis towards which the constrained object orients.*/
    pub m_AimVector: Vector3f,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The rotation used when the sources have a total weight of 0.*/
    pub m_RotationAtRest: Vector3f,
    /**Represents an offset from the constrained orientation.*/
    pub m_RotationOffset: Vector3f,
    pub m_Sources: Vec<ConstraintSource>,
    pub m_UpType: i32,
    /**The up vector.*/
    pub m_UpVector: Vector3f,
    /**The weight of the constraint component.*/
    pub m_Weight: f32,
    /**The world up object, used to calculate the world up vector when the world up Type is AimConstraint.WorldUpType.ObjectUp or AimConstraint.WorldUpType.ObjectRotationUp.*/
    pub m_WorldUpObject: PPtr, /*<Transform>*/
    /**The world up Vector used when the world up type is AimConstraint.WorldUpType.Vector or AimConstraint.WorldUpType.ObjectRotationUp.*/
    pub m_WorldUpVector: Vector3f,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_Active: Option<bool>,
    /// bool: (2018.4.15f1 - 2021.2.16f1)
    pub m_IsContraintActive: Option<bool>,
}

/// AndroidAssetPackImporter is a  class of the Unity engine since version 2020.3.42f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AndroidAssetPackImporter.html):
/**
Represents an Android asset pack directory in a project.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidAssetPackImporter {
    /**Get or set the AssetBundle name.*/
    pub m_AssetBundleName: String,
    /**Get or set the AssetBundle variant.*/
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    /**Get or set any user data.*/
    pub m_UserData: String,
}

/// Animation is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animation.html):
/**
The animation component is used to play back animations.
You can assign animation clips to the animation component and control playback from your script.

The animation system in Unity is weight-based and supports Animation Blending, Additive animations, Animation Mixing, Layers and full control over all aspects of playback.For an overview of animation scripting in Unity please read this introduction.AnimationState can be used to change the layer of an animation, modify playback speed, and for direct control over blending and mixing.Also Animation supports enumerators. Looping through all AnimationStates is performed like this:
See Also: An overview of animation scripting in Unity is here.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Animation {
    /**When turned on, animations will be executed in the physics loop. This is only useful in conjunction with kinematic rigidbodies.*/
    pub m_AnimatePhysics: bool,
    pub m_Animation: PPtr, /*<AnimationClip>*/
    pub m_Animations: Vec<PPtr /*<AnimationClip>*/>,
    /**Controls culling of this Animation component.*/
    pub m_CullingType: i32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Should the default animation clip (the Animation.clip property) automatically start playing on startup?*/
    pub m_PlayAutomatically: bool,
    /**How should time beyond the playback range of the clip be treated?*/
    pub m_WrapMode: i32,
    /// AABB: (3.4.0 - 3.4.0)
    pub m_UserAABB: Option<AABB>,
}

/// AnimationClip is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AnimationClip.html):
/**
Stores keyframe based animations.
AnimationClip is used by Animation to play back animations.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationClip {
    pub m_Bounds: AABB,
    pub m_Compressed: bool,
    pub m_CompressedRotationCurves: Vec<CompressedAnimationCurve>,
    /**Animation Events for this animation clip.*/
    pub m_Events: Vec<AnimationEvent>,
    pub m_FloatCurves: Vec<FloatCurve>,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_PositionCurves: Vec<Vector3Curve>,
    pub m_RotationCurves: Vec<QuaternionCurve>,
    pub m_SampleRate: f32,
    pub m_ScaleCurves: Vec<Vector3Curve>,
    /**Sets the default wrap mode used in the animation state.*/
    pub m_WrapMode: i32,
    /// AnimationClipBindingConstant: (5.6.0b2 - 2022.2.0b16)
    pub m_ClipBindingConstant: Option<AnimationClipBindingConstant>,
    /// Vec<Vector3Curve>: (5.6.0b2 - 2022.2.0b16)
    pub m_EulerCurves: Option<Vec<Vector3Curve>>,
    /**Returns true if the Animation has animation on the root transform.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_HasGenericRootTransform: Option<bool>,
    /**Returns true if the AnimationClip has editor curves for its root motion.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_HasMotionFloatCurves: Option<bool>,
    /**Set to true if the AnimationClip will be used with the Legacy Animation component ( instead of the Animator ).*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_Legacy: Option<bool>,
    /// ClipMuscleConstant: (5.6.0b2 - 2022.2.0b16)
    pub m_MuscleClip: Option<ClipMuscleConstant>,
    /// u32: (5.6.0b2 - 2022.2.0b16)
    pub m_MuscleClipSize: Option<u32>,
    /// Vec<PPtrCurve>: (5.6.0b2 - 2022.2.0b16)
    pub m_PPtrCurves: Option<Vec<PPtrCurve>>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_UseHighQualityCurve: Option<bool>,
}

/// AnimationClipBindingConstant is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationClipBindingConstant {
    pub genericBindings: Vec<GenericBinding>,
    pub pptrCurveMapping: Vec<PPtr /*<Object>*/>,
}

/// AnimationClipOverride is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationClipOverride {
    pub m_OriginalClip: PPtr, /*<AnimationClip>*/
    pub m_OverrideClip: PPtr, /*<AnimationClip>*/
}

/// AnimationCurve is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AnimationCurve.html):
/**
Store a collection of Keyframes that can be evaluated over time.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationCurve {
    pub m_Curve: Vec<Keyframe>,
    pub m_PostInfinity: i32,
    pub m_PreInfinity: i32,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_RotationOrder: Option<i32>,
}

/// AnimationEvent is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AnimationEvent.html):
/**
AnimationEvent lets you call a script function similar to SendMessage as part of playing back an animation.
Animation events support functions that take zero or one parameter.
The parameter can be a float, an int, a string, an object reference, or an AnimationEvent.
A more detailed example below shows a more complex
      way of creating an animation.  In this script example the Animator
      component is accessed and a Clip from it obtained.  (This clip was
      set up in the Animation window.)  The clip lasts for 2 seconds.  An
      AnimationEvent is created, and has parameters set.  The parameters include
      the function PrintEvent() which will handle the event. The event is then
      added to the clip.  This all happens in Start().  Once the game has launched
      the event is called after 1.3s and then repeats every 2s.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationEvent {
    pub data: String,
    /**Float parameter that is stored in the event and will be sent to the function.*/
    pub floatParameter: f32,
    /**The name of the function that will be called.*/
    pub functionName: String,
    /**Int parameter that is stored in the event and will be sent to the function.*/
    pub intParameter: i32,
    /**Function call options.*/
    pub messageOptions: i32,
    /**Object reference parameter that is stored in the event and will be sent to the function.*/
    pub objectReferenceParameter: PPtr, /*<Object>*/
    /**The time at which the event will be fired off.*/
    pub time: f32,
}

/// AnimationManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationManager {}

/// Animator is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animator.html):
/**
Interface to control the Mecanim animation system.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Animator {
    pub m_AllowConstantClipSamplingOptimization: bool,
    /**Should root motion be applied?*/
    pub m_ApplyRootMotion: bool,
    /**Gets/Sets the current Avatar.*/
    pub m_Avatar: PPtr, /*<Avatar>*/
    pub m_Controller: PPtr, /*<RuntimeAnimatorController>*/
    /**Controls culling of this Animator component.*/
    pub m_CullingMode: i32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Returns true if the object has a transform hierarchy.*/
    pub m_HasTransformHierarchy: bool,
    pub m_LinearVelocityBlending: bool,
    /**Specifies the update mode of the Animator.*/
    pub m_UpdateMode: i32,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_KeepAnimatorControllerStateOnDisable: Option<bool>,
    /**Automatic stabilization of feet during transition and blending.*/
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub m_StabilizeFeet: Option<bool>,
}

/// AnimatorCondition is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AnimatorCondition.html):
/**
Condition that is used to determine if a transition must be taken.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorCondition {
    pub m_ConditionEvent: String,
    pub m_ConditionMode: i32,
    pub m_EventTreshold: f32,
}

/// AnimatorController is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AnimatorController.html):
/**
The Animator Controller controls animation through layers with state machines, controlled by parameters.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorController {
    /**Retrieves all AnimationClip used by the controller.*/
    pub m_AnimationClips: Vec<PPtr /*<AnimationClip>*/>,
    pub m_Controller: ControllerConstant,
    pub m_ControllerSize: u32,
    pub m_MultiThreadedStateMachine: bool,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_StateMachineBehaviourVectorDescription: StateMachineBehaviourVectorDescription,
    pub m_StateMachineBehaviours: Vec<PPtr /*<MonoBehaviour>*/>,
    pub m_TOS: Vec<(u32, String)>,
}

/// AnimatorOverrideController is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AnimatorOverrideController.html):
/**
Interface to control Animator Override Controller.
Animator Override Controller is used to override Animation Clips from a controller to specialize animations for a given Avatar.
Swapping Animator.runtimeAnimatorController with an AnimatorOverrideController based on the same AnimatorController at runtime doesn't reset state machine's current state.There are three ways to use the Animator Override Controller.
1. Create an Animator Override Controller in the Editor.
2. Change one Animation Clip per frame at runtime (Basic use case).
In this case the indexer operator AnimatorOverrideController.this[string] could be used, but be careful as each call will trigger a reallocation of the animator's clip bindings.
3. Changing many Animation Clips per frame at runtime (Advanced use case).

The AnimatorOverrideController.ApplyOverrides method is well suited for this case as it reduce the number of animator's clips bindings reallocation to only one per call.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorOverrideController {
    pub m_Clips: Vec<AnimationClipOverride>,
    pub m_Controller: PPtr, /*<RuntimeAnimatorController>*/
    /**The name of the object.*/
    pub m_Name: String,
}

/// AnimatorState is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AnimatorState.html):
/**
States are the basic building blocks of a state machine. Each state contains a Motion ( AnimationClip or BlendTree) which will play while the character is in that state. When an event in the game triggers a state transition, the character will be left in a new state whose animation sequence will then take over.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorState {
    /**Offset at which the animation loop starts. Useful for synchronizing looped animations.Units is normalized time.*/
    pub m_CycleOffset: f32,
    /**The animator controller parameter that drives the cycle offset value.*/
    pub m_CycleOffsetParameter: String,
    /**Define if the cycle offset value is driven by an Animator controller parameter or by the value set in the editor.*/
    pub m_CycleOffsetParameterActive: bool,
    /**Should Foot IK be respected for this state.*/
    pub m_IKOnFeet: bool,
    /**Should the state be mirrored.*/
    pub m_Mirror: bool,
    /**The animator controller parameter that drives the mirror value.*/
    pub m_MirrorParameter: String,
    /**Define if the mirror value is driven by an Animator controller parameter or by the value set in the editor.*/
    pub m_MirrorParameterActive: bool,
    /**The motion assigned to this state.*/
    pub m_Motion: PPtr, /*<Motion>*/
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Position: Vector3f,
    /**The default speed of the motion.*/
    pub m_Speed: f32,
    /**The animator controller parameter that drives the speed value.*/
    pub m_SpeedParameter: String,
    /**Define if the speed value is driven by an Animator controller parameter or by the value set in the editor.*/
    pub m_SpeedParameterActive: bool,
    pub m_StateMachineBehaviours: Vec<PPtr /*<MonoBehaviour>*/>,
    /**A tag can be used to identify a state.*/
    pub m_Tag: String,
    /**The transitions that are going out of the state.*/
    pub m_Transitions: Vec<PPtr /*<AnimatorStateTransition>*/>,
    /**Whether or not the AnimatorStates writes back the default values for properties that are not animated by its Motion.*/
    pub m_WriteDefaultValues: bool,
    /**If timeParameterActive is true, the value of this Parameter will be used instead of normalized time.*/
    /// String: (2017.4.33f1 - 2022.2.0b16)
    pub m_TimeParameter: Option<String>,
    /**If true, use value of given Parameter as normalized time.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_TimeParameterActive: Option<bool>,
}

/// AnimatorStateMachine is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AnimatorStateMachine.html):
/**
A graph controlling the interaction of states. Each state references a motion.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorStateMachine {
    /**The position of the AnyState node.*/
    pub m_AnyStatePosition: Vector3f,
    /**The list of AnyState transitions.*/
    pub m_AnyStateTransitions: Vec<PPtr /*<AnimatorStateTransition>*/>,
    pub m_ChildStateMachines: Vec<ChildAnimatorStateMachine>,
    pub m_ChildStates: Vec<ChildAnimatorState>,
    /**The state that the state machine will be in when it starts.*/
    pub m_DefaultState: PPtr, /*<AnimatorState>*/
    /**The position of the entry node.*/
    pub m_EntryPosition: Vector3f,
    /**The list of entry transitions in the state machine.*/
    pub m_EntryTransitions: Vec<PPtr /*<AnimatorTransition>*/>,
    /**The position of the exit node.*/
    pub m_ExitPosition: Vector3f,
    /**The name of the object.*/
    pub m_Name: String,
    /**The position of the parent state machine node. Only valid when in a hierachic state machine.*/
    pub m_ParentStateMachinePosition: Vector3f,
    pub m_StateMachineBehaviours: Vec<PPtr /*<MonoBehaviour>*/>,
    pub m_StateMachineTransitions: Vec<(
        PPtr, /*<AnimatorStateMachine>*/
        Vec<PPtr /*<AnimatorTransition>*/>,
    )>,
}

/// AnimatorStateTransition is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AnimatorStateTransition.html):
/**
Transitions define when and how the state machine switch from one state to another. AnimatorStateTransition always originate from an Animator State (or AnyState) and have timing parameters.
A transition happens when all its conditions are met.  AnimatorStateTransition derives from AnimatorTransitionBase.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorStateTransition {
    /**Set to true to allow or disallow transition to self during AnyState transition.*/
    pub m_CanTransitionToSelf: bool,
    /**AnimatorCondition conditions that need to be met for a transition to happen.*/
    pub m_Conditions: Vec<AnimatorCondition>,
    pub m_DstState: PPtr,        /*<AnimatorState>*/
    pub m_DstStateMachine: PPtr, /*<AnimatorStateMachine>*/
    /**If AnimatorStateTransition.hasExitTime is true, exitTime represents the exact time at which the transition can take effect.This is represented in normalized time, so for example an exit time of 0.75 means that on the first frame where 75% of the animation has played, the Exit Time condition will be true. On the next frame, the condition will be false.For looped animations, transitions with exit times smaller than 1 will be evaluated every loop, so you can use this to time your transition with the proper timing in the animation, every loop.Transitions with exit times greater than one will be evaluated only once, so they can be used to exit at a specific time, after a fixed number of loops. For example, a transition with an exit time of 3.5 will be evaluated once, after three and a half loops.*/
    pub m_ExitTime: f32,
    /**When active the transition will have an exit time condition.*/
    pub m_HasExitTime: bool,
    /**Determines whether the duration of the transition is reported in a fixed duration in seconds or as a normalized time.*/
    pub m_HasFixedDuration: bool,
    /**Which AnimatorState transitions can interrupt the Transition.*/
    pub m_InterruptionSource: i32,
    /**Is the transition destination the exit of the current state machine.*/
    pub m_IsExit: bool,
    /**Mutes the transition. The transition will never occur.*/
    pub m_Mute: bool,
    /**The name of the object.*/
    pub m_Name: String,
    /**The Transition can be interrupted by a transition that has a higher priority.*/
    pub m_OrderedInterruption: bool,
    /**Mutes all other transitions in the source state.*/
    pub m_Solo: bool,
    pub m_TransitionDuration: f32,
    pub m_TransitionOffset: f32,
}

/// AnimatorTransition is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AnimatorTransition.html):
/**
Transitions define when and how the state machine switch from on state to another. AnimatorTransition always originate from a StateMachine or a StateMachine entry. They do not define timing parameters.
A transition happens when all its conditions are met.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorTransition {
    /**AnimatorCondition conditions that need to be met for a transition to happen.*/
    pub m_Conditions: Vec<AnimatorCondition>,
    pub m_DstState: PPtr,        /*<AnimatorState>*/
    pub m_DstStateMachine: PPtr, /*<AnimatorStateMachine>*/
    /**Is the transition destination the exit of the current state machine.*/
    pub m_IsExit: bool,
    /**Mutes the transition. The transition will never occur.*/
    pub m_Mute: bool,
    /**The name of the object.*/
    pub m_Name: String,
    /**Mutes all other transitions in the source state.*/
    pub m_Solo: bool,
}

/// AnimatorTransitionBase is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AnimatorTransitionBase.html):
/**
Base class for animator transitions. Transitions define when and how the state machine switches from one state to another.
A transition happens when all its conditions are met.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorTransitionBase {
    /**AnimatorCondition conditions that need to be met for a transition to happen.*/
    pub m_Conditions: Vec<AnimatorCondition>,
    pub m_DstState: PPtr,        /*<AnimatorState>*/
    pub m_DstStateMachine: PPtr, /*<AnimatorStateMachine>*/
    /**Is the transition destination the exit of the current state machine.*/
    pub m_IsExit: bool,
    /**Mutes the transition. The transition will never occur.*/
    pub m_Mute: bool,
    /**The name of the object.*/
    pub m_Name: String,
    /**Mutes all other transitions in the source state.*/
    pub m_Solo: bool,
}

/// Annotation is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Annotation {
    pub m_ClassID: i32,
    pub m_Flags: i32,
    pub m_GizmoEnabled: bool,
    pub m_IconEnabled: bool,
    pub m_ScriptClass: String,
}

/// AnnotationManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnnotationManager {
    pub m_CurrentPreset_m_AnnotationList: Vec<Annotation>,
    pub m_RecentlyChanged: Vec<Annotation>,
    /// f32: (2021.2.16f1 - 2022.2.0b16)
    pub m_FadeGizmoSize: Option<f32>,
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub m_FadeGizmos: Option<bool>,
    /// f32: (3.4.0 - 3.4.0)
    pub m_IconSize: Option<f32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_ShowGrid: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_ShowSelectionOutline: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_ShowSelectionWire: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_Use3dGizmos: Option<bool>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_WorldIconSize: Option<f32>,
}

/// AreaEffector2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AreaEffector2D.html):
/**
Applies forces within an area.
When the source Collider2D is a trigger, the effector will apply forces whenever the target Collider2D overlaps the source.  When the source Collider isn't a trigger, the effector will apply forces whenever the target Collider2D is in contact with the source only.This effector is designed primarily to work with source Collider2D that are set as triggers so that target Collider2D can overlap the defined area.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AreaEffector2D {
    /**The angular drag to apply to rigid-bodies.*/
    pub m_AngularDrag: f32,
    /**The mask used to select specific layers allowed to interact with the effector.*/
    pub m_ColliderMask: BitField,
    /**The linear drag to apply to rigid-bodies.*/
    pub m_Drag: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The angle of the force to be applied.*/
    pub m_ForceAngle: f32,
    /**The magnitude of the force to be applied.*/
    pub m_ForceMagnitude: f32,
    /**The target for where the effector applies any force.*/
    pub m_ForceTarget: i32,
    /**The variation of the magnitude of the force to be applied.*/
    pub m_ForceVariation: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Should the collider-mask be used or the global collision matrix?*/
    pub m_UseColliderMask: bool,
    /**Should the forceAngle use global space?*/
    pub m_UseGlobalAngle: bool,
}

/// ArticulationBody is a  class of the Unity engine since version 2020.1.0a20.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ArticulationBody.html):
/**
A body that forms part of a Physics articulation.
An articulation is a set of bodies arranged in a logical tree. The parent-child link in this tree reflects that the bodies have their relative motion constrained. Articulations are solved by a Featherstone solver that works in reduced coordinates - that is each body has relative coordinates to its parent but only along the unlocked degrees of freedom. This guarantees there is no unwanted stretch.

Like with regular Joints, there are two anchors for each pair of connected articulation bodies. One anchor is defined in the parent body's reference frame, whereas the other one is defined in the child's reference frame. Changing the constraints, you directly affect the allowed space for relative positions of the two anchors. For instance, ArticulationDofLock.LockedMotion will not allow any relative motion at all.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ArticulationBody {
    /**Position of the anchor relative to this body.*/
    pub m_AnchorPosition: Vector3f,
    /**Rotation of the anchor relative to this body.*/
    pub m_AnchorRotation: Quaternionf,
    /**Damping factor that affects how this body resists rotations.*/
    pub m_AngularDamping: f32,
    pub m_ArticulationJointType: i32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Allows you to specify that this body is not movable.*/
    pub m_Immovable: bool,
    /**Allows you to specify the amount of friction that is applied as a result of the parent body moving relative to this body.*/
    pub m_JointFriction: f32,
    /**Damping factor that affects how this body resists linear motion.*/
    pub m_LinearDamping: f32,
    pub m_LinearX: i32,
    pub m_LinearY: i32,
    pub m_LinearZ: i32,
    /**The mass of this articulation body.*/
    pub m_Mass: f32,
    /**Position of the anchor relative to this body's parent.*/
    pub m_ParentAnchorPosition: Vector3f,
    /**Rotation of the anchor relative to this body's parent.*/
    pub m_ParentAnchorRotation: Quaternionf,
    pub m_SwingY: i32,
    pub m_SwingZ: i32,
    pub m_Twist: i32,
    /**The properties of drive along or around X.*/
    pub m_XDrive: ArticulationDrive,
    /**The properties of drive along or around Y.*/
    pub m_YDrive: ArticulationDrive,
    /**The properties of drive along or around Z.*/
    pub m_ZDrive: ArticulationDrive,
    /**The center of mass of the body defined in local space.*/
    /// Vector3f: (2022.2.0b16 - 2022.2.0b16)
    pub m_CenterOfMass: Option<Vector3f>,
    /**The ArticulationBody's collision detection mode.*/
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_CollisionDetectionMode: Option<i32>,
    /// bool: (2020.1.0a20 - 2020.3.42f1)
    pub m_ComputeParentAnchor: Option<bool>,
    /**The additional layers that all Colliders attached to this ArticulationBody should exclude when deciding if the Collider can come into contact with another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ImplicitCom: Option<bool>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ImplicitTensor: Option<bool>,
    /**The additional layers that all Colliders attached to this ArticulationBody should include when deciding if a the Collider can come into contact with another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /// Quaternionf: (2022.2.0b16 - 2022.2.0b16)
    pub m_InertiaRotation: Option<Quaternionf>,
    /**The inertia tensor of this body.*/
    /// Vector3f: (2022.2.0b16 - 2022.2.0b16)
    pub m_InertiaTensor: Option<Vector3f>,
    /**Whether the parent anchor should be computed automatically or not.*/
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub m_MatchAnchors: Option<bool>,
    /**Controls whether gravity affects this articulation body.*/
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_UseGravity: Option<bool>,
}

/// ArticulationDrive is a sub class of the Unity engine since version 2020.1.0a20.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ArticulationDrive.html):
/**
Drive applies forces and torques to the connected bodies.
Drive moves the body along one degree of freedom, be it a linear motion along a particular axis or a rotational motion around a particular axis. The drive will apply force to the body that is calculated from the current value of the drive, using this formula: F = stiffness * (currentPosition - target) - damping * (currentVelocity - targetVelocity). In this formula, currentPosition and currentVelocity are linear position and linear velocity in case of the linear drive. In case of the rotational drive, currentPosition and currentVelocity correspond to the angle and angular velocity respectively.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ArticulationDrive {
    /**The damping of the spring attached to this drive.*/
    pub damping: f32,
    /**The maximum force this drive can apply to a body.*/
    pub forceLimit: f32,
    /**The lower limit of motion for a particular degree of freedom.*/
    pub lowerLimit: f32,
    /**The stiffness of the spring connected to this drive.*/
    pub stiffness: f32,
    /**The target value the drive will try to reach.*/
    pub target: f32,
    /**The velocity of the body this drive will try to reach.*/
    pub targetVelocity: f32,
    /**The upper limit of motion for a particular degree of freedom.*/
    pub upperLimit: f32,
    /**Specifies which drive type to use for this drive.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub driveType: Option<i32>,
}

/// AspectRatios is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AspectRatios {
    pub Others: bool,
    /// bool: (3.4.0 - 2021.2.16f1)
    #[serde(alias = "16:10")]
    pub _16_10: Option<bool>,
    /// bool: (3.4.0 - 2021.2.16f1)
    #[serde(alias = "16:9")]
    pub _16_9: Option<bool>,
    /// bool: (3.4.0 - 2021.2.16f1)
    #[serde(alias = "4:3")]
    pub _4_3: Option<bool>,
    /// bool: (3.4.0 - 2021.2.16f1)
    #[serde(alias = "5:4")]
    pub _5_4: Option<bool>,
}

/// AssemblyDefinitionAsset is a  class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssemblyDefinitionAsset {
    pub m_Name: String,
    pub m_Script: String,
}

/// AssemblyDefinitionImporter is a  class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssemblyDefinitionImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_Name: String,
    pub m_UserData: String,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// AssemblyDefinitionReferenceAsset is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssemblyDefinitionReferenceAsset {
    pub m_Name: String,
    pub m_Script: String,
}

/// AssemblyDefinitionReferenceImporter is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssemblyDefinitionReferenceImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// Asset is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/VersionControl.Asset.html):
/**
This class containes information about the version control state of an asset.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub children: Vec<GUID>,
    pub labels: AssetLabels,
    pub mainRepresentation: LibraryRepresentation,
    pub parent: GUID,
    pub representations: Vec<LibraryRepresentation>,
    /// i32: (3.4.0 - 2020.1.0a20)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// i32: (5.6.0b2 - 2020.1.0a20)
    pub assetBundleIndex: Option<i32>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub digest: Option<MdFour>,
    /// Vec<(String, GUID)>: (2018.4.15f1 - 2020.1.0a20)
    pub guidOfPathLocationDependencies: Option<Vec<(String, GUID)>>,
    /// Hash128: (5.6.0b2 - 2020.1.0a20)
    pub hash: Option<Hash128>,
    /// Vec<GUID>: (2018.4.15f1 - 2020.1.0a20)
    pub hashOfImportedAssetDependencies: Option<Vec<GUID>>,
    /// Vec<GUID>: (2018.4.15f1 - 2020.1.0a20)
    pub hashOfSourceAssetDependencies: Option<Vec<GUID>>,
    /// i32: (5.6.0b2 - 2020.1.0a20)
    pub importerClassId: Option<i32>,
    /// u32: (5.6.0b2 - 2020.1.0a20)
    pub importerVersionHash: Option<u32>,
    /// u32: (3.4.0 - 3.4.0)
    #[serde(alias = "metaModificationDate[0]")]
    pub metaModificationDate_0_: Option<u32>,
    /// u32: (3.4.0 - 3.4.0)
    #[serde(alias = "metaModificationDate[1]")]
    pub metaModificationDate_1_: Option<u32>,
    /// u32: (3.4.0 - 3.4.0)
    #[serde(alias = "modificationDate[0]")]
    pub modificationDate_0_: Option<u32>,
    /// u32: (3.4.0 - 3.4.0)
    #[serde(alias = "modificationDate[1]")]
    pub modificationDate_1_: Option<u32>,
    /// String: (2017.4.33f1 - 2020.1.0a20)
    pub scriptedImporterClassID: Option<String>,
}

/// AssetBundle is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AssetBundle.html):
/**
AssetBundles let you stream additional assets via the UnityWebRequest class and instantiate them at runtime.
Create AssetBundles by calling BuildPipeline.BuildAssetBundles or using the Scriptable Build Pipeline package.A bundle built for any of the standalone platforms can only be loaded on that platform, for example a bundle build on iOS is not compatible with Android.
This is because shaders, textures and other types of data are built into platform-specific formats based on the BuildTarget.See Also: UnityWebRequestAssetBundle.GetAssetBundle, Loading Resources at Runtime, BuildPipeline.BuildAssetBundles.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetBundle {
    pub m_Container: Vec<(String, AssetInfo)>,
    pub m_MainAsset: AssetInfo,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_PreloadTable: Vec<PPtr /*<Object>*/>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AssetBundleName: Option<String>,
    /// Vec<String>: (5.6.0b2 - 2022.2.0b16)
    pub m_Dependencies: Option<Vec<String>>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExplicitDataLayout: Option<i32>,
    /**Return true if the AssetBundle is a streamed Scene AssetBundle.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_IsStreamedSceneAssetBundle: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_PathFlags: Option<i32>,
    /// u32: (5.6.0b2 - 2022.2.0b16)
    pub m_RuntimeCompatibility: Option<u32>,
    /// Vec<(String, String)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_SceneHashes: Option<Vec<(String, String)>>,
    /// Vec<AssetBundleScriptInfo>: (3.4.0 - 3.4.0)
    pub m_ScriptCompatibility: Option<Vec<AssetBundleScriptInfo>>,
}

/// AssetBundleFullName is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetBundleFullName {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

/// AssetBundleInfo is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Build.Content.AssetBundleInfo.html):
/**
Container for holding asset loading information for an AssetBundle to be built.
Note: this class and its members exist to provide low-level support for the Scriptable Build Pipeline package. This is intended for internal use only; use the Scriptable Build Pipeline package to implement a fully featured build pipeline. You can install this via the Unity Package Manager.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetBundleInfo {
    pub AssetBundleDependencies: Vec<i32>,
    pub AssetBundleHash: Hash128,
}

/// AssetBundleManifest is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AssetBundleManifest.html):
/**
Manifest for all the AssetBundles in the build.
See Also: BuildPipeline.BuildAssetBundles, AssetBundle.GetAllAssetNames
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetBundleManifest {
    pub AssetBundleInfos: Vec<(i32, AssetBundleInfo)>,
    pub AssetBundleNames: Vec<(i32, String)>,
    pub AssetBundlesWithVariant: Vec<i32>,
    /**The name of the object.*/
    pub m_Name: String,
}

/// AssetBundleScriptInfo is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetBundleScriptInfo {
    pub assemblyName: String,
    pub className: String,
    pub hash: u32,
    pub nameSpace: String,
}

/// AssetDatabase is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AssetDatabase.html):
/**
An Interface for accessing assets and performing operations on assets.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetDatabase {
    pub m_Assets: Vec<(GUID, Asset)>,
    /// Vec<(i32, AssetBundleFullName)>: (5.6.0b2 - 5.6.0b2)
    pub m_AssetBundleNames: Option<Vec<(i32, AssetBundleFullName)>>,
    /// Vec<(String, AssetTimeStamp)>: (5.6.0b2 - 5.6.0b2)
    pub m_AssetTimeStamps: Option<Vec<(String, AssetTimeStamp)>>,
    /// AssetDatabaseMetrics: (5.6.0b2 - 5.6.0b2)
    pub m_Metrics: Option<AssetDatabaseMetrics>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub m_UnityShadersVersion: Option<i32>,
    /// Vec<(i32, u32)>: (5.6.0b2 - 5.6.0b2)
    pub m_lastValidVersionHashes: Option<Vec<(i32, u32)>>,
}

/// AssetDatabaseMetrics is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetDatabaseMetrics {
    pub totalAssetCount: i32,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub nonProAssetCount: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub nonProAssetsCreatedAfterProLicense: Option<i32>,
}

/// AssetDatabaseV1 is a  class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetDatabaseV1 {
    pub m_AssetBundleNames: Vec<(i32, AssetBundleFullName)>,
    pub m_AssetTimeStamps: Vec<(String, AssetTimeStamp)>,
    pub m_Assets: Vec<(GUID, Asset)>,
    pub m_Metrics: AssetDatabaseMetrics,
    pub m_UnityShadersVersion: i32,
    pub m_lastValidVersions: Vec<(AssetImporterHashKey, u32)>,
}

/// AssetImporter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AssetImporter.html):
/**
Base class from which asset importers for specific asset types derive.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetImporter {
    pub m_FileIDToRecycleName: Vec<(i32, String)>,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_NewHashIdentity: MdFour,
    pub m_OldHashIdentity: MdFour,
}

/// AssetImporterHashKey is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetImporterHashKey {
    pub ScriptClass: String,
    /// i32: (2017.4.33f1 - 2020.1.0a20)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// AssetImporterLog is a  class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetImporterLog {
    pub m_Logs: Vec<AssetImporter_ImportError>,
    pub m_Name: String,
}

/// AssetImporter_ImportError is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetImporter_ImportError {
    pub error: String,
    pub file: String,
    pub line: i32,
    pub mode: i32,
    pub object: PPtr, /*<Object>*/
}

/// AssetInfo is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetInfo {
    pub asset: PPtr, /*<Object>*/
    pub preloadIndex: i32,
    pub preloadSize: i32,
}

/// AssetLabels is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetLabels {
    pub m_Labels: Vec<String>,
}

/// AssetMetaData is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetMetaData {
    pub assetStoreRef: u64,
    pub guid: GUID,
    pub labels: Vec<String>,
    pub originalName: String,
    pub pathName: String,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub licenseType: Option<i32>,
    /// u32: (3.4.0 - 5.6.0b2)
    pub originalChangeset: Option<u32>,
    /// MdFour: (3.4.0 - 3.4.0); Hash128: (5.6.0b2 - 5.6.0b2)
    pub originalDigest: Option<Enum_MdFour__Hash128>,
    /// GUID: (3.4.0 - 5.6.0b2)
    pub originalParent: Option<GUID>,
    /// u64: (5.6.0b2 - 5.6.0b2)
    pub timeCreated: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_MdFour__Hash128 {
    MdFour(MdFour),
    Hash128(Hash128),
}

/// AssetServerCache is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetServerCache {
    pub m_CachesInitialized: i32,
    pub m_CommitItemSelection: Vec<GUID>,
    pub m_DeletedItems: Vec<(GUID, DeletedItem)>,
    pub m_Items: Vec<(GUID, Item)>,
    pub m_LastCommitMessage: String,
    pub m_LatestServerChangeset: i32,
    pub m_ModifiedItems: Vec<(GUID, Item)>,
    pub m_WorkingItemMetaData: Vec<(GUID, CachedAssetMetaData)>,
}

/// AssetTimeStamp is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetTimeStamp {
    /// u32: (5.6.0b2 - 2020.1.0a20)
    #[serde(alias = "metaModificationDate[0]")]
    pub metaModificationDate_0_: Option<u32>,
    /// u32: (5.6.0b2 - 2020.1.0a20)
    #[serde(alias = "metaModificationDate[1]")]
    pub metaModificationDate_1_: Option<u32>,
    /// u32: (5.6.0b2 - 2020.1.0a20)
    #[serde(alias = "modificationDate[0]")]
    pub modificationDate_0_: Option<u32>,
    /// u32: (5.6.0b2 - 2020.1.0a20)
    #[serde(alias = "modificationDate[1]")]
    pub modificationDate_1_: Option<u32>,
}

/// AudioBuildInfo is a  class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioBuildInfo {
    pub m_AudioClipCount: i32,
    pub m_AudioMixerCount: i32,
    pub m_IsAudioDisabled: bool,
}

/// AudioChorusFilter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioChorusFilter.html):
/**
The Audio Chorus Filter takes an Audio Clip and processes it creating a chorus effect.
The chorus effect modulates the original sound by a sinusoid low frequency oscillator (LFO). The output sounds like there are multiple sources emitting the same sound with slight variations (resembling a choir).See Also: Audio Chorus Filter information.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioChorusFilter {
    /**Chorus delay in ms. 0.1 to 100.0. Default = 40.0 ms.*/
    pub m_Delay: f32,
    /**Chorus modulation depth. 0.0 to 1.0. Default = 0.03.*/
    pub m_Depth: f32,
    /**Volume of original signal to pass to output. 0.0 to 1.0. Default = 0.5.*/
    pub m_DryMix: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Chorus modulation rate in hz. 0.0 to 20.0. Default = 0.8 hz.*/
    pub m_Rate: f32,
    /**Volume of 1st chorus tap. 0.0 to 1.0. Default = 0.5.*/
    pub m_WetMix1: f32,
    /**Volume of 2nd chorus tap. This tap is 90 degrees out of phase of the first tap. 0.0 to 1.0. Default = 0.5.*/
    pub m_WetMix2: f32,
    /**Volume of 3rd chorus tap. This tap is 90 degrees out of phase of the second tap. 0.0 to 1.0. Default = 0.5.*/
    pub m_WetMix3: f32,
    /// f32: (3.4.0 - 3.4.0)
    pub m_FeedBack: Option<f32>,
}

/// AudioClip is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioClip.html):
/**
A container for audio data.
An AudioClip stores the audio file either compressed as ogg vorbis or uncompressed.

AudioClips are referenced and used by AudioSources to play sounds.See Also: AudioClip component in the Components Reference.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioClip {
    /**The name of the object.*/
    pub m_Name: String,
    /// bool: (3.4.0 - 3.4.0)
    pub m_3D: Option<bool>,
    /**Returns true if this audio clip is ambisonic (read-only).*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_Ambisonic: Option<bool>,
    /// Vec<u8>: (3.4.0 - 3.4.0)
    pub m_AudioData: Option<Vec<u8>>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_BitsPerSample: Option<i32>,
    /**The number of channels in the audio clip. (Read Only)*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_Channels: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_CompressionFormat: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_Format: Option<i32>,
    /**The sample frequency of the clip in Hertz. (Read Only)*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_Frequency: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_IsTrackerFormat: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_Legacy3D: Option<bool>,
    /**The length of the audio clip in seconds. (Read Only)*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_Length: Option<f32>,
    /**Corresponding to the "Load In Background" flag in the inspector, when this flag is set, the loading will happen delayed without blocking the main thread.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_LoadInBackground: Option<bool>,
    /**The load type of the clip (read-only).*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_LoadType: Option<i32>,
    /**Preloads audio data of the clip when the clip asset is loaded. When this flag is off, scripts have to call AudioClip.LoadAudioData() to load the data before the clip can be played. Properties like length, channels and format are available before the audio data has been loaded.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_PreloadAudioData: Option<bool>,
    /// StreamedResource: (5.6.0b2 - 2022.2.0b16)
    pub m_Resource: Option<StreamedResource>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_Stream: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SubsoundIndex: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_Type: Option<i32>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_UseHardware: Option<bool>,
}

/// AudioDistortionFilter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioDistortionFilter.html):
/**
The Audio Distortion Filter distorts the sound from an AudioSource or sounds reaching the AudioListener.
See Also: Audio Distortion Filter information.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioDistortionFilter {
    /**Distortion value. 0.0 to 1.0. Default = 0.5.*/
    pub m_DistortionLevel: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
}

/// AudioEchoFilter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioEchoFilter.html):
/**
The Audio Echo Filter repeats a sound after a given Delay, attenuating the repetitions based on the Decay Ratio.
See Also: Audio Echo Filter information.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioEchoFilter {
    /**Echo decay per delay. 0 to 1. 1.0 = No decay, 0.0 = total decay (i.e. simple 1 line delay). Default = 0.5.*/
    pub m_DecayRatio: f32,
    /**Echo delay in ms. 10 to 5000. Default = 500.*/
    pub m_Delay: Enum_u32__f32,
    /**Volume of original signal to pass to output. 0.0 to 1.0. Default = 1.0.*/
    pub m_DryMix: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Volume of echo signal to pass to output. 0.0 to 1.0. Default = 1.0.*/
    pub m_WetMix: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_u32__f32 {
    u32(u32),
    f32(f32),
}

/// AudioHighPassFilter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioHighPassFilter.html):
/**
The Audio High Pass Filter passes high frequencies of an AudioSource, and cuts off signals with frequencies lower than the Cutoff Frequency.
See Also: Audio High Pass Filter information.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioHighPassFilter {
    /**Highpass cutoff frequency in hz. 10.0 to 22000.0. Default = 5000.0.*/
    pub m_CutoffFrequency: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Determines how much the filter's self-resonance isdampened.*/
    pub m_HighpassResonanceQ: f32,
}

/// AudioImporter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioImporter.html):
/**
Audio importer lets you modify AudioClip import settings from editor scripts.
Settings of this class match the ones exposed in Audio Import Settings.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioImporter {
    pub m_3D: bool,
    /**Force audioclips to mono?*/
    pub m_ForceToMono: bool,
    /**The name of the object.*/
    pub m_Name: String,
    /// Vec<u8>: (3.4.0 - 3.4.0)
    #[serde(alias = "audio preview data")]
    pub audio_preview_data: Option<Vec<u8>>,
    /**When this flag is set, the audio clip will be treated as being ambisonic.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_Ambisonic: Option<bool>,
    /**Get or set the AssetBundle name.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AssetBundleName: Option<String>,
    /**Get or set the AssetBundle variant.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AssetBundleVariant: Option<String>,
    /// SampleSettings: (5.6.0b2 - 2022.2.0b16)
    pub m_DefaultSettings: Option<SampleSettings>,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.0)
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_Format: Option<i32>,
    /**Corresponding to the "Load In Background" flag in the AudioClip inspector, when this flag is set, the loading of the clip will happen delayed without blocking the main thread.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_LoadInBackground: Option<bool>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_Loopable: Option<bool>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_NewHashIdentity: Option<MdFour>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_Normalize: Option<bool>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_OldHashIdentity: Option<MdFour>,
    /// AudioImporterOutput: (5.6.0b2 - 2022.2.0b16)
    pub m_Output: Option<AudioImporterOutput>,
    /// Vec<(i32, SampleSettings)>: (5.6.0b2 - 2022.2.0b16)
    pub m_PlatformSettingOverrides: Option<Vec<(i32, SampleSettings)>>,
    /// bool: (5.6.0b2 - 2021.2.16f1)
    pub m_PreloadAudioData: Option<bool>,
    /// PreviewData: (5.6.0b2 - 2022.2.0b16)
    pub m_PreviewData: Option<PreviewData>,
    /// u32: (3.4.0 - 3.4.0)
    pub m_PreviewDataLength: Option<u32>,
    /// f32: (3.4.0 - 3.4.0)
    pub m_Quality: Option<f32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_Stream: Option<i32>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_UseHardware: Option<bool>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /**Get or set any user data.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_UserData: Option<String>,
}

/// AudioImporterOutput is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioImporterOutput {
    pub editorOutputContainerFormat: i32,
    pub editorOutputSettings: SampleSettings,
    pub outputContainerFormat: i32,
    pub outputSettings: SampleSettings,
    /// StreamedResource: (5.6.0b2 - 2017.4.33f1)
    pub playerResource: Option<StreamedResource>,
}

/// AudioListener is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioListener.html):
/**
Representation of a listener in 3D space.
This class implements a microphone-like device. It records the sounds around it and plays that through the player's speakers.

You can only have one listener in a Scene.See Also: AudioSource, AudioListener component in the Components Reference.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioListener {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
}

/// AudioLowPassFilter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioLowPassFilter.html):
/**
The Audio Low Pass Filter passes low frequencies of an AudioSource or all sounds reaching an AudioListener, while removing frequencies higher than the Cutoff Frequency.
See Also: Audio Low Pass Filter information.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioLowPassFilter {
    pub lowpassLevelCustomCurve: AnimationCurve,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Determines how much the filter's self-resonance is dampened.*/
    pub m_LowpassResonanceQ: f32,
    /**Lowpass cutoff frequency in hz. 10.0 to 22000.0. Default = 5000.0.*/
    /// f32: (3.4.0 - 3.4.0)
    pub m_CutoffFrequency: Option<f32>,
}

/// AudioManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioManager {
    pub m_DSPBufferSize: i32,
    pub m_Volume: f32,
    /// i32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "Default Speaker Mode")]
    pub Default_Speaker_Mode: Option<i32>,
    /// f32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "Doppler Factor")]
    pub Doppler_Factor: Option<f32>,
    /// f32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "Rolloff Scale")]
    pub Rolloff_Scale: Option<f32>,
    /// String: (2017.4.33f1 - 2022.2.0b16)
    pub m_AmbisonicDecoderPlugin: Option<String>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_DisableAudio: Option<bool>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_RealVoiceCount: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RequestedDSPBufferSize: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SampleRate: Option<i32>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_SpatializerPlugin: Option<String>,
    /// f32: (3.4.0 - 3.4.0)
    pub m_SpeedOfSound: Option<f32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_VirtualVoiceCount: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_VirtualizeEffects: Option<bool>,
}

/// AudioMixer is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Audio.AudioMixer.html):
/**
AudioMixer asset.
This is a singleton representing a specific audio mixer asset in the project.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixer {
    pub m_EnableSuspend: bool,
    pub m_MasterGroup: PPtr, /*<AudioMixerGroup>*/
    pub m_MixerConstant: AudioMixerConstant,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_OutputGroup: PPtr, /*<AudioMixerGroup>*/
    pub m_Snapshots: Vec<PPtr /*<AudioMixerSnapshot>*/>,
    pub m_StartSnapshot: PPtr, /*<AudioMixerSnapshot>*/
    pub m_SuspendThreshold: f32,
    /**How time should progress for this AudioMixer. Used during Snapshot transitions.*/
    pub m_UpdateMode: i32,
}

/// AudioMixerConstant is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixerConstant {
    pub effectGUIDs: Vec<GUID>,
    pub effects: Vec<EffectConstant>,
    pub exposedParameterIndices: Vec<u32>,
    pub exposedParameterNames: Vec<u32>,
    pub groupGUIDs: Vec<GUID>,
    pub groupNameBuffer: Vec<char>,
    pub groups: Vec<GroupConstant>,
    pub numSideChainBuffers: u32,
    pub pluginEffectNameBuffer: Vec<char>,
    pub snapshotGUIDs: Vec<GUID>,
    pub snapshotNameBuffer: Vec<char>,
    pub snapshots: Vec<SnapshotConstant>,
    /// Vec<GroupConnection>: (2021.2.16f1 - 2021.2.16f1)
    pub groupConnections: Option<Vec<GroupConnection>>,
}

/// AudioMixerController is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixerController {
    pub m_EnableSuspend: bool,
    pub m_MasterGroup: PPtr, /*<AudioMixerGroup>*/
    pub m_MixerConstant: AudioMixerConstant,
    pub m_Name: String,
    pub m_OutputGroup: PPtr, /*<AudioMixerGroup>*/
    pub m_Snapshots: Vec<PPtr /*<AudioMixerSnapshot>*/>,
    pub m_StartSnapshot: PPtr, /*<AudioMixerSnapshot>*/
    pub m_SuspendThreshold: f32,
    pub m_UpdateMode: i32,
}

/// AudioMixerEffectController is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixerEffectController {
    pub m_Bypass: bool,
    pub m_EffectID: GUID,
    pub m_EffectName: String,
    pub m_EnableWetMix: bool,
    pub m_MixLevel: GUID,
    pub m_Name: String,
    pub m_Parameters: Vec<Parameter>,
    pub m_SendTarget: PPtr, /*<AudioMixerEffectController>*/
}

/// AudioMixerGroup is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Audio.AudioMixerGroup.html):
/**
Object representing a group in the mixer.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixerGroup {
    pub m_AudioMixer: PPtr, /*<AudioMixer>*/
    pub m_Children: Vec<PPtr /*<AudioMixerGroup>*/>,
    pub m_GroupID: GUID,
    /**The name of the object.*/
    pub m_Name: String,
}

/// AudioMixerGroupController is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixerGroupController {
    pub m_AudioMixer: PPtr, /*<AudioMixer>*/
    pub m_Children: Vec<PPtr /*<AudioMixerGroup>*/>,
    pub m_GroupID: GUID,
    pub m_Name: String,
}

/// AudioMixerSnapshot is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Audio.AudioMixerSnapshot.html):
/**
Object representing a snapshot in the mixer.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixerSnapshot {
    pub m_AudioMixer: PPtr, /*<AudioMixer>*/
    /**The name of the object.*/
    pub m_Name: String,
    pub m_SnapshotID: GUID,
}

/// AudioMixerSnapshotController is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixerSnapshotController {
    pub m_AudioMixer: PPtr, /*<AudioMixer>*/
    pub m_Name: String,
    pub m_SnapshotID: GUID,
}

/// AudioReverbFilter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioReverbFilter.html):
/**
The Audio Reverb Filter takes an Audio Clip and distorts it to create a custom reverb effect.
See Also: Audio Reverb Filter information.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioReverbFilter {
    /**Decay HF Ratio : High-frequency to low-frequency decay time ratio. Ranges from 0.1 to 2.0. Default is 0.5.*/
    pub m_DecayHFRatio: f32,
    /**Reverberation decay time at low-frequencies in seconds. Ranges from 0.1 to 20.0. Default is 1.0.*/
    pub m_DecayTime: f32,
    /**Reverberation density (modal density) in percent. Ranges from 0.0 to 100.0. Default is 100.0.*/
    pub m_Density: f32,
    /**Reverberation diffusion (echo density) in percent. Ranges from 0.0 to 100.0. Default is 100.0.*/
    pub m_Diffusion: f32,
    /**Mix level of dry signal in output in millibels (mB). Ranges from -10000.0 to 0.0. Default is 0.*/
    pub m_DryLevel: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Reference high frequency in hertz (Hz). Ranges from 1000.0 to 20000.0. Default is 5000.0.*/
    pub m_HFReference: f32,
    /**Reference low-frequency in hertz (Hz). Ranges from 20.0 to 1000.0. Default is 250.0.*/
    pub m_LFReference: f32,
    /**Late reverberation level relative to room effect in millibels (mB). Ranges from -10000.0 to 2000.0. Default is 0.0.*/
    pub m_ReflectionsDelay: f32,
    /**Early reflections level relative to room effect in millibels (mB). Ranges from -10000.0 to 1000.0. Default is -10000.0.*/
    pub m_ReflectionsLevel: f32,
    /**Late reverberation delay time relative to first reflection in seconds. Ranges from 0.0 to 0.1. Default is 0.04.*/
    pub m_ReverbDelay: f32,
    /**Late reverberation level relative to room effect in millibels (mB). Ranges from -10000.0 to 2000.0. Default is 0.0.*/
    pub m_ReverbLevel: f32,
    /**Set/Get reverb preset properties.*/
    pub m_ReverbPreset: i32,
    /**Room effect level at low frequencies in millibels (mB). Ranges from -10000.0 to 0.0. Default is 0.0.*/
    pub m_Room: f32,
    /**Room effect high-frequency level re. low frequency level in millibels (mB). Ranges from -10000.0 to 0.0. Default is 0.0.*/
    pub m_RoomHF: f32,
    /**Room effect low-frequency level in millibels (mB). Ranges from -10000.0 to 0.0. Default is 0.0.*/
    pub m_RoomLF: f32,
    /// f32: (3.4.0 - 5.6.0b2)
    pub m_RoomRolloff: Option<f32>,
}

/// AudioReverbZone is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioReverbZone.html):
/**
Reverb Zones are used when you want to create location based ambient effects in the Scene.
As the Audio Listener moves into a Reverb Zone, the ambient effect associated with the zone is gradually applied.

At the max distance there is no effect and at the min distance the effect is fully applied.

For example you can gradually change your character's footsteps sounds and create the

feeling like you where entering into a cavern, going trough a room,

swimming underwater, etc.You can always mix reverb zones to have combined effects.

For more info check Reverb Zones in the manual.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioReverbZone {
    /**High-frequency to mid-frequency decay time ratio.*/
    pub m_DecayHFRatio: f32,
    /**Reverberation decay time at mid frequencies.*/
    pub m_DecayTime: f32,
    /**Value that controls the modal density in the late reverberation decay.*/
    pub m_Density: f32,
    /**Value that controls the echo density in the late reverberation decay.*/
    pub m_Diffusion: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Reference high frequency (hz).*/
    pub m_HFReference: f32,
    /**Reference low frequency (hz).*/
    pub m_LFReference: f32,
    /**The distance from the centerpoint that the reverb will not have any effect. Default = 15.0.*/
    pub m_MaxDistance: f32,
    /**The distance from the centerpoint that the reverb will have full effect at. Default = 10.0.*/
    pub m_MinDistance: f32,
    /**Early reflections level relative to room effect.*/
    pub m_Reflections: i32,
    /**Initial reflection delay time.*/
    pub m_ReflectionsDelay: f32,
    /**Late reverberation level relative to room effect.*/
    pub m_Reverb: i32,
    /**Late reverberation delay time relative to initial reflection.*/
    pub m_ReverbDelay: f32,
    /**Set/Get reverb preset properties.*/
    pub m_ReverbPreset: i32,
    /**Room effect level (at mid frequencies).*/
    pub m_Room: i32,
    /**Relative room effect level at high frequencies.*/
    pub m_RoomHF: i32,
    /**Relative room effect level at low frequencies.*/
    pub m_RoomLF: i32,
    /// f32: (3.4.0 - 5.6.0b2)
    pub m_RoomRolloffFactor: Option<f32>,
}

/// AudioSource is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioSource.html):
/**
A representation of audio sources in 3D.
An AudioSource is attached to a GameObject for playing back sounds in a 3D environment.

In order to play 3D sounds you also need to have a AudioListener.

The audio listener is normally attached to the camera you want to use.

Whether sounds are played in 3D or 2D is determined by AudioImporter settings.You can play a single audio clip using Play, Pause and Stop.

You can also adjust its volume while playing using the volume property, or seek using time.

Multiple sounds can be played on one AudioSource using PlayOneShot.

You can play a clip at a static position in 3D space using PlayClipAtPoint.See Also: AudioListener, AudioClip, AudioSource component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioSource {
    /**Bypass effects (Applied from filter components or global listener filters).*/
    pub BypassEffects: bool,
    /**Sets the Doppler scale for this AudioSource.*/
    pub DopplerLevel: f32,
    /**Is the audio clip looping?*/
    pub Loop: bool,
    /**(Logarithmic rolloff) MaxDistance is the distance a sound stops attenuating at.*/
    pub MaxDistance: f32,
    /**Within the Min distance the AudioSource will cease to grow louder in volume.*/
    pub MinDistance: f32,
    /**Un- / Mutes the AudioSource. Mute sets the volume=0, Un-Mute restore the original volume.*/
    pub Mute: bool,
    pub Pan2D: f32,
    /**Sets the priority of the AudioSource.*/
    pub Priority: i32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The pitch of the audio source.*/
    pub m_Pitch: f32,
    /**If set to true, the audio source will automatically start playing on awake.*/
    pub m_PlayOnAwake: bool,
    /**The volume of the audio source (0.0 to 1.0).*/
    pub m_Volume: f32,
    pub m_audioClip: PPtr, /*<AudioClip>*/
    pub panLevelCustomCurve: AnimationCurve,
    pub rolloffCustomCurve: AnimationCurve,
    /**Sets/Gets how the AudioSource attenuates over distance.*/
    pub rolloffMode: i32,
    pub spreadCustomCurve: AnimationCurve,
    /**When set global effects on the AudioListener will not be applied to the audio signal generated by the AudioSource. Does not apply if the AudioSource is playing into a mixer group.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub BypassListenerEffects: Option<bool>,
    /**When set doesn't route the signal from an AudioSource into the global reverb associated with reverb zones.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub BypassReverbZones: Option<bool>,
    /**The target group to which the AudioSource should route its signal.*/
    /// PPtr/*<AudioMixerGroup>*/: (5.6.0b2 - 2022.2.0b16)
    pub OutputAudioMixerGroup: Option<PPtr /*<AudioMixerGroup>*/>,
    /**Enables or disables spatialization.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub Spatialize: Option<bool>,
    /**Determines if the spatializer effect is inserted before or after the effect filters.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub SpatializePostEffects: Option<bool>,
    /// AnimationCurve: (5.6.0b2 - 2022.2.0b16)
    pub reverbZoneMixCustomCurve: Option<AnimationCurve>,
}

/// AutoOffMeshLinkData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AutoOffMeshLinkData {
    pub m_Area: u8,
    pub m_End: Vector3f,
    pub m_LinkDirection: u8,
    pub m_LinkType: u16,
    pub m_Radius: f32,
    pub m_Start: Vector3f,
}

/// Avatar is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Avatar.html):
/**
Avatar definition.
See Also: Animator.avatar, AvatarBuilder.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Avatar {
    pub m_Avatar: AvatarConstant,
    pub m_AvatarSize: u32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_TOS: Vec<(u32, String)>,
    /**Returns the HumanDescription used to create this Avatar.*/
    /// HumanDescription: (2019.3.0f4 - 2022.2.0b16)
    pub m_HumanDescription: Option<HumanDescription>,
}

/// AvatarConstant is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarConstant {
    pub m_AvatarSkeleton: OffsetPtr,
    pub m_AvatarSkeletonPose: OffsetPtr,
    pub m_DefaultPose: OffsetPtr,
    pub m_Human: OffsetPtr,
    pub m_HumanSkeletonIndexArray: Vec<i32>,
    pub m_HumanSkeletonReverseIndexArray: Vec<i32>,
    pub m_RootMotionBoneIndex: i32,
    pub m_RootMotionBoneX: xform,
    pub m_RootMotionSkeleton: OffsetPtr,
    pub m_RootMotionSkeletonIndexArray: Vec<i32>,
    pub m_RootMotionSkeletonPose: OffsetPtr,
    pub m_SkeletonNameIDArray: Vec<u32>,
}

/// AvatarMask is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AvatarMask.html):
/**
AvatarMask is used to mask out humanoid body parts and transforms.
They can be used when importing animation or in an animator controller layer.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarMask {
    pub m_Elements: Vec<TransformMaskElement>,
    pub m_Mask: Vec<u32>,
    /**The name of the object.*/
    pub m_Name: String,
}

/// Behaviour is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Behaviour.html):
/**
Behaviours are Components that can be enabled or disabled.
See Also: MonoBehaviour and Component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Behaviour {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
}

/// BillboardAsset is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/BillboardAsset.html):
/**
BillboardAsset describes how a billboard is rendered.
Billboards are a level-of-detail (LOD) method of drawing complicated 3D objects in a simpler manner if they are further away. Because the object is further away, there is often less requirement to draw the object at full detail due to its size on-screen and low likelihood of being a focal point in the Camera view. Instead, the object can be pre-rendered to a texture, and this texture used on a very simple Camera-facing Mesh of flat geometry (often simply a quadrilateral) known as a billboard. At great distances an object does not significantly change its orientation relative to the camera, so a billboard looks much like the object it represents from frame to frame, without having to be redrawn from the model. The BillboardAsset class allows the creation of billboards that are rendered from several directions, allowing a BillboardAsset to efficiently represent an object at low level of detail from any approximately-horizontal viewpoint.A BillboardAsset is usually created by importing SpeedTree assets. You can also create your own once you know how the billboard is described.SpeedTree billboard geometry is usually more complex than a plain quadrilateral. By using more vertices to cut out the empty part of the billboard image, rendering performance can potentially be improved due to the graphics system not having to draw as many redundant transparent pixels. You have access to the geometry data via BillboardAsset.vertices and BillboardAsset.indices.All vertices are considered in UV-space (see Fig. 1 below), because the geometry is due to be textured by the billboard images. UV vertices are easily expanded to 3D-space vertices by knowing the billboard's width, height, bottom, and what direction the billboard is currently facing. Assuming we have a billboard located at (0,0,0) looking at negative Z axis, the 3D-space coordinates are calculated as:X = (u - 0.5) * width
Y = v * height + bottom
Z = 0

Figure 1: How UV vertices are expanded to 3D verticesIn order to display something similar to the real 3D mesh being billboarded, SpeedTree billboards select an appropriate image from several pre-rendered images according to the current view direction. The images in Figure 2 below are created by capturing the rendered image of the 3D tree at different view angles, evenly distributed around the Y-axis. The first image always starts at positive X axis direction (or 0° if you imagine a unit circle from above).

Figure 2: How the eight billboard images are bakedAll images should be atlased together in one single texture. Each image is represented as a Vector4 of {left u, top v, width in u, height in v} in the atlas. You have access to all the images via BillboardAsset.imageTexCoords. SpeedTree Modeler always exports a normal texture alongside the diffuse texture for even better approximation of the lighting, and it shares the same atlas layout with the diffuse texutre.Once the BillboardAsset is constructed, use BillboardRenderer to render it.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BillboardAsset {
    /**Height of the billboard that is below ground.*/
    pub bottom: f32,
    /**Height of the billboard.*/
    pub height: f32,
    pub imageTexCoords: Vec<Vector4f>,
    pub indices: Vec<u16>,
    /**The name of the object.*/
    pub m_Name: String,
    /**The material used for rendering.*/
    pub material: PPtr, /*<Material>*/
    pub vertices: Vec<Vector2f>,
    /**Width of the billboard.*/
    pub width: f32,
}

/// BillboardRenderer is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/BillboardRenderer.html):
/**
Renders a billboard from a BillboardAsset.
BillboardRenderers that share the same BillboardAsset can be rendered in a batch if they are next to each other in the order of rendering. The batching is always enabled regardless of whether dynamic batching is enabled or not.Due to the always-upright nature of a tree billboard, BillboardRenderer can only rotate around Y-axis.See Also: BillboardAsset.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BillboardRenderer {
    /**The BillboardAsset to render.*/
    pub m_Billboard: PPtr, /*<BillboardAsset>*/
    pub m_CastShadows: u8,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The light probe interpolation type.*/
    pub m_LightProbeUsage: u8,
    pub m_LightProbeVolumeOverride: PPtr, /*<GameObject>*/
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    pub m_Materials: Vec<PPtr /*<Material>*/>,
    pub m_MotionVectors: u8,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    pub m_ProbeAnchor: PPtr, /*<Transform>*/
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: u8,
    /**Should reflection probes be used for this Renderer?*/
    pub m_ReflectionProbeUsage: u8,
    pub m_SortingLayer: i16,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    pub m_StaticBatchInfo: StaticBatchInfo,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    /// u8: (2017.4.33f1 - 2022.2.0b16)
    pub m_DynamicOccludee: Option<u8>,
    /// u8: (2020.1.0a20 - 2022.2.0b16)
    pub m_RayTraceProcedural: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0f4 - 2022.2.0b16)
    pub m_RayTracingMode: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on.*/
    /// u32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RenderingLayerMask: Option<u32>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_SortingLayerID: Option<i32>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.2.16f1 - 2022.2.0b16)
    pub m_StaticShadowCaster: Option<u8>,
}

/// BitField is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct BitField {
    pub m_Bits: u32,
}

/// BlendShapeData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendShapeData {
    pub channels: Vec<MeshBlendShapeChannel>,
    pub fullWeights: Vec<f32>,
    pub shapes: Vec<MeshBlendShape>,
    pub vertices: Vec<BlendShapeVertex>,
}

/// BlendShapeVertex is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendShapeVertex {
    pub index: u32,
    pub normal: Vector3f,
    pub tangent: Vector3f,
    pub vertex: Vector3f,
}

/// BlendTree is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.BlendTree.html):
/**
Blend trees are used to blend continuously animation between their children. They can either be 1D or 2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendTree {
    /**Parameter that is used to compute the blending weight of the children in 1D blend trees or on the X axis of a 2D blend tree.*/
    pub m_BlendParameter: String,
    /**Parameter that is used to compute the blending weight of the children on the Y axis of a 2D blend tree.*/
    pub m_BlendParameterY: String,
    /**The Blending type can be either 1D or different types of 2D.*/
    pub m_BlendType: i32,
    pub m_Childs: Vec<ChildMotion>,
    /**Sets the maximum threshold that will be used by the ChildMotion. Only used when useAutomaticThresholds is true.*/
    pub m_MaxThreshold: f32,
    /**Sets the minimum threshold that will be used by the ChildMotion. Only used when useAutomaticThresholds is true.*/
    pub m_MinThreshold: f32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_NormalizedBlendValues: bool,
    /**When active, the children's thresholds are automatically spread between 0 and 1.*/
    pub m_UseAutomaticThresholds: bool,
}

/// BoneInfluence is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct BoneInfluence {
    /// i32: (3.4.0 - 5.6.0b2)
    #[serde(alias = "boneIndex[0]")]
    pub boneIndex_0_: Option<i32>,
    /// i32: (3.4.0 - 5.6.0b2)
    #[serde(alias = "boneIndex[1]")]
    pub boneIndex_1_: Option<i32>,
    /// i32: (3.4.0 - 5.6.0b2)
    #[serde(alias = "boneIndex[2]")]
    pub boneIndex_2_: Option<i32>,
    /// i32: (3.4.0 - 5.6.0b2)
    #[serde(alias = "boneIndex[3]")]
    pub boneIndex_3_: Option<i32>,
    /// f32: (3.4.0 - 5.6.0b2)
    #[serde(alias = "weight[0]")]
    pub weight_0_: Option<f32>,
    /// f32: (3.4.0 - 5.6.0b2)
    #[serde(alias = "weight[1]")]
    pub weight_1_: Option<f32>,
    /// f32: (3.4.0 - 5.6.0b2)
    #[serde(alias = "weight[2]")]
    pub weight_2_: Option<f32>,
    /// f32: (3.4.0 - 5.6.0b2)
    #[serde(alias = "weight[3]")]
    pub weight_3_: Option<f32>,
}

/// BoneWeights4 is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct BoneWeights4 {
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    #[serde(alias = "boneIndex[0]")]
    pub boneIndex_0_: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    #[serde(alias = "boneIndex[1]")]
    pub boneIndex_1_: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    #[serde(alias = "boneIndex[2]")]
    pub boneIndex_2_: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    #[serde(alias = "boneIndex[3]")]
    pub boneIndex_3_: Option<i32>,
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    #[serde(alias = "weight[0]")]
    pub weight_0_: Option<f32>,
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    #[serde(alias = "weight[1]")]
    pub weight_1_: Option<f32>,
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    #[serde(alias = "weight[2]")]
    pub weight_2_: Option<f32>,
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    #[serde(alias = "weight[3]")]
    pub weight_3_: Option<f32>,
}

/// BoxCollider is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/BoxCollider.html):
/**
A box-shaped primitive collider.
See Also: SphereCollider, CapsuleCollider, PhysicMaterial, Rigidbody.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxCollider {
    /**The center of the box, measured in the object's local space.*/
    pub m_Center: Vector3f,
    /**Enabled Colliders will collide with other Colliders, disabled Colliders won't.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Specify if this collider is configured as a trigger.*/
    pub m_IsTrigger: bool,
    /**The material used by the collider.*/
    pub m_Material: PPtr, /*<PhysicMaterial>*/
    /**The size of the box, measured in the object's local space.*/
    pub m_Size: Vector3f,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ProvidesContacts: Option<bool>,
}

/// BoxCollider2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/BoxCollider2D.html):
/**
Collider for 2D physics representing an axis-aligned rectangle.
See Also: CircleCollider2D, PolygonCollider2D, EdgeCollider2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxCollider2D {
    /**The density of the collider used to calculate its mass (when auto mass is enabled).*/
    pub m_Density: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    pub m_Material: PPtr, /*<PhysicsMaterial2D>*/
    /**The local offset of the collider geometry.*/
    pub m_Offset: Vector2f,
    /**The width and height of the rectangle.*/
    pub m_Size: Vector2f,
    /**Sets whether the Collider will be used or not used by a CompositeCollider2D.*/
    pub m_UsedByComposite: bool,
    /**Whether the collider is used by an attached effector or not.*/
    pub m_UsedByEffector: bool,
    /**Determines whether the BoxCollider2D's shape is automatically updated based on a SpriteRenderer's tiling properties.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_AutoTiling: Option<bool>,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_CallbackLayers: Option<BitField>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**Controls the radius of all edges created by the collider.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_EdgeRadius: Option<f32>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
    /// SpriteTilingProperty: (2017.4.33f1 - 2022.2.0b16)
    pub m_SpriteTilingProperty: Option<SpriteTilingProperty>,
}

/// BrokenPrefabAsset is a  class of the Unity engine since version 2022.2.0b16.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/BrokenPrefabAsset.html):
/**
BrokenPrefabAsset is for Prefab files where the file content cannot be loaded without errors.
A Prefab Asset can be broken if the content of the file invalid or if it is a Variant Prefab where the parent Prefab is either invalid or missing.Search for t:BrokenPrefabAsset in the project browser to see which assets are of that type.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BrokenPrefabAsset {
    pub m_BrokenParentPrefab: PPtr, /*<BrokenPrefabAsset>*/
    /**Returns true if the content of the file is valid.*/
    pub m_IsPrefabFileValid: bool,
    /**Returns true if the prefab is a variant.*/
    pub m_IsVariant: bool,
    pub m_IsWarning: bool,
    pub m_Message: String,
    /**The name of the object.*/
    pub m_Name: String,
}

/// BufferBinding is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct BufferBinding {
    pub m_Index: i32,
    pub m_NameIndex: i32,
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub m_ArraySize: Option<i32>,
}

/// BuildReport is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Build.Reporting.BuildReport.html):
/**
The BuildReport API gives you information about the Unity build process.
A BuildReport object is returned by BuildPipeline.BuildPlayer and can be used to discover information about the files output, the build steps taken, and other platform-specific information such as native code stripping.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildReport {
    pub m_Appendices: Vec<PPtr /*<Object>*/>,
    pub m_BuildSteps: Vec<BuildStepInfo>,
    pub m_Files: Vec<BuildReportFile>,
    /**The name of the object.*/
    pub m_Name: String,
    /**A BuildSummary containing overall statistics and data about the build process.*/
    pub m_Summary: BuildSummary,
}

/// BuildReportFile is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildReportFile {
    pub id: u32,
    pub path: String,
    pub role: String,
    pub totalSize: u64,
}

/// BuildReportPackedAssetInfo is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildReportPackedAssetInfo {
    pub buildTimeAssetPath: String,
    pub classID: i32,
    pub fileID: i64,
    pub packedSize: i128,
    pub sourceAssetGUID: GUID,
    /// u64: (2019.3.0f4 - 2022.2.0b16)
    pub offset: Option<u64>,
}

/// BuildReportScenesUsingAsset is a sub class of the Unity engine since version 2020.1.0a20.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildReportScenesUsingAsset {
    pub assetPath: String,
    pub scenePaths: Vec<String>,
}

/// BuildSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Build.Content.BuildSettings.html):
/**
Struct containing information on how to build content.
Note: this struct and its members exist to provide low-level support for the Scriptable Build Pipeline package. This is intended for internal use only; use the Scriptable Build Pipeline package to implement a fully featured build pipeline. You can install this via the Unity Package Manager.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSettings {
    pub enableDynamicBatching: bool,
    pub hasAdvancedVersion: bool,
    pub hasPROVersion: bool,
    pub hasPublishingRights: bool,
    pub hasShadows: bool,
    pub isDebugBuild: bool,
    pub isEducationalBuild: bool,
    pub m_AuthToken: String,
    pub m_Version: String,
    /// GUID: (2017.4.33f1 - 2022.2.0b16)
    pub buildGUID: Option<GUID>,
    /// Vec<String>: (5.6.0b2 - 2022.2.0b16)
    pub buildTags: Option<Vec<String>>,
    /// Vec<String>: (5.6.0b2 - 2022.2.0b16)
    pub enabledVRDevices: Option<Vec<String>>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub hasClusterRendering: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub hasLocalLightShadows: Option<bool>,
    /// bool: (3.4.0 - 3.4.0)
    pub hasRenderTexture: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub hasSoftShadows: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub isEmbedded: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub isNoWatermarkBuild: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub isPrototypingBuild: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub isTrial: Option<bool>,
    /// Vec<String>: (3.4.0 - 3.4.0)
    pub levels: Option<Vec<String>>,
    /// Vec<i32>: (5.6.0b2 - 2022.2.0b16)
    pub m_GraphicsAPIs: Option<Vec<i32>>,
    /// Vec<String>: (5.6.0b2 - 2022.2.0b16)
    pub preloadedPlugins: Option<Vec<String>>,
    /// Vec<(i32, Hash128)>: (5.6.0b2 - 2020.1.0a20)
    pub runtimeClassHashes: Option<Vec<(i32, Hash128)>>,
    /// Vec<String>: (5.6.0b2 - 2022.2.0b16)
    pub scenes: Option<Vec<String>>,
    /// Vec<(Hash128, Hash128)>: (5.6.0b2 - 2020.1.0a20)
    pub scriptHashes: Option<Vec<(Hash128, Hash128)>>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub usesOnMouseEvents: Option<bool>,
}

/// BuildStepInfo is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildStepInfo {
    pub messages: Vec<BuildStepMessage>,
    pub stepName: String,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub depth: Option<i32>,
    /// u64: (5.6.0b2 - 2017.4.33f1)
    pub duration: Option<u64>,
    /// u64: (2018.4.15f1 - 2022.2.0b16)
    pub durationTicks: Option<u64>,
}

/// BuildStepMessage is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Build.Reporting.BuildStepMessage.html):
/**
Contains information about a single log message recorded during the build process.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildStepMessage {
    /**The text content of the log message.*/
    pub content: String,
    /**The LogType of the log message.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// BuildSummary is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Build.Reporting.BuildSummary.html):
/**
Contains overall summary information about a build.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSummary {
    pub assetBundleOptions: i32,
    pub buildType: i32,
    pub crc: u32,
    /**The BuildOptions used for the build, as passed to BuildPipeline.BuildPlayer.*/
    pub options: i32,
    /**The output path for the build, as provided to BuildPipeline.BuildPlayer.*/
    pub outputPath: String,
    pub platformGroupName: String,
    pub platformName: String,
    /**The total number of errors and exceptions recorded during the build process.*/
    pub totalErrors: i32,
    /**The total size of the build output, in bytes.*/
    pub totalSize: u64,
    /**The total number of warnings recorded during the build process.*/
    pub totalWarnings: i32,
    /// GUID: (2017.4.33f1 - 2022.2.0b16)
    pub buildGUID: Option<GUID>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub buildResult: Option<i32>,
    /// DateTime: (2018.4.15f1 - 2022.2.0b16)
    pub buildStartTime: Option<DateTime>,
    /// String: (5.6.0b2 - 2017.4.33f1)
    pub name: Option<String>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub subtarget: Option<i32>,
    /// bool: (5.6.0b2 - 5.6.0b2)
    pub success: Option<bool>,
    /// u64: (5.6.0b2 - 2017.4.33f1)
    pub totalTimeMS: Option<u64>,
    /// u64: (2018.4.15f1 - 2022.2.0b16)
    pub totalTimeTicks: Option<u64>,
}

/// BuildTargetSettings is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildTargetSettings {
    pub m_BuildTarget: String,
    pub m_TextureFormat: i32,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_LoadingBehavior: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_MaxTextureSize: Option<i32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_TextureHeight: Option<i32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_TextureWidth: Option<i32>,
}

/// BuildTextureStackReference is a sub class of the Unity engine since version 2020.1.0a20.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildTextureStackReference {
    pub groupName: String,
    pub itemName: String,
}

/// BuiltAssetBundleInfo is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuiltAssetBundleInfo {
    pub bundleArchiveFile: u32,
    pub bundleName: String,
    pub packagedFileIndices: Vec<u32>,
}

/// BuiltAssetBundleInfoSet is a  class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuiltAssetBundleInfoSet {
    pub bundleInfos: Vec<BuiltAssetBundleInfo>,
}

/// BuiltinShaderSettings is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuiltinShaderSettings {
    pub m_Mode: i32,
    pub m_Shader: PPtr, /*<Shader>*/
}

/// BuoyancyEffector2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/BuoyancyEffector2D.html):
/**
Applies forces to simulate buoyancy, fluid-flow and fluid drag.
When any Collider2D overlap the area defined by the effector, calculations are made to determine if they are below the surfaceLevel.  If they are not, no forces are applied.  If they are then the effector will apply buoyancy forces in an attempt to move the Collider2D to the surfaceLevel i.e. they will float.This effector is designed primarily to work with Collider2D that are set as triggers so that Collider2D can overlap the defined area and have buoyancy forces applied to them.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BuoyancyEffector2D {
    /**A force applied to slow angular movement of any Collider2D in contact with the effector.*/
    pub m_AngularDrag: f32,
    /**The mask used to select specific layers allowed to interact with the effector.*/
    pub m_ColliderMask: BitField,
    /**The density of the fluid used to calculate the buoyancy forces.*/
    pub m_Density: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The angle of the force used to similate fluid flow.*/
    pub m_FlowAngle: f32,
    /**The magnitude of the force used to similate fluid flow.*/
    pub m_FlowMagnitude: f32,
    /**The random variation of the force used to similate fluid flow.*/
    pub m_FlowVariation: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**A force applied to slow linear movement of any Collider2D in contact with the effector.*/
    pub m_LinearDrag: f32,
    /**Defines an arbitrary horizontal line that represents the fluid surface level.*/
    pub m_SurfaceLevel: f32,
    /**Should the collider-mask be used or the global collision matrix?*/
    pub m_UseColliderMask: bool,
}

/// CachedAssetMetaData is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct CachedAssetMetaData {
    pub guid: GUID,
    pub originalChangeset: u32,
    pub originalDigest: Enum_MdFour__Hash128,
    pub originalName: String,
    pub originalParent: GUID,
    pub pathName: String,
}

/// CachedSpriteAtlas is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct CachedSpriteAtlas {
    pub alphaTextures: Vec<PPtr /*<Texture2D>*/>,
    pub frames: Vec<((GUID, i64), SpriteRenderData)>,
    pub textures: Vec<PPtr /*<Texture2D>*/>,
}

/// CachedSpriteAtlasRuntimeData is a  class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct CachedSpriteAtlasRuntimeData {
    pub alphaTextures: Vec<PPtr /*<Texture2D>*/>,
    pub frames: Vec<((GUID, i64), SpriteAtlasData)>,
    pub textures: Vec<PPtr /*<Texture2D>*/>,
    /// Hash128: (2020.1.0a20 - 2022.2.0b16)
    pub currentPackingHash: Option<Hash128>,
}

/// Camera is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Camera.html):
/**
A Camera is a device through which the player views the world.
A screen space point is defined in pixels. The bottom-left of the screen is (0,0); the right-top

is (pixelWidth,pixelHeight). The z position is in world units from the Camera.A viewport space point is normalized and relative to the Camera. The bottom-left of the Camera is

(0,0); the top-right is (1,1). The z position is in world units from the Camera.A world space point is defined in global coordinates (for example, Transform.position).See Also: camera component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Camera {
    /**The color with which the screen will be cleared.*/
    pub m_BackGroundColor: ColorRGBA,
    /**How the camera clears the background.*/
    pub m_ClearFlags: u32,
    /**This is used to render parts of the Scene selectively.*/
    pub m_CullingMask: BitField,
    /**Camera's depth in the camera rendering order.*/
    pub m_Depth: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_NormalizedViewPortRect: Rectf,
    /**The rendering path that should be used, if possible.*/
    pub m_RenderingPath: i32,
    /**Destination render texture.*/
    pub m_TargetTexture: PPtr, /*<RenderTexture>*/
    /**Is the camera orthographic (true) or perspective (false)?*/
    pub orthographic: bool,
    /**The distance of the far clipping plane from the Camera, in world units.*/
    /// f32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "far clip plane")]
    pub far_clip_plane: Option<f32>,
    /**The vertical field of view of the Camera, in degrees.*/
    /// f32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "field of view")]
    pub field_of_view: Option<f32>,
    /**Dynamic Resolution Scaling.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_AllowDynamicResolution: Option<bool>,
    /**MSAA rendering.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_AllowMSAA: Option<bool>,
    /**The camera anamorphism. To use this property, enable UsePhysicalProperties.*/
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub m_Anamorphism: Option<f32>,
    /**The camera aperture. To use this property, enable UsePhysicalProperties.*/
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub m_Aperture: Option<f32>,
    /**The camera barrel clipping. To use this property, enable UsePhysicalProperties.*/
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub m_BarrelClipping: Option<f32>,
    /**The blade count in the lens of the camera. To use this property, enable UsePhysicalProperties.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_BladeCount: Option<i32>,
    /**The curvature of the blades. To use this property, enable UsePhysicalProperties.*/
    /// Vector2f: (2022.2.0b16 - 2022.2.0b16)
    pub m_Curvature: Option<Vector2f>,
    /**The camera focal length, expressed in millimeters. To use this property, enable UsePhysicalProperties.*/
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub m_FocalLength: Option<f32>,
    /**The focus distance of the lens. To use this property, enable UsePhysicalProperties.*/
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub m_FocusDistance: Option<f32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_ForceIntoRT: Option<bool>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_GateFitMode: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_HDR: Option<bool>,
    /**The sensor sensitivity of the camera. To use this property, enable UsePhysicalProperties.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_Iso: Option<i32>,
    /**The lens offset of the camera. The lens shift is relative to the sensor size. For example, a lens shift of 0.5 offsets the sensor by half its horizontal size.*/
    /// Vector2f: (2018.4.15f1 - 2022.2.0b16)
    pub m_LensShift: Option<Vector2f>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_OcclusionCulling: Option<bool>,
    /**The size of the camera sensor, expressed in millimeters.*/
    /// Vector2f: (2018.4.15f1 - 2022.2.0b16)
    pub m_SensorSize: Option<Vector2f>,
    /**The exposure time of the camera, in seconts. To use this property, enable UsePhysicalProperties.*/
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub m_ShutterSpeed: Option<f32>,
    /**Distance to a point where virtual eyes converge.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_StereoConvergence: Option<f32>,
    /// bool: (5.6.0b2 - 5.6.0b2)
    pub m_StereoMirrorMode: Option<bool>,
    /**The distance between the virtual eyes. Use this to query or set the current eye separation. Note that most VR devices provide this value, in which case setting the value will have no effect.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_StereoSeparation: Option<f32>,
    /**Set the target display for this Camera.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_TargetDisplay: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_TargetEye: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_projectionMatrixMode: Option<i32>,
    /**The distance of the near clipping plane from the the Camera, in world units.*/
    /// f32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "near clip plane")]
    pub near_clip_plane: Option<f32>,
    /**Camera's half-size when in orthographic mode.*/
    /// f32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "orthographic size")]
    pub orthographic_size: Option<f32>,
}

/// Canvas is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Canvas.html):
/**
Element that can be used for screen rendering.
Elements on a canvas are rendered AFTER Scene rendering, either from an attached camera or using overlay mode.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Canvas {
    pub m_Camera: PPtr, /*<Camera>*/
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Allows for nested canvases to override pixelPerfect settings inherited from parent canvases.*/
    pub m_OverridePixelPerfect: bool,
    /**Override the sorting of canvas.*/
    pub m_OverrideSorting: bool,
    /**Force elements in the canvas to be aligned with pixels. Only applies with renderMode is Screen Space.*/
    pub m_PixelPerfect: bool,
    /**How far away from the camera is the Canvas generated.*/
    pub m_PlaneDistance: f32,
    pub m_ReceivesEvents: bool,
    /**Is the Canvas in World or Overlay mode?*/
    pub m_RenderMode: i32,
    pub m_SortingBucketNormalizedSize: f32,
    /**Unique ID of the Canvas' sorting layer.*/
    pub m_SortingLayerID: i32,
    /**Canvas' order within a sorting layer.*/
    pub m_SortingOrder: i16,
    /**For Overlay mode, display index on which the UI canvas will appear.*/
    pub m_TargetDisplay: i8,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_AdditionalShaderChannelsFlag: Option<i32>,
    /**Should the Canvas size be updated based on the render target when a manual Camera.Render call is performed.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_UpdateRectTransformForStandalone: Option<i32>,
}

/// CanvasGroup is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CanvasGroup.html):
/**
A Canvas placable element that can be used to modify children Alpha, Raycasting, Enabled state.
A canvas group can be used to modify the state of children elements.An example of this would be a window which fades in over time, by modifying the alpha value of the group the children elements will be affected. The result alpha will be the multiplied result of any nested groups, multiplied with the canvas elements alpha.You can configure Canvas Groups to not block raycasts. When you configure a Canvas Group to not block raycasts, graphic raycasting ignores anything in the group.Let's say you have a Canvas GameObject with a CanvasGroup component on it, and you set the CanvasGroup component's alpha to 0. In that case, the Canvas does not render any of its child GameObjects.

Now suppose that the Canvas also has a child CanvasGroup GameObject that you do want to render. If you enable IgnoreParentGroups for the CanvasGroup GameObject, the parent Canvas does not render any of its child GameObjects, including the CanvasGroup you want to render.

To get the child CanvasGroup GameObject, do one of two things:

In the parent Canvas, set the CanvasGroup component's alpha to a small, non-zero value.

Add a Canvas component to the child CanvasGroup GameObject that you want to render.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CanvasGroup {
    /**Set the alpha of the group.*/
    pub m_Alpha: f32,
    /**Does this group block raycasting (allow collision).*/
    pub m_BlocksRaycasts: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Should the group ignore parent groups?*/
    pub m_IgnoreParentGroups: bool,
    /**Is the group interactable (are the elements beneath the group enabled).*/
    pub m_Interactable: bool,
}

/// CanvasRenderer is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CanvasRenderer.html):
/**
A component that will render to the screen after all normal rendering has completed when attached to a Canvas. Designed for GUI application.
See Also:Canvas.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CanvasRenderer {
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Indicates whether geometry emitted by this renderer can be ignored when the vertex color alpha is close to zero for every vertex of the mesh.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_CullTransparentMesh: Option<bool>,
}

/// CapsuleCollider is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CapsuleCollider.html):
/**
A capsule-shaped primitive collider.
Capsules are cylinders with a half-sphere at each end.See Also: BoxCollider, SphereCollider, PhysicMaterial, Rigidbody.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CapsuleCollider {
    /**The center of the capsule, measured in the object's local space.*/
    pub m_Center: Vector3f,
    /**The direction of the capsule.*/
    pub m_Direction: i32,
    /**Enabled Colliders will collide with other Colliders, disabled Colliders won't.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The height of the capsule measured in the object's local space.*/
    pub m_Height: f32,
    /**Specify if this collider is configured as a trigger.*/
    pub m_IsTrigger: bool,
    /**The material used by the collider.*/
    pub m_Material: PPtr, /*<PhysicMaterial>*/
    /**The radius of the sphere, measured in the object's local space.*/
    pub m_Radius: f32,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ProvidesContacts: Option<bool>,
}

/// CapsuleCollider2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CapsuleCollider2D.html):
/**
A capsule-shaped primitive collider.
Capsules are boxes with a semi-circle at each end.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CapsuleCollider2D {
    /**The density of the collider used to calculate its mass (when auto mass is enabled).*/
    pub m_Density: f32,
    /**The direction that the capsule sides can extend.*/
    pub m_Direction: i32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    pub m_Material: PPtr, /*<PhysicsMaterial2D>*/
    /**The local offset of the collider geometry.*/
    pub m_Offset: Vector2f,
    /**The width and height of the capsule area.*/
    pub m_Size: Vector2f,
    /**Sets whether the Collider will be used or not used by a CompositeCollider2D.*/
    pub m_UsedByComposite: bool,
    /**Whether the collider is used by an attached effector or not.*/
    pub m_UsedByEffector: bool,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_CallbackLayers: Option<BitField>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
}

/// Channel is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub attributeName: String,
    pub byteOffset: i32,
    pub curve: AnimationCurve,
}

/// ChannelInfo is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/MPE.ChannelInfo.html):
/**
A structure that contains the connection information of a Channel in ChannelService.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub dimension: u8,
    pub format: u8,
    pub offset: u8,
    pub stream: u8,
}

/// CharacterController is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CharacterController.html):
/**
A CharacterController allows you to easily do movement constrained by collisions without having to deal with a rigidbody.
A CharacterController is not affected by forces and will only move when you call the Move function.

It will then carry out the movement but be constrained by collisions.See Also: Character Controller component and Character animation examples
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterController {
    /**The center of the character's capsule relative to the transform's position.*/
    pub m_Center: Vector3f,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The height of the character's capsule.*/
    pub m_Height: f32,
    /**Gets or sets the minimum move distance of the character controller.*/
    pub m_MinMoveDistance: f32,
    /**The radius of the character's capsule.*/
    pub m_Radius: f32,
    /**The character's collision skin width.*/
    pub m_SkinWidth: f32,
    /**The character controllers slope limit in degrees.*/
    pub m_SlopeLimit: f32,
    /**The character controllers step offset in meters.*/
    pub m_StepOffset: f32,
    /**Enabled Colliders will collide with other Colliders, disabled Colliders won't.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_Enabled: Option<bool>,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**Specify if this collider is configured as a trigger.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_IsTrigger: Option<bool>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
    /**The material used by the collider.*/
    /// PPtr/*<PhysicMaterial>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_Material: Option<PPtr /*<PhysicMaterial>*/>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ProvidesContacts: Option<bool>,
}

/// CharacterInfo is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CharacterInfo.html):
/**
Specification for how to render a character from the font texture. See Font.characterInfo.
See Also: Example at Font.RequestCharactersInTexture.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterInfo {
    /**Unicode value of the character.*/
    pub index: u32,
    pub uv: Rectf,
    pub vert: Rectf,
    /**The horizontal distance, rounded to the nearest integer, from the origin of this character to the origin of the next character.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub advance: Option<f32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub flipped: Option<bool>,
    /// f32: (3.4.0 - 3.4.0)
    pub width: Option<f32>,
}

/// CharacterJoint is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CharacterJoint.html):
/**
Character Joints are mainly used for Ragdoll effects.
They are an extended ball-socket joint which allows you to limit the joint on each axis.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterJoint {
    /**The Position of the anchor around which the joints motion is constrained.*/
    pub m_Anchor: Vector3f,
    /**The Direction of the axis around which the body is constrained.*/
    pub m_Axis: Vector3f,
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break. To be able to break, a joint must be _Locked_ or _Limited_ on the axis of rotation where the torque is being applied. This means that some joints cannot break, such as an unconstrained Configurable Joint.*/
    pub m_BreakTorque: f32,
    /**A reference to another rigidbody this joint connects to.*/
    pub m_ConnectedBody: PPtr, /*<Rigidbody>*/
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The upper limit around the primary axis of the character joint.*/
    pub m_HighTwistLimit: SoftJointLimit,
    /**The lower limit around the primary axis of the character joint.*/
    pub m_LowTwistLimit: SoftJointLimit,
    /**The angular limit of rotation (in degrees) around the primary axis of the character joint.*/
    pub m_Swing1Limit: SoftJointLimit,
    /**The angular limit of rotation (in degrees) around the primary axis of the character joint.*/
    pub m_Swing2Limit: SoftJointLimit,
    /**The secondary axis around which the joint can rotate.*/
    pub m_SwingAxis: Vector3f,
    /**Should the connectedAnchor be calculated automatically?*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    /**Position of the anchor relative to the connected Rigidbody.*/
    /// Vector3f: (5.6.0b2 - 2022.2.0b16)
    pub m_ConnectedAnchor: Option<Vector3f>,
    /**A reference to an articulation body this joint connects to.*/
    /// PPtr/*<ArticulationBody>*/: (2020.3.42f1 - 2022.2.0b16)
    pub m_ConnectedArticulationBody: Option<PPtr /*<ArticulationBody>*/>,
    /**The scale to apply to the inverse mass and inertia tensor of the connected body prior to solving the constraints.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ConnectedMassScale: Option<f32>,
    /**Enable collision between bodies connected with the joint.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_EnableCollision: Option<bool>,
    /**Toggle preprocessing for this joint.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_EnablePreprocessing: Option<bool>,
    /**Brings violated constraints back into alignment even when the solver fails.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_EnableProjection: Option<bool>,
    /**The scale to apply to the inverse mass and inertia tensor of the body prior to solving the constraints.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_MassScale: Option<f32>,
    /**Set the angular tolerance threshold (in degrees) for projection.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_ProjectionAngle: Option<f32>,
    /**Set the linear tolerance threshold for projection.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_ProjectionDistance: Option<f32>,
    /**The configuration of the spring attached to the swing limits of the joint.*/
    /// SoftJointLimitSpring: (5.6.0b2 - 2022.2.0b16)
    pub m_SwingLimitSpring: Option<SoftJointLimitSpring>,
    /**The configuration of the spring attached to the twist limits of the joint.*/
    /// SoftJointLimitSpring: (5.6.0b2 - 2022.2.0b16)
    pub m_TwistLimitSpring: Option<SoftJointLimitSpring>,
}

/// ChildAnimatorState is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.ChildAnimatorState.html):
/**
Structure that represents a state in the context of its parent state machine.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ChildAnimatorState {
    /**The position the the state node in the context of its parent state machine.*/
    pub m_Position: Vector3f,
    /**The state.*/
    pub m_State: PPtr, /*<AnimatorState>*/
}

/// ChildAnimatorStateMachine is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.ChildAnimatorStateMachine.html):
/**
Structure that represents a state machine in the context of its parent state machine.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ChildAnimatorStateMachine {
    /**The position of the state machine node in the context of its parent state machine.*/
    pub m_Position: Vector3f,
    /**The state machine.*/
    pub m_StateMachine: PPtr, /*<AnimatorStateMachine>*/
}

/// ChildMotion is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.ChildMotion.html):
/**
Structure that represents a motion in the context of its parent blend tree.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ChildMotion {
    /**Normalized time offset of the child.*/
    pub m_CycleOffset: f32,
    /**The parameter used by the child when used in a BlendTree of type BlendTreeType.Direct.*/
    pub m_DirectBlendParameter: String,
    /**Mirror of the child.*/
    pub m_Mirror: bool,
    /**The motion itself.*/
    pub m_Motion: PPtr, /*<Motion>*/
    /**The position of the child. Used in 2D blend trees.*/
    pub m_Position: Vector2f,
    /**The threshold of the child. Used in 1D blend trees.*/
    pub m_Threshold: f32,
    /**The relative speed of the child.*/
    pub m_TimeScale: f32,
}

/// CircleCollider2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CircleCollider2D.html):
/**
Collider for 2D physics representing an circle.
See Also: BoxCollider class, PolygonCollider2D class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CircleCollider2D {
    /**The density of the collider used to calculate its mass (when auto mass is enabled).*/
    pub m_Density: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    pub m_Material: PPtr, /*<PhysicsMaterial2D>*/
    /**The local offset of the collider geometry.*/
    pub m_Offset: Vector2f,
    /**Radius of the circle.*/
    pub m_Radius: f32,
    /**Sets whether the Collider will be used or not used by a CompositeCollider2D.*/
    pub m_UsedByComposite: bool,
    /**Whether the collider is used by an attached effector or not.*/
    pub m_UsedByEffector: bool,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_CallbackLayers: Option<BitField>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
}

/// ClampVelocityModule is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClampVelocityModule {
    pub dampen: f32,
    pub enabled: bool,
    pub inWorldSpace: bool,
    pub magnitude: MinMaxCurve,
    pub separateAxis: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    /// MinMaxCurve: (2017.4.33f1 - 2022.2.0b16)
    pub drag: Option<MinMaxCurve>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub multiplyDragByParticleSize: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub multiplyDragByParticleVelocity: Option<bool>,
}

/// ClassInfo is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClassInfo {
    pub m_AssemblyNameIndex: i32,
    pub m_ClassName: String,
    pub m_IsUnityClass: bool,
    pub m_MethodIndex: i32,
    pub m_NamespaceIndex: i32,
    pub m_NumOfMethods: i32,
}

/// ClassMethodInfo is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClassMethodInfo {
    pub m_ClassIndex: i32,
    pub m_MethodName: String,
    pub m_OrderNumber: i32,
}

/// Clip is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct Clip {
    pub m_ConstantClip: ConstantClip,
    pub m_DenseClip: DenseClip,
    pub m_StreamedClip: StreamedClip,
    /// Box<OffsetPtr>: (5.6.0b2 - 2017.4.33f1)
    pub m_Binding: Option<Box<OffsetPtr>>,
}

/// ClipAnimationInfo is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClipAnimationInfo {
    pub firstFrame: Enum_i32__f32,
    pub lastFrame: Enum_i32__f32,
    pub name: String,
    pub wrapMode: i32,
    /// bool: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "loop")]
    pub _loop: Option<bool>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub additiveReferencePoseFrame: Option<f32>,
    /// Vec<u32>: (5.6.0b2 - 2022.2.0b16)
    pub bodyMask: Option<Vec<u32>>,
    /// Vec<ClipAnimationInfoCurve>: (5.6.0b2 - 2022.2.0b16)
    pub curves: Option<Vec<ClipAnimationInfoCurve>>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub cycleOffset: Option<f32>,
    /// Vec<AnimationEvent>: (5.6.0b2 - 2022.2.0b16)
    pub events: Option<Vec<AnimationEvent>>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub hasAdditiveReferencePose: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub heightFromFeet: Option<bool>,
    /// i64: (2019.3.0f4 - 2022.2.0b16)
    pub internalID: Option<i64>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub keepOriginalOrientation: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub keepOriginalPositionXZ: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub keepOriginalPositionY: Option<bool>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub level: Option<f32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub loopBlend: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub loopBlendOrientation: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub loopBlendPositionXZ: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub loopBlendPositionY: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub loopTime: Option<bool>,
    /// PPtr/*<AvatarMask>*/: (5.6.0b2 - 2022.2.0b16)
    pub maskSource: Option<PPtr /*<AvatarMask>*/>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub maskType: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub mirror: Option<bool>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub orientationOffsetY: Option<f32>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub takeName: Option<String>,
    /// Vec<TransformMaskElement>: (5.6.0b2 - 2022.2.0b16)
    pub transformMask: Option<Vec<TransformMaskElement>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_i32__f32 {
    i32(i32),
    f32(f32),
}

/// ClipAnimationInfoCurve is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ClipAnimationInfoCurve.html):
/**
Stores a curve and its name that will be used to create additional curves during the import process.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ClipAnimationInfoCurve {
    /**The animation curve.*/
    pub curve: AnimationCurve,
    /**The name of the animation curve.*/
    pub name: String,
}

/// ClipMuscleConstant is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClipMuscleConstant {
    pub m_AverageAngularSpeed: f32,
    pub m_AverageSpeed: float3,
    pub m_Clip: OffsetPtr,
    pub m_CycleOffset: f32,
    pub m_DeltaPose: HumanPose,
    pub m_HeightFromFeet: bool,
    pub m_IndexArray: Vec<i32>,
    pub m_KeepOriginalOrientation: bool,
    pub m_KeepOriginalPositionXZ: bool,
    pub m_KeepOriginalPositionY: bool,
    pub m_LeftFootStartX: xform,
    pub m_Level: f32,
    pub m_LoopBlend: bool,
    pub m_LoopBlendOrientation: bool,
    pub m_LoopBlendPositionXZ: bool,
    pub m_LoopBlendPositionY: bool,
    pub m_LoopTime: bool,
    pub m_Mirror: bool,
    pub m_OrientationOffsetY: f32,
    pub m_RightFootStartX: xform,
    pub m_StartAtOrigin: bool,
    pub m_StartTime: f32,
    pub m_StartX: xform,
    pub m_StopTime: f32,
    pub m_StopX: xform,
    pub m_ValueArrayDelta: Vec<ValueDelta>,
    pub m_ValueArrayReferencePose: Vec<f32>,
}

/// Cloth is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Cloth.html):
/**
The Cloth class provides an interface to cloth simulation physics.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Cloth {
    /**Bending stiffness of the cloth.*/
    pub m_BendingStiffness: f32,
    /**An array of CapsuleColliders which this Cloth instance should collide with.*/
    pub m_CapsuleColliders: Vec<PPtr /*<CapsuleCollider>*/>,
    /**The cloth skinning coefficients used to set up how the cloth interacts with the skinned mesh.*/
    pub m_Coefficients: Vec<ClothConstrainCoefficients>,
    /**How much to increase mass of colliding particles.*/
    pub m_CollisionMassScale: f32,
    /**Damp cloth motion.*/
    pub m_Damping: f32,
    /**Is this cloth enabled?*/
    pub m_Enabled: u8,
    /**A constant, external acceleration applied to the cloth.*/
    pub m_ExternalAcceleration: Vector3f,
    /**The friction of the cloth when colliding with the character.*/
    pub m_Friction: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**A random, external acceleration applied to the cloth.*/
    pub m_RandomAcceleration: Vector3f,
    /**Cloth's sleep threshold.*/
    pub m_SleepThreshold: f32,
    pub m_SolverFrequency: f32,
    /**An array of ClothSphereColliderPairs which this Cloth instance should collide with.*/
    pub m_SphereColliders:
        Vec<Enum_PPtr___SphereCollider_____PPtr___SphereCollider______ClothSphereColliderPair>,
    /**Stretching stiffness of the cloth.*/
    pub m_StretchingStiffness: f32,
    pub m_UseContinuousCollision: bool,
    /**Should gravity affect the cloth simulation?*/
    pub m_UseGravity: bool,
    /**Use Tether Anchors.*/
    pub m_UseTethers: bool,
    /**How much world-space acceleration of the character will affect cloth vertices.*/
    pub m_WorldAccelerationScale: f32,
    /**How much world-space movement of the character will affect cloth vertices.*/
    pub m_WorldVelocityScale: f32,
    /// Vec<u32>: (2017.4.33f1 - 2022.2.0b16)
    pub m_SelfAndInterCollisionIndices: Option<Vec<u32>>,
    /**Minimum distance at which two cloth particles repel each other (default: 0.0).*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_SelfCollisionDistance: Option<f32>,
    /**Self-collision stiffness defines how strong the separating impulse should be for colliding particles.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_SelfCollisionStiffness: Option<f32>,
    /**Add one virtual particle per triangle to improve collision stability.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_UseVirtualParticles: Option<bool>,
    /// Vec<u32>: (2017.4.33f1 - 2022.2.0b16)
    pub m_VirtualParticleIndices: Option<Vec<u32>>,
    /// Vec<Vector3f>: (2017.4.33f1 - 2022.2.0b16)
    pub m_VirtualParticleWeights: Option<Vec<Vector3f>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_PPtr___SphereCollider_____PPtr___SphereCollider______ClothSphereColliderPair {
    PPtr___SphereCollider_____PPtr___SphereCollider___(
        (
            PPtr, /*<SphereCollider>*/
            PPtr, /*<SphereCollider>*/
        ),
    ),
    ClothSphereColliderPair(ClothSphereColliderPair),
}

/// ClothAttachment is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClothAttachment {
    pub m_Collider: PPtr, /*<Collider>*/
    pub m_Tearable: bool,
    pub m_TwoWayInteraction: bool,
}

/// ClothConstrainCoefficients is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClothConstrainCoefficients {
    pub collisionSphereDistance: f32,
    pub maxDistance: f32,
    /// f32: (3.4.0 - 3.4.0)
    pub collisionSphereRadius: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    pub maxDistanceBias: Option<f32>,
}

/// ClothRenderer is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClothRenderer {
    pub m_CastShadows: bool,
    pub m_Enabled: bool,
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_LightmapIndex: u8,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_Materials: Vec<PPtr /*<Material>*/>,
    pub m_PauseWhenNotVisible: bool,
    pub m_ReceiveShadows: bool,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    pub m_SubsetIndices: Vec<u32>,
}

/// ClothSphereColliderPair is a sub class of the Unity engine since version 2019.3.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ClothSphereColliderPair.html):
/**
A pair of SphereColliders used to define shapes for Cloth objects to collide against.
A ClothSphereColliderPair can contain either a single valid SphereCollider instance (with the second one being null), or a pair of two SphereColliders. In the former cases the ClothSphereColliderPair just represents a single SphereCollider for the cloth to collide against. In the latter case, it represents a conic capsule shape defined by the two spheres, and the cone connecting the two. Conic capsule shapes are useful for modelling limbs of a character.Select the cloth object to see a visualization of Cloth colliders shapes in the Scene view.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ClothSphereColliderPair {
    /**The first SphereCollider of a ClothSphereColliderPair.*/
    pub first: PPtr, /*<SphereCollider>*/
    /**The second SphereCollider of a ClothSphereColliderPair.*/
    pub second: PPtr, /*<SphereCollider>*/
}

/// CloudWebServicesManager is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct CloudWebServicesManager {}

/// ClusterInput is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ClusterInput.html):
/**
Interface for reading and writing inputs in a Unity Cluster.
ClusterInput provides access to VRPN devices by connecting to a VRPN server. It also provides access to writeable inputs. All inputs managed by ClusterInput will be replicated to the rest of the connected slaves in the cluster. Using ClusterInput is much like using the traditional Input system in Unity.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterInput {
    pub m_DeviceName: String,
    pub m_Index: i32,
    pub m_Name: String,
    pub m_ServerUrl: String,
    pub m_Type: i32,
}

/// ClusterInputManager is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterInputManager {
    pub m_Inputs: Vec<ClusterInput>,
}

/// CollabEditorSettings is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct CollabEditorSettings {
    pub inProgressEnabled: bool,
}

/// CollisionModule is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.CollisionModule.html):
/**
Script interface for the CollisionModule of a Particle System.
See Also: ParticleSystem, ParticleSystem.collision.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CollisionModule {
    /**Control which Layers this Particle System collides with.*/
    pub collidesWith: BitField,
    pub collidesWithDynamic: bool,
    pub collisionMessages: bool,
    pub collisionMode: i32,
    /**Specifies whether the CollisionModule is enabled or disabled.*/
    pub enabled: bool,
    pub interiorCollisions: bool,
    /**How much force is applied to each particle after a collision.*/
    pub m_Bounce: MinMaxCurve,
    /**How much speed does each particle lose after a collision.*/
    pub m_Dampen: MinMaxCurve,
    pub m_EnergyLossOnCollision: MinMaxCurve,
    /**The maximum number of collision shapes Unity considers for particle collisions. It ignores excess shapes. Terrains take priority.*/
    pub maxCollisionShapes: i32,
    /**Kill particles whose speed goes above this threshold, after a collision.*/
    pub maxKillSpeed: f32,
    /**Kill particles whose speed falls below this threshold, after a collision.*/
    pub minKillSpeed: f32,
    /**Specifies the accuracy of particle collisions against colliders in the Scene.*/
    pub quality: i32,
    /**A multiplier that Unity applies to the size of each particle before collisions are processed.*/
    pub radiusScale: f32,
    /**Size of voxels in the collision cache.*/
    pub voxelSize: f32,
    /**The type of particle collision to perform.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /**How much force is applied to a Collider when hit by particles from this Particle System.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub colliderForce: Option<f32>,
    /// Vec<PPtr/*<Transform>*/>: (2020.3.42f1 - 2022.2.0b16)
    pub m_Planes: Option<Vec<PPtr /*<Transform>*/>>,
    /**Specifies whether the physics system considers the collision angle when it applies forces from particles to Colliders.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub multiplyColliderForceByCollisionAngle: Option<bool>,
    /**Specifies whether the physics system considers particle sizes when it applies forces to Colliders.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub multiplyColliderForceByParticleSize: Option<bool>,
    /**Specifies whether the physics system considers particle speeds when it applies forces to Colliders.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub multiplyColliderForceByParticleSpeed: Option<bool>,
    /// PPtr/*<Transform>*/: (5.6.0b2 - 2020.1.0a20)
    pub plane0: Option<PPtr /*<Transform>*/>,
    /// PPtr/*<Transform>*/: (5.6.0b2 - 2020.1.0a20)
    pub plane1: Option<PPtr /*<Transform>*/>,
    /// PPtr/*<Transform>*/: (5.6.0b2 - 2020.1.0a20)
    pub plane2: Option<PPtr /*<Transform>*/>,
    /// PPtr/*<Transform>*/: (5.6.0b2 - 2020.1.0a20)
    pub plane3: Option<PPtr /*<Transform>*/>,
    /// PPtr/*<Transform>*/: (5.6.0b2 - 2020.1.0a20)
    pub plane4: Option<PPtr /*<Transform>*/>,
    /// PPtr/*<Transform>*/: (5.6.0b2 - 2020.1.0a20)
    pub plane5: Option<PPtr /*<Transform>*/>,
}

/// ColorBySpeedModule is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.ColorBySpeedModule.html):
/**
Script interface for the ColorBySpeedModule of a Particle System.
See Also: ParticleSystem, ParticleSystem.colorBySpeed.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ColorBySpeedModule {
    /**Specifies whether the ColorBySpeedModule is enabled or disabled.*/
    pub enabled: bool,
    pub gradient: MinMaxGradient,
    /**Apply the color gradient between these minimum and maximum speeds.*/
    pub range: Vector2f,
}

/// ColorModule is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ColorModule {
    pub enabled: bool,
    pub gradient: MinMaxGradient,
}

/// ColorRGBA is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ColorRGBA {
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub a: Option<f32>,
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub b: Option<f32>,
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub g: Option<f32>,
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub r: Option<f32>,
    /// u32: (3.4.0 - 2017.4.33f1)
    pub rgba: Option<u32>,
}

/// Component is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Component.html):
/**
Base class for everything attached to a GameObject.
Note that your code will never directly create a Component.  Instead, you write script code, and attach the script to a GameObject.
See Also: ScriptableObject as a way to create scripts that do not attach to any GameObject.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
}

/// ComponentPair is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentPair {
    pub component: PPtr, /*<Component>*/
}

/// CompositeCollider2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CompositeCollider2D.html):
/**
A Collider that can merge other Colliders together.
A CompositeCollider2D merges other Colliders together when their Collider2D.usedByComposite is set to true.When a Collider is used by a Composite Collider, the Editor will ignore and not show the Collider2D.sharedMaterial, Collider2D.isTrigger & Collider2D.usedByComposite properties. The same properties on the CompositeCollider2D will be used instead. You should set these properties on the Composite Collider instead to merge all Colliders into the Composite Collider.Composite Colliders can only merge BoxCollider2D and PolygonCollider2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CompositeCollider2D {
    pub m_ColliderPaths: Vec<SubCollider>,
    pub m_CompositePaths: Polygon2D,
    /**The density of the collider used to calculate its mass (when auto mass is enabled).*/
    pub m_Density: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Specifies when to generate the Composite Collider geometry.*/
    pub m_GenerationType: i32,
    /**Specifies the type of geometry the Composite Collider should generate.*/
    pub m_GeometryType: i32,
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    pub m_Material: PPtr, /*<PhysicsMaterial2D>*/
    /**The local offset of the collider geometry.*/
    pub m_Offset: Vector2f,
    /**Sets whether the Collider will be used or not used by a CompositeCollider2D.*/
    pub m_UsedByComposite: bool,
    /**Whether the collider is used by an attached effector or not.*/
    pub m_UsedByEffector: bool,
    /**Controls the minimum distance allowed between generated vertices.*/
    pub m_VertexDistance: f32,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_CallbackLayers: Option<BitField>,
    /// PPtr/*<GameObject>*/: (2022.2.0b16 - 2022.2.0b16)
    pub m_CompositeGameObject: Option<PPtr /*<GameObject>*/>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**Controls the radius of all edges created by the Collider.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_EdgeRadius: Option<f32>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
    /**Vertices are offset by this distance when compositing multiple physic shapes. Any vertices between shapes within this distance are combined.*/
    /// f32: (2019.3.0f4 - 2022.2.0b16)
    pub m_OffsetDistance: Option<f32>,
    /**When the value is true, the Collider uses an additional Delaunay triangulation step to produce the Collider mesh. When the value is false, this additional step does not occur.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_UseDelaunayMesh: Option<bool>,
}

/// CompressedAnimationCurve is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompressedAnimationCurve {
    pub m_Path: String,
    pub m_PostInfinity: i32,
    pub m_PreInfinity: i32,
    pub m_Slopes: PackedBitVector,
    pub m_Times: PackedBitVector,
    pub m_Values: PackedBitVector,
}

/// CompressedMesh is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompressedMesh {
    pub m_BoneIndices: PackedBitVector,
    pub m_NormalSigns: PackedBitVector,
    pub m_Normals: PackedBitVector,
    pub m_TangentSigns: PackedBitVector,
    pub m_Tangents: PackedBitVector,
    pub m_Triangles: PackedBitVector,
    pub m_UV: PackedBitVector,
    pub m_Vertices: PackedBitVector,
    pub m_Weights: PackedBitVector,
    /// PackedBitVector: (3.4.0 - 3.4.0)
    pub m_BindPoses: Option<PackedBitVector>,
    /// PackedBitVector: (5.6.0b2 - 2022.2.0b16)
    pub m_FloatColors: Option<PackedBitVector>,
    /// u32: (5.6.0b2 - 2022.2.0b16)
    pub m_UVInfo: Option<u32>,
}

/// ComputeBufferCounter is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeBufferCounter {
    pub bindpoint: i32,
    pub offset: i32,
}

/// ComputeShader is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ComputeShader.html):
/**
Compute Shader asset.
Compute shaders are programs that run on the GPU outside of the normal rendering pipeline.

They correspond to compute shader assets in the project (.compute files).Compute shader support can be queried runtime using SystemInfo.supportsComputeShaders. See Compute Shaders overview for more info about platforms supporting compute shaders.See Also: ComputeBuffer class, Compute Shaders overview.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShader {
    /**The name of the object.*/
    pub m_Name: String,
    pub variants: Vec<Enum_ComputeShaderPlatformVariant__ComputeShaderVariant>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_ComputeShaderPlatformVariant__ComputeShaderVariant {
    ComputeShaderPlatformVariant(ComputeShaderPlatformVariant),
    ComputeShaderVariant(ComputeShaderVariant),
}

/// ComputeShaderBuiltinSampler is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderBuiltinSampler {
    pub bindPoint: i32,
    pub sampler: i64,
}

/// ComputeShaderCB is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderCB {
    pub byteSize: i32,
    pub name: String,
    pub params: Vec<ComputeShaderParam>,
}

/// ComputeShaderImporter is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ComputeShaderImporter.html):
/**
Define compute shader import settings in the Unity Editor.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderImporter {
    /**Get or set the AssetBundle name.*/
    pub m_AssetBundleName: String,
    /**Get or set the AssetBundle variant.*/
    pub m_AssetBundleVariant: String,
    /**The name of the object.*/
    pub m_Name: String,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /// u32: (5.6.0b2 - 2020.3.42f1)
    pub m_CurrentAPIMask: Option<u32>,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /**This property has no effect.*/
    /// i32: (2020.3.42f1 - 2021.2.16f1)
    pub m_PreprocessorOverride: Option<i32>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// ComputeShaderKernel is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderKernel {
    pub builtinSamplers: Vec<ComputeShaderBuiltinSampler>,
    pub cbs: Vec<ComputeShaderResource>,
    pub code: Vec<u8>,
    pub inBuffers: Vec<ComputeShaderResource>,
    pub outBuffers: Vec<ComputeShaderResource>,
    pub textures: Vec<ComputeShaderResource>,
    pub threadGroupSize: Vec<u32>,
    /// Vec<u32>: (2020.1.0a20 - 2022.2.0b16)
    pub cbVariantIndices: Option<Vec<u32>>,
    /// String: (5.6.0b2 - 2019.3.0f4)
    pub name: Option<String>,
    /// i64: (2021.2.16f1 - 2022.2.0b16)
    pub requirements: Option<i64>,
}

/// ComputeShaderKernelParent is a sub class of the Unity engine since version 2020.1.0a20.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderKernelParent {
    pub name: String,
    /// Vec<String>: (2022.2.0b16 - 2022.2.0b16)
    pub dynamicKeywords: Option<Vec<String>>,
    /// Vec<String>: (2020.3.42f1 - 2022.2.0b16)
    pub globalKeywords: Option<Vec<String>>,
    /// Vec<String>: (2020.3.42f1 - 2022.2.0b16)
    pub localKeywords: Option<Vec<String>>,
    /// Vec<ComputeShaderKernel>: (2022.2.0b16 - 2022.2.0b16)
    pub uniqueVariants: Option<Vec<ComputeShaderKernel>>,
    /// Vec<String>: (2020.1.0a20 - 2020.1.0a20)
    pub validKeywords: Option<Vec<String>>,
    /// Vec<(String, u32)>: (2022.2.0b16 - 2022.2.0b16)
    pub variantIndices: Option<Vec<(String, u32)>>,
    /// Vec<(String, ComputeShaderKernel)>: (2020.1.0a20 - 2021.2.16f1)
    pub variantMap: Option<Vec<(String, ComputeShaderKernel)>>,
}

/// ComputeShaderParam is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderParam {
    pub arraySize: i64,
    pub colCount: i64,
    pub name: String,
    pub offset: i64,
    pub rowCount: i64,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// ComputeShaderPlatformVariant is a sub class of the Unity engine since version 2020.1.0a20.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderPlatformVariant {
    pub constantBuffers: Vec<ComputeShaderCB>,
    pub kernels: Vec<ComputeShaderKernelParent>,
    pub resourcesResolved: bool,
    pub targetLevel: i32,
    pub targetRenderer: i32,
}

/// ComputeShaderResource is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderResource {
    pub bindPoint: i32,
    pub generatedName: String,
    pub name: String,
    /// ComputeBufferCounter: (5.6.0b2 - 2017.4.33f1)
    pub counter: Option<ComputeBufferCounter>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub samplerBindPoint: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub texDimension: Option<i32>,
}

/// ComputeShaderVariant is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderVariant {
    pub constantBuffers: Vec<ComputeShaderCB>,
    pub kernels: Vec<ComputeShaderKernel>,
    pub resourcesResolved: bool,
    pub targetLevel: i32,
    pub targetRenderer: i32,
}

/// ConfigSetting is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSetting {
    pub flags: u32,
    pub value: String,
}

/// ConfigurableJoint is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ConfigurableJoint.html):
/**
The configurable joint is an extremely flexible joint giving you complete control over rotation and linear motion.
You can build all other joints with it and much more but it is also more complicated to setup.

It gives you control over motors, drives and joint limits for each rotation axis and and linear degree of freedom.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurableJoint {
    /**The Position of the anchor around which the joints motion is constrained.*/
    pub m_Anchor: Vector3f,
    /**Definition of how the joint's rotation will behave around its local X axis. Only used if Rotation Drive Mode is Swing & Twist.*/
    pub m_AngularXDrive: JointDrive,
    /**Allow rotation around the X axis to be Free, completely Locked, or Limited according to Low and High Angular XLimit.*/
    pub m_AngularXMotion: i32,
    /**Boundary defining rotation restriction, based on delta from original rotation.*/
    pub m_AngularYLimit: SoftJointLimit,
    /**Allow rotation around the Y axis to be Free, completely Locked, or Limited according to Angular YLimit.*/
    pub m_AngularYMotion: i32,
    /**Definition of how the joint's rotation will behave around its local Y and Z axes. Only used if Rotation Drive Mode is Swing & Twist.*/
    pub m_AngularYZDrive: JointDrive,
    /**Boundary defining rotation restriction, based on delta from original rotation.*/
    pub m_AngularZLimit: SoftJointLimit,
    /**Allow rotation around the Z axis to be Free, completely Locked, or Limited according to Angular ZLimit.*/
    pub m_AngularZMotion: i32,
    /**The Direction of the axis around which the body is constrained.*/
    pub m_Axis: Vector3f,
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break. To be able to break, a joint must be _Locked_ or _Limited_ on the axis of rotation where the torque is being applied. This means that some joints cannot break, such as an unconstrained Configurable Joint.*/
    pub m_BreakTorque: f32,
    /**If enabled, all Target values will be calculated in world space instead of the object's local space.*/
    pub m_ConfiguredInWorldSpace: bool,
    /**A reference to another rigidbody this joint connects to.*/
    pub m_ConnectedBody: PPtr, /*<Rigidbody>*/
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Boundary defining upper rotation restriction, based on delta from original rotation.*/
    pub m_HighAngularXLimit: SoftJointLimit,
    /**Boundary defining movement restriction, based on distance from the joint's origin.*/
    pub m_LinearLimit: SoftJointLimit,
    /**Boundary defining lower rotation restriction, based on delta from original rotation.*/
    pub m_LowAngularXLimit: SoftJointLimit,
    /**Set the angular tolerance threshold (in degrees) for projection.If the joint deviates by more than this angle around its locked angular degrees of freedom,the solver will move the bodies to close the angle.Setting a very small tolerance may result in simulation jitter or other artifacts.Sometimes it is not possible to project (for example when the joints form a cycle).*/
    pub m_ProjectionAngle: f32,
    /**Set the linear tolerance threshold for projection.If the joint separates by more than this distance along its locked degrees of freedom, the solverwill move the bodies to close the distance.Setting a very small tolerance may result in simulation jitter or other artifacts.Sometimes it is not possible to project (for example when the joints form a cycle).*/
    pub m_ProjectionDistance: f32,
    /**Brings violated constraints back into alignment even when the solver fails. Projection is not a physical process and does not preserve momentum or respect collision geometry. It is best avoided if practical, but can be useful in improving simulation quality where joint separation results in unacceptable artifacts.*/
    pub m_ProjectionMode: i32,
    /**Control the object's rotation with either X & YZ or Slerp Drive by itself.*/
    pub m_RotationDriveMode: i32,
    /**The joint's secondary axis.*/
    pub m_SecondaryAxis: Vector3f,
    /**Definition of how the joint's rotation will behave around all local axes. Only used if Rotation Drive Mode is Slerp Only.*/
    pub m_SlerpDrive: JointDrive,
    /**This is a Vector3. It defines the desired angular velocity that the joint should rotate into.*/
    pub m_TargetAngularVelocity: Vector3f,
    /**The desired position that the joint should move into.*/
    pub m_TargetPosition: Vector3f,
    /**This is a Quaternion. It defines the desired rotation that the joint should rotate into.*/
    pub m_TargetRotation: Quaternionf,
    /**The desired velocity that the joint should move along.*/
    pub m_TargetVelocity: Vector3f,
    /**Definition of how the joint's movement will behave along its local X axis.*/
    pub m_XDrive: JointDrive,
    /**Allow movement along the X axis to be Free, completely Locked, or Limited according to Linear Limit.*/
    pub m_XMotion: i32,
    /**Definition of how the joint's movement will behave along its local Y axis.*/
    pub m_YDrive: JointDrive,
    /**Allow movement along the Y axis to be Free, completely Locked, or Limited according to Linear Limit.*/
    pub m_YMotion: i32,
    /**Definition of how the joint's movement will behave along its local Z axis.*/
    pub m_ZDrive: JointDrive,
    /**Allow movement along the Z axis to be Free, completely Locked, or Limited according to Linear Limit.*/
    pub m_ZMotion: i32,
    /**The configuration of the spring attached to the angular X limit of the joint.*/
    /// SoftJointLimitSpring: (5.6.0b2 - 2022.2.0b16)
    pub m_AngularXLimitSpring: Option<SoftJointLimitSpring>,
    /**The configuration of the spring attached to the angular Y and angular Z limits of the joint.*/
    /// SoftJointLimitSpring: (5.6.0b2 - 2022.2.0b16)
    pub m_AngularYZLimitSpring: Option<SoftJointLimitSpring>,
    /**Should the connectedAnchor be calculated automatically?*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    /**Position of the anchor relative to the connected Rigidbody.*/
    /// Vector3f: (5.6.0b2 - 2022.2.0b16)
    pub m_ConnectedAnchor: Option<Vector3f>,
    /**A reference to an articulation body this joint connects to.*/
    /// PPtr/*<ArticulationBody>*/: (2020.3.42f1 - 2022.2.0b16)
    pub m_ConnectedArticulationBody: Option<PPtr /*<ArticulationBody>*/>,
    /**The scale to apply to the inverse mass and inertia tensor of the connected body prior to solving the constraints.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ConnectedMassScale: Option<f32>,
    /**Enable collision between bodies connected with the joint.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_EnableCollision: Option<bool>,
    /**Toggle preprocessing for this joint.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_EnablePreprocessing: Option<bool>,
    /**The configuration of the spring attached to the linear limit of the joint.*/
    /// SoftJointLimitSpring: (5.6.0b2 - 2022.2.0b16)
    pub m_LinearLimitSpring: Option<SoftJointLimitSpring>,
    /**The scale to apply to the inverse mass and inertia tensor of the body prior to solving the constraints.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_MassScale: Option<f32>,
    /**Enable this property to swap the order in which the physics engine processes the Rigidbodies involved in the joint. This results in different joint motion but has no impact on Rigidbodies and anchors.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_SwapBodies: Option<bool>,
}

/// ConstantBuffer is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantBuffer {
    pub m_MatrixParams: Vec<MatrixParameter>,
    pub m_NameIndex: i32,
    pub m_Size: i32,
    pub m_VectorParams: Vec<VectorParameter>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_IsPartialCB: Option<bool>,
    /// Vec<StructParameter>: (2017.4.33f1 - 2022.2.0b16)
    pub m_StructParams: Option<Vec<StructParameter>>,
}

/// ConstantClip is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantClip {
    pub data: Vec<f32>,
}

/// ConstantForce is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ConstantForce.html):
/**
A force applied constantly.
This is a small physics utility class used to apply a continous force to an object.Rigidbody.AddForce applies a force to the Rigidbody only for one frame, thus you have to keep calling the function.

ConstantForce on the other hand will apply the force every frame until you change the force or torque to a new value.See Also: Rigidbody.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantForce {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The force applied to the rigidbody every frame.*/
    pub m_Force: Vector3f,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The force - relative to the rigid bodies coordinate system - applied every frame.*/
    pub m_RelativeForce: Vector3f,
    /**The torque - relative to the rigid bodies coordinate system - applied every frame.*/
    pub m_RelativeTorque: Vector3f,
    /**The torque applied to the rigidbody every frame.*/
    pub m_Torque: Vector3f,
}

/// ConstantForce2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ConstantForce2D.html):
/**
Applies both linear and angular (torque) forces continuously to the rigidbody each physics update.
This is equivalent of calling Rigidbody2D.AddForce, Rigidbody2D.AddRelativeForce and Rigidbody2D.AddTorque each physics update.

See Also: Rigidbody2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantForce2D {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The linear force applied to the rigidbody each physics update.*/
    pub m_Force: Vector2f,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The linear force, relative to the rigid-body coordinate system, applied each physics update.*/
    pub m_RelativeForce: Vector2f,
    /**The torque applied to the rigidbody each physics update.*/
    pub m_Torque: f32,
}

/// ConstraintSource is a sub class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.ConstraintSource.html):
/**
Represents a source for the constraint.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstraintSource {
    /**The transform component of the source object.*/
    pub sourceTransform: PPtr, /*<Transform>*/
    /**The weight of the source in the evaluation of the constraint.*/
    pub weight: f32,
}

/// ControllerConstant is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ControllerConstant {
    pub m_DefaultValues: OffsetPtr,
    pub m_LayerArray: Vec<OffsetPtr>,
    pub m_StateMachineArray: Vec<OffsetPtr>,
    pub m_Values: OffsetPtr,
}

/// CrashReportManager is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct CrashReportManager {}

/// CrashReportingSettings is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CrashReporting.CrashReportingSettings.html):
/**
Editor API for the Unity Services editor feature. Normally CrashReporting is enabled from the Services window, but if writing your own editor extension, this API can be used.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CrashReportingSettings {
    pub m_Enabled: bool,
    pub m_EventUrl: String,
    /// u32: (2018.4.15f1 - 2022.2.0b16)
    pub m_LogBufferSize: Option<u32>,
    /// String: (2017.4.33f1 - 2017.4.33f1)
    pub m_NativeEventUrl: Option<String>,
}

/// Cubemap is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Cubemap.html):
/**
Class for handling cube maps, Use this to create or modify existing cube map assets.
This class does not support Cubemap creation with a Crunch compression TextureFormat.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Cubemap {
    pub m_ColorSpace: i32,
    pub m_CompleteImageSize: i64,
    /**Height of the Texture in pixels (Read Only).*/
    pub m_Height: i32,
    pub m_ImageCount: i32,
    /**Whether Unity stores an additional copy of this texture's pixel data in CPU-addressable memory.*/
    pub m_IsReadable: bool,
    pub m_LightmapFormat: i32,
    pub m_MipCount: i32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_SourceTextures: Vec<PPtr /*<Texture2D>*/>,
    pub m_StreamData: StreamingInfo,
    pub m_TextureDimension: i32,
    pub m_TextureFormat: i32,
    pub m_TextureSettings: GLTextureSettings,
    /**Width of the Texture in pixels (Read Only).*/
    pub m_Width: i32,
    /// Vec<u8>: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "image data")]
    pub image_data: Option<Vec<u8>>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.1.0a20 - 2021.2.16f1)
    pub m_IgnoreMasterTextureLimit: Option<bool>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_IgnoreMipmapLimit: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub m_IsPreProcessed: Option<bool>,
    /// String: (2022.2.0b16 - 2022.2.0b16)
    pub m_MipmapLimitGroupName: Option<String>,
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub m_MipsStripped: Option<i32>,
    /// Vec<u8>: (2020.3.42f1 - 2022.2.0b16)
    pub m_PlatformBlob: Option<Vec<u8>>,
    /**Determines whether mipmap streaming is enabled for this Texture.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_StreamingMipmaps: Option<bool>,
    /**Sets the relative priority for this Texture when reducing memory size to fit within the memory budget.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_StreamingMipmapsPriority: Option<i32>,
}

/// CubemapArray is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CubemapArray.html):
/**
Class for handling Cubemap arrays.
Modern graphics APIs (e.g. D3D10.1 and later, OpenGL 4.0 and later, Metal on macOS, PS4) support "cubemap arrays",

which are arrays of same size & format cubemaps. From the shader side, they are treated as a single resource, and sampling

them needs an extra coordinate that indicates which array element to sample from.Typically cubemap arrays are useful for implementing efficient reflection probe, lighting and shadowing systems

(all reflection/cookie/shadow cubemaps in a single array).Cubemap arrays do not have an import pipeline for them, and must be created from code, either at runtime or in editor

scripts. Using Graphics.CopyTexture is useful for fast copying of pixel data from regular Cubemap textures into

elements of a cubemap array. From editor scripts, a common way of creating serialized cubemap array is to create it,

fill with data (either via Graphics.CopyTexture from regular cubemaps, or via SetPixels or

SetPixels32) and save it as an asset via AssetDatabase.CreateAsset.Note that not all platforms and GPUs support cubemap arrays. Use SystemInfo.supportsCubemapArrayTextures to check. Also, this class does not support CubemapArray creation with a Crunch compression TextureFormat.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CubemapArray {
    pub m_ColorSpace: i32,
    /**Number of cubemaps in the array (Read Only).*/
    pub m_CubemapCount: i32,
    pub m_DataSize: u32,
    /**Texture format (Read Only).*/
    pub m_Format: i32,
    /**Whether Unity stores an additional copy of this texture's pixel data in CPU-addressable memory.*/
    pub m_IsReadable: bool,
    pub m_MipCount: i32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_StreamData: StreamingInfo,
    pub m_TextureSettings: GLTextureSettings,
    /**Width of the Texture in pixels (Read Only).*/
    pub m_Width: i32,
    /// Vec<u8>: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "image data")]
    pub image_data: Option<Vec<u8>>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_UsageMode: Option<i32>,
}

/// CustomCollider2D is a  class of the Unity engine since version 2021.2.16f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CustomCollider2D.html):
/**
Represents a Collider2D that is configured by assigning PhysicsShape2D geometry to it via a PhysicsShapeGroup2D.
Unlike all other Collider2D which are defined indirectly by controlling geometric parameters such as the size of a Box or the radius of a Circle, here the CustomCollider2D is defined entirely by adding, removing and modifying PhysicsShape2D. This results in a fully customized Collider2D containing an unlimited number of low-level PhysicsShape2D which can represent any type of shape or shapes, or emulate any other existing Collider2D such as the CircleCollider2D, BoxCollider2D, CapsuleCollider2D, EdgeCollider2D, CompositeCollider2D or TilemapCollider2D.Alongside the customized geometry, there is full support for all existing Collider2D functionality such as triggers, physics materials, queries  etc.When assigning PhysicsShape2D to the CustomCollider2D, you can do so either during Edit mode or Play mode.When modifying the CustomCollider2D during Edit mode, all assigned PhysicsShape2D and associated vertices will be saved with the Unity Scene. When the Unity Scene is loaded again, the CustomCollider2D will maintain its configuration. In this way, it acts like any other Collider2D that you make changes to during Edit mode. Using this ability, Edit mode authoring scripts can be used to create custom geometry.When modifing the CustomCollider2D during Play mode, all assigned PhysicsShape2D and associated vertices will be lost when exiting Play mode. This acts like any other Collider2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomCollider2D {
    pub m_CustomShapes: PhysicsShapeGroup2D,
    /**The density of the collider used to calculate its mass (when auto mass is enabled).*/
    pub m_Density: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    pub m_Material: PPtr, /*<PhysicsMaterial2D>*/
    /**The local offset of the collider geometry.*/
    pub m_Offset: Vector2f,
    /**Sets whether the Collider will be used or not used by a CompositeCollider2D.*/
    pub m_UsedByComposite: bool,
    /**Whether the collider is used by an attached effector or not.*/
    pub m_UsedByEffector: bool,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_CallbackLayers: Option<BitField>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
}

/// CustomDataModule is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.CustomDataModule.html):
/**
Script interface for the CustomDataModule of a Particle System.
See Also: ParticleSystem, ParticleSystem.customData.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomDataModule {
    pub color0: MinMaxGradient,
    pub color1: MinMaxGradient,
    /**Specifies whether the CustomDataModule is enabled or disabled.*/
    pub enabled: bool,
    pub mode0: i32,
    pub mode1: i32,
    pub vector0_0: MinMaxCurve,
    pub vector0_1: MinMaxCurve,
    pub vector0_2: MinMaxCurve,
    pub vector0_3: MinMaxCurve,
    pub vector1_0: MinMaxCurve,
    pub vector1_1: MinMaxCurve,
    pub vector1_2: MinMaxCurve,
    pub vector1_3: MinMaxCurve,
    pub vectorComponentCount0: i32,
    pub vectorComponentCount1: i32,
}

/// CustomRenderTexture is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CustomRenderTexture.html):
/**
Custom Render Textures are an extension to Render Textures that allow you to render directly to the Texture using a Shader.
Custom Render Textures are an extension to Render Textures that allow you to update a texture with a Shader, and then use it in a regular Material. This is useful for implementing all kinds of complex simulations, for instance: water caustics, ripple simulations for rain effects, or splatting liquids against a wall. Also provided is a scripting and Shader framework to help with more complicated configurations like partial or multi-pass updates, and varying update frequency.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomRenderTexture {
    /**The antialiasing level for the RenderTexture.*/
    pub m_AntiAliasing: i32,
    pub m_ColorFormat: i32,
    /**The bit field that you can use to enable or disable update on each of the cubemap faces. The bit order from least to most significant bit is as follows: +X, -X, +Y, -Y, +Z, -Z.*/
    pub m_CubemapFaceMask: u32,
    pub m_CurrentUpdateZoneSpace: i32,
    /**Dimensionality (type) of the Texture (Read Only).*/
    pub m_Dimension: i32,
    /**When this parameter is set to true, Unity double-buffers the Custom Render Texture so that you can access it during its own update.*/
    pub m_DoubleBuffered: bool,
    pub m_GenerateMips: bool,
    /**Height of the Texture in pixels (Read Only).*/
    pub m_Height: i32,
    pub m_InitColor: ColorRGBA,
    pub m_InitMaterial: PPtr, /*<Material>*/
    pub m_InitTexture: PPtr,  /*<Texture>*/
    /**Determine how Unity initializes a texture.*/
    pub m_InitializationMode: i32,
    /**The Material that Unity uses to initialize the content of a Custom Render Texture.*/
    pub m_Material: PPtr, /*<Material>*/
    pub m_MipMap: bool,
    /**The name of the object.*/
    pub m_Name: String,
    /**Does this render texture use sRGB read/write conversions? (Read Only).*/
    pub m_SRGB: bool,
    /**The Shader Pass Unity uses to update the Custom Render Texture.*/
    pub m_ShaderPass: u32,
    pub m_TextureSettings: GLTextureSettings,
    /**Determine how Unity updates the Custom Render Texture.*/
    pub m_UpdateMode: i32,
    /**The period in seconds that Unity updates real-time Custom Render Textures. A value of 0.0 means Unity updates real-time Custom Render Textures every frame.*/
    pub m_UpdatePeriod: f32,
    /**The space in which Unity expresses update zones. You can set this to Normalized or Pixel space.*/
    pub m_UpdateZoneSpace: i32,
    pub m_UpdateZones: Vec<UpdateZoneInfo>,
    /**Volume extent of a 3D render texture or number of slices of array texture.*/
    pub m_VolumeDepth: i32,
    /**Width of the Texture in pixels (Read Only).*/
    pub m_Width: i32,
    /**When this parameter is set to true, Unity wraps Update zones around the border of the Custom Render Texture. Otherwise, Unity clamps Update zones at the border of the Custom Render Texture.*/
    pub m_WrapUpdateZones: bool,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_BindMS: Option<bool>,
    /// i32: (5.6.0b2 - 2020.3.42f1)
    pub m_DepthFormat: Option<i32>,
    /**The format of the depth/stencil buffer.*/
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_DepthStencilFormat: Option<i32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_DownscaleFallback: Option<bool>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_EnableCompatibleFormat: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_InitSource: Option<i32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_MipCount: Option<i32>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_ShadowSamplingMode: Option<i32>,
    /**Is the render texture marked to be scaled by the Dynamic Resolution system.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_UseDynamicScale: Option<bool>,
}

/// DataTemplate is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct DataTemplate {
    pub m_Father: PPtr, /*<DataTemplate>*/
    pub m_IsDataTemplate: bool,
    pub m_LastMergeIdentifier: GUID,
    pub m_Name: String,
    pub m_Objects: Vec<PPtr /*<EditorExtension>*/>,
}

/// DateTime is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct DateTime {
    pub ticks: i64,
}

/// DefaultAsset is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/DefaultAsset.html):
/**
DefaultAsset is used for assets that do not have a specific type (yet).
Search for t:DefaultAsset in the project browser to see which assets are of that type.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultAsset {
    pub m_IsWarning: bool,
    pub m_Message: String,
    /**The name of the object.*/
    pub m_Name: String,
}

/// DefaultImporter is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_Name: String,
    pub m_UserData: String,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// DefaultPreset is a sub class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Presets.DefaultPreset.html):
/**
This structure defines a default Preset.
See Preset.GetDefaultListForType and Preset.SetDefaultListForType for usage.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultPreset {
    /**The Preset applied to an object instance when it matches the search filter defined by DefaultPreset.m_Filter.*/
    pub m_Preset: PPtr, /*<Preset>*/
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_Disabled: Option<bool>,
    /**The search filter that is compared against the object instance. The DefaultPreset.m_Preset is applied to the object instance if it matches the search filter.*/
    /// String: (2019.3.0f4 - 2022.2.0b16)
    pub m_Filter: Option<String>,
}

/// DefaultPresetList is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultPresetList {
    #[serde(alias = "type")]
    pub _type: PresetType,
    pub defaultPresets: Vec<DefaultPreset>,
}

/// DeletedItem is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedItem {
    pub changeset: i32,
    pub digest: Enum_MdFour__Hash128,
    pub fullPath: String,
    pub guid: GUID,
    pub parent: GUID,
    /// i32: (3.4.0 - 5.6.0b2)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// DenseClip is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct DenseClip {
    pub m_BeginTime: f32,
    pub m_CurveCount: u32,
    pub m_FrameCount: i32,
    pub m_SampleArray: Vec<f32>,
    pub m_SampleRate: f32,
}

/// Derived is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct Derived {}

/// DetailDatabase is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct DetailDatabase {
    pub WavingGrassTint: ColorRGBA,
    pub m_DetailPrototypes: Vec<DetailPrototype>,
    pub m_PatchCount: i32,
    pub m_PatchSamples: i32,
    pub m_Patches: Vec<DetailPatch>,
    pub m_PreloadTextureAtlasData: Vec<PPtr /*<Texture2D>*/>,
    pub m_TreeInstances: Vec<TreeInstance>,
    pub m_TreePrototypes: Vec<TreePrototype>,
    pub m_WavingGrassAmount: f32,
    pub m_WavingGrassSpeed: f32,
    pub m_WavingGrassStrength: f32,
    /// PPtr/*<Shader>*/: (2022.2.0b16 - 2022.2.0b16)
    #[serde(alias = "m_DefaultShaders[0]")]
    pub m_DefaultShaders_0_: Option<PPtr /*<Shader>*/>,
    /// PPtr/*<Shader>*/: (2022.2.0b16 - 2022.2.0b16)
    #[serde(alias = "m_DefaultShaders[1]")]
    pub m_DefaultShaders_1_: Option<PPtr /*<Shader>*/>,
    /// PPtr/*<Shader>*/: (2022.2.0b16 - 2022.2.0b16)
    #[serde(alias = "m_DefaultShaders[2]")]
    pub m_DefaultShaders_2_: Option<PPtr /*<Shader>*/>,
    /// PPtr/*<Shader>*/: (2019.3.0f4 - 2020.3.42f1)
    pub m_DetailBillboardShader: Option<PPtr /*<Shader>*/>,
    /// PPtr/*<Shader>*/: (2019.3.0f4 - 2020.3.42f1)
    pub m_DetailMeshGrassShader: Option<PPtr /*<Shader>*/>,
    /// PPtr/*<Shader>*/: (2019.3.0f4 - 2020.3.42f1)
    pub m_DetailMeshLitShader: Option<PPtr /*<Shader>*/>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_DetailScatterMode: Option<i32>,
    /// Vec<Vector3f>: (3.4.0 - 2020.1.0a20)
    pub m_RandomRotations: Option<Vec<Vector3f>>,
}

/// DetailPatch is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct DetailPatch {
    pub layerIndices: Vec<u8>,
    /// AABB: (3.4.0 - 2020.1.0a20)
    pub bounds: Option<AABB>,
    /// Vec<u8>: (2022.2.0b16 - 2022.2.0b16)
    pub coverage: Option<Vec<u8>>,
    /// Vec<u8>: (3.4.0 - 2021.2.16f1)
    pub numberOfObjects: Option<Vec<u8>>,
}

/// DetailPrototype is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/DetailPrototype.html):
/**
Detail prototype used by the Terrain GameObject.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DetailPrototype {
    /**Color when the DetailPrototypes are "dry".*/
    pub dryColor: ColorRGBA,
    /**Color when the DetailPrototypes are "healthy".*/
    pub healthyColor: ColorRGBA,
    /**Maximum height of the grass billboards (if render mode is GrassBillboard).*/
    pub maxHeight: f32,
    /**Maximum width of the grass billboards (if render mode is GrassBillboard).*/
    pub maxWidth: f32,
    /**Minimum height of the grass billboards (if render mode is GrassBillboard).*/
    pub minHeight: f32,
    /**Minimum width of the grass billboards (if render mode is GrassBillboard).*/
    pub minWidth: f32,
    /**Controls the spatial frequency of the noise pattern used to vary the scale and color of the detail objects.*/
    pub noiseSpread: f32,
    /**GameObject used by the DetailPrototype.*/
    pub prototype: PPtr, /*<GameObject>*/
    /**Texture used by the DetailPrototype.*/
    pub prototypeTexture: PPtr, /*<Texture2D>*/
    /**Render mode for the DetailPrototype.*/
    pub renderMode: i32,
    /**Indicates whether this detail prototype uses the Mesh object from the GameObject specified by prototype.*/
    pub usePrototypeMesh: i32,
    /**Rotate detail axis parallel to the ground's normal direction, so that the detail is perpendicular to the ground.*/
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub alignToGround: Option<f32>,
    /// f32: (3.4.0 - 2020.1.0a20)
    pub bendFactor: Option<f32>,
    /**Controls detail density for this detail prototype, relative to it's size.*/
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub density: Option<f32>,
    /// f32: (2020.3.42f1 - 2022.2.0b16)
    pub holeTestRadius: Option<f32>,
    /// f32: (3.4.0 - 2020.3.42f1)
    pub lightmapFactor: Option<f32>,
    /**Specifies the random seed value for detail object placement.*/
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub noiseSeed: Option<i32>,
    /**Controls how Unity generates the detail positions.*/
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub positionJitter: Option<f32>,
    /**Controls the detail's target coverage.*/
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub targetCoverage: Option<f32>,
    /**Indicates the global density scale set in the terrain settings affects this detail prototype.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub useDensityScaling: Option<i32>,
    /**Indicates whether this detail prototype uses  GPU Instancing for rendering.*/
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub useInstancing: Option<i32>,
}

/// DeviceNone is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceNone {}

/// DirectorGenericBinding is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectorGenericBinding {
    pub key: PPtr,   /*<Object>*/
    pub value: PPtr, /*<Object>*/
}

/// DistanceJoint2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/DistanceJoint2D.html):
/**
Joint that keeps two Rigidbody2D objects a fixed distance apart.
Note that unlike the SpringJoint2D component, the distance separating the objects is truly fixed and does not allow for any stretching.See Also: HingeJoint2D class, SliderJoint2D class, SpringJoint2D class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DistanceJoint2D {
    /**The joint's anchor point on the object that has the joint component.*/
    pub m_Anchor: Vector2f,
    /**Should the connectedAnchor be calculated automatically?*/
    pub m_AutoConfigureConnectedAnchor: bool,
    /**Should the distance be calculated automatically?*/
    pub m_AutoConfigureDistance: bool,
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break.*/
    pub m_BreakTorque: f32,
    /**The joint's anchor point on the second object (ie, the one which doesn't have the joint component).*/
    pub m_ConnectedAnchor: Vector2f,
    pub m_ConnectedRigidBody: PPtr, /*<Rigidbody2D>*/
    /**The distance separating the two ends of the joint.*/
    pub m_Distance: f32,
    /**Should the two rigid bodies connected with this joint collide with each other?*/
    pub m_EnableCollision: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Whether to maintain a maximum distance only or not.  If not then the absolute distance will be maintained instead.*/
    pub m_MaxDistanceOnly: bool,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_BreakAction: Option<i32>,
}

/// EdgeCollider2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/EdgeCollider2D.html):
/**
Collider for 2D physics representing an arbitrary set of connected edges (lines) defined by its vertices.
See Also: BoxCollider2D, CircleCollider2D, PolygonCollider2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct EdgeCollider2D {
    /**The density of the collider used to calculate its mass (when auto mass is enabled).*/
    pub m_Density: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    pub m_Material: PPtr, /*<PhysicsMaterial2D>*/
    /**The local offset of the collider geometry.*/
    pub m_Offset: Vector2f,
    /**Get or set the points defining multiple continuous edges.*/
    pub m_Points: Vec<Vector2f>,
    /**Sets whether the Collider will be used or not used by a CompositeCollider2D.*/
    pub m_UsedByComposite: bool,
    /**Whether the collider is used by an attached effector or not.*/
    pub m_UsedByEffector: bool,
    /**Defines the position of a virtual point adjacent to the end point of the EdgeCollider2D.*/
    /// Vector2f: (2020.1.0a20 - 2022.2.0b16)
    pub m_AdjacentEndPoint: Option<Vector2f>,
    /**Defines the position of a virtual point adjacent to the start point of the EdgeCollider2D.*/
    /// Vector2f: (2020.1.0a20 - 2022.2.0b16)
    pub m_AdjacentStartPoint: Option<Vector2f>,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_CallbackLayers: Option<BitField>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**Controls the radius of all edges created by the collider.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_EdgeRadius: Option<f32>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
    /**Set this to true to use the adjacentEndPoint to form the collision normal that is used to calculate the collision response when a collision occurs at the Edge Collider's end point. Set this to false to not use the adjacentEndPoint, and the collision normal becomes the direction of motion of the collision.*/
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub m_UseAdjacentEndPoint: Option<bool>,
    /**Set this to true to use the adjacentStartPoint to form the collision normal that is used to calculate the collision response when a collision occurs at the Edge Collider's start point. Set this to false to not use the adjacentStartPoint, and the collision normal becomes the direction of motion of the collision.*/
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub m_UseAdjacentStartPoint: Option<bool>,
}

/// EditorBuildSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/EditorBuildSettings.html):
/**
This class allows you to modify the Editor Build Settings via script.
See EditorBuildSettings.scenes for an example of how to use this class.
See Also: EditorBuildSettingsScene, EditorUserBuildSettings, BuildPlayerOptions.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorBuildSettings {
    pub m_Scenes: Vec<Scene>,
    /// Vec<(String, PPtr/*<Object>*/)>: (2018.4.15f1 - 2022.2.0b16)
    pub m_configObjects: Option<Vec<(String, PPtr /*<Object>*/)>>,
}

/// EditorExtensionImpl is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorExtensionImpl {
    /// Vec<u8>: (3.4.0 - 3.4.0)
    pub gFlattenedTypeTree: Option<Vec<u8>>,
    /// PPtr/*<DataTemplate>*/: (3.4.0 - 3.4.0)
    pub m_DataTemplate: Option<PPtr /*<DataTemplate>*/>,
    /// PPtr/*<EditorExtension>*/: (3.4.0 - 3.4.0)
    pub m_Object: Option<PPtr /*<EditorExtension>*/>,
    /// bitset: (3.4.0 - 3.4.0)
    pub m_OverrideVariable: Option<bitset>,
    /// PPtr/*<EditorExtensionImpl>*/: (3.4.0 - 3.4.0)
    pub m_TemplateFather: Option<PPtr /*<EditorExtensionImpl>*/>,
}

/// EditorProjectAccess is a  class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorProjectAccess {
    pub m_Name: String,
}

/// EditorSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/EditorSettings.html):
/**
User settings for Unity Editor.
Use EditorSettings to apply Editor Project Settings to your Unity Project. You can control settings such as version control, streaming settings, and Asset serialization.See Also: Editor Project Settings
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorSettings {
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_AssetNamingUsesSpace: Option<bool>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_AssetPipelineMode: Option<i32>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_AsyncShaderCompilation: Option<bool>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_Bc7TextureCompressor: Option<i32>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_CacheServerDownloadBatchSize: Option<i32>,
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub m_CacheServerEnableAuth: Option<bool>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_CacheServerEnableDownload: Option<bool>,
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub m_CacheServerEnableTls: Option<bool>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_CacheServerEnableUpload: Option<bool>,
    /// String: (2019.3.0f4 - 2022.2.0b16)
    pub m_CacheServerEndpoint: Option<String>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_CacheServerMode: Option<i32>,
    /// String: (2019.3.0f4 - 2022.2.0b16)
    pub m_CacheServerNamespacePrefix: Option<String>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_CacheServerValidationMode: Option<i32>,
    /// bool: (2020.1.0a20 - 2021.2.16f1)
    pub m_CachingShaderPreprocessor: Option<bool>,
    /// CollabEditorSettings: (2017.4.33f1 - 2019.3.0f4)
    pub m_CollabEditorSettings: Option<CollabEditorSettings>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_DefaultBehaviorMode: Option<i32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_DisableCookiesInLightmapper: Option<bool>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_EnableEditorAsyncCPUTextureLoading: Option<bool>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_EnableEnlightenBakedGI: Option<bool>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_EnableTextureStreamingInEditMode: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_EnableTextureStreamingInPlayMode: Option<bool>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_EnterPlayModeOptions: Option<i32>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_EnterPlayModeOptionsEnabled: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_EtcTextureBestCompressor: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_EtcTextureCompressorBehavior: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_EtcTextureFastCompressor: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_EtcTextureNormalCompressor: Option<i32>,
    /// i32: (3.4.0 - 3.4.0); String: (5.6.0b2 - 2019.3.0f4)
    pub m_ExternalVersionControlSupport: Option<Enum_i32__String>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_GameObjectNamingDigits: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_GameObjectNamingScheme: Option<i32>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_InspectorUseIMGUIDefaultInspector: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_LineEndingsForNewScripts: Option<i32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_PrefabModeAllowAutoSave: Option<bool>,
    /// PPtr/*<SceneAsset>*/: (2018.4.15f1 - 2022.2.0b16)
    pub m_PrefabRegularEnvironment: Option<PPtr /*<SceneAsset>*/>,
    /// PPtr/*<SceneAsset>*/: (2018.4.15f1 - 2022.2.0b16)
    pub m_PrefabUIEnvironment: Option<PPtr /*<SceneAsset>*/>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_ProjectGenerationIncludedExtensions: Option<String>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_ProjectGenerationRootNamespace: Option<String>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_RefreshImportMode: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SerializationMode: Option<i32>,
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub m_SerializeInlineMappingsOnOneLine: Option<bool>,
    /// bool: (2019.3.0f4 - 2019.3.0f4)
    pub m_ShowLightmapResolutionOverlay: Option<bool>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_SpritePackerCacheSize: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SpritePackerMode: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SpritePackerPaddingPower: Option<i32>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_UseLegacyProbeSampleCount: Option<bool>,
    /// String: (5.6.0b2 - 2017.4.33f1)
    pub m_UserGeneratedProjectSuffix: Option<String>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_WebSecurityEmulationEnabled: Option<i32>,
    /// String: (3.4.0 - 3.4.0)
    pub m_WebSecurityEmulationHostUrl: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_i32__String {
    i32(i32),
    String(String),
}

/// EditorUserBuildSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/EditorUserBuildSettings.html):
/**
User build settings for the Editor
See Also: EditorBuildSettings.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorUserBuildSettings {
    pub m_ActiveBuildTarget: i32,
    pub m_AllowDebugging: bool,
    pub m_ArchitectureFlags: i32,
    pub m_BuildLocation: Vec<String>,
    pub m_ConnectProfiler: bool,
    pub m_Development: bool,
    pub m_InstallInBuildFolder: bool,
    pub m_SelectedAndroidSubtarget: i32,
    pub m_SelectedBuildTargetGroup: i32,
    pub m_SelectedStandaloneTarget: i32,
    /// String: (2021.2.16f1 - 2022.2.0b16)
    pub m_ActiveBuildPlatformGroupName: Option<String>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_ActiveBuildTargetGroup: Option<i32>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_ActiveStandaloneBuildSubtarget: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_AndroidBuildSystem: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_AndroidBuildType: Option<i32>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_AndroidCreateSymbols: Option<i32>,
    /// bool: (2018.4.15f1 - 2020.3.42f1)
    pub m_AndroidCreateSymbolsZip: Option<bool>,
    /// String: (2018.4.15f1 - 2022.2.0b16)
    pub m_AndroidCurrentDeploymentTargetId: Option<String>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_AndroidDebugMinification: Option<i32>,
    /// String: (2017.4.33f1 - 2022.2.0b16)
    pub m_AndroidDeviceSocketAddress: Option<String>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_AndroidReleaseMinification: Option<i32>,
    /// bool: (2018.4.15f1 - 2018.4.15f1)
    pub m_AndroidUseLegacySdkTools: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_BuildAppBundle: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_BuildScriptsOnly: Option<bool>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_BuildWithDeepProfilingSupport: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_CompressFilesInPackage: Option<bool>,
    /// bool: (5.6.0b2 - 2020.3.42f1)
    pub m_CompressWithPsArc: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_CreateRomFileForSwitch: Option<bool>,
    /// bool: (5.6.0b2 - 2020.1.0a20)
    pub m_CreateSolutionFileForSwitch: Option<bool>,
    /// bool: (2018.4.15f1 - 2018.4.15f1)
    pub m_DatalessPlayer: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_EnableDebugPadForSwitch: Option<bool>,
    /// bool: (5.6.0b2 - 2020.3.42f1)
    pub m_EnableHeadlessMode: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_EnableHeapInspectorForSwitch: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_EnableRomCompressionForSwitch: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_ExplicitArrayBoundsChecks: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_ExplicitDivideByZeroChecks: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_ExplicitNullChecks: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_ExportAsGoogleAndroidProject: Option<bool>,
    /// String: (2017.4.33f1 - 2018.4.15f1)
    pub m_FacebookAccessToken: Option<String>,
    /// bool: (5.6.0b2 - 5.6.0b2)
    pub m_FacebookCreatePackageForSubmission: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_ForceInstallation: Option<bool>,
    /// bool: (5.6.0b2 - 5.6.0b2)
    pub m_ForceOptimizeScriptCompilation: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_GenerateNintendoSwitchShaderInfo: Option<bool>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub m_GenerateWSAReferenceProjects: Option<bool>,
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub m_HTCSScriptDebuggingForSwitch: Option<bool>,
    /// i32: (2021.2.16f1 - 2021.2.16f1)
    pub m_Il2CppCodeGeneration: Option<i32>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_MovePackageToDiscOuterEdge: Option<bool>,
    /// bool: (2018.4.15f1 - 2019.3.0f4)
    pub m_NVNDrawValidation: Option<bool>,
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub m_NVNDrawValidationHeavy: Option<bool>,
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub m_NVNDrawValidationLight: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_NVNGraphicsDebuggerForSwitch: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_NVNShaderDebugging: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_NeedSubmissionMaterials: Option<bool>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_OverrideMaxTextureSize: Option<i32>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_OverrideTextureCompression: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_PS4HardwareTarget: Option<i32>,
    /// String: (2021.2.16f1 - 2022.2.0b16)
    pub m_PathOnRemoteDevice: Option<String>,
    /// Vec<(String, PlatformSettingsData)>: (5.6.0b2 - 2022.2.0b16)
    pub m_PlatformSettings: Option<Vec<(String, PlatformSettingsData)>>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_RedirectWritesToHostMountForSwitch: Option<bool>,
    /// String: (2021.2.16f1 - 2022.2.0b16)
    pub m_RemoteDeviceAddress: Option<String>,
    /// String: (2021.2.16f1 - 2022.2.0b16)
    pub m_RemoteDeviceExports: Option<String>,
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub m_RemoteDeviceInfo: Option<bool>,
    /// String: (2021.2.16f1 - 2022.2.0b16)
    pub m_RemoteDeviceUsername: Option<String>,
    /// String: (2020.3.42f1 - 2022.2.0b16)
    pub m_RomCompressionConfigForSwitch: Option<String>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_RomCompressionLevelForSwitch: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_RomCompressionTypeForSwitch: Option<i32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_SaveADFForSwitch: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_SelectedAndroidETC2Fallback: Option<i32>,
    /// String: (2021.2.16f1 - 2022.2.0b16)
    pub m_SelectedBuildPlatformGroupName: Option<String>,
    /// Vec<(String, i32)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_SelectedCompressionType: Option<Vec<(String, i32)>>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_SelectedEmbeddedLinuxArchitecture: Option<i32>,
    /// i32: (5.6.0b2 - 2018.4.15f1)
    pub m_SelectedFacebookTarget: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SelectedIOSBuildType: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SelectedPS4Subtarget: Option<i32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_SelectedPSMSubtarget: Option<i32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_SelectedPSP2Subtarget: Option<i32>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_SelectedQNXArchitecture: Option<i32>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_SelectedQNXOsVersion: Option<i32>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_SelectedStandaloneBuildSubtarget: Option<i32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_SelectedTizenSubtarget: Option<i32>,
    /// String: (2018.4.15f1 - 2022.2.0b16)
    pub m_SelectedWSAArchitecture: Option<String>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SelectedWSABuildAndRunDeployTarget: Option<i32>,
    /// String: (2018.4.15f1 - 2022.2.0b16)
    pub m_SelectedWSAMinUWPSDK: Option<String>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub m_SelectedWSASDK: Option<i32>,
    /// i32: (5.6.0b2 - 2020.3.42f1)
    pub m_SelectedWSASubtarget: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SelectedWSAUWPBuildType: Option<i32>,
    /// String: (2017.4.33f1 - 2022.2.0b16)
    pub m_SelectedWSAUWPSDK: Option<String>,
    /// String: (2017.4.33f1 - 2022.2.0b16)
    pub m_SelectedWSAUWPVSVersion: Option<String>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_SelectedWebGLSubtarget: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_SelectedWiiSubtarget: Option<i32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_SelectedWiiUBootMode: Option<i32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_SelectedWiiUBuildOutput: Option<i32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_SelectedWiiUDebugLevel: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_SelectedXboxOneDeployDrive: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SelectedXboxOneDeployMethod: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SelectedXboxSubtarget: Option<i32>,
    /// bool: (3.4.0 - 2020.3.42f1)
    pub m_SymlinkLibraries: Option<bool>,
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub m_SymlinkSources: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_SymlinkTrampoline: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_UseLegacyNvnPoolAllocatorForSwitch: Option<bool>,
    /// Vec<bool>: (5.6.0b2 - 2018.4.15f1)
    pub m_WSADotNetNativeEnabled: Option<Vec<bool>>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_WaitForPlayerConnection: Option<bool>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub m_WebGLUsePreBuiltUnityEngine: Option<bool>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_WebPlayerNaClSupport: Option<bool>,
    /// bool: (3.4.0 - 5.6.0b2)
    pub m_WebPlayerOfflineDeployment: Option<bool>,
    /// bool: (3.4.0 - 5.6.0b2)
    pub m_WebPlayerStreamed: Option<bool>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub m_WiiUEnableNetAPI: Option<bool>,
    /// String: (2019.3.0f4 - 2022.2.0b16)
    pub m_WindowsDevicePortalAddress: Option<String>,
    /// String: (2019.3.0f4 - 2022.2.0b16)
    pub m_WindowsDevicePortalUsername: Option<String>,
    /// bool: (2018.4.15f1 - 2018.4.15f1)
    pub m_WsaHolographicRemoting: Option<bool>,
    /// String: (5.6.0b2 - 2018.4.15f1)
    pub m_XboxOneNetworkSharePath: Option<String>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_XboxOneStreamingInstallLaunchChunkRange: Option<i32>,
    /// String: (5.6.0b2 - 2018.4.15f1)
    pub m_XboxOneUsername: Option<String>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_macosXcodeBuildConfig: Option<i32>,
}

/// EditorUserSettings is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorUserSettings {
    pub m_ConfigSettings: Vec<(String, ConfigSetting)>,
    pub m_SemanticMergeMode: i32,
    pub m_VCAutomaticAdd: bool,
    pub m_VCDebugCmd: bool,
    pub m_VCDebugCom: bool,
    pub m_VCDebugOut: bool,
    pub m_VCShowFailedCheckout: bool,
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub m_ArtifactGarbageCollection: Option<bool>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_DesiredImportWorkerCount: Option<i32>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_IdleImportWorkerShutdownDelay: Option<i32>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_StandbyImportWorkerCount: Option<i32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_VCAllowAsyncUpdate: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_VCHierarchyOverlayIcons: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_VCOtherOverlayIcons: Option<bool>,
    /// bool: (2019.3.0f4 - 2020.1.0a20)
    pub m_VCOverlayIcons: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_VCOverwriteFailedCheckoutAssets: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_VCProjectOverlayIcons: Option<bool>,
}

/// EffectConstant is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct EffectConstant {
    pub bypass: bool,
    pub groupConstantIndex: u32,
    pub parameterIndices: Vec<u32>,
    pub prevEffectIndex: u32,
    pub sendTargetEffectIndex: u32,
    pub wetMixLevelIndex: u32,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// EllipsoidParticleEmitter is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct EllipsoidParticleEmitter {
    pub angularVelocity: f32,
    pub emitterVelocityScale: f32,
    pub localVelocity: Vector3f,
    pub m_Ellipsoid: Vector3f,
    pub m_Emit: bool,
    pub m_Enabled: bool,
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_MinEmitterRange: f32,
    pub m_OneShot: bool,
    pub maxEmission: f32,
    pub maxEnergy: f32,
    pub maxSize: f32,
    pub minEmission: f32,
    pub minEnergy: f32,
    pub minSize: f32,
    pub rndAngularVelocity: f32,
    pub rndRotation: bool,
    pub rndVelocity: Vector3f,
    pub tangentVelocity: Vector3f,
    pub worldVelocity: Vector3f,
    /// bool: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "Simulate in Worldspace?")]
    pub Simulate_in_Worldspace_: Option<bool>,
}

/// EmbeddedNativeType is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddedNativeType {
    pub m_FloatArray: Vec<f32>,
    pub m_String: String,
}

/// EmissionModule is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.EmissionModule.html):
/**
Script interface for the EmissionModule of a Particle System.
See Also: ParticleSystem, ParticleSystem.emission.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct EmissionModule {
    /**Specifies whether the EmissionModule is enabled or disabled.*/
    pub enabled: bool,
    /**The current number of bursts.*/
    pub m_BurstCount: i32,
    /**The rate at which the emitter spawns new particles over distance.*/
    pub rateOverDistance: MinMaxCurve,
    /**The rate at which the emitter spawns new particles over time.*/
    pub rateOverTime: MinMaxCurve,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub cnt0: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub cnt1: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub cnt2: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub cnt3: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub cntmax0: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub cntmax1: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub cntmax2: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub cntmax3: Option<i32>,
    /// Vec<ParticleSystemEmissionBurst>: (2017.4.33f1 - 2022.2.0b16)
    pub m_Bursts: Option<Vec<ParticleSystemEmissionBurst>>,
    /// f32: (5.6.0b2 - 5.6.0b2)
    pub time0: Option<f32>,
    /// f32: (5.6.0b2 - 5.6.0b2)
    pub time1: Option<f32>,
    /// f32: (5.6.0b2 - 5.6.0b2)
    pub time2: Option<f32>,
    /// f32: (5.6.0b2 - 5.6.0b2)
    pub time3: Option<f32>,
}

/// EmptyObject is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyObject {}

/// EnlightenRendererInformation is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnlightenRendererInformation {
    pub dynamicLightmapSTInSystem: Vector4f,
    pub instanceHash: Hash128,
    pub renderer: PPtr, /*<Object>*/
    pub systemId: i32,
}

/// EnlightenSceneMapping is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnlightenSceneMapping {
    pub m_Probesets: Vec<Hash128>,
    pub m_Renderers: Vec<EnlightenRendererInformation>,
    pub m_SystemAtlases: Vec<EnlightenSystemAtlasInformation>,
    pub m_Systems: Vec<EnlightenSystemInformation>,
    pub m_TerrainChunks: Vec<EnlightenTerrainChunksInformation>,
}

/// EnlightenSystemAtlasInformation is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnlightenSystemAtlasInformation {
    pub atlasHash: Hash128,
    pub atlasSize: i32,
    pub firstSystemId: i32,
}

/// EnlightenSystemInformation is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnlightenSystemInformation {
    pub atlasIndex: i32,
    pub atlasOffsetX: i32,
    pub atlasOffsetY: i32,
    pub inputSystemHash: Hash128,
    pub radiositySystemHash: Hash128,
    pub rendererIndex: u32,
    pub rendererSize: u32,
}

/// EnlightenTerrainChunksInformation is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnlightenTerrainChunksInformation {
    pub firstSystemId: i32,
    pub numChunksInX: i32,
    pub numChunksInY: i32,
}

/// ExpandedData is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandedData {
    pub m_ClassID: i32,
    pub m_ExpandedProperties: Vec<String>,
    pub m_InspectorExpanded: bool,
    pub m_ScriptClass: String,
}

/// ExposedReferenceTable is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExposedReferenceTable {
    pub m_References: Vec<(String, PPtr /*<Object>*/)>,
}

/// Expression is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct Expression {
    pub op: i32,
    pub valueIndex: i32,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    #[serde(alias = "data[0]")]
    pub data_0_: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    #[serde(alias = "data[1]")]
    pub data_1_: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    #[serde(alias = "data[2]")]
    pub data_2_: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    #[serde(alias = "data[3]")]
    pub data_3_: Option<i32>,
}

/// ExternalForcesModule is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.ExternalForcesModule.html):
/**
Script interface for the ExternalForcesModule of a Particle System.
See Also: ParticleSystem, ParticleSystem.externalForces.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalForcesModule {
    /**Specifies whether the ExternalForcesModule is enabled or disabled.*/
    pub enabled: bool,
    /**Apply all Force Fields belonging to a matching Layer to this Particle System.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub influenceFilter: Option<i32>,
    /// Vec<PPtr/*<ParticleSystemForceField>*/>: (2018.4.15f1 - 2022.2.0b16)
    pub influenceList: Option<Vec<PPtr /*<ParticleSystemForceField>*/>>,
    /**Particle System Force Field Components with a matching Layer affect this Particle System.*/
    /// BitField: (2018.4.15f1 - 2022.2.0b16)
    pub influenceMask: Option<BitField>,
    /**Multiplies the magnitude of external forces affecting the particles.*/
    /// f32: (5.6.0b2 - 2018.4.15f1)
    pub multiplier: Option<f32>,
    /**Multiplies the magnitude of applied external forces.*/
    /// MinMaxCurve: (2019.3.0f4 - 2022.2.0b16)
    pub multiplierCurve: Option<MinMaxCurve>,
}

/// FastPropertyName is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct FastPropertyName {
    pub name: String,
}

/// FixedJoint is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/FixedJoint.html):
/**
The Fixed joint groups together 2 rigidbodies, making them stick together in their bound position.
See Also: CharacterJoint, HingeJoint, SpringJoint, ConfigurableJoint.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct FixedJoint {
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break. To be able to break, a joint must be _Locked_ or _Limited_ on the axis of rotation where the torque is being applied. This means that some joints cannot break, such as an unconstrained Configurable Joint.*/
    pub m_BreakTorque: f32,
    /**A reference to another rigidbody this joint connects to.*/
    pub m_ConnectedBody: PPtr, /*<Rigidbody>*/
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**A reference to an articulation body this joint connects to.*/
    /// PPtr/*<ArticulationBody>*/: (2020.3.42f1 - 2022.2.0b16)
    pub m_ConnectedArticulationBody: Option<PPtr /*<ArticulationBody>*/>,
    /**The scale to apply to the inverse mass and inertia tensor of the connected body prior to solving the constraints.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ConnectedMassScale: Option<f32>,
    /**Enable collision between bodies connected with the joint.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_EnableCollision: Option<bool>,
    /**Toggle preprocessing for this joint.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_EnablePreprocessing: Option<bool>,
    /**The scale to apply to the inverse mass and inertia tensor of the body prior to solving the constraints.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_MassScale: Option<f32>,
}

/// FixedJoint2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/FixedJoint2D.html):
/**
Connects two Rigidbody2D together at their anchor points using a configurable spring.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct FixedJoint2D {
    /**The joint's anchor point on the object that has the joint component.*/
    pub m_Anchor: Vector2f,
    /**Should the connectedAnchor be calculated automatically?*/
    pub m_AutoConfigureConnectedAnchor: bool,
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break.*/
    pub m_BreakTorque: f32,
    /**The joint's anchor point on the second object (ie, the one which doesn't have the joint component).*/
    pub m_ConnectedAnchor: Vector2f,
    pub m_ConnectedRigidBody: PPtr, /*<Rigidbody2D>*/
    /**The amount by which the spring force is reduced in proportion to the movement speed.*/
    pub m_DampingRatio: f32,
    /**Should the two rigid bodies connected with this joint collide with each other?*/
    pub m_EnableCollision: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The frequency at which the spring oscillates around the distance between the objects.*/
    pub m_Frequency: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_BreakAction: Option<i32>,
}

/// Flare is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Flare.html):
/**
A flare asset. Read more about flares in the components reference.
The flare class has no properties. It needs to be setup up in the inspector.

You can reference flares and assign them to a Light at runtime.See Also: Flare assets, LensFlare class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Flare {
    pub m_Elements: Vec<FlareElement>,
    pub m_FlareTexture: PPtr, /*<Texture>*/
    /**The name of the object.*/
    pub m_Name: String,
    pub m_TextureLayout: i32,
    pub m_UseFog: bool,
}

/// FlareElement is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct FlareElement {
    pub m_Color: ColorRGBA,
    pub m_Fade: bool,
    pub m_ImageIndex: u32,
    pub m_Position: f32,
    pub m_Rotate: bool,
    pub m_Size: f32,
    pub m_UseLightColor: bool,
    pub m_Zoom: bool,
}

/// FloatCurve is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct FloatCurve {
    pub attribute: String,
    pub classID: i32,
    pub curve: AnimationCurve,
    pub path: String,
    pub script: PPtr, /*<MonoScript>*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub flags: Option<i32>,
}

/// Font is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Font.html):
/**
Script interface for font assets.
You can use this class to dynamically switch fonts on Text Meshes.See Also: TextMesh.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Font {
    /**The ascent of the font.*/
    pub m_Ascent: f32,
    pub m_AsciiStartOffset: i32,
    pub m_CharacterRects: Vec<CharacterInfo>,
    pub m_ConvertCase: i32,
    pub m_DefaultMaterial: PPtr, /*<Material>*/
    pub m_DefaultStyle: u32,
    pub m_FontData: Vec<char>,
    pub m_FontNames: Vec<String>,
    /**The default size of the font.*/
    pub m_FontSize: f32,
    pub m_KerningValues: Vec<((u16, u16), f32)>,
    pub m_LineSpacing: f32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Texture: PPtr, /*<Texture>*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_CharacterPadding: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_CharacterSpacing: Option<i32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_Descent: Option<f32>,
    /// Vec<PPtr/*<Font>*/>: (5.6.0b2 - 2022.2.0b16)
    pub m_FallbackFonts: Option<Vec<PPtr /*<Font>*/>>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_FontCountX: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_FontCountY: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_FontRenderingMode: Option<i32>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_GridFont: Option<bool>,
    /// f32: (3.4.0 - 3.4.0)
    pub m_Kerning: Option<f32>,
    /// Vec<(i32, f32)>: (3.4.0 - 3.4.0)
    pub m_PerCharacterKerning: Option<Vec<(i32, f32)>>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_PixelScale: Option<f32>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_ShouldRoundAdvanceValue: Option<bool>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_Tracking: Option<f32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_UseLegacyBoundsCalculation: Option<bool>,
}

/// ForceModule is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ForceModule {
    pub enabled: bool,
    pub inWorldSpace: bool,
    pub randomizePerFrame: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
}

/// FrictionJoint2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/FrictionJoint2D.html):
/**
Applies both force and torque to reduce both the linear and angular velocities to zero.
The joint constantly tries to reduce both the ::Rigidbody2D::velocity and ::Rigidbody2D::angularVelocity to zero.  Unlike contact friction which requires two colliders to be in contact, force and torque here are applied continuously.You can control both the maximum force using maxForce and maximum torque using maxTorque.  Because you can use very high force or torque limits, you can essentially reduce an objects movement to almost zero.A typical usage for this joint might be to simulate top-down surface friction or to simulate stiff rotation of an object.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct FrictionJoint2D {
    /**The joint's anchor point on the object that has the joint component.*/
    pub m_Anchor: Vector2f,
    /**Should the connectedAnchor be calculated automatically?*/
    pub m_AutoConfigureConnectedAnchor: bool,
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break.*/
    pub m_BreakTorque: f32,
    /**The joint's anchor point on the second object (ie, the one which doesn't have the joint component).*/
    pub m_ConnectedAnchor: Vector2f,
    pub m_ConnectedRigidBody: PPtr, /*<Rigidbody2D>*/
    /**Should the two rigid bodies connected with this joint collide with each other?*/
    pub m_EnableCollision: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The maximum force that can be generated when trying to maintain the friction joint constraint.*/
    pub m_MaxForce: f32,
    /**The maximum torque that can be generated when trying to maintain the friction joint constraint.*/
    pub m_MaxTorque: f32,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_BreakAction: Option<i32>,
}

/// GISettings is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct GISettings {
    pub m_AlbedoBoost: f32,
    pub m_BounceScale: f32,
    pub m_EnableBakedLightmaps: bool,
    pub m_EnableRealtimeLightmaps: bool,
    pub m_EnvironmentLightingMode: u32,
    pub m_IndirectOutputScale: f32,
    /// f32: (5.6.0b2 - 2017.4.33f1)
    pub m_TemporalCoherenceThreshold: Option<f32>,
}

/// GLTextureSettings is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct GLTextureSettings {
    pub m_Aniso: i32,
    pub m_FilterMode: i32,
    pub m_MipBias: f32,
    /// i32: (3.4.0 - 5.6.0b2)
    pub m_WrapMode: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_WrapU: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_WrapV: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_WrapW: Option<i32>,
}

/// GUID is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct GUID {
    /// u32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "data[0]")]
    pub data_0_: Option<u32>,
    /// u32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "data[1]")]
    pub data_1_: Option<u32>,
    /// u32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "data[2]")]
    pub data_2_: Option<u32>,
    /// u32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "data[3]")]
    pub data_3_: Option<u32>,
}

/// GUIDSerializer is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct GUIDSerializer {
    pub guidToPath: Vec<(GUID, String)>,
}

/// GUIText is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct GUIText {
    pub m_Alignment: i16,
    pub m_Anchor: i16,
    pub m_Enabled: u8,
    pub m_Font: PPtr, /*<Font>*/
    pub m_FontSize: i32,
    pub m_FontStyle: i32,
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_LineSpacing: f32,
    pub m_Material: PPtr, /*<Material>*/
    pub m_PixelCorrect: bool,
    pub m_PixelOffset: Vector2f,
    pub m_TabSize: f32,
    pub m_Text: String,
    /// ColorRGBA: (5.6.0b2 - 2018.4.15f1)
    pub m_Color: Option<ColorRGBA>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub m_RichText: Option<bool>,
}

/// GUITexture is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct GUITexture {
    pub m_BottomBorder: i32,
    pub m_Color: ColorRGBA,
    pub m_Enabled: u8,
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_LeftBorder: i32,
    pub m_PixelInset: Rectf,
    pub m_RightBorder: i32,
    pub m_Texture: PPtr, /*<Texture>*/
    pub m_TopBorder: i32,
}

/// GameManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameManager {}

/// GameObject is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/GameObject.html):
/**
Base class for all entities in Unity Scenes.
Note: Many variables in the GameObject class have been removed. To access GameObject.renderer in csharp, for example, use GetComponent<Renderer>() instead.      See Also: Component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct GameObject {
    pub m_Component: Vec<Enum_i32__PPtr___Component______ComponentPair>,
    pub m_IsActive: Enum_u8__bool,
    /**The layer the GameObject is in.*/
    pub m_Layer: u32,
    /**The name of the object.*/
    pub m_Name: String,
    /**The tag of this GameObject.*/
    pub m_Tag: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_i32__PPtr___Component______ComponentPair {
    i32__PPtr___Component___((i32, PPtr /*<Component>*/)),
    ComponentPair(ComponentPair),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_u8__bool {
    u8(u8),
    bool(bool),
}

/// GameObjectRecorder is a  class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.GameObjectRecorder.html):
/**
Records the changing properties of a GameObject as the Scene runs and saves the information into an AnimationClip.
This class binds GameObject properties, records their values as they change in the running Scene, and saves the result in an AnimationClip. The recorded GameObject is called root in the class, and you can also bind the properties of any child of root.See the following code example on how this class can be implemented and to set what gets recorded.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct GameObjectRecorder {
    /**The name of the object.*/
    pub m_Name: String,
}

/// GenericBinding is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericBinding {
    pub attribute: u32,
    pub customType: u8,
    pub isPPtrCurve: u8,
    pub path: u32,
    pub script: PPtr, /*<Object>*/
    pub typeID: i32,
    /// u8: (2022.2.0b16 - 2022.2.0b16)
    pub isIntCurve: Option<u8>,
    /// u8: (2022.2.0b16 - 2022.2.0b16)
    pub isSerializeReferenceCurve: Option<u8>,
}

/// GlobalGameManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalGameManager {}

/// Google is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct Google {
    pub depthFormat: i32,
    pub enableTransitionView: bool,
}

/// Gradient is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Gradient.html):
/**
Represents a Gradient used for animating colors.
Gradients allow animating or interpolating colors by having several "color keys" and "alpha keys". Color keys and alpha keys are separate, and each key has a time specified for it, ranging from 0.0 (0%) to 1.0 (100%). Note that the alpha and colors keys will be automatically sorted by time value and that it is ensured to always have a minimum of 2 color keys and 2 alpha keys.How the colors are interpolated between the keys is controlled by GradientMode.Public Gradient variables used in scripts automatically display the gradient editor in the inspector window. GradientUsageAttribute allows specifying whether the gradient colors should be high dynamic range for editing.
See Also: GradientColorKey, GradientAlphaKey, SerializedProperty.gradientValue.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Gradient {
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub atime0: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub atime1: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub atime2: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub atime3: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub atime4: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub atime5: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub atime6: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub atime7: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub ctime0: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub ctime1: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub ctime2: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub ctime3: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub ctime4: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub ctime5: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub ctime6: Option<u16>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub ctime7: Option<u16>,
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub key0: Option<ColorRGBA>,
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub key1: Option<ColorRGBA>,
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub key2: Option<ColorRGBA>,
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub key3: Option<ColorRGBA>,
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub key4: Option<ColorRGBA>,
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub key5: Option<ColorRGBA>,
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub key6: Option<ColorRGBA>,
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub key7: Option<ColorRGBA>,
    /**Indicates the color space that the gradient color keys are using.*/
    /// i8: (2022.2.0b16 - 2022.2.0b16)
    pub m_ColorSpace: Option<i8>,
    /// ColorRGBA: (3.4.0 - 3.4.0)
    #[serde(alias = "m_Color[0]")]
    pub m_Color_0_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 3.4.0)
    #[serde(alias = "m_Color[1]")]
    pub m_Color_1_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 3.4.0)
    #[serde(alias = "m_Color[2]")]
    pub m_Color_2_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 3.4.0)
    #[serde(alias = "m_Color[3]")]
    pub m_Color_3_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 3.4.0)
    #[serde(alias = "m_Color[4]")]
    pub m_Color_4_: Option<ColorRGBA>,
    /**Controls how the gradient colors are interpolated.*/
    /// i32: (5.6.0b2 - 2021.2.16f1); u8: (2022.2.0b16 - 2022.2.0b16)
    pub m_Mode: Option<i32>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    pub m_NumAlphaKeys: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    pub m_NumColorKeys: Option<u8>,
}

/// GraphicsSettings is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rendering.GraphicsSettings.html):
/**
Script interface for Graphics Settings.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub m_AlwaysIncludedShaders: Vec<PPtr /*<Shader>*/>,
    pub m_Deferred: BuiltinShaderSettings,
    pub m_DeferredReflections: BuiltinShaderSettings,
    pub m_DepthNormals: BuiltinShaderSettings,
    pub m_LensFlare: BuiltinShaderSettings,
    pub m_LightHalo: BuiltinShaderSettings,
    pub m_LightsUseLinearIntensity: bool,
    pub m_MotionVectors: BuiltinShaderSettings,
    pub m_PreloadedShaders: Vec<PPtr /*<ShaderVariantCollection>*/>,
    pub m_ScreenSpaceShadows: BuiltinShaderSettings,
    pub m_SpritesDefaultMaterial: PPtr, /*<Material>*/
    pub m_TierSettings_Tier1: TierGraphicsSettings,
    pub m_TierSettings_Tier2: TierGraphicsSettings,
    pub m_TierSettings_Tier3: TierGraphicsSettings,
    pub m_TransparencySortAxis: Vector3f,
    pub m_TransparencySortMode: i32,
    /// bool: (2019.3.0f4 - 2019.3.0f4)
    pub m_AllowEnlightenSupportForUpgradedProject: Option<bool>,
    /// PPtr/*<MonoBehaviour>*/: (2017.4.33f1 - 2022.2.0b16)
    pub m_CustomRenderPipeline: Option<PPtr /*<MonoBehaviour>*/>,
    /// u32: (2020.3.42f1 - 2022.2.0b16)
    pub m_DefaultRenderingLayerMask: Option<u32>,
    /// BuiltinShaderSettings: (5.6.0b2 - 2021.2.16f1)
    pub m_LegacyDeferred: Option<BuiltinShaderSettings>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LightProbeOutsideHullStrategy: Option<i32>,
    /// bool: (5.6.0b2 - 5.6.0b2)
    pub m_LightsUseCCT: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_LightsUseColorTemperature: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_LogWhenShaderIsCompiled: Option<bool>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_PreloadShadersBatchTimeLimit: Option<i32>,
    /// Vec<(String, PPtr/*<Object>*/)>: (2021.2.16f1 - 2022.2.0b16)
    pub m_SRPDefaultSettings: Option<Vec<(String, PPtr /*<Object>*/)>>,
    /// Vec<PlatformShaderDefines>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ShaderDefinesPerShaderCompiler: Option<Vec<PlatformShaderDefines>>,
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub m_VideoShadersIncludeMode: Option<i32>,
}

/// Grid is a  class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Grid.html):
/**
Grid is the base class for plotting a layout of uniformly spaced points and lines.
The Grid component stores dimensional data of the layout of the grid and provides helper functions to retrieve information about the grid, such as the conversion between the cell location and local space location of items within the grid.The layout of the Grid component is in the XY plane with the origin of the grid always beginning at (0, 0) and the X and Y coordinates of the grid only as positive values.Implements the interface GridLayout.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Grid {
    /**The size of the gap between each cell in the layout.*/
    pub m_CellGap: Vector3f,
    /**The layout of the cells.*/
    pub m_CellLayout: i32,
    /**The size of each cell in the layout.*/
    pub m_CellSize: Vector3f,
    /**The cell swizzle for the layout.*/
    pub m_CellSwizzle: i32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
}

/// GroupConnection is a sub class of the Unity engine since version 2021.2.16f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupConnection {
    pub sendEffectIndex: u32,
    pub sourceGroupIndex: u32,
    pub targetGroupIndex: u32,
}

/// GroupConstant is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupConstant {
    pub bypassEffects: bool,
    pub mute: bool,
    pub parentConstantIndex: i32,
    pub pitchIndex: u32,
    pub solo: bool,
    pub volumeIndex: u32,
    /// u32: (2019.3.0f4 - 2020.1.0a20)
    pub sendIndex: Option<u32>,
}

/// Halo is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Halo {
    pub m_Color: ColorRGBA,
    pub m_Enabled: u8,
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_Size: f32,
}

/// HandPose is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct HandPose {
    pub m_CloseOpen: f32,
    pub m_DoFArray: Vec<f32>,
    pub m_Grab: f32,
    pub m_GrabX: xform,
    pub m_InOut: f32,
    pub m_Override: f32,
}

/// Hash128 is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Hash128.html):
/**
Represents  a 128-bit hash value.
Use Hash128 to uniquely identify a piece of data. A 128-bit hash value has an extremely
low probability of hash collisions, so you can assume that if the hash values of two pieces of data are identical,
then the data is identical too. For example, to quickly determine whether texture pixel contents have changed, or
if they are identical between several textures, you can use Texture.imageContentsHash.To compute the hash values for some data, use the Hash128.Compute function. To compute the hash
values incrementally for several pieces of data, use Hash128.Append.
The hash algorithm used to compute Hash128 values is SpookyHash V2. Note that while this hash algorithm is quite fast to compute and has good hash distribution qualities, it is not a cryptographic hash function.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Hash128 {
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[0]")]
    pub bytes_0_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[10]")]
    pub bytes_10_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[11]")]
    pub bytes_11_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[12]")]
    pub bytes_12_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[13]")]
    pub bytes_13_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[14]")]
    pub bytes_14_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[15]")]
    pub bytes_15_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[1]")]
    pub bytes_1_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[2]")]
    pub bytes_2_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[3]")]
    pub bytes_3_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[4]")]
    pub bytes_4_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[5]")]
    pub bytes_5_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[6]")]
    pub bytes_6_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[7]")]
    pub bytes_7_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[8]")]
    pub bytes_8_: Option<u8>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "bytes[9]")]
    pub bytes_9_: Option<u8>,
}

/// HeightMeshBVNode is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct HeightMeshBVNode {
    pub i: i32,
    pub max: Vector3f,
    pub min: Vector3f,
    pub n: i32,
}

/// HeightMeshData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct HeightMeshData {
    pub m_Bounds: AABB,
    pub m_Indices: Vec<i32>,
    pub m_Nodes: Vec<HeightMeshBVNode>,
    pub m_Vertices: Vec<Vector3f>,
}

/// Heightmap is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Heightmap {
    pub m_Heights: Vec<i16>,
    pub m_Levels: i32,
    pub m_MinMaxPatchHeights: Vec<f32>,
    pub m_PrecomputedError: Vec<f32>,
    pub m_Scale: Vector3f,
    /// PPtr/*<PhysicMaterial>*/: (3.4.0 - 3.4.0)
    pub m_DefaultPhysicMaterial: Option<PPtr /*<PhysicMaterial>*/>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_EnableHolesTextureCompression: Option<bool>,
    /// i32: (3.4.0 - 2018.4.15f1)
    pub m_Height: Option<i32>,
    /// Vec<u8>: (2019.3.0f4 - 2022.2.0b16)
    pub m_Holes: Option<Vec<u8>>,
    /// Vec<u8>: (2019.3.0f4 - 2022.2.0b16)
    pub m_HolesLOD: Option<Vec<u8>>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_Resolution: Option<i32>,
    /// f32: (5.6.0b2 - 2018.4.15f1)
    pub m_Thickness: Option<f32>,
    /// i32: (3.4.0 - 2018.4.15f1)
    pub m_Width: Option<i32>,
}

/// HeightmapData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct HeightmapData {
    pub terrainData: PPtr, /*<Object>*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub isRotated: Option<bool>,
    /// Vector3f: (5.6.0b2 - 2021.2.16f1)
    pub position: Option<Vector3f>,
    /// Matrix4x4f: (2022.2.0b16 - 2022.2.0b16)
    pub surfaceToTerrain: Option<Matrix4x4f>,
}

/// HierarchicalSceneData is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct HierarchicalSceneData {
    pub m_SceneGUID: GUID,
}

/// HierarchyState is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct HierarchyState {
    pub expanded: Vec<PPtr /*<Object>*/>,
    #[serde(alias = "scrollposition.x")]
    pub scrollposition_x: f32,
    #[serde(alias = "scrollposition.y")]
    pub scrollposition_y: f32,
    pub selection: Vec<PPtr /*<Object>*/>,
}

/// HingeJoint is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/HingeJoint.html):
/**
The HingeJoint groups together 2 rigid bodies, constraining them to move like connected by a hinge.
This joint is great for, well, doors, but can also be used to model chains, etc...The HingeJoint has a motor which can be used to make the hinge spin around the joints axis.

A spring which attempts to reach for a target angle by spinning around the joints axis.

And a limit which constrains the joint angle.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct HingeJoint {
    /**The Position of the anchor around which the joints motion is constrained.*/
    pub m_Anchor: Vector3f,
    /**The Direction of the axis around which the body is constrained.*/
    pub m_Axis: Vector3f,
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break. To be able to break, a joint must be _Locked_ or _Limited_ on the axis of rotation where the torque is being applied. This means that some joints cannot break, such as an unconstrained Configurable Joint.*/
    pub m_BreakTorque: f32,
    /**A reference to another rigidbody this joint connects to.*/
    pub m_ConnectedBody: PPtr, /*<Rigidbody>*/
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Limit of angular rotation (in degrees) on the hinge joint.*/
    pub m_Limits: JointLimits,
    /**The motor will apply a force up to a maximum force to achieve the target velocity in degrees per second.*/
    pub m_Motor: JointMotor,
    /**The spring attempts to reach a target angle by adding spring and damping forces.*/
    pub m_Spring: JointSpring,
    /**Enables the joint's limits. Disabled by default.*/
    pub m_UseLimits: bool,
    /**Enables the joint's motor. Disabled by default.*/
    pub m_UseMotor: bool,
    /**Enables the joint's spring. Disabled by default.*/
    pub m_UseSpring: bool,
    /**Should the connectedAnchor be calculated automatically?*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    /**Position of the anchor relative to the connected Rigidbody.*/
    /// Vector3f: (5.6.0b2 - 2022.2.0b16)
    pub m_ConnectedAnchor: Option<Vector3f>,
    /**A reference to an articulation body this joint connects to.*/
    /// PPtr/*<ArticulationBody>*/: (2020.3.42f1 - 2022.2.0b16)
    pub m_ConnectedArticulationBody: Option<PPtr /*<ArticulationBody>*/>,
    /**The scale to apply to the inverse mass and inertia tensor of the connected body prior to solving the constraints.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ConnectedMassScale: Option<f32>,
    /**Enable collision between bodies connected with the joint.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_EnableCollision: Option<bool>,
    /**Toggle preprocessing for this joint.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_EnablePreprocessing: Option<bool>,
    /**If enabled, the angle of the hinge is extended to [-360, 360] degrees.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExtendedLimits: Option<bool>,
    /**The scale to apply to the inverse mass and inertia tensor of the body prior to solving the constraints.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_MassScale: Option<f32>,
    /**Defines whether the HingeJoint.spring outputs accelerations instead of forces.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_UseAcceleration: Option<bool>,
}

/// HingeJoint2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/HingeJoint2D.html):
/**
Joint that allows a Rigidbody2D object to rotate around a point in space or a point on another object.
See Also: DistanceJoint2D, SliderJoint2D, SpringJoint2D, JointAngleLimits2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct HingeJoint2D {
    /**The joint's anchor point on the object that has the joint component.*/
    pub m_Anchor: Vector2f,
    pub m_AngleLimits: JointAngleLimits2D,
    /**Should the connectedAnchor be calculated automatically?*/
    pub m_AutoConfigureConnectedAnchor: bool,
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break.*/
    pub m_BreakTorque: f32,
    /**The joint's anchor point on the second object (ie, the one which doesn't have the joint component).*/
    pub m_ConnectedAnchor: Vector2f,
    pub m_ConnectedRigidBody: PPtr, /*<Rigidbody2D>*/
    /**Should the two rigid bodies connected with this joint collide with each other?*/
    pub m_EnableCollision: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Parameters for the motor force applied to the joint.*/
    pub m_Motor: JointMotor2D,
    /**Should limits be placed on the range of rotation?*/
    pub m_UseLimits: bool,
    /**Should the joint be rotated automatically by a motor torque?*/
    pub m_UseMotor: bool,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_BreakAction: Option<i32>,
}

/// HoloLens is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct HoloLens {
    pub depthBufferSharingEnabled: bool,
    pub depthFormat: i32,
}

/// HumanBone is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/HumanBone.html):
/**
The mapping between a bone in the model and the conceptual bone in the Mecanim human anatomy.
The names of the Mecanim human bone and the bone in the model are stored along with the limiting muscle values that constrain the bone's rotation during animation.
See Also: HumanDescription, AvatarBuilder.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct HumanBone {
    /**The name of the bone to which the Mecanim human bone is mapped.*/
    pub m_BoneName: String,
    /**The name of the Mecanim human bone to which the bone from the model is mapped.*/
    pub m_HumanName: String,
    /**The rotation limits that define the muscle for this bone.*/
    pub m_Limit: SkeletonBoneLimit,
}

/// HumanDescription is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/HumanDescription.html):
/**
Class that holds humanoid avatar parameters to pass to the AvatarBuilder.BuildHumanAvatar function.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct HumanDescription {
    /**Amount by which the arm's length is allowed to stretch when using IK.*/
    pub m_ArmStretch: f32,
    pub m_ArmTwist: f32,
    /**Modification to the minimum distance between the feet of a humanoid model.*/
    pub m_FeetSpacing: f32,
    pub m_ForeArmTwist: f32,
    pub m_HasExtraRoot: bool,
    /**True for any human that has a translation Degree of Freedom (DoF). It is set to false by default.*/
    pub m_HasTranslationDoF: bool,
    /**Mapping between Mecanim bone names and bone names in the rig.*/
    pub m_Human: Vec<HumanBone>,
    /**Amount by which the leg's length is allowed to stretch when using IK.*/
    pub m_LegStretch: f32,
    pub m_LegTwist: f32,
    pub m_RootMotionBoneName: String,
    /**List of bone Transforms to include in the model.*/
    pub m_Skeleton: Vec<SkeletonBone>,
    pub m_SkeletonHasParents: bool,
    /**Defines how the upper leg's roll/twisting is distributed between the thigh and knee joints.*/
    pub m_UpperLegTwist: f32,
    /// f32: (2019.3.0f4 - 2022.2.0b16)
    pub m_GlobalScale: Option<f32>,
    /// Quaternionf: (5.6.0b2 - 2017.4.33f1)
    pub m_RootMotionBoneRotation: Option<Quaternionf>,
}

/// HumanGoal is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct HumanGoal {
    pub m_HintT: float3,
    pub m_HintWeightT: f32,
    pub m_WeightR: f32,
    pub m_WeightT: f32,
    pub m_X: xform,
}

/// HumanPose is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/HumanPose.html):
/**
Retargetable humanoid pose.
Represents a humanoid pose that is completely abstracted from any skeleton rig.
See Also: HumanPoseHandler.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct HumanPose {
    pub m_DoFArray: Vec<f32>,
    pub m_GoalArray: Vec<HumanGoal>,
    pub m_LeftHandPose: HandPose,
    pub m_LookAtPosition: float3,
    pub m_LookAtWeight: float4,
    pub m_RightHandPose: HandPose,
    pub m_RootX: xform,
    pub m_TDoFArray: Vec<float3>,
}

/// HumanTemplate is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct HumanTemplate {
    pub m_BoneTemplate: Vec<(String, String)>,
    pub m_Name: String,
}

/// IHVImageFormatImporter is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/IHVImageFormatImporter.html):
/**
Use IHVImageFormatImporter to modify Texture2D import settings for Textures in IHV (Independent Hardware Vendor) formats such as .DDS and .PVR from Editor scripts.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct IHVImageFormatImporter {
    /**Get or set the AssetBundle name.*/
    pub m_AssetBundleName: String,
    /**Get or set the AssetBundle variant.*/
    pub m_AssetBundleVariant: String,
    /**Is texture data readable from scripts.*/
    pub m_IsReadable: bool,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_TextureSettings: GLTextureSettings,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /**Enable if the texture should ignore any texture mipmap limit settings set in the Project Settings.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_IgnoreMipmapLimit: Option<bool>,
    /**Name of the texture mipmap limit group to which this texture belongs.*/
    /// String: (2022.2.0b16 - 2022.2.0b16)
    pub m_MipmapLimitGroupName: Option<String>,
    /**Enable mipmap streaming for this texture.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_StreamingMipmaps: Option<bool>,
    /**Relative priority for this texture when reducing memory size in order to hit the memory budget.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_StreamingMipmapsPriority: Option<i32>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_sRGBTexture: Option<bool>,
}

/// Image is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/UIElements.Image.html):
/**
A VisualElement representing a source texture.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub m_Format: i64,
    pub m_Height: i32,
    pub m_RowBytes: i32,
    pub m_Width: i32,
    /// Vec<u8>: (3.4.0 - 2020.1.0a20)
    #[serde(alias = "image data")]
    pub image_data: Option<Vec<u8>>,
}

/// ImportLog is a  class of the Unity engine since version 2022.2.0b16.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AssetImporters.ImportLog.html):
/**
Container class that holds the collection of logs generated by an importer during the import process.
See Also: AssetImportContext.LogImportError, AssetImportContext.LogImportWarning.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportLog {
    pub m_Logs: Vec<ImportLog_ImportLogEntry>,
    /**The name of the object.*/
    pub m_Name: String,
}

/// ImportLog_ImportLogEntry is a sub class of the Unity engine since version 2022.2.0b16.
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportLog_ImportLogEntry {
    pub file: String,
    pub line: i32,
    pub message: String,
    pub mode: i32,
    pub object: PPtr, /*<Object>*/
}

/// InheritVelocityModule is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.InheritVelocityModule.html):
/**
The Inherit Velocity Module controls how the velocity of the emitter is transferred to the particles as they are emitted.
NOTE: The inherit velocity module only has an effect if the Particle System is set to simulate in world space. If the system is simulating in local space, this module is ignored.

See Also: ParticleSystem, ParticleSystem.inheritVelocity.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct InheritVelocityModule {
    /**Specifies whether the InheritVelocityModule is enabled or disabled.*/
    pub enabled: bool,
    /**Curve to define how much of the emitter velocity the system applies during the lifetime of a particle.*/
    pub m_Curve: MinMaxCurve,
    /**Specifies how to apply emitter velocity to particles.*/
    pub m_Mode: i32,
}

/// InitialModule is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct InitialModule {
    pub enabled: bool,
    pub gravityModifier: MinMaxCurve,
    pub maxNumParticles: i32,
    pub randomizeRotationDirection: f32,
    pub rotation3D: bool,
    pub size3D: bool,
    pub startColor: MinMaxGradient,
    pub startLifetime: MinMaxCurve,
    pub startRotation: MinMaxCurve,
    pub startRotationX: MinMaxCurve,
    pub startRotationY: MinMaxCurve,
    pub startSize: MinMaxCurve,
    pub startSizeY: MinMaxCurve,
    pub startSizeZ: MinMaxCurve,
    pub startSpeed: MinMaxCurve,
    /// Vector3f: (2021.2.16f1 - 2022.2.0b16)
    pub customEmitterVelocity: Option<Vector3f>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub gravitySource: Option<i32>,
}

/// InputAxis is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct InputAxis {
    pub altNegativeButton: String,
    pub altPositiveButton: String,
    pub axis: i32,
    pub dead: f32,
    pub descriptiveName: String,
    pub descriptiveNegativeName: String,
    pub gravity: f32,
    pub invert: bool,
    pub joyNum: i32,
    pub m_Name: String,
    pub negativeButton: String,
    pub positiveButton: String,
    pub sensitivity: f32,
    pub snap: bool,
    /// i32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// InputImportSettings is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct InputImportSettings {
    pub name: String,
    pub value: SubstanceValue,
}

/// InputManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct InputManager {
    pub m_Axes: Vec<InputAxis>,
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub m_UsePhysicalKeys: Option<bool>,
}

/// InspectorExpandedState is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct InspectorExpandedState {
    pub m_ExpandedData: Vec<ExpandedData>,
}

/// IntPoint is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct IntPoint {
    pub X: i64,
    pub Y: i64,
}

/// InteractiveCloth is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct InteractiveCloth {
    pub m_AttachedColliders: Vec<ClothAttachment>,
    pub m_AttachmentResponse: f32,
    pub m_AttachmentTearFactor: f32,
    pub m_BendingStiffness: f32,
    pub m_CollisionResponse: f32,
    pub m_Damping: f32,
    pub m_Density: f32,
    pub m_Enabled: u8,
    pub m_ExternalAcceleration: Vector3f,
    pub m_Friction: f32,
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_Mesh: PPtr,       /*<Mesh>*/
    pub m_Pressure: f32,
    pub m_RandomAcceleration: Vector3f,
    pub m_SelfCollision: bool,
    pub m_StretchingStiffness: f32,
    pub m_TearFactor: f32,
    pub m_Thickness: f32,
    pub m_UseGravity: bool,
}

/// Item is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Progress.Item.html):
/**
A data structure that provides information about a progress indicator.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub markedForRemoval: bool,
    /// i32: (3.4.0 - 5.6.0b2)
    #[serde(alias = "(int&)downloadResolution")]
    pub downloadResolution: Option<i32>,
    /// i32: (3.4.0 - 5.6.0b2)
    #[serde(alias = "(int&)nameConflictResolution")]
    pub nameConflictResolution: Option<i32>,
}

/// JointAngleLimits2D is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointAngleLimits2D.html):
/**
Angular limits on the rotation of a Rigidbody2D object around a HingeJoint2D.
See Also: Rigidbody2D class, HingeJoint2D class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct JointAngleLimits2D {
    pub m_LowerAngle: f32,
    pub m_UpperAngle: f32,
}

/// JointDrive is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointDrive.html):
/**
How the joint's movement will behave along its local X axis.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct JointDrive {
    /**Amount of force applied to push the object toward the defined direction.*/
    pub maximumForce: f32,
    /**Resistance strength against the Position Spring. Only used if mode includes Position.*/
    pub positionDamper: f32,
    /**Strength of a rubber-band pull toward the defined direction. Only used if mode includes Position.*/
    pub positionSpring: f32,
    /// i32: (3.4.0 - 3.4.0)
    pub mode: Option<i32>,
    /**Defines whether the drive is an acceleration drive or a force drive.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub useAcceleration: Option<i32>,
}

/// JointLimits is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointLimits.html):
/**
JointLimits is used by the HingeJoint to limit the joints angle.
See Also: HingeJoint.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct JointLimits {
    /**The upper angular limit (in degrees) of the joint.*/
    pub max: f32,
    /**The lower angular limit (in degrees) of the joint.*/
    pub min: f32,
    /**The minimum impact velocity which will cause the joint to bounce.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub bounceMinVelocity: Option<f32>,
    /**Determines the size of the bounce when the joint hits it's limit. Also known as restitution.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub bounciness: Option<f32>,
    /**Distance inside the limit value at which the limit will be considered to be active by the solver.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub contactDistance: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    pub maxBounce: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    pub minBounce: Option<f32>,
}

/// JointMotor is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointMotor.html):
/**
The JointMotor is used to motorize a joint.
For example the HingeJoint can be told to rotate at a given speed and force.
The joint will then attempt to reach the velocity with the given maximum force.
See Also: HingeJoint.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct JointMotor {
    /**The motor will apply a force.*/
    pub force: f32,
    /**If freeSpin is enabled the motor will only accelerate but never slow down.*/
    pub freeSpin: i32,
    /**The motor will apply a force up to force to achieve targetVelocity.*/
    pub targetVelocity: f32,
}

/// JointMotor2D is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointMotor2D.html):
/**
Parameters for the optional motor force applied to a Joint2D.
See Also: HingeJoint2D class, SliderJoint2D class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct JointMotor2D {
    pub m_MaximumMotorForce: f32,
    /**The desired speed for the Rigidbody2D to reach as it moves with the joint.*/
    pub m_MotorSpeed: f32,
}

/// JointSpring is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointSpring.html):
/**
JointSpring is used add a spring force to HingeJoint and PhysicMaterial.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct JointSpring {
    /**The damper force uses to dampen the spring.*/
    pub damper: f32,
    /**The spring forces used to reach the target position.*/
    pub spring: f32,
    /**The target position the joint attempts to reach.*/
    pub targetPosition: f32,
}

/// JointSuspension2D is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointSuspension2D.html):
/**
Joint suspension is used to define how suspension works on a WheelJoint2D.
See Also: WheelJoint2D class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct JointSuspension2D {
    /**The world angle (in degrees) along which the suspension will move.*/
    pub m_Angle: f32,
    /**The amount by which the suspension spring force is reduced in proportion to the movement speed.*/
    pub m_DampingRatio: f32,
    /**The frequency at which the suspension spring oscillates.*/
    pub m_Frequency: f32,
}

/// JointTranslationLimits2D is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointTranslationLimits2D.html):
/**
Motion limits of a Rigidbody2D object along a SliderJoint2D.
See Also: Rigidbody2D class, SliderJoint2D class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct JointTranslationLimits2D {
    pub m_LowerTranslation: f32,
    pub m_UpperTranslation: f32,
}

/// Keyframe is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Keyframe.html):
/**
A single keyframe that can be injected into an animation curve.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Keyframe {
    pub inSlope: Quaternionf,
    pub outSlope: Quaternionf,
    /**The time of the keyframe.*/
    pub time: f32,
    /**The value of the curve at keyframe.*/
    pub value: Quaternionf,
    /**Sets the incoming weight for this key. The incoming weight affects the slope of the curve from the previous key to this key.*/
    /// Quaternionf: (2018.4.15f1 - 2022.2.0b16)
    pub inWeight: Option<Quaternionf>,
    /**Sets the outgoing weight for this key. The outgoing weight affects the slope of the curve from this key to the next key.*/
    /// Quaternionf: (2018.4.15f1 - 2022.2.0b16)
    pub outWeight: Option<Quaternionf>,
    /**Weighted mode for the keyframe.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub weightedMode: Option<i32>,
}

/// LOD is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LOD.html):
/**
Structure for building a LOD for passing to the SetLODs function.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LOD {
    /**Width of the cross-fade transition zone (proportion to the current LOD's whole length) [0-1]. Only used if it's not animated.*/
    pub fadeTransitionWidth: f32,
    /**List of renderers for this LOD level.*/
    pub renderers: Vec<LODRenderer>,
    pub screenRelativeHeight: f32,
}

/// LODGroup is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LODGroup.html):
/**
LODGroup lets you group multiple Renderers into LOD levels.
This can be used to switch between different LOD levels at runtime based on size on screen.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LODGroup {
    /**Specify if the cross-fading should be animated by time. The animation duration is specified globally as crossFadeAnimationDuration.*/
    pub m_AnimateCrossFading: bool,
    /**Allows you to enable or disable the LODGroup.*/
    pub m_Enabled: bool,
    /**The LOD fade mode used.*/
    pub m_FadeMode: i32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_LODs: Vec<LOD>,
    /**The local reference point against which the LOD distance is calculated.*/
    pub m_LocalReferencePoint: Vector3f,
    /**The size of the LOD object in local space.*/
    pub m_Size: f32,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_LastLODIsBillboard: Option<bool>,
}

/// LODRenderer is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct LODRenderer {
    pub renderer: PPtr, /*<Renderer>*/
}

/// LayoutDataOne is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutDataOne {
    pub m_FloatArray: Vec<f32>,
}

/// LayoutDataThree is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutDataThree {
    pub m_AnotherFloatArray: Vec<f32>,
}

/// LayoutDataTwo is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutDataTwo {
    pub m_FloatValue: f32,
    pub m_IntegerValue: i32,
}

/// LensFlare is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LensFlare.html):
/**
Script interface for a Lens flare component.
This allows you to change the brightness and color of lens flares at runtime.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LensFlare {
    /**The strength of the flare.*/
    pub m_Brightness: f32,
    /**The color of the flare.*/
    pub m_Color: ColorRGBA,
    pub m_Directional: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The flare asset to use.*/
    pub m_Flare: PPtr, /*<Flare>*/
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_IgnoreLayers: BitField,
    /**The fade speed of the flare.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_FadeSpeed: Option<f32>,
}

/// LibraryAssetImporter is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryAssetImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_Name: String,
    pub m_UserData: String,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// LibraryRepresentation is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryRepresentation {
    pub name: String,
    pub scriptClassName: String,
    pub thumbnail: Image,
    pub thumbnailClassID: i32,
    /// u16: (5.6.0b2 - 2020.1.0a20)
    pub flags: Option<u16>,
    /// GUID: (5.6.0b2 - 2020.1.0a20)
    pub guid: Option<GUID>,
    /// i64: (5.6.0b2 - 2020.1.0a20)
    pub localIdentifier: Option<i64>,
    /// PPtr/*<EditorExtension>*/: (3.4.0 - 3.4.0)
    pub object: Option<PPtr /*<EditorExtension>*/>,
    /// String: (5.6.0b2 - 2020.1.0a20)
    pub path: Option<String>,
}

/// LifetimeByEmitterSpeedModule is a sub class of the Unity engine since version 2020.1.0a20.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.LifetimeByEmitterSpeedModule.html):
/**
The Lifetime By Emitter Speed Module controls the initial lifetime of each particle based on the speed of the emitter when the particle was spawned.
This module multiplies the start lifetime of particles with a value that depends on the speed of the object that spawned them. For most Particle Systems, this is the GameObject velocity, but for sub-emitters, the velocity comes from the parent particle that the sub-emitter particle originated from.See Also: ParticleSystem, ParticleSystem.MainModule.startLifetime.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LifetimeByEmitterSpeedModule {
    /**Use this property to enable or disable the LifetimeByEmitterSpeed module.*/
    pub enabled: bool,
    /**Use this curve to define which value to multiply the start lifetime of a particle with, based on the speed of the emitter when the particle is spawned.*/
    pub m_Curve: MinMaxCurve,
    /**Control the start lifetime multiplier between these minimum and maximum speeds of the emitter.*/
    pub m_Range: Vector2f,
}

/// Light is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Light.html):
/**
Script interface for light components.
Use this to control all aspects of Unity's lights. The properties are an exact match for the

values shown in the Inspector.Usually lights are just created in the editor but sometimes you want to create a light from a script:
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    /**The color of the light.*/
    pub m_Color: ColorRGBA,
    /**The cookie texture projected by the light.*/
    pub m_Cookie: PPtr, /*<Texture>*/
    /**The size of a directional light's cookie.*/
    pub m_CookieSize: f32,
    /**This is used to light certain objects in the Scene selectively.*/
    pub m_CullingMask: BitField,
    pub m_DrawHalo: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The flare asset to use for this light.*/
    pub m_Flare: PPtr, /*<Flare>*/
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The Intensity of a light is multiplied with the Light color.*/
    pub m_Intensity: f32,
    pub m_Lightmapping: i32,
    /**The range of the light.*/
    pub m_Range: f32,
    /**How to render the light.*/
    pub m_RenderMode: i32,
    /**How this light casts shadows*/
    pub m_Shadows: ShadowSettings,
    /**The angle of the light's spotlight cone in degrees.*/
    pub m_SpotAngle: f32,
    /**The type of the light.*/
    pub m_Type: i32,
    /// bool: (3.4.0 - 3.4.0)
    pub m_ActuallyLightmapped: Option<bool>,
    /**The size of the area light (Editor only).*/
    /// Vector2f: (5.6.0b2 - 2022.2.0b16)
    pub m_AreaSize: Option<Vector2f>,
    /**This property describes the output of the last Global Illumination bake.*/
    /// LightBakingOutput: (5.6.0b2 - 2022.2.0b16)
    pub m_BakingOutput: Option<LightBakingOutput>,
    /**The multiplier that defines the strength of the bounce lighting.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_BounceIntensity: Option<f32>,
    /**Bounding sphere used to override the regular light bounding sphere during culling.*/
    /// Vector4f: (2019.3.0f4 - 2022.2.0b16)
    pub m_BoundingSphereOverride: Option<Vector4f>,
    /// f32: (5.6.0b2 - 5.6.0b2)
    pub m_CCT: Option<f32>,
    /**The color temperature of the light.          Correlated Color Temperature (abbreviated as CCT) is multiplied with the color filter when calculating the final color of a light source. The color temperature of the electromagnetic radiation emitted from an ideal black body is defined as its surface temperature in Kelvin. White is 6500K according to the D65 standard. A candle light is 1800K and a soft warm light bulb is 2700K.          If you want to use colorTemperature, GraphicsSettings.lightsUseLinearIntensity and Light.useColorTemperature has to be enabled.          See Also: GraphicsSettings.lightsUseLinearIntensity, GraphicsSettings.useColorTemperature.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ColorTemperature: Option<f32>,
    /**The angle of the light's spotlight inner cone in degrees.*/
    /// f32: (2019.3.0f4 - 2022.2.0b16)
    pub m_InnerSpotAngle: Option<f32>,
    /**Allows you to override the global Shadowmask Mode per light. Only use this with render pipelines that can handle per light Shadowmask modes. Incompatible with the legacy renderers.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_LightShadowCasterMode: Option<i32>,
    /**Determines which rendering LayerMask this Light affects.*/
    /// u32: (2019.3.0f4 - 2022.2.0b16)
    pub m_RenderingLayerMask: Option<u32>,
    /**This property describes the shape of the spot light. Only Scriptable Render Pipelines use this property; the built-in renderer does not support it.*/
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_Shape: Option<i32>,
    /**Set to true to override light bounding sphere for culling.*/
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_UseBoundingSphereOverride: Option<bool>,
    /**Set to true to use the color temperature.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_UseColorTemperature: Option<bool>,
    /**Whether to cull shadows for this Light when the Light is outside of the view frustum.*/
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_UseViewFrustumForShadowCasterCull: Option<bool>,
}

/// LightBakingOutput is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightBakingOutput.html):
/**
Struct describing the result of a Global Illumination bake for a given light.
The example below demonstrates how you can check the baked status of a light and change its active state.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightBakingOutput {
    /**In case of a LightmapBakeType.Mixed light, contains the index of the light as seen from the occlusion probes point of view if any, otherwise -1.*/
    pub probeOcclusionLightIndex: i32,
    /**Is the light contribution already stored in lightmaps and/or lightprobes?*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub isBaked: Option<bool>,
    /// LightmapBakeMode: (2017.4.33f1 - 2022.2.0b16)
    pub lightmapBakeMode: Option<LightmapBakeMode>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub lightmappingMask: Option<i32>,
    /**In case of a LightmapBakeType.Mixed light, contains the index of the occlusion mask channel to use if any, otherwise -1.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub occlusionMaskChannel: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub shadowMaskChannel: Option<i32>,
}

/// LightProbeData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct LightProbeData {
    pub m_NonTetrahedralizedProbeSetIndexMap: Vec<(Hash128, i32)>,
    pub m_Positions: Vec<Vector3f>,
    pub m_ProbeSets: Vec<ProbeSetIndex>,
    pub m_Tetrahedralization: ProbeSetTetrahedralization,
}

/// LightProbeGroup is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightProbeGroup.html):
/**
Light Probe Group.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightProbeGroup {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
}

/// LightProbeOcclusion is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct LightProbeOcclusion {
    pub m_Occlusion: Vec<f32>,
    pub m_ProbeOcclusionLightIndex: Vec<i32>,
    /// Vec<i8>: (2017.4.33f1 - 2022.2.0b16)
    pub m_OcclusionMaskChannel: Option<Vec<i8>>,
    /// Vec<i8>: (5.6.0b2 - 5.6.0b2)
    pub m_ShadowMaskChannel: Option<Vec<i8>>,
}

/// LightProbeProxyVolume is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightProbeProxyVolume.html):
/**
The Light Probe Proxy Volume component offers the possibility to use higher resolution lighting for large non-static GameObjects.
By default, a probe-lit Renderer receives lighting from a single Light Probe that is interpolated from the surrounding Light Probes in the Scene. Because of this, GameObjects have constant ambient lighting regardless of their position on the surface. The light has have a rotational gradient because it's using spherical harmonics, but it lacks a spatial gradient. This is more noticeable on larger GameObjects and Particle Systems. The lighting across the GameObject matches the lighting at the anchor point, and if the GameObject straddles a lighting gradient, parts of the GameObject will look incorrect.This component will generate a 3D grid of interpolated Light Probes inside a bounding volume. The resolution of the grid can be user-specified. The spherical harmonics coefficients of the interpolated Light Probes are updated into 3D textures, which are sampled at render time to compute the contribution to the diffuse ambient lighting. This adds a spatial gradient to probe-lit GameObjects.See Also: Light Probes.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightProbeProxyVolume {
    /**The bounding box mode for generating the 3D grid of interpolated Light Probes.*/
    pub m_BoundingBoxMode: i32,
    pub m_BoundingBoxOrigin: Vector3f,
    pub m_BoundingBoxSize: Vector3f,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The mode in which the interpolated Light Probe positions are generated.*/
    pub m_ProbePositionMode: i32,
    /**Sets the way the Light Probe Proxy Volume refreshes.*/
    pub m_RefreshMode: i32,
    /**The resolution mode for generating the grid of interpolated Light Probes.*/
    pub m_ResolutionMode: i32,
    pub m_ResolutionProbesPerUnit: f32,
    pub m_ResolutionX: u32,
    pub m_ResolutionY: u32,
    pub m_ResolutionZ: u32,
    /**The texture data format used by the Light Probe Proxy Volume 3D texture.*/
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_DataFormat: Option<i32>,
    /**Determines how many Spherical Harmonics bands will be evaluated to compute the ambient color.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_QualityMode: Option<i32>,
}

/// LightProbes is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightProbes.html):
/**
Stores light probe data for all currently loaded Scenes.
The data includes: probe positions, Spherical Harmonics (SH) coefficients and the tetrahedral tessellation.You can modify the coefficients and update the tetrahedral tessellation at runtime. You can also swap the entire LightProbes object for a different pre-baked one using LightmapSettings.lightProbes.See Also: Light Probes in the Unity Manual, LightmapSettings, ReceiveGI.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightProbes {
    pub m_BakedCoefficients: Vec<SphericalHarmonicsL2>,
    pub m_BakedLightOcclusion: Vec<LightProbeOcclusion>,
    pub m_Data: LightProbeData,
    /**The name of the object.*/
    pub m_Name: String,
}

/// LightingDataAsset is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightingDataAsset.html):
/**
The lighting data asset used by the active Scene.
Please note that modifying this value currently does not affect the Scene immediately, the lighting data is only patched into the active Scene when loading the Scene.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightingDataAsset {
    pub m_BakedAmbientProbeInLinear: SphericalHarmonicsL2,
    pub m_BakedReflectionProbeCubemaps: Vec<PPtr /*<Texture>*/>,
    pub m_BakedReflectionProbes: Vec<SceneObjectIdentifier>,
    pub m_EnlightenData: Vec<u8>,
    pub m_EnlightenDataVersion: i32,
    pub m_EnlightenSceneMapping: EnlightenSceneMapping,
    pub m_EnlightenSceneMappingRendererIDs: Vec<SceneObjectIdentifier>,
    pub m_LightBakingOutputs: Vec<LightBakingOutput>,
    pub m_LightProbes: PPtr, /*<LightProbes>*/
    pub m_LightmappedRendererData: Vec<RendererData>,
    pub m_LightmappedRendererDataIDs: Vec<SceneObjectIdentifier>,
    pub m_Lightmaps: Vec<LightmapData>,
    pub m_LightmapsMode: i32,
    pub m_Lights: Vec<SceneObjectIdentifier>,
    /**The name of the object.*/
    pub m_Name: String,
    /// Vec<PPtr/*<Texture2D>*/>: (2019.3.0f4 - 2022.2.0b16)
    pub m_AOTextures: Option<Vec<PPtr /*<Texture2D>*/>>,
    /// Vec<String>: (2018.4.15f1 - 2022.2.0b16)
    pub m_BakedReflectionProbeCubemapCacheFiles: Option<Vec<String>>,
    /// Vec<String>: (2018.4.15f1 - 2022.2.0b16)
    pub m_LightmapsCacheFiles: Option<Vec<String>>,
    /// PPtr/*<SceneAsset>*/: (2017.4.33f1 - 2022.2.0b16)
    pub m_Scene: Option<PPtr /*<SceneAsset>*/>,
}

/// LightingSettings is a  class of the Unity engine since version 2020.1.0a20.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightingSettings.html):
/**
An object containing settings for precomputing lighting data, that Unity can serialize as a Lighting Settings Asset.
When the Unity Editor precomputes lighting data for a Scene that uses the Baked Global Illumination system or the Enlighten Realtime Global Illumination system, it uses settings from a LightingSettings object. The same LightingSettings object can be assigned to more than one Scene, which makes it possible to share settings across multiple Scenes.The following example shows how to create a LightingSettings object and assign it to the active Scene using the Lightmapping.lightingSettings API:
The following example shows how to create a LightingSettings object, and save it to disk as a Lighting Settings Asset using the AssetDatabase.CreateAsset API.
See Also: Lighting Settings Asset.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightingSettings {
    /**The intensity of surface albedo throughout the Scene when considered in lighting calculations. This value influences the energy of light at each bounce. (Editor only).*/
    pub m_AlbedoBoost: f32,
    pub m_BounceScale: f32,
    pub m_EnableBakedLightmaps: bool,
    pub m_EnableRealtimeLightmaps: bool,
    pub m_GIWorkflowMode: i32,
    /**The name of the object.*/
    pub m_Name: String,
    /**Determines the lightmap that Unity stores environment lighting in.*/
    pub m_RealtimeEnvironmentLighting: bool,
    pub m_UsingShadowmask: bool,
    /// f32: (2020.3.42f1 - 2022.2.0b16)
    pub m_IndirectOutputScale: Option<f32>,
}

/// LightmapBakeMode is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct LightmapBakeMode {
    pub lightmapBakeType: i32,
    pub mixedLightingMode: i32,
}

/// LightmapData is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightmapData.html):
/**
Data of a lightmap.
A Scene can have several lightmaps stored in it, and Renderer components can use those
lightmaps. This makes it possible to use the same material on multiple objects, while
each object can refer to a different lightmap or different portion of the same lightmap.You must set the following properties or Unity might render objects incorrectly:
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightmapData {
    pub m_Lightmap: PPtr, /*<Texture2D>*/
    /// PPtr/*<Texture2D>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_DirLightmap: Option<PPtr /*<Texture2D>*/>,
    /// PPtr/*<Texture2D>*/: (3.4.0 - 3.4.0)
    pub m_IndirectLightmap: Option<PPtr /*<Texture2D>*/>,
    /**Texture storing occlusion mask per light (ShadowMask, up to four lights).*/
    /// PPtr/*<Texture2D>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_ShadowMask: Option<PPtr /*<Texture2D>*/>,
}

/// LightmapParameters is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightmapParameters.html):
/**
Configures how Unity bakes lighting and can be assigned to a LightingSettings instance or asset.
Note that Unity's built-in Lightmap Parameters Assets are read-only.


See Also: LightmapParameters.SetLightmapParametersForLightingSettings.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightmapParameters {
    /**The maximum number of times to supersample a texel to reduce aliasing in AO.*/
    pub AOAntiAliasingSamples: i32,
    /**The number of rays to cast for computing ambient occlusion.*/
    pub AOQuality: i32,
    /**The maximum number of times to supersample a texel to reduce aliasing.*/
    pub antiAliasingSamples: i32,
    /**The percentage of rays shot from a ray origin that must hit front faces to be considered usable.*/
    pub backFaceTolerance: f32,
    /**BakedLightmapTag is an integer that affects the assignment to baked lightmaps. Objects with different values for bakedLightmapTag are guaranteed to not be assigned to the same lightmap even if the other baking parameters are the same.*/
    pub bakedLightmapTag: i32,
    /**The radius (in texels) of the post-processing filter that blurs baked direct lighting.*/
    pub blurRadius: i32,
    /**Controls the resolution at which Enlighten Realtime Global Illumination stores and can transfer input light.*/
    pub clusterResolution: f32,
    /**The number of rays used for lights with an area. Allows for accurate soft shadowing.*/
    pub directLightQuality: i32,
    pub edgeStitching: i32,
    /**The amount of data used for Enlighten Realtime Global Illumination texels. Specifies how detailed view of the Scene a texel has. Small values mean more averaged out lighting.*/
    pub irradianceBudget: i32,
    /**The number of rays to cast for computing irradiance form factors.*/
    pub irradianceQuality: i32,
    /**If enabled, the object appears transparent during GlobalIllumination lighting calculations.*/
    pub isTransparent: i32,
    /**The name of the object.*/
    pub m_Name: String,
    /**Maximum size of gaps that can be ignored for GI (multiplier on pixel size).*/
    pub modellingTolerance: f32,
    /**The distance to offset the ray origin from the geometry when performing ray tracing, in modelling units. Unity applies the offset to all baked lighting: direct lighting, indirect lighting, environment lighting and ambient occlusion.*/
    pub pushoff: f32,
    /**The texel resolution per meter used for real-time lightmaps. This value is multiplied by LightingSettings.indirectResolution.*/
    pub resolution: f32,
    /**System tag is an integer identifier. It lets you force an object into a different Enlighten Realtime Global Illumination system even though all the other parameters are the same.*/
    pub systemTag: i32,
    /**If enabled, objects sharing the same lightmap parameters will be packed into LightmapParameters.maxLightmapCount lightmaps.*/
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub limitLightmapCount: Option<bool>,
    /**The maximum number of lightmaps created for objects sharing the same lightmap parameters. This property is ignored if LightmapParameters.limitLightmapCount is false.*/
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub maxLightmapCount: Option<i32>,
}

/// LightmapSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightmapSettings.html):
/**
Stores lightmaps of the Scene.
A Scene can have several lightmaps stored in it, and Renderer components can use those
lightmaps. This makes it possible to use the same material on multiple objects, while
each object can refer to a different lightmap or different portion of the same lightmap.See Also: LightmapData class, Renderer.lightmapIndex
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightmapSettings {
    pub m_Lightmaps: Vec<LightmapData>,
    pub m_LightmapsMode: i32,
    /// EnlightenSceneMapping: (5.6.0b2 - 2022.2.0b16)
    pub m_EnlightenSceneMapping: Option<EnlightenSceneMapping>,
    /// GISettings: (5.6.0b2 - 2022.2.0b16)
    pub m_GISettings: Option<GISettings>,
    /// PPtr/*<LightProbes>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_LightProbes: Option<PPtr /*<LightProbes>*/>,
    /// PPtr/*<LightingSettings>*/: (2020.1.0a20 - 2022.2.0b16)
    pub m_LightingSettings: Option<PPtr /*<LightingSettings>*/>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub m_RuntimeCPUUsage: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub m_ShadowMaskMode: Option<i32>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_UseDualLightmapsInForward: Option<bool>,
    /// bool: (2017.4.33f1 - 2019.3.0f4)
    pub m_UseShadowmask: Option<bool>,
}

/// LightsModule is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.LightsModule.html):
/**
Access the ParticleSystem Lights Module.
This module allows you to attach real-time Lights to a percentage of your particles.The Lights Module is a simple and powerful module that allows particles to cast light onto their environment easily. Lights can inherit properties from the particles they are attached to, such as color and size. Point and Spot Lights are supported, including shadow casting and Light cookies.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightsModule {
    pub color: bool,
    /**Specifies whether the LightsModule is enabled or disabled.*/
    pub enabled: bool,
    /**Define a curve to apply custom intensity scaling to particle Lights.*/
    pub intensity: bool,
    pub intensityCurve: MinMaxCurve,
    /**Select what Light Prefab you want to base your particle lights on.*/
    pub light: PPtr, /*<Light>*/
    /**Set a limit on how many Lights this Module can create.*/
    pub maxLights: i32,
    pub randomDistribution: bool,
    /**Define a curve to apply custom range scaling to particle Lights.*/
    pub range: bool,
    pub rangeCurve: MinMaxCurve,
    /**Choose what proportion of particles receive a dynamic light.*/
    pub ratio: f32,
}

/// LineParameters is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct LineParameters {
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub alignment: Option<i32>,
    /// Gradient: (5.6.0b2 - 2022.2.0b16)
    pub colorGradient: Option<Gradient>,
    /// f32: (3.4.0 - 3.4.0)
    pub endWidth: Option<f32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub generateLightingData: Option<bool>,
    /// ColorRGBA: (3.4.0 - 3.4.0)
    pub m_EndColor: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 3.4.0)
    pub m_StartColor: Option<ColorRGBA>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub numCapVertices: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub numCornerVertices: Option<i32>,
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub shadowBias: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    pub startWidth: Option<f32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub textureMode: Option<i32>,
    /// Vector2f: (2022.2.0b16 - 2022.2.0b16)
    pub textureScale: Option<Vector2f>,
    /// AnimationCurve: (5.6.0b2 - 2022.2.0b16)
    pub widthCurve: Option<AnimationCurve>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub widthMultiplier: Option<f32>,
}

/// LineRenderer is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LineRenderer.html):
/**
The line renderer is used to draw free-floating lines in 3D space.
This class is a script interface for a line renderer component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LineRenderer {
    pub m_CastShadows: Enum_bool__u8,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapTilingOffset: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    pub m_Materials: Vec<PPtr /*<Material>*/>,
    pub m_Parameters: LineParameters,
    pub m_Positions: Vec<Vector3f>,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: Enum_bool__u8,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    /**If enabled, the lines are defined in world space.*/
    pub m_UseWorldSpace: bool,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ApplyActiveColorSpace: Option<bool>,
    /// u8: (2017.4.33f1 - 2022.2.0b16)
    pub m_DynamicOccludee: Option<u8>,
    /**The light probe interpolation type.*/
    /// u8: (5.6.0b2 - 2022.2.0b16)
    pub m_LightProbeUsage: Option<u8>,
    /// PPtr/*<GameObject>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_LightProbeVolumeOverride: Option<PPtr /*<GameObject>*/>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub m_LightmapIndexDynamic: Option<u16>,
    /// Vector4f: (5.6.0b2 - 2022.2.0b16)
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    /**Connect the start and end positions of the line together to form a continuous loop.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_Loop: Option<bool>,
    /**Specifies how the LineRenderer interacts with SpriteMask.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_MaskInteraction: Option<i32>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    pub m_MotionVectors: Option<u8>,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr/*<Transform>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_ProbeAnchor: Option<PPtr /*<Transform>*/>,
    /// u8: (2020.1.0a20 - 2022.2.0b16)
    pub m_RayTraceProcedural: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0f4 - 2022.2.0b16)
    pub m_RayTracingMode: Option<u8>,
    /**Should reflection probes be used for this Renderer?*/
    /// u8: (5.6.0b2 - 2022.2.0b16)
    pub m_ReflectionProbeUsage: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on.*/
    /// u32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RenderingLayerMask: Option<u32>,
    /// i16: (5.6.0b2 - 2022.2.0b16)
    pub m_SortingLayer: Option<i16>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_SortingLayerID: Option<i32>,
    /**Renderer's order within a sorting layer.*/
    /// i16: (5.6.0b2 - 2022.2.0b16)
    pub m_SortingOrder: Option<i16>,
    /// StaticBatchInfo: (5.6.0b2 - 2022.2.0b16)
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.2.16f1 - 2022.2.0b16)
    pub m_StaticShadowCaster: Option<u8>,
    /// Vec<u32>: (3.4.0 - 3.4.0)
    pub m_SubsetIndices: Option<Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_bool__u8 {
    bool(bool),
    u8(u8),
}

/// LocalizationAsset is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LocalizationAsset.html):
/**
An asset to represent a table of localized strings for one specific locale.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizationAsset {
    /**The name of the object.*/
    pub m_Name: String,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    #[serde(alias = "Editor Asset")]
    pub Editor_Asset: Option<bool>,
    /**ISO Code used to identify the locale. ex: en-uk, zh-hans, ja*/
    /// String: (2018.4.15f1 - 2022.2.0b16)
    #[serde(alias = "Locale ISO Code")]
    pub Locale_ISO_Code: Option<String>,
    /// Vec<(String, String)>: (2018.4.15f1 - 2022.2.0b16)
    #[serde(alias = "String Table")]
    pub String_Table: Option<Vec<(String, String)>>,
}

/// LocalizationImporter is a  class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizationImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_Name: String,
    pub m_UserData: String,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// LookAtConstraint is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.LookAtConstraint.html):
/**
Constrains the orientation of an object relative to the position of one or more source objects, such that the object is facing the average position of the sources.
                The LookAtConstraint is a simplified AimConstraint typically used with a Camera.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LookAtConstraint {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The rotation angle along the z axis of the object. The constraint uses this property to calculate the world up vector when Animations.LookAtConstraint.UseUpObject is false.*/
    pub m_Roll: f32,
    /**The rotation used when the sources have a total weight of 0.*/
    pub m_RotationAtRest: Vector3f,
    /**Represents an offset from the constrained orientation.*/
    pub m_RotationOffset: Vector3f,
    pub m_Sources: Vec<ConstraintSource>,
    /**Determines how the up vector is calculated.*/
    pub m_UseUpObject: bool,
    /**The weight of the constraint component.*/
    pub m_Weight: f32,
    /**The world up object, used to calculate the world up vector when Animations.LookAtConstraint.UseUpObject is true.*/
    pub m_WorldUpObject: PPtr, /*<Transform>*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_Active: Option<bool>,
    /// bool: (2018.4.15f1 - 2021.2.16f1)
    pub m_IsContraintActive: Option<bool>,
}

/// Lumin is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct Lumin {
    pub depthFormat: i32,
    pub enableGLCache: bool,
    pub frameTiming: i32,
    pub glCacheMaxBlobSize: u32,
    pub glCacheMaxFileSize: u32,
}

/// Material is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Material.html):
/**
The material class.
This class exposes all properties from a material, allowing you to animate them.

You can also use it to set custom shader properties that can't be accessed through

the inspector (e.g. matrices).In order to get the material used by an object, use the Renderer.material property.See Also: Materials, Shaders.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    /**The name of the object.*/
    pub m_Name: String,
    pub m_SavedProperties: UnityPropertySheet,
    /**The shader used by the material.*/
    pub m_Shader: PPtr, /*<Shader>*/
    /// Vec<String>: (2017.4.33f1 - 2022.2.0b16)
    pub disabledShaderPasses: Option<Vec<String>>,
    /// Vec<BuildTextureStackReference>: (2020.1.0a20 - 2022.2.0b16)
    pub m_BuildTextureStacks: Option<Vec<BuildTextureStackReference>>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_CustomRenderQueue: Option<i32>,
    /**Gets and sets whether the Double Sided Global Illumination setting is enabled for this material.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_DoubleSidedGI: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_EnableInstancingVariants: Option<bool>,
    /// Vec<String>: (2022.2.0b16 - 2022.2.0b16)
    pub m_InvalidKeywords: Option<Vec<String>>,
    /// u32: (5.6.0b2 - 2022.2.0b16)
    pub m_LightmapFlags: Option<u32>,
    /**An array containing names of the local shader keywords that are currently enabled for this material.*/
    /// String: (5.6.0b2 - 2021.2.16f1)
    pub m_ShaderKeywords: Option<String>,
    /// Vec<String>: (2022.2.0b16 - 2022.2.0b16)
    pub m_ValidKeywords: Option<Vec<String>>,
    /// Vec<(String, String)>: (5.6.0b2 - 2022.2.0b16)
    pub stringTagMap: Option<Vec<(String, String)>>,
}

/// MaterialImportOutput is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialImportOutput {
    pub baked: i32,
    pub currentSettings: BuildTargetSettings,
}

/// MaterialInstanceSettings is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialInstanceSettings {
    pub buildTargetSettings: Vec<BuildTargetSettings>,
    pub inputs: Vec<InputImportSettings>,
    pub materialInformation: ProceduralMaterialInformation,
    pub materialProperties: UnityPropertySheet,
    pub name: String,
    pub prototypeName: String,
    pub shaderName: String,
    pub textureParameters: Vec<InputImportSettings>,
    /// u32: (5.6.0b2 - 2017.4.33f1)
    pub lightmapFlags: Option<u32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub renderQueue: Option<i32>,
    /// PPtr/*<Shader>*/: (5.6.0b2 - 2017.4.33f1)
    pub shader: Option<PPtr /*<Shader>*/>,
    /// String: (5.6.0b2 - 2017.4.33f1)
    pub shaderKeywords: Option<String>,
    /// Vec<ProceduralTextureAssignment>: (5.6.0b2 - 2017.4.33f1)
    pub textureAssignments: Option<Vec<ProceduralTextureAssignment>>,
}

/// Matrix3x4f is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
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

/// Matrix4x4f is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
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

/// MatrixParameter is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct MatrixParameter {
    pub m_ArraySize: i32,
    pub m_Index: i32,
    pub m_NameIndex: i32,
    pub m_RowCount: i8,
    pub m_Type: i8,
}

/// MdFour is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct MdFour {
    #[serde(alias = "md4 hash")]
    pub md4_hash: Vec<u8>,
}

/// MemorySettings is a  class of the Unity engine since version 2021.2.16f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct MemorySettings {}

/// Mesh is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Mesh.html):
/**
A class that allows you to create or modify meshes.
Meshes contain vertices and multiple triangle arrays.Conceptually, all vertex data is stored in separate arrays of the same size. For example, if you have

a mesh of 100 Vertices, and want to have a position, normal and two texture coordinates

for each vertex, then the mesh should have vertices, normals, uv and uv2

arrays, each being 100 in size. Data for i-th vertex is at index "i" in each array.For every vertex there can be a vertex position, normal, tangent, color and up to 8 texture coordinates.

Texture coordinates most often are 2D data (Vector2), but it is possible to make them

Vector3 or Vector4 if needed. This is most often used for holding arbitrary data in mesh

vertices, for special effects used in shaders. For skinned meshes, the vertex data can also

contain boneWeights.The mesh face data, i.e. the triangles it is made of, is simply three vertex indices for each triangle.

For example, if the mesh has 10 triangles, then the triangles array should be 30 numbers,

with each number indicating which vertex to use. The first three elements in the triangles array are

the indices for the vertices that make up that triangle; the second three elements make up

another triangle and so on.Note that while triangle meshes are the most common use case, Unity also supports other

mesh topology types, for example Line or Point meshes. For line meshes, each line

is composed of two vertex indices and so on. See SetIndices and MeshTopology.
Simple vs Advanced Mesh APIThe Mesh class has two sets of methods for assigning data to a Mesh from script. The "simple" set of methods provide a basis for setting the indices, triangle, normals, tangents, etc. These methods include validation checks, for example to ensure that you are not passing in data that would include out-of-bounds indices. They represent the standard way to assign Mesh data from script in Unity.The "simple" methods are: SetColors, SetIndices, SetNormals, SetTangents, SetTriangles, SetUVs, SetVertices, SetBoneWeights.There is also an "advanced" set of methods, which allow you to directly write to the mesh data with control over whether any checks or validation should be performed. These methods are intended for advanced use cases which require maximum performance. They are faster, but allow you to skip the checks on the data you supply. If you use these methods you must make sure that you are not supplying invalid data, because Unity will not check for you.The "advanced" methods are: SetVertexBufferParams, SetVertexBufferData, SetIndexBufferParams, SetIndexBufferData, SetSubMesh, and you can use the MeshUpdateFlags to control which checks or validation are performed or omitted. Use AcquireReadOnlyMeshData to take a read-only snapshot of Mesh data that you can use with C# Jobs and Burst, and AllocateWritableMeshData with ApplyAndDisposeWritableMeshData to create Meshes from C# Jobs and Burst.Manipulating meshes from a scriptThere are three common tasks that might want to use the Mesh API for:1. Building a mesh from scratch:

should always be done in the following order:

a) Assign vertices

b) Assign triangles.
2. Modifying vertex attributes every frame:

a) Get vertices

b) Modify them

c) Assign them back to the mesh.
3. Continously changing the mesh triangles and vertices:

a) Call Clear to start fresh

b) Assign vertices and other attributes

c) Assign triangle indices.It is important to call Clear before assigning new vertices or triangles. Unity always checks the supplied triangle indices whether they don't reference out of bounds vertices.

Calling Clear then assigning vertices then triangles makes sure you never have out of bounds data.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Mesh {
    pub m_BindPose: Vec<Matrix4x4f>,
    pub m_CompressedMesh: CompressedMesh,
    pub m_IndexBuffer: Vec<u8>,
    pub m_LocalAABB: AABB,
    pub m_MeshCompression: u8,
    pub m_MeshUsageFlags: i32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_SubMeshes: Vec<SubMesh>,
    /// Vec<u8>: (5.6.0b2 - 2022.2.0b16)
    pub m_BakedConvexCollisionMesh: Option<Vec<u8>>,
    /// Vec<u8>: (5.6.0b2 - 2022.2.0b16)
    pub m_BakedTriangleCollisionMesh: Option<Vec<u8>>,
    /// Vec<u32>: (5.6.0b2 - 2022.2.0b16)
    pub m_BoneNameHashes: Option<Vec<u32>>,
    /// Vec<MinMaxAABB>: (2019.3.0f4 - 2022.2.0b16)
    pub m_BonesAABB: Option<Vec<MinMaxAABB>>,
    /// Vec<u32>: (3.4.0 - 3.4.0)
    pub m_CollisionTriangles: Option<Vec<u32>>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_CollisionVertexCount: Option<i32>,
    /**Vertex colors of the Mesh.*/
    /// Vec<ColorRGBA>: (3.4.0 - 3.4.0)
    pub m_Colors: Option<Vec<ColorRGBA>>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_CookingOptions: Option<i32>,
    /**Format of the mesh index buffer data.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_IndexFormat: Option<i32>,
    /**Returns true if the Mesh is read/write enabled, or false if it is not.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_IsReadable: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_KeepIndices: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_KeepVertices: Option<bool>,
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    #[serde(alias = "m_MeshMetrics[0]")]
    pub m_MeshMetrics_0_: Option<f32>,
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    #[serde(alias = "m_MeshMetrics[1]")]
    pub m_MeshMetrics_1_: Option<f32>,
    /**The normals of the Mesh.*/
    /// Vec<Vector3f>: (3.4.0 - 3.4.0)
    pub m_Normals: Option<Vec<Vector3f>>,
    /// u32: (5.6.0b2 - 2022.2.0b16)
    pub m_RootBoneNameHash: Option<u32>,
    /// BlendShapeData: (5.6.0b2 - 2022.2.0b16)
    pub m_Shapes: Option<BlendShapeData>,
    /// Vec<BoneInfluence>: (3.4.0 - 5.6.0b2); Vec<BoneWeights4>: (2017.4.33f1 - 2017.4.33f1)
    pub m_Skin: Option<Vec<Enum_BoneInfluence__BoneWeights4>>,
    /// StreamingInfo: (2018.4.15f1 - 2022.2.0b16)
    pub m_StreamData: Option<StreamingInfo>,
    /**The tangents of the Mesh.*/
    /// Vec<Vector4f>: (3.4.0 - 3.4.0)
    pub m_Tangents: Option<Vec<Vector4f>>,
    /**The texture coordinates (UVs) in the first channel.*/
    /// Vec<Vector2f>: (3.4.0 - 3.4.0)
    pub m_UV: Option<Vec<Vector2f>>,
    /// Vec<Vector2f>: (3.4.0 - 3.4.0)
    pub m_UV1: Option<Vec<Vector2f>>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_Use16BitIndices: Option<i32>,
    /// VariableBoneCountWeights: (2019.3.0f4 - 2022.2.0b16)
    pub m_VariableBoneCountWeights: Option<VariableBoneCountWeights>,
    /// VertexData: (5.6.0b2 - 2022.2.0b16)
    pub m_VertexData: Option<VertexData>,
    /**Returns a copy of the vertex positions or assigns a new vertex positions array.*/
    /// Vec<Vector3f>: (3.4.0 - 3.4.0)
    pub m_Vertices: Option<Vec<Vector3f>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_BoneInfluence__BoneWeights4 {
    BoneInfluence(BoneInfluence),
    BoneWeights4(BoneWeights4),
}

/// MeshBlendShape is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshBlendShape {
    pub firstVertex: u32,
    pub hasNormals: bool,
    pub hasTangents: bool,
    pub vertexCount: u32,
}

/// MeshBlendShapeChannel is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshBlendShapeChannel {
    pub frameCount: i32,
    pub frameIndex: i32,
    pub name: String,
    pub nameHash: u32,
}

/// MeshCollider is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/MeshCollider.html):
/**
A mesh collider allows you to do collision detection between meshes and primitives.
See Also: BoxCollider, CapsuleCollider, PhysicMaterial, Rigidbody.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshCollider {
    /**Use a convex collider from the mesh.*/
    pub m_Convex: bool,
    /**Enabled Colliders will collide with other Colliders, disabled Colliders won't.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Specify if this collider is configured as a trigger.*/
    pub m_IsTrigger: bool,
    /**The material used by the collider.*/
    pub m_Material: PPtr, /*<PhysicMaterial>*/
    pub m_Mesh: PPtr, /*<Mesh>*/
    /**Options used to enable or disable certain features in mesh cooking.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_CookingOptions: Option<i32>,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /// bool: (5.6.0b2 - 5.6.0b2)
    pub m_InflateMesh: Option<bool>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ProvidesContacts: Option<bool>,
    /// f32: (5.6.0b2 - 2017.4.33f1)
    pub m_SkinWidth: Option<f32>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_SmoothSphereCollisions: Option<bool>,
}

/// MeshFilter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/MeshFilter.html):
/**
A class to access the Mesh of the mesh filter.
Use this with a procedural mesh interface. See Also: Mesh class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshFilter {
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Returns either a new mesh or a duplicate of the existing mesh, and assigns it to the mesh filter.*/
    pub m_Mesh: PPtr, /*<Mesh>*/
}

/// MeshParticleEmitter is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshParticleEmitter {
    pub angularVelocity: f32,
    pub emitterVelocityScale: f32,
    pub localVelocity: Vector3f,
    pub m_Emit: bool,
    pub m_Enabled: bool,
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_InterpolateTriangles: bool,
    pub m_MaxNormalVelocity: f32,
    pub m_Mesh: PPtr, /*<Mesh>*/
    pub m_MinNormalVelocity: f32,
    pub m_OneShot: bool,
    pub m_Systematic: bool,
    pub maxEmission: f32,
    pub maxEnergy: f32,
    pub maxSize: f32,
    pub minEmission: f32,
    pub minEnergy: f32,
    pub minSize: f32,
    pub rndAngularVelocity: f32,
    pub rndRotation: bool,
    pub rndVelocity: Vector3f,
    pub tangentVelocity: Vector3f,
    pub worldVelocity: Vector3f,
    /// bool: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "Simulate in Worldspace?")]
    pub Simulate_in_Worldspace_: Option<bool>,
}

/// MeshRenderer is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/MeshRenderer.html):
/**
Renders meshes inserted by the MeshFilter or TextMesh.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshRenderer {
    /**Vertex attributes in this mesh will override or add attributes of the primary mesh in the MeshRenderer.*/
    pub m_AdditionalVertexStreams: PPtr, /*<Mesh>*/
    pub m_CastShadows: u8,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The light probe interpolation type.*/
    pub m_LightProbeUsage: u8,
    pub m_LightProbeVolumeOverride: PPtr, /*<GameObject>*/
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    pub m_Materials: Vec<PPtr /*<Material>*/>,
    pub m_MotionVectors: u8,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    pub m_ProbeAnchor: PPtr, /*<Transform>*/
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: u8,
    /**Should reflection probes be used for this Renderer?*/
    pub m_ReflectionProbeUsage: u8,
    pub m_SortingLayer: i16,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    pub m_StaticBatchInfo: StaticBatchInfo,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    /// u8: (2017.4.33f1 - 2022.2.0b16)
    pub m_DynamicOccludee: Option<u8>,
    /**Vertex attributes that override the primary mesh when the MeshRenderer uses lightmaps in the Realtime Global Illumination system.*/
    /// PPtr/*<Mesh>*/: (2020.1.0a20 - 2022.2.0b16)
    pub m_EnlightenVertexStream: Option<PPtr /*<Mesh>*/>,
    /// u8: (2020.1.0a20 - 2022.2.0b16)
    pub m_RayTraceProcedural: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0f4 - 2022.2.0b16)
    pub m_RayTracingMode: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on.*/
    /// u32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RenderingLayerMask: Option<u32>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_SortingLayerID: Option<i32>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.2.16f1 - 2022.2.0b16)
    pub m_StaticShadowCaster: Option<u8>,
}

/// MinMaxAABB is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct MinMaxAABB {
    pub m_Max: Vector3f,
    pub m_Min: Vector3f,
}

/// MinMaxCurve is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.MinMaxCurve.html):
/**
Script interface for a Min-Max Curve.
Min-Max Curve. describes functions which take a value between a minimum and maximum limit and return a value based on ParticleSystem.MinMaxCurve.mode. Depending on the mode, this may return randomized values.

For modes that require curves, the value returned is dependent on one or two curves designed in the ParticleSystem Inspector, that can be evaluated to a single value between -n and n, where n is a constant also set in the Inspector. See ParticleSystemCurveMode for more information.See Also: ParticleSystem.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MinMaxCurve {
    pub maxCurve: AnimationCurve,
    pub minCurve: AnimationCurve,
    pub minMaxState: i32,
    pub scalar: f32,
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub minScalar: Option<f32>,
}

/// MinMaxGradient is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.MinMaxGradient.html):
/**
Script interface for a Min-Max Gradient.
This contains two Gradients, and returns a Color based on ParticleSystem.MinMaxGradient.mode. Depending on the mode, this may return the value randomized.

Gradients are edited via the ParticleSystem Inspector once a ParticleSystemGradientMode requiring them has been selected. Some modes do not require gradients, only colors.

See Also: ParticleSystem.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MinMaxGradient {
    pub maxColor: ColorRGBA,
    pub maxGradient: Gradient,
    pub minColor: ColorRGBA,
    pub minGradient: Gradient,
    pub minMaxState: i32,
}

/// MipmapLimitSettings is a sub class of the Unity engine since version 2022.2.0b16.
#[derive(Debug, Serialize, Deserialize)]
pub struct MipmapLimitSettings {
    pub limitBias: i32,
    pub limitBiasMode: i32,
}

/// ModelImporter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ModelImporter.html):
/**
Model importer lets you modify model import settings from editor scripts.
Settings of this class match the ones exposed in Mesh Import Settings.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelImporter {
    pub m_AddColliders: bool,
    /**Animation compression setting.*/
    pub m_AnimationCompression: i32,
    /**Allowed error of animation position compression.*/
    pub m_AnimationPositionError: f32,
    /**Allowed error of animation rotation compression.*/
    pub m_AnimationRotationError: f32,
    /**Allowed error of animation scale compression.*/
    pub m_AnimationScaleError: f32,
    /**The default wrap mode for the generated animation clips.*/
    pub m_AnimationWrapMode: i32,
    pub m_BakeSimulation: bool,
    /**Animation clips to split animation into. See Also: ModelImporterClipAnimation.*/
    pub m_ClipAnimations: Vec<ClipAnimationInfo>,
    /**Global scale factor for importing.*/
    pub m_GlobalScale: f32,
    pub m_HasExtraRoot: bool,
    pub m_ImportedRoots: Vec<PPtr /*<GameObject>*/>,
    /**Mesh compression setting.*/
    pub m_MeshCompression: i32,
    /**The name of the object.*/
    pub m_Name: String,
    /**Detect file units and import as 1FileUnit=1UnityUnit, otherwise it will import as 1cm=1UnityUnit.*/
    pub m_UseFileUnits: bool,
    pub normalSmoothAngle: f32,
    /**Computes the axis conversion on geometry and animation for Models defined in an axis system that differs from Unity's (left handed, Z forward, Y-up).                     When enabled, Unity transforms the geometry and animation data in order to convert the axis.                     When disabled, Unity transforms the root GameObject of the hierarchy in order to convert the axis.*/
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub bakeAxisConversion: Option<bool>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub blendShapeNormalImportMode: Option<i32>,
    /**Generate secondary UV set for lightmapping.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub generateSecondaryUV: Option<bool>,
    /**Format of the imported mesh index buffer data.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub indexFormat: Option<i32>,
    /**If this is true, any quad faces that exist in the mesh data before it is imported are kept as quads instead of being split into two triangles, for the purposes of tessellation. Set this to false to disable this behavior.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub keepQuads: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub legacyComputeAllNormalsFromSmoothingGroupsWhenMeshHasBlendShapes: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_AddHumanoidExtraRootOnlyWhenUsingAvatar: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_AdditionalBone: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_AnimationDoRetargetingWarnings: Option<bool>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AnimationImportErrors: Option<String>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AnimationImportWarnings: Option<String>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AnimationRetargetingWarnings: Option<String>,
    /**Animator generation mode.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_AnimationType: Option<i32>,
    /**Get or set the AssetBundle name.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AssetBundleName: Option<String>,
    /**Get or set the AssetBundle variant.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AssetBundleVariant: Option<String>,
    /**Generate auto mapping if no avatarSetup is provided when importing humanoid animation.*/
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_AutoGenerateAvatarMappingIfUnspecified: Option<bool>,
    /**The Avatar generation of the imported model.*/
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_AvatarSetup: Option<i32>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub m_CopyAvatar: Option<bool>,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /**Animation optimization setting.*/
    /// Vec<String>: (5.6.0b2 - 2022.2.0b16)
    pub m_ExtraExposedTransformPaths: Option<Vec<String>>,
    /**A list of default FBX properties to treat as user properties during OnPostprocessGameObjectWithUserProperties.*/
    /// Vec<String>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExtraUserProperties: Option<Vec<String>>,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.0); Vec<(i64, String)>: (5.6.0b2 - 2018.4.15f1)
    pub m_FileIDToRecycleName: Option<Vec<(i64, String)>>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_FileIdsGeneration: Option<i32>,
    /**Scaling factor used when useFileScale is set to true (Read-only).*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_FileScale: Option<f32>,
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub m_FileScaleFactor: Option<f32>,
    /// String: (2018.4.15f1 - 2022.2.0b16)
    pub m_FileScaleUnit: Option<String>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_FirstImportVersion: Option<i32>,
    /**Animation generation options.*/
    /// i32: (3.4.0 - 3.4.0)
    pub m_GenerateAnimations: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_GenerateMaterials: Option<i32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_HasEmbeddedTextures: Option<bool>,
    /// bool: (2018.4.15f1 - 2018.4.15f1)
    pub m_HasPreviousCalculatedGlobalScale: Option<bool>,
    /**The human description that is used to generate an Avatar during the import process.*/
    /// HumanDescription: (5.6.0b2 - 2022.2.0b16)
    pub m_HumanDescription: Option<HumanDescription>,
    /**Controls how much oversampling is used when importing humanoid animations for retargeting.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_HumanoidOversampling: Option<i32>,
    /**Import animated custom properties from file.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_ImportAnimatedCustomProperties: Option<bool>,
    /**Import animation from file.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_ImportAnimation: Option<bool>,
    /**Import BlendShapes deform percent.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ImportBlendShapeDeformPercent: Option<bool>,
    /**Controls import of BlendShapes.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_ImportBlendShapes: Option<bool>,
    /**Controls import of cameras. Basic properties like field of view, near plane distance and far plane distance can be animated.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_ImportCameras: Option<bool>,
    /**Import animation constraints.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_ImportConstraints: Option<bool>,
    /**Controls import of lights. Note that because light are defined differently in DCC tools, some light types or properties may not be exported. Basic properties like color and intensity can be animated.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_ImportLights: Option<bool>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub m_ImportMaterials: Option<bool>,
    /**Use visibility properties to enable or disable MeshRenderer components.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_ImportVisibility: Option<bool>,
    /**Generates the list of all imported take.*/
    /// Vec<TakeInfo>: (5.6.0b2 - 2022.2.0b16)
    pub m_ImportedTakeInfos: Option<Vec<TakeInfo>>,
    /// Vec<((i32, i64), String)>: (2019.3.0f4 - 2022.2.0b16)
    pub m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
    /**Are mesh vertices and indices accessible from script?*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_IsReadable: Option<bool>,
    /// Vec<f32>: (5.6.0b2 - 2022.2.0b16)
    pub m_LODScreenPercentages: Option<Vec<f32>>,
    /// PPtr/*<Avatar>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_LastHumanDescriptionAvatarSource: Option<PPtr /*<Avatar>*/>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_LegacyGenerateAnimations: Option<i32>,
    /**Material creation options.*/
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_MaterialImportMode: Option<i32>,
    /**Material import location options.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_MaterialLocation: Option<i32>,
    /**Material naming setting.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_MaterialName: Option<i32>,
    /**Existing material search setting.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_MaterialSearch: Option<i32>,
    /// Vec<SourceAssetIdentifier>: (2017.4.33f1 - 2022.2.0b16)
    pub m_Materials: Option<Vec<SourceAssetIdentifier>>,
    /// bool: (3.4.0 - 3.4.0)
    #[serde(alias = "m_MeshSettings.generateSecondaryUV")]
    pub m_MeshSettings_generateSecondaryUV: Option<bool>,
    /// i32: (3.4.0 - 3.4.0)
    #[serde(alias = "m_MeshSettings.normalImportMode")]
    pub m_MeshSettings_normalImportMode: Option<i32>,
    /// f32: (3.4.0 - 3.4.0)
    #[serde(alias = "m_MeshSettings.secondaryUVAngleDistortion")]
    pub m_MeshSettings_secondaryUVAngleDistortion: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    #[serde(alias = "m_MeshSettings.secondaryUVAreaDistortion")]
    pub m_MeshSettings_secondaryUVAreaDistortion: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    #[serde(alias = "m_MeshSettings.secondaryUVHardAngle")]
    pub m_MeshSettings_secondaryUVHardAngle: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    #[serde(alias = "m_MeshSettings.secondaryUVPackMargin")]
    pub m_MeshSettings_secondaryUVPackMargin: Option<f32>,
    /// bool: (3.4.0 - 3.4.0)
    #[serde(alias = "m_MeshSettings.swapUVChannels")]
    pub m_MeshSettings_swapUVChannels: Option<bool>,
    /// i32: (3.4.0 - 3.4.0)
    #[serde(alias = "m_MeshSettings.tangentImportMode")]
    pub m_MeshSettings_tangentImportMode: Option<i32>,
    /**The path of the transform used to generation the motion of the animation.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_MotionNodeName: Option<String>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_NewHashIdentity: Option<MdFour>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_NodeNameCollisionStrategy: Option<i32>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_OldHashIdentity: Option<MdFour>,
    /**Animation optimization setting.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_OptimizeGameObjects: Option<bool>,
    /**If true, always create an explicit Prefab root. Otherwise, if the model has a single root, it is reused as the Prefab root.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_PreserveHierarchy: Option<bool>,
    /// f32: (2018.4.15f1 - 2018.4.15f1)
    pub m_PreviousCalculatedGlobalScale: Option<f32>,
    /**Generates the list of all imported Animations.*/
    /// Vec<GUID>: (5.6.0b2 - 2022.2.0b16)
    pub m_ReferencedClips: Option<Vec<GUID>>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_RemapMaterialsIfMaterialImportModeIsNone: Option<bool>,
    /**Removes constant animation curves with values identical to the object initial scale value.*/
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub m_RemoveConstantScaleCurves: Option<bool>,
    /**If set to false, the importer will not resample curves when possible.Read more about animation curve resampling.Notes:- Some unsupported FBX features (such as PreRotation or PostRotation on transforms) will override this setting. In these situations, animation curves will still be resampled even if the setting is disabled. For best results, avoid using PreRotation, PostRotation and GetRotationPivot.- This option was introduced in Version 5.3. Prior to this version, Unity's import behaviour was as if this option was always enabled. Therefore enabling the option gives the same behaviour as pre-5.3 animation import.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_ResampleCurves: Option<bool>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_RigImportErrors: Option<String>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_RigImportWarnings: Option<String>,
    /**Sorts the gameObject hierarchy by name.*/
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_SortHierarchyByName: Option<bool>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_SplitAnimations: Option<bool>,
    /**Enables strict checks on imported vertex data.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_StrictVertexDataChecks: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_SupportsEmbeddedMaterials: Option<bool>,
    /**Use FileScale when importing.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_UseFileScale: Option<bool>,
    /**When disabled, imported material albedo colors are converted to gamma space. This property should be disabled when using linear color space in Player rendering settings.The default value is true.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_UseSRGBMaterialColor: Option<bool>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /**Get or set any user data.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_UserData: Option<String>,
    /**The maximum number of bones per vertex stored in this mesh data.*/
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub maxBonesPerVertex: Option<i32>,
    /**Options to control the optimization of mesh data during asset import.*/
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub meshOptimizationFlags: Option<i32>,
    /**Minimum bone weight to keep.*/
    /// f32: (2019.3.0f4 - 2022.2.0b16)
    pub minBoneWeight: Option<f32>,
    /**Normal generation options for ModelImporter.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub normalCalculationMode: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub normalImportMode: Option<i32>,
    /**Source of smoothing information for calculation of normals.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub normalSmoothingSource: Option<i32>,
    /**Only import bones where they are connected to vertices.*/
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub optimizeBones: Option<bool>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub optimizeMeshForGPU: Option<bool>,
    /**Threshold for angle distortion (in degrees) when generating secondary UV.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub secondaryUVAngleDistortion: Option<f32>,
    /**Threshold for area distortion when generating secondary UV.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub secondaryUVAreaDistortion: Option<f32>,
    /**Hard angle (in degrees) for generating secondary UV.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub secondaryUVHardAngle: Option<f32>,
    /**Method to use for handling margins when generating secondary UV.*/
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub secondaryUVMarginMethod: Option<i32>,
    /**The minimum lightmap resolution in texels per unit that the associated model is expected to have.*/
    /// f32: (2020.1.0a20 - 2022.2.0b16)
    pub secondaryUVMinLightmapResolution: Option<f32>,
    /**The minimum object scale that the associated model is expected to have.*/
    /// f32: (2020.1.0a20 - 2022.2.0b16)
    pub secondaryUVMinObjectScale: Option<f32>,
    /**Margin to be left between charts when packing secondary UV.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub secondaryUVPackMargin: Option<f32>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub skinWeightsMode: Option<i32>,
    /// bool: (3.4.0 - 3.4.0)
    pub splitTangentsAcrossUV: Option<bool>,
    /**Swap primary and secondary UV channels when importing.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub swapUVChannels: Option<bool>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub tangentImportMode: Option<i32>,
    /**Combine vertices that share the same position in space.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub weldVertices: Option<bool>,
}

/// Module is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub controlledByBuiltinPackage: bool,
    pub dependencies: Vec<String>,
    pub name: String,
    pub strippable: bool,
}

/// MonoAssemblyImporter is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct MonoAssemblyImporter {
    pub m_ExecutionOrder: Vec<(String, i32)>,
    pub m_FileIDToRecycleName: Vec<(i32, String)>,
    pub m_IconMap: Vec<(String, PPtr /*<Texture2D>*/)>,
    pub m_Name: String,
    pub m_NewHashIdentity: MdFour,
    pub m_OldHashIdentity: MdFour,
}

/// MonoBehaviour is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/MonoBehaviour.html):
/**
MonoBehaviour is a base class that many Unity scripts derive from.
MonoBehaviour offers life cycle functions that make it easier to develop with Unity.MonoBehaviours always exist as a Component of a GameObject, and can be instantiated with GameObject.AddComponent.  Objects that need to exist independently of a GameObject should derive from ScriptableObject instead.A MonoBehaviour can be deleted with Object.Destroy or Object.DestroyImmediate.  When the parent GameObject is destroyed all components are automatically deleted, including MonoBehaviours.After the underlying component is destroyed, the C# object for the MonoBehaviour remains in memory until garbage is collected. A MonoBehaviour in this state acts as if it is null. For example, it returns true for a "obj == null" check.

However, this class doesn't support the null-conditional operator  (?.) and the null-coalescing operator (??).When a MonoBehaviour is serialized, the value of C# fields are included according to Unity's Serialization rules. See Script Serialization for details.

The serialized data also includes internal properties, such as the reference to the MonoScript that tracks the implementation class for the object.For code samples, see the individual MonoBehaviour methods.Note: There is a checkbox for enabling or disabling MonoBehaviour in the Unity Editor.  It disables

functions when unticked.  If none of these functions are present in the script, the Unity Editor does not

display the checkbox.  The functions are:Start()
Update()
FixedUpdate()
LateUpdate()
OnGUI()
OnDisable()
OnEnable()See Also: The Deactivating GameObjects page in the manual.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MonoBehaviour {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Script: PPtr, /*<MonoScript>*/
}

/// MonoImporter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/MonoImporter.html):
/**
Represents a C# script in the project.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MonoImporter {
    pub executionOrder: i16,
    pub icon: PPtr, /*<Texture2D>*/
    pub m_DefaultReferences: Vec<(String, PPtr /*<Object>*/)>,
    /**The name of the object.*/
    pub m_Name: String,
    /**Get or set the AssetBundle name.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AssetBundleName: Option<String>,
    /**Get or set the AssetBundle variant.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AssetBundleVariant: Option<String>,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.0)
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_NewHashIdentity: Option<MdFour>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_OldHashIdentity: Option<MdFour>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /**Get or set any user data.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_UserData: Option<String>,
}

/// MonoManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct MonoManager {
    pub m_Scripts: Vec<PPtr /*<MonoScript>*/>,
    /// Vec<String>: (3.4.0 - 2020.1.0a20)
    pub m_AssemblyNames: Option<Vec<String>>,
    /// Vec<i32>: (2017.4.33f1 - 2020.1.0a20)
    pub m_AssemblyTypes: Option<Vec<i32>>,
    /// Vec<(i32, Hash128)>: (2020.3.42f1 - 2022.2.0b16)
    pub m_RuntimeClassHashes: Option<Vec<(i32, Hash128)>>,
    /// Vec<(Hash128, Hash128)>: (2020.3.42f1 - 2022.2.0b16)
    pub m_ScriptHashes: Option<Vec<(Hash128, Hash128)>>,
}

/// MonoScript is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/MonoScript.html):
/**
Representation of Script assets.
This class represents C# files stored in the project.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MonoScript {
    pub m_AssemblyName: String,
    pub m_ClassName: String,
    pub m_ExecutionOrder: i32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Namespace: String,
    pub m_PropertiesHash: Enum_u32__Hash128,
    /// bool: (3.4.0 - 2017.4.33f1)
    pub m_IsEditorScript: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_u32__Hash128 {
    u32(u32),
    Hash128(Hash128),
}

/// MovieImporter is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct MovieImporter {
    pub m_Name: String,
    pub m_Quality: f32,
    /// String: (5.6.0b2 - 2018.4.15f1)
    pub m_AssetBundleName: Option<String>,
    /// String: (5.6.0b2 - 2018.4.15f1)
    pub m_AssetBundleVariant: Option<String>,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2018.4.15f1)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.0)
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub m_LinearTexture: Option<bool>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_NewHashIdentity: Option<MdFour>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_OldHashIdentity: Option<MdFour>,
    /// String: (5.6.0b2 - 2018.4.15f1)
    pub m_UserData: Option<String>,
}

/// MovieTexture is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/MovieTexture.html):
/**
MovieTexture has been removed. Use VideoPlayer instead.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MovieTexture {
    /**The name of the object.*/
    pub m_Name: String,
    /// PPtr/*<AudioClip>*/: (3.4.0 - 2018.4.15f1)
    pub m_AudioClip: Option<PPtr /*<AudioClip>*/>,
    /// i32: (5.6.0b2 - 2018.4.15f1)
    pub m_ColorSpace: Option<i32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// bool: (3.4.0 - 2018.4.15f1)
    pub m_Loop: Option<bool>,
    /// Vec<u8>: (3.4.0 - 2018.4.15f1)
    pub m_MovieData: Option<Vec<u8>>,
}

/// MultiArtifactTestImporter is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiArtifactTestImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// MultiModeParameter is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiModeParameter {
    pub mode: i32,
    pub speed: MinMaxCurve,
    pub spread: f32,
    /// f32: (2017.4.33f1 - 2017.4.33f1)
    pub value: Option<f32>,
}

/// NScreenBridge is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct NScreenBridge {}

/// NameToObjectMap is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct NameToObjectMap {
    pub m_ObjectToName: Vec<(PPtr /*<Shader>*/, String)>,
}

/// NamedObject is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct NamedObject {
    pub m_Name: String,
}

/// NativeFormatImporter is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct NativeFormatImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_Name: String,
    pub m_UserData: String,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /// i64: (2017.4.33f1 - 2022.2.0b16)
    pub m_MainObjectFileID: Option<i64>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// NativeObjectType is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct NativeObjectType {
    pub m_Inner: NativeType,
}

/// NativeType is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct NativeType {
    pub a: i32,
    pub b: f32,
    pub embedded: EmbeddedNativeType,
}

/// NavMeshAgent is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AI.NavMeshAgent.html):
/**
Navigation mesh agent.
This component is attached to a mobile character in the game to allow it to navigate the Scene using the NavMesh. See the Navigation section of the manual for further details.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshAgent {
    /**The avoidance priority level.*/
    pub avoidancePriority: i32,
    /**The maximum acceleration of an agent as it follows a path, given in units / sec^2.*/
    pub m_Acceleration: f32,
    /**The type ID for the agent.*/
    pub m_AgentTypeID: i32,
    /**Maximum turning speed in (deg/s) while following a path.*/
    pub m_AngularSpeed: f32,
    /**Should the agent brake automatically to avoid overshooting the destination point?*/
    pub m_AutoBraking: bool,
    /**Should the agent attempt to acquire a new path if the existing path becomes invalid?*/
    pub m_AutoRepath: bool,
    /**Should the agent move across OffMeshLinks automatically?*/
    pub m_AutoTraverseOffMeshLink: bool,
    /**The relative vertical displacement of the owning GameObject.*/
    pub m_BaseOffset: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The height of the agent for purposes of passing under obstacles, etc.*/
    pub m_Height: f32,
    /**The level of quality of avoidance.*/
    pub m_ObstacleAvoidanceType: i32,
    /**The avoidance radius for the agent.*/
    pub m_Radius: f32,
    /**Maximum movement speed when following a path.*/
    pub m_Speed: f32,
    /**Stop within this distance from the target position.*/
    pub m_StoppingDistance: f32,
    pub m_WalkableMask: u32,
}

/// NavMeshAreaData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshAreaData {
    pub cost: f32,
    pub name: String,
}

/// NavMeshBuildDebugSettings is a sub class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AI.NavMeshBuildDebugSettings.html):
/**
Specify which of the temporary data generated while building the NavMesh should be retained in memory after the process has completed.
It is possible to collect and display in the Editor the intermediate data used in the process of building the navigation mesh using the NavMeshBuilder. This can help with diagnosing those situations when the resulting NavMesh isn’t of the expected shape.
Input Geometry, Regions, Polygonal Mesh Detail and Raw Contours shown after building the NavMesh with debug optionsThe process for computing a NavMesh comprises of several sequential steps:
i. decomposing the Scene's terrain and meshes into triangles;
ii. rasterizing the input triangles into a 3D voxel representation and finding ledges;
iii. partitioning the voxels lying at the surface into simpler horizontal regions;
iv. finding a tight-fitting contour for each of these regions;
v. simplifying the contours into polygonal shapes;
vi. creating a mesh of convex polygons based on all the contours combined;
vii. refining the polygonal mesh into a triangulated version that approximates better the Scene's original geometry.Through the use of the debug functionality the results from each stage can be captured and displayed separately, whereas normally they would get discarded when the NavMesh construction is completed.Depending on the Scene composition this debug data can be considerably large in size. It is stored in memory in a compressed manner but gets further expanded when being displayed.Notes:
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshBuildDebugSettings {
    /**Specify which types of debug data to collect when building the NavMesh.*/
    pub m_Flags: u8,
}

/// NavMeshBuildSettings is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AI.NavMeshBuildSettings.html):
/**
The NavMeshBuildSettings struct allows you to specify a collection of settings which describe the dimensions and limitations of a particular agent type.
You might want to define multiple NavMeshBuildSettings if your game involves characters with large differences in height, width or climbing ability.You can also use this struct to control the precision and granularity of the build process, by setting the voxel and tile sizes. Some of the values are coupled, meaning there are constraints on the values based on other values. For example, it’s not valid for agentClimb to be larger than agentHeight.
To help diagnose violations of these rules, a special method ValidationReport can be evaluated.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshBuildSettings {
    /**The maximum vertical step size an agent can take.*/
    pub agentClimb: f32,
    /**The height of the agent for baking in world units.*/
    pub agentHeight: f32,
    /**The radius of the agent for baking in world units.*/
    pub agentRadius: f32,
    /**The maximum slope angle which is walkable (angle in degrees).*/
    pub agentSlope: f32,
    /**The agent type ID the NavMesh will be baked for.*/
    pub agentTypeID: i32,
    pub cellSize: f32,
    /**Maximum agent drop height.*/
    pub ledgeDropHeight: f32,
    pub manualCellSize: Enum_bool__i32,
    pub manualTileSize: Enum_bool__i32,
    /**Maximum agent jump distance.*/
    pub maxJumpAcrossDistance: f32,
    /**The approximate minimum area of individual NavMesh regions.*/
    pub minRegionArea: f32,
    /**Sets the tile size in voxel units.*/
    pub tileSize: i32,
    /// bool: (5.6.0b2 - 5.6.0b2); i32: (2017.4.33f1 - 2021.2.16f1)
    pub accuratePlacement: Option<Enum_bool__i32>,
    /**Enables the creation of additional data needed to determine the height at any position on the NavMesh more accurately.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub buildHeightMesh: Option<i32>,
    /**Options for collecting debug data during the build process.*/
    /// NavMeshBuildDebugSettings: (2017.4.33f1 - 2022.2.0b16)
    pub debug: Option<NavMeshBuildDebugSettings>,
    /**The maximum number of worker threads that the build process can utilize when building a NavMesh with these settings.*/
    /// u32: (2020.3.42f1 - 2022.2.0b16)
    pub maxJobWorkers: Option<u32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub preserveTilesOutsideBounds: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_bool__i32 {
    bool(bool),
    i32(i32),
}

/// NavMeshData is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AI.NavMeshData.html):
/**
Contains and represents NavMesh data.
An object of this class can be used for creating instances of NavMeshes. See NavMesh.AddNavMeshData. The contained NavMesh can be built and updated using the build API. See NavMeshBuilder and methods therein.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshData {
    pub m_HeightMeshes: Vec<HeightMeshData>,
    pub m_Heightmaps: Vec<HeightmapData>,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_NavMeshBuildSettings: NavMeshBuildSettings,
    pub m_NavMeshTiles: Vec<NavMeshTileData>,
    pub m_OffMeshLinks: Vec<AutoOffMeshLinkData>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_AgentTypeID: Option<i32>,
    /**Gets or sets the world space position of the NavMesh data.*/
    /// Vector3f: (2017.4.33f1 - 2022.2.0b16)
    pub m_Position: Option<Vector3f>,
    /**Gets or sets the orientation of the NavMesh data.*/
    /// Quaternionf: (2017.4.33f1 - 2022.2.0b16)
    pub m_Rotation: Option<Quaternionf>,
    /**Returns the bounding volume of the input geometry used to build this NavMesh (Read Only).*/
    /// AABB: (2017.4.33f1 - 2022.2.0b16)
    pub m_SourceBounds: Option<AABB>,
}

/// NavMeshObstacle is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AI.NavMeshObstacle.html):
/**
An obstacle for NavMeshAgents to avoid.
A NavMeshObstacle is cylindrical in shape and can move around the surface of the NavMesh with a specified velocity. By default, the obstacle will only affect the agent's avoidance behaviour rather than the pathfinding. This means that the agent will ignore the obstacle when plotting a path but will sidestep around it while moving along the path. If carving is enabled, the obstacle will create a temporary "hole" in the NavMesh. The hole will be recognised by the pathfinding, so paths will be plotted to avoid the obstacle. This means that if, say, an obstacle blocks a narrow gap, the pathfinding will seek an alternative route to the target. Without carving, the agent will head for the gap but won't be able to pass until the obstacle is clear.See Also: NavMeshAgent.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshObstacle {
    pub m_Carve: bool,
    /**Should this obstacle be carved when it is constantly moving?*/
    pub m_CarveOnlyStationary: bool,
    /**The center of the obstacle, measured in the object's local space.*/
    pub m_Center: Vector3f,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    pub m_Extents: Vector3f,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_MoveThreshold: f32,
    /**The shape of the obstacle.*/
    pub m_Shape: i32,
    pub m_TimeToStationary: f32,
}

/// NavMeshProjectSettings is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshProjectSettings {
    pub areas: Vec<NavMeshAreaData>,
    pub m_LastAgentTypeID: i32,
    pub m_SettingNames: Vec<String>,
    pub m_Settings: Vec<NavMeshBuildSettings>,
}

/// NavMeshSettings is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshSettings {
    pub m_NavMeshData: PPtr, /*<NavMeshData>*/
}

/// NavMeshTileData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshTileData {
    pub m_Hash: Hash128,
    pub m_MeshData: Vec<u8>,
}

/// NetworkManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkManager {
    pub m_AssetToPrefab: Vec<(GUID, PPtr /*<GameObject>*/)>,
    pub m_DebugLevel: i32,
    pub m_Sendrate: f32,
}

/// NetworkView is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/NetworkView.html):
/**
The network view is the binding material of multiplayer games.
With this you can define exactly what is to be synchronized over the network and how
it should be done. Game objects can have NetworkView components which can be
configured to watch other components for the object.
For more information see the  Network View manual page and the component reference page.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkView {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_Observed: PPtr, /*<Component>*/
    pub m_StateSynchronization: i32,
    pub m_ViewID: NetworkViewID,
}

/// NetworkViewID is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/NetworkViewID.html):
/**
The NetworkViewID is a unique identifier for a network view instance in a multiplayer game.
It is imporatant that this is a unique number accross all clients and that they can generate
these numbers themselves, or else network synchronization will break.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkViewID {
    pub m_ID: u32,
    pub m_Type: u32,
}

/// NewAnimationTrack is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewAnimationTrack {
    pub m_ClassID: i32,
    pub m_Curves: Vec<Channel>,
    pub m_Name: String,
}

/// NoiseModule is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.NoiseModule.html):
/**
Script interface for the NoiseModule.
The Noise Module allows you to apply turbulence to the movement of your particles. Use the low quality settings to create computationally efficient Noise, or simulate smoother, richer Noise with the higher quality settings. You can also choose to define the behavior of the Noise individually for each axis.See Also: ParticleSystem, ParticleSystem.noise.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct NoiseModule {
    /**Higher frequency noise reduces the strength by a proportional amount, if enabled.*/
    pub damping: bool,
    /**Specifies whether the the NoiseModule is enabled or disabled.*/
    pub enabled: bool,
    /**Low values create soft, smooth noise, and high values create rapidly changing noise.*/
    pub frequency: f32,
    /**When combining each octave, scale the intensity by this amount.*/
    pub octaveMultiplier: f32,
    /**When combining each octave, zoom in by this amount.*/
    pub octaveScale: f32,
    pub octaves: i32,
    /**Generate 1D, 2D or 3D noise.*/
    pub quality: i32,
    /**Define how the noise values are remapped.*/
    pub remap: MinMaxCurve,
    /**Enable remapping of the final noise values, allowing for noise values to be translated into different values.*/
    pub remapEnabled: bool,
    /**Define how the noise values are remapped on the y-axis, when using the ParticleSystem.NoiseModule.separateAxes option.*/
    pub remapY: MinMaxCurve,
    /**Define how the noise values are remapped on the z-axis, when using the ParticleSystem.NoiseModule.separateAxes option.*/
    pub remapZ: MinMaxCurve,
    /**Scroll the noise map over the Particle System.*/
    pub scrollSpeed: MinMaxCurve,
    /**Control the noise separately for each axis.*/
    pub separateAxes: bool,
    /**How strong the overall noise effect is.*/
    pub strength: MinMaxCurve,
    /**Define the strength of the effect on the y-axis, when using the ParticleSystem.NoiseModule.separateAxes option.*/
    pub strengthY: MinMaxCurve,
    /**Define the strength of the effect on the z-axis, when using the ParticleSystem.NoiseModule.separateAxes option.*/
    pub strengthZ: MinMaxCurve,
    /**How much the noise affects the particle positions.*/
    /// MinMaxCurve: (2017.4.33f1 - 2022.2.0b16)
    pub positionAmount: Option<MinMaxCurve>,
    /**How much the noise affects the particle rotation, in degrees per second.*/
    /// MinMaxCurve: (2017.4.33f1 - 2022.2.0b16)
    pub rotationAmount: Option<MinMaxCurve>,
    /**How much the noise affects the particle sizes, applied as a multiplier on the size of each particle.*/
    /// MinMaxCurve: (2017.4.33f1 - 2022.2.0b16)
    pub sizeAmount: Option<MinMaxCurve>,
}

/// NonAlignedStruct is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct NonAlignedStruct {
    pub m_Bool: bool,
}

/// OcclusionArea is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/OcclusionArea.html):
/**
OcclusionArea is an area in which occlusion culling is performed.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct OcclusionArea {
    /**Center of the occlusion area relative to the transform.*/
    pub m_Center: Vector3f,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_IsViewVolume: bool,
    /**Size that the occlusion area will have.*/
    pub m_Size: Vector3f,
    /// bool: (3.4.0 - 3.4.0)
    pub m_IsTargetVolume: Option<bool>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_TargetResolution: Option<i32>,
}

/// OcclusionCullingData is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct OcclusionCullingData {
    pub m_Name: String,
    pub m_PVSData: Vec<u8>,
    pub m_Scenes: Vec<OcclusionScene>,
}

/// OcclusionCullingSettings is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct OcclusionCullingSettings {
    pub m_OcclusionCullingData: PPtr, /*<OcclusionCullingData>*/
    pub m_Portals: Vec<PPtr /*<OcclusionPortal>*/>,
    pub m_SceneGUID: GUID,
    pub m_StaticRenderers: Vec<PPtr /*<Renderer>*/>,
}

/// OcclusionPortal is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/OcclusionPortal.html):
/**
The portal for dynamically changing occlusion at runtime.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct OcclusionPortal {
    pub m_Center: Vector3f,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Gets / sets the portal's open state.*/
    pub m_Open: bool,
    pub m_Size: Vector3f,
}

/// OcclusionScene is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct OcclusionScene {
    pub indexPortals: i32,
    pub indexRenderers: i32,
    pub scene: GUID,
    pub sizePortals: i32,
    pub sizeRenderers: i32,
}

/// Oculus is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct Oculus {
    pub dashSupport: bool,
    pub sharedDepthBuffer: bool,
    /// bool: (2018.4.15f1 - 2020.1.0a20)
    pub lowOverheadMode: Option<bool>,
    /// bool: (2018.4.15f1 - 2020.1.0a20)
    pub protectedContext: Option<bool>,
    /// bool: (2018.4.15f1 - 2020.1.0a20)
    pub v2Signing: Option<bool>,
}

/// OffMeshLink is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AI.OffMeshLink.html):
/**
Link allowing movement outside the planar navigation mesh.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct OffMeshLink {
    /**Is link active.*/
    pub m_Activated: bool,
    pub m_AgentTypeID: i32,
    pub m_AreaIndex: u32,
    /**Automatically update endpoints.*/
    pub m_AutoUpdatePositions: bool,
    /**Can link be traversed in both directions.*/
    pub m_BiDirectional: bool,
    /**Modify pathfinding cost for the link.*/
    pub m_CostOverride: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    pub m_End: PPtr, /*<Transform>*/
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_Start: PPtr, /*<Transform>*/
}

/// OffsetPtr is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct OffsetPtr {
    pub data: Clip,
}

/// Output is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub hasEmptyFontData: bool,
}

/// PPtrCurve is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct PPtrCurve {
    pub attribute: String,
    pub classID: i32,
    pub curve: Vec<PPtrKeyframe>,
    pub path: String,
    pub script: PPtr, /*<MonoScript>*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub flags: Option<i32>,
}

/// PPtrKeyframe is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct PPtrKeyframe {
    pub time: f32,
    pub value: PPtr, /*<Object>*/
}

/// PackageManifest is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManifest {
    pub m_Name: String,
    pub m_Script: String,
}

/// PackageManifestImporter is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManifestImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// PackedAssets is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Build.Reporting.PackedAssets.html):
/**
An extension to the BuildReport class that tracks how Assets contribute to the size of the build.
The build process generates a PackedAssets report for each .sharedAssets or .resource file, or for each AssetBundle.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PackedAssets {
    /**An array of PackedAssetInfo objects that holds information about the Assets that are included in the PackedAssets bundle, such as packed Asset size and type.*/
    pub m_Contents: Vec<BuildReportPackedAssetInfo>,
    /**The header size of the packed Asset file.*/
    pub m_Overhead: u64,
    /**The file path to the Asset package, relative to the Data folder of the build.*/
    pub m_ShortPath: String,
    /// u32: (5.6.0b2 - 2020.3.42f1)
    pub m_File: Option<u32>,
}

/// PackedBitVector is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackedBitVector {
    pub m_BitSize: u8,
    pub m_Data: Vec<u8>,
    pub m_NumItems: u32,
    pub m_Range: f32,
    pub m_Start: f32,
}

/// PackingSettings is a sub class of the Unity engine since version 2020.1.0a20.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackingSettings {
    pub allowAlphaSplitting: bool,
    pub blockOffset: i32,
    pub enableRotation: bool,
    pub enableTightPacking: bool,
    pub padding: i32,
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub enableAlphaDilation: Option<bool>,
}

/// Parameter is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub m_GUID: GUID,
    pub m_ParameterName: String,
}

/// ParentConstraint is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.ParentConstraint.html):
/**
Constrains the orientation and translation of an object to one or more source objects. The constrained object behaves as if it is in the hierarchy of the sources.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ParentConstraint {
    pub m_AffectRotationX: bool,
    pub m_AffectRotationY: bool,
    pub m_AffectRotationZ: bool,
    pub m_AffectTranslationX: bool,
    pub m_AffectTranslationY: bool,
    pub m_AffectTranslationZ: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The rotation used when the sources have a total weight of 0.*/
    pub m_RotationAtRest: Vector3f,
    /**The rotation offsets from the constrained orientation.*/
    pub m_RotationOffsets: Vec<Vector3f>,
    pub m_Sources: Vec<ConstraintSource>,
    /**The position of the object in local space, used when the sources have a total weight of 0.*/
    pub m_TranslationAtRest: Vector3f,
    /**The translation offsets from the constrained orientation.*/
    pub m_TranslationOffsets: Vec<Vector3f>,
    /**The weight of the constraint component.*/
    pub m_Weight: f32,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_Active: Option<bool>,
    /// bool: (2018.4.15f1 - 2021.2.16f1)
    pub m_IsContraintActive: Option<bool>,
}

/// ParserBindChannels is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParserBindChannels {
    pub m_Channels: Vec<ShaderBindChannel>,
    pub m_SourceMap: i32,
}

/// ParticleAnimator is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticleAnimator {
    pub autodestruct: bool,
    pub damping: f32,
    pub force: Vector3f,
    pub localRotationAxis: Vector3f,
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub rndForce: Vector3f,
    pub sizeGrow: f32,
    pub stopSimulation: bool,
    pub worldRotationAxis: Vector3f,
    /// bool: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "Does Animate Color?")]
    pub Does_Animate_Color_: Option<bool>,
    /// ColorRGBA: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "colorAnimation[0]")]
    pub colorAnimation_0_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "colorAnimation[1]")]
    pub colorAnimation_1_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "colorAnimation[2]")]
    pub colorAnimation_2_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "colorAnimation[3]")]
    pub colorAnimation_3_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "colorAnimation[4]")]
    pub colorAnimation_4_: Option<ColorRGBA>,
}

/// ParticleRenderer is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticleRenderer {
    pub m_CameraVelocityScale: f32,
    pub m_CastShadows: Enum_bool__u8,
    pub m_Enabled: bool,
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_LengthScale: f32,
    pub m_LightmapIndex: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_Materials: Vec<PPtr /*<Material>*/>,
    pub m_MaxParticleSize: f32,
    pub m_ReceiveShadows: Enum_bool__u8,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    pub m_StretchParticles: i32,
    pub m_VelocityScale: f32,
    /// UVAnimation: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "UV Animation")]
    pub UV_Animation: Option<UVAnimation>,
    /// u8: (2017.4.33f1 - 2017.4.33f1)
    pub m_DynamicOccludee: Option<u8>,
    /// u8: (5.6.0b2 - 2017.4.33f1)
    pub m_LightProbeUsage: Option<u8>,
    /// PPtr/*<GameObject>*/: (5.6.0b2 - 2017.4.33f1)
    pub m_LightProbeVolumeOverride: Option<PPtr /*<GameObject>*/>,
    /// u16: (5.6.0b2 - 2017.4.33f1)
    pub m_LightmapIndexDynamic: Option<u16>,
    /// Vector4f: (5.6.0b2 - 2017.4.33f1)
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    /// u8: (5.6.0b2 - 2017.4.33f1)
    pub m_MotionVectors: Option<u8>,
    /// PPtr/*<Transform>*/: (5.6.0b2 - 2017.4.33f1)
    pub m_ProbeAnchor: Option<PPtr /*<Transform>*/>,
    /// u8: (5.6.0b2 - 2017.4.33f1)
    pub m_ReflectionProbeUsage: Option<u8>,
    /// i16: (5.6.0b2 - 2017.4.33f1)
    pub m_SortingLayer: Option<i16>,
    /// i32: (2017.4.33f1 - 2017.4.33f1)
    pub m_SortingLayerID: Option<i32>,
    /// i16: (5.6.0b2 - 2017.4.33f1)
    pub m_SortingOrder: Option<i16>,
    /// StaticBatchInfo: (5.6.0b2 - 2017.4.33f1)
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    /// Vec<u32>: (3.4.0 - 3.4.0)
    pub m_SubsetIndices: Option<Vec<u32>>,
}

/// ParticleSystem is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.html):
/**
Script interface for the Built-in Particle System. Unity's powerful and versatile particle system implementation.
General parameters

The Particle System's general parameters are kept inside a special Main module. These parameters are visible in the Inspector above all the other modules:In script, these parameters are accessible through ParticleSystem.main.Accessing module propertiesParticle System properties are grouped by the module they belong to, such as ParticleSystem.noise and ParticleSystem.emission. These properties are structs, but do not behave like normal C# structs. They are simply interfaces directly into the native code, so it is important to know how to use them, compared to a normal C# struct.The key difference is that it is not necessary to assign the struct back to the Particle System component. When you set any property on a module struct, Unity immediately assigns that value to the Particle System.Also, because each module is a struct, you must cache it in a local variable before you can assign any new values to the module. For example, instead of:
ParticleSystem.emission.enabled = true;    // Doesn't compile

write:
var emission = ParticleSystem.emission;    // Stores the module in a local variable
emission.enabled = true;    // Applies the new value directly to the Particle SystemModule effect multipliersEvery module has special multiplier properties that allow you to change the overall effect of a curve without having to edit the curve itself. These multiplier properties are all named after the curve they affect - for instance ParticleSystem.emission.rateMultiplier controls the overall effect of ParticleSystem.emission.rate in a given system.Constant value shorthandParameters support a shorthand notation for simple constant values. To set a constant value for a parameter, all you need to do is assign a number to it. It is not necessary to create a MinMaxCurve or MinMaxGradient object in the ParticleSystemCurveMode.Constant mode.For example, instead of:
var emission = ParticleSystem.emission;
emission.rate = new ParticleSystem.MinMaxCurve(5.0f);

write:
var emission = ParticleSystem.emission;
emission.rate = 5.0f;Performance note: When setting properties on particle modules, the settings are passed immediately into native code. This gives the best performance. This means that setting properties on a module struct doesn't set something in script that requires setting back to the Particle System; it all happens automatically.See Also: Particle.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticleSystem {
    pub ClampVelocityModule: ClampVelocityModule,
    pub CollisionModule: CollisionModule,
    pub ColorBySpeedModule: ColorBySpeedModule,
    pub ColorModule: ColorModule,
    pub CustomDataModule: CustomDataModule,
    pub EmissionModule: EmissionModule,
    pub ExternalForcesModule: ExternalForcesModule,
    pub ForceModule: ForceModule,
    pub InheritVelocityModule: InheritVelocityModule,
    pub InitialModule: InitialModule,
    pub LightsModule: LightsModule,
    pub NoiseModule: NoiseModule,
    pub RotationBySpeedModule: RotationBySpeedModule,
    pub RotationModule: RotationModule,
    pub ShapeModule: ShapeModule,
    pub SizeBySpeedModule: SizeBySpeedModule,
    pub SizeModule: SizeModule,
    pub SubModule: SubModule,
    pub TrailModule: TrailModule,
    pub TriggerModule: TriggerModule,
    pub UVModule: UVModule,
    pub VelocityModule: VelocityModule,
    pub autoRandomSeed: bool,
    pub lengthInSec: f32,
    pub looping: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub moveWithCustomTransform: PPtr, /*<Transform>*/
    pub moveWithTransform: i32,
    pub playOnAwake: bool,
    pub prewarm: bool,
    /**Override the random seed used for the Particle System emission.*/
    pub randomSeed: i32,
    pub scalingMode: i32,
    pub simulationSpeed: f32,
    pub startDelay: MinMaxCurve,
    /// LifetimeByEmitterSpeedModule: (2020.1.0a20 - 2022.2.0b16)
    pub LifetimeByEmitterSpeedModule: Option<LifetimeByEmitterSpeedModule>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub cullingMode: Option<i32>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub emitterVelocityMode: Option<i32>,
    /// Vector2f: (2018.4.15f1 - 2022.2.0b16)
    pub ringBufferLoopRange: Option<Vector2f>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub ringBufferMode: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub stopAction: Option<i32>,
    /// bool: (2017.4.33f1 - 2020.3.42f1)
    pub useRigidbodyForVelocity: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub useUnscaledTime: Option<bool>,
}

/// ParticleSystemEmissionBurst is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticleSystemEmissionBurst {
    pub countCurve: MinMaxCurve,
    pub cycleCount: i32,
    pub repeatInterval: f32,
    pub time: f32,
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub probability: Option<f32>,
}

/// ParticleSystemForceField is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystemForceField.html):
/**
Script interface for Particle System Force Fields.
Particle System Force Fields can be used to influence groups of particles that enter each field's zone of influence.The shape of the Force Field can be set to a variety of shapes, and how the particles are affected is controlled by various properties in the Force Field.As part of choosing the shape, you may define a start and end range. The end range describes the maximum extent of the shape, and the start range can be used to create a hollow shape.A number of forces can be applied to particles that are within this volume: directional, gravitational, rotational, drag, and a vector field.The settings for each type of force make use of the MinMaxCurve type, which is also used in the Particle System. This type allows you to set simple uniform values, or more complicated values that vary per-particle, and vary over the lifetime of each particle.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticleSystemForceField {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_Parameters: ParticleSystemForceFieldParameters,
}

/// ParticleSystemForceFieldParameters is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticleSystemForceFieldParameters {
    pub m_DirectionCurveX: MinMaxCurve,
    pub m_DirectionCurveY: MinMaxCurve,
    pub m_DirectionCurveZ: MinMaxCurve,
    pub m_DragCurve: MinMaxCurve,
    pub m_EndRange: f32,
    pub m_GravityCurve: MinMaxCurve,
    pub m_GravityFocus: f32,
    pub m_Length: f32,
    pub m_MultiplyDragByParticleSize: bool,
    pub m_MultiplyDragByParticleVelocity: bool,
    pub m_RotationAttractionCurve: MinMaxCurve,
    pub m_RotationRandomness: Vector2f,
    pub m_RotationSpeedCurve: MinMaxCurve,
    pub m_Shape: i32,
    pub m_StartRange: f32,
    pub m_VectorField: PPtr, /*<Texture3D>*/
    pub m_VectorFieldAttractionCurve: MinMaxCurve,
    pub m_VectorFieldSpeedCurve: MinMaxCurve,
}

/// ParticleSystemRenderer is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystemRenderer.html):
/**
Use this class to render particles on to the screen.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticleSystemRenderer {
    /**How much do the particles stretch depending on the Camera's speed.*/
    pub m_CameraVelocityScale: f32,
    pub m_CastShadows: u8,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**How much are the particles stretched in their direction of motion, defined as the length of the particle compared to its width.*/
    pub m_LengthScale: f32,
    /**The light probe interpolation type.*/
    pub m_LightProbeUsage: u8,
    pub m_LightProbeVolumeOverride: PPtr, /*<GameObject>*/
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    pub m_Materials: Vec<PPtr /*<Material>*/>,
    /**Clamp the maximum particle size.*/
    pub m_MaxParticleSize: f32,
    /**The Mesh that the particle uses instead of a billboarded Texture.*/
    pub m_Mesh: PPtr, /*<Mesh>*/
    pub m_Mesh1: PPtr, /*<Mesh>*/
    pub m_Mesh2: PPtr, /*<Mesh>*/
    pub m_Mesh3: PPtr, /*<Mesh>*/
    /**Clamp the minimum particle size.*/
    pub m_MinParticleSize: f32,
    pub m_MotionVectors: u8,
    /**Specifies how to calculate lighting for the billboard.*/
    pub m_NormalDirection: f32,
    /**Modify the pivot point used for rotating particles.*/
    pub m_Pivot: Vector3f,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    pub m_ProbeAnchor: PPtr, /*<Transform>*/
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: u8,
    /**Should reflection probes be used for this Renderer?*/
    pub m_ReflectionProbeUsage: u8,
    pub m_RenderAlignment: i32,
    /**Specifies how the system draws particles.*/
    pub m_RenderMode: u16,
    /**Specifies how to sort particles within a system.*/
    pub m_SortMode: u16,
    /**Biases Particle System sorting amongst other transparencies.*/
    pub m_SortingFudge: f32,
    pub m_SortingLayer: i16,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    pub m_StaticBatchInfo: StaticBatchInfo,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    pub m_UseCustomVertexStreams: bool,
    /**Specifies how much particles stretch depending on their velocity.*/
    pub m_VelocityScale: f32,
    pub m_VertexStreams: Vec<u8>,
    /**Allow billboard particles to roll around their z-axis.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_AllowRoll: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_ApplyActiveColorSpace: Option<bool>,
    /// u8: (2017.4.33f1 - 2022.2.0b16)
    pub m_DynamicOccludee: Option<u8>,
    /**Enables GPU Instancing on platforms that support it.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_EnableGPUInstancing: Option<bool>,
    /**Flip a percentage of the particles, along each axis.*/
    /// Vector3f: (2018.4.15f1 - 2022.2.0b16)
    pub m_Flip: Option<Vector3f>,
    /**Enables freeform stretching behavior.*/
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub m_FreeformStretching: Option<bool>,
    /**Specifies how the Particle System Renderer interacts with SpriteMask.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_MaskInteraction: Option<i32>,
    /**Specifies how the system randomly assigns meshes to particles.*/
    /// u8: (2021.2.16f1 - 2022.2.0b16)
    pub m_MeshDistribution: Option<u8>,
    /// f32: (2021.2.16f1 - 2022.2.0b16)
    pub m_MeshWeighting: Option<f32>,
    /// f32: (2021.2.16f1 - 2022.2.0b16)
    pub m_MeshWeighting1: Option<f32>,
    /// f32: (2021.2.16f1 - 2022.2.0b16)
    pub m_MeshWeighting2: Option<f32>,
    /// f32: (2021.2.16f1 - 2022.2.0b16)
    pub m_MeshWeighting3: Option<f32>,
    /// u8: (2020.1.0a20 - 2022.2.0b16)
    pub m_RayTraceProcedural: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0f4 - 2022.2.0b16)
    pub m_RayTracingMode: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on.*/
    /// u32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RenderingLayerMask: Option<u32>,
    /**Rotate the particles based on the direction they are stretched in. This is added on top of other particle rotation.*/
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub m_RotateWithStretchDirection: Option<bool>,
    /**Apply a shadow bias to prevent self-shadowing artifacts. The specified value is the proportion of the particle size.*/
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub m_ShadowBias: Option<f32>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_SortingLayerID: Option<i32>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.2.16f1 - 2022.2.0b16)
    pub m_StaticShadowCaster: Option<u8>,
}

/// PerLODSettings is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct PerLODSettings {
    pub castShadows: bool,
    pub enableBump: bool,
    pub enableHue: bool,
    pub height: f32,
    pub receiveShadows: bool,
    pub reflectionProbeUsage: i32,
    pub useLightProbes: bool,
    pub windQuality: i32,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub enableSubsurface: Option<bool>,
}

/// PerformanceReportingManager is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceReportingManager {}

/// PerformanceReportingSettings is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Analytics.PerformanceReportingSettings.html):
/**
Normally performance reporting is enabled from the Services window, but if writing your own editor extension, this API can be used.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceReportingSettings {
    pub m_Enabled: bool,
}

/// PhysicMaterial is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PhysicMaterial.html):
/**
Physics material describes how to handle colliding objects (friction, bounciness).
See Also: Collider.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicMaterial {
    /**Determines how the bounciness is combined.*/
    pub bounceCombine: i32,
    /**How bouncy is the surface? A value of 0 will not bounce. A value of 1 will bounce without any loss of energy.*/
    pub bounciness: f32,
    /**The friction used when already moving.  This value is usually between 0 and 1.*/
    pub dynamicFriction: f32,
    /**Determines how the friction is combined.*/
    pub frictionCombine: i32,
    /**The name of the object.*/
    pub m_Name: String,
    /**The friction coefficient used when an object is lying on a surface.*/
    pub staticFriction: f32,
    /// f32: (3.4.0 - 3.4.0)
    pub dynamicFriction2: Option<f32>,
    /// Vector3f: (3.4.0 - 3.4.0)
    pub frictionDirection2: Option<Vector3f>,
    /// f32: (3.4.0 - 3.4.0)
    pub staticFriction2: Option<f32>,
}

/// Physics2DSettings is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct Physics2DSettings {
    pub m_AngularSleepTolerance: f32,
    pub m_BaumgarteScale: f32,
    pub m_BaumgarteTimeOfImpactScale: f32,
    pub m_DefaultMaterial: PPtr, /*<PhysicsMaterial2D>*/
    pub m_Gravity: Vector2f,
    pub m_LayerCollisionMatrix: Vec<u32>,
    pub m_LinearSleepTolerance: f32,
    pub m_MaxAngularCorrection: f32,
    pub m_MaxLinearCorrection: f32,
    pub m_MaxRotationSpeed: f32,
    pub m_MaxTranslationSpeed: f32,
    pub m_PositionIterations: i32,
    pub m_QueriesHitTriggers: bool,
    pub m_QueriesStartInColliders: bool,
    pub m_TimeToSleep: f32,
    pub m_VelocityIterations: i32,
    pub m_VelocityThreshold: f32,
    /// bool: (2017.4.33f1 - 2019.3.0f4)
    pub m_AutoSimulation: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_AutoSyncTransforms: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_CallbacksOnDisable: Option<bool>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub m_ChangeStopsCallbacks: Option<bool>,
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_DefaultContactOffset: Option<f32>,
    /// PhysicsJobOptions2D: (2018.4.15f1 - 2022.2.0b16)
    pub m_JobOptions: Option<PhysicsJobOptions2D>,
    /// f32: (5.6.0b2 - 5.6.0b2)
    pub m_MinPenetrationForPenalty: Option<f32>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_ReuseCollisionCallbacks: Option<bool>,
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub m_SimulationMode: Option<i32>,
}

/// PhysicsJobOptions2D is a sub class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PhysicsJobOptions2D.html):
/**
A set of options that control how physics operates when using the job system to multithread the physics simulation.
Multithreaded physics is currently an experimental feature. As such, many options are exposed that allow performance configuration that may not be available when the feature moves out of experimental status.A physics simulation executes in the following discrete stages:• Find New Contacts
• Contact Collision
• Discrete Solver (Clear Island Flags -> Discrete Island Traversal -> Discrete Island Solver -> Synchronize Fixtures -> Find New Contacts)
• Continuous Solver (Clear Island Flags > Continuous Island Traversal -> Discrete Island Solver -> Synchronize Fixtures -> Find New Contacts)
• Clear Body Forces
• Update Trigger ContactsThese stages execute in the order given above. Each stage is run as a job "task". Each task executes sub job tasks, which are shown in parenthesis above. When executing a job, physics simulation may process bodies, contacts, joints, and so on, across multiple job threads. You can task each of these threads with executing a specific number of items, such as bodies, contacts and joints. Many of the options provided here allow you to control the minimum number of items assigned to each job. Raising the minimum can reduce the number of jobs required. This is because running a lot of jobs, each processing only a few items, is usually not very efficient. The default settings provide a decent performance to job balance, however you are free to experiment.Additionally, prior to the simulation being run, Rigidbody2D interpolation/extrapolation poses are stored ready for per-frame interpolation/extrapolation.  These are also executed using the job system and are controlled here.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsJobOptions2D {
    /**Controls the minimum number of bodies to be cleared in each simulation job.*/
    pub m_ClearBodyForcesPerJob: i32,
    /**Controls the minimum number of flags to be cleared in each simulation job.*/
    pub m_ClearFlagsPerJob: i32,
    /**Controls the minimum number of contacts to collide in each simulation job.*/
    pub m_CollideContactsPerJob: i32,
    /**Controls the minimum number of nearest contacts to find in each simulation job.*/
    pub m_FindNearestContactsPerJob: i32,
    /**Controls the minimum number of Rigidbody2D being interpolated in each simulation job.*/
    pub m_InterpolationPosesPerJob: i32,
    /**Controls the minimum number of bodies to solve in each simulation job when performing island solving.*/
    pub m_IslandSolverBodiesPerJob: i32,
    /**Scales the cost of each body during discrete island solving.*/
    pub m_IslandSolverBodyCostScale: i32,
    /**Scales the cost of each contact during discrete island solving.*/
    pub m_IslandSolverContactCostScale: i32,
    /**Controls the minimum number of contacts to solve in each simulation job when performing island solving.*/
    pub m_IslandSolverContactsPerJob: i32,
    /**The minimum threshold cost of all bodies, contacts and joints in an island during discrete island solving.*/
    pub m_IslandSolverCostThreshold: i32,
    /**Scales the cost of each joint during discrete island solving.*/
    pub m_IslandSolverJointCostScale: i32,
    /**Controls the minimum number of new contacts to find in each simulation job.*/
    pub m_NewContactsPerJob: i32,
    /**Controls the minimum number of fixtures to synchronize in the broadphase during continuous island solving in each simulation job.*/
    pub m_SyncContinuousFixturesPerJob: i32,
    /**Controls the minimum number of fixtures to synchronize in the broadphase during discrete island solving in each simulation job.*/
    pub m_SyncDiscreteFixturesPerJob: i32,
    /**Controls the minimum number of trigger contacts to update in each simulation job.*/
    pub m_UpdateTriggerContactsPerJob: i32,
    /**Should physics simulation sort multi-threaded results to maintain processing order consistency?*/
    pub useConsistencySorting: bool,
    /**Should physics simulation use multithreading?*/
    pub useMultithreading: bool,
}

/// PhysicsManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsManager {
    pub m_BounceThreshold: f32,
    pub m_DefaultMaterial: PPtr, /*<PhysicMaterial>*/
    pub m_Gravity: Vector3f,
    pub m_LayerCollisionMatrix: Vec<u32>,
    /// bool: (2017.4.33f1 - 2021.2.16f1)
    pub m_AutoSimulation: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_AutoSyncTransforms: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_BroadphaseType: Option<i32>,
    /// Vector3f: (2019.3.0f4 - 2022.2.0b16)
    pub m_ClothGravity: Option<Vector3f>,
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ClothInterCollisionDistance: Option<f32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_ClothInterCollisionSettingsToggle: Option<bool>,
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ClothInterCollisionStiffness: Option<f32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ContactPairsMode: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ContactsGeneration: Option<i32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_DefaultContactOffset: Option<f32>,
    /// f32: (2019.3.0f4 - 2022.2.0b16)
    pub m_DefaultMaxAngularSpeed: Option<f32>,
    /// f32: (2020.3.42f1 - 2022.2.0b16)
    pub m_DefaultMaxDepenetrationVelocity: Option<f32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_DefaultSolverIterations: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_DefaultSolverVelocityIterations: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_EnableAdaptiveForce: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_EnableEnhancedDeterminism: Option<bool>,
    /// bool: (5.6.0b2 - 5.6.0b2)
    pub m_EnablePCM: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_EnableUnifiedHeightmaps: Option<bool>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_FrictionType: Option<i32>,
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub m_ImprovedPatchFriction: Option<bool>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_InvokeCollisionCallbacks: Option<bool>,
    /// f32: (3.4.0 - 3.4.0)
    pub m_MaxAngularVelocity: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    pub m_MinPenetrationForPenalty: Option<f32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_QueriesHitBackfaces: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_QueriesHitTriggers: Option<bool>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_RaycastsHitTriggers: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_ReuseCollisionCallbacks: Option<bool>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_SimulationMode: Option<i32>,
    /// f32: (3.4.0 - 3.4.0)
    pub m_SleepAngularVelocity: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_SleepThreshold: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    pub m_SleepVelocity: Option<f32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_SolverIterationCount: Option<i32>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_SolverType: Option<i32>,
    /// AABB: (2017.4.33f1 - 2022.2.0b16)
    pub m_WorldBounds: Option<AABB>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_WorldSubdivisions: Option<i32>,
}

/// PhysicsMaterial2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PhysicsMaterial2D.html):
/**
Asset type that defines the surface properties of a Collider2D.
Note: the 3D equivalent of this class is spelt as "PhysicMaterial" with no S, but this class is spelt "PhysicsMaterial" with an S.See Also: PhysicMaterial class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsMaterial2D {
    /**The degree of elasticity during collisions.*/
    pub bounciness: f32,
    /**Coefficient of friction.*/
    pub friction: f32,
    /**The name of the object.*/
    pub m_Name: String,
}

/// PhysicsShape is a sub class of the Unity engine since version 2021.2.16f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsShape {
    pub m_AdjacentEnd: Vector2f,
    pub m_AdjacentStart: Vector2f,
    pub m_Radius: f32,
    pub m_ShapeType: i32,
    pub m_UseAdjacentEnd: i32,
    pub m_UseAdjacentStart: i32,
    pub m_VertexCount: i32,
    pub m_VertexStartIndex: i32,
}

/// PhysicsShapeGroup2D is a sub class of the Unity engine since version 2021.2.16f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PhysicsShapeGroup2D.html):
/**
Represents a group of PhysicsShape2D and their geometry.
A shape group represents multiple PhysicsShape2D of the same or mixed PhysicsShapeType2D along with their geometry. It is comprised of a single list of vertices (GetShapeVertices) along with a list of PhysicsShape2D which refer to specific ranges of those vertices i.e. they index into the list of vertices. Some shape types (PhysicsShapeType2D) use a fixed number of vertices and some use a variable number of vertices therefore this single vertices list is a compact and efficient representation for multiple PhysicsShape2D in a group.The shape group can be created by using the following methods:
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsShapeGroup2D {
    pub m_Shapes: Vec<PhysicsShape>,
    pub m_Vertices: Vec<Vector2f>,
}

/// Pipeline is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline {
    pub m_GameObject: PPtr, /*<GameObject>*/
}

/// PlatformEffector2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PlatformEffector2D.html):
/**
Applies "platform" behaviour such as one-way collisions etc.
When the source Collider2D is a trigger, the effector will apply forces whenever the target Collider2D overlaps the source.  When the source Collider isn't a trigger, the effector will apply forces whenever the target Collider2D is in contact with the source only.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformEffector2D {
    /**The mask used to select specific layers allowed to interact with the effector.*/
    pub m_ColliderMask: BitField,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The rotational offset angle from the local 'up'.*/
    pub m_RotationalOffset: f32,
    /**The angle of an arc that defines the sides of the platform centered on the local 'left' and 'right' of the effector. Any collision normals within this arc are considered for the 'side' behaviours.*/
    pub m_SideArc: f32,
    /**The angle of an arc that defines the surface of the platform centered of the local 'up' of the effector.*/
    pub m_SurfaceArc: f32,
    /**Should the collider-mask be used or the global collision matrix?*/
    pub m_UseColliderMask: bool,
    /**Should the one-way collision behaviour be used?*/
    pub m_UseOneWay: bool,
    /**Ensures that all contacts controlled by the one-way behaviour act the same.*/
    pub m_UseOneWayGrouping: bool,
    /**Should bounce be used on the platform sides?*/
    pub m_UseSideBounce: bool,
    /**Should friction be used on the platform sides?*/
    pub m_UseSideFriction: bool,
}

/// PlatformModuleSetup is a  class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformModuleSetup {
    pub modules: Vec<Module>,
}

/// PlatformSettings is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformSettings {
    pub m_AllowsAlphaSplitting: bool,
    pub m_BuildTarget: String,
    pub m_CompressionQuality: i32,
    pub m_CrunchedCompression: bool,
    pub m_MaxTextureSize: i32,
    pub m_Overridden: bool,
    pub m_TextureCompression: i32,
    pub m_TextureFormat: i32,
}

/// PlatformSettingsData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformSettingsData {
    pub enabled: bool,
    pub settings: Vec<(String, String)>,
}

/// PlatformShaderDefines is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformShaderDefines {
    pub defines_Tier1: Vec<u32>,
    pub defines_Tier2: Vec<u32>,
    pub defines_Tier3: Vec<u32>,
    pub shaderPlatform: i32,
}

/// PlayableDirector is a  class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Playables.PlayableDirector.html):
/**
Instantiates a PlayableAsset and controls playback of Playable objects.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableDirector {
    pub m_DirectorUpdateMode: i32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    pub m_ExposedReferences: ExposedReferenceTable,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_InitialState: i32,
    /**The time at which the Playable should start when first played.*/
    pub m_InitialTime: f64,
    /**The PlayableAsset that is used to instantiate a playable for playback.*/
    pub m_PlayableAsset: PPtr, /*<Object>*/
    pub m_SceneBindings: Vec<DirectorGenericBinding>,
    pub m_WrapMode: i32,
}

/// PlayerSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PlayerSettings.html):
/**
Player Settings is where you define various parameters for the final game that you will build in Unity. Some of these values are used in the Resolution Dialog that launches when you open a standalone game.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerSettings {
    pub AndroidProfiler: bool,
    pub allowedAutorotateToLandscapeLeft: bool,
    pub allowedAutorotateToLandscapeRight: bool,
    pub allowedAutorotateToPortrait: bool,
    pub allowedAutorotateToPortraitUpsideDown: bool,
    pub captureSingleScreen: bool,
    pub companyName: String,
    pub defaultScreenHeight: i32,
    pub defaultScreenHeightWeb: i32,
    pub defaultScreenOrientation: i32,
    pub defaultScreenWidth: i32,
    pub defaultScreenWidthWeb: i32,
    pub productName: String,
    pub runInBackground: bool,
    pub targetDevice: i32,
    pub use32BitDisplayBuffer: bool,
    pub useMacAppStoreValidation: bool,
    pub useOSAutorotation: bool,
    pub usePlayerLog: bool,
    /// Hash128: (2020.3.42f1 - 2022.2.0b16)
    pub AID: Option<Hash128>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub AndroidEnableSustainedPerformanceMode: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub AndroidFilterTouchesWhenObscured: Option<bool>,
    /// String: (3.4.0 - 3.4.0)
    pub AndroidLicensePublicKey: Option<String>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub D3DHDRBitDepth: Option<i32>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    #[serde(alias = "Force IOS Speakers When Recording")]
    pub Force_IOS_Speakers_When_Recording: Option<bool>,
    /// bool: (3.4.0 - 3.4.0)
    #[serde(alias = "Override IPod Music")]
    pub Override_IPod_Music: Option<bool>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    #[serde(alias = "Prepare IOS For Recording")]
    pub Prepare_IOS_For_Recording: Option<bool>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub accelerometerFrequency: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub activeInputHandler: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub allowFullscreenSwitch: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub androidBlitType: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub androidDefaultWindowHeight: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub androidDefaultWindowWidth: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub androidFullscreenMode: Option<i32>,
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub androidMaxAspectRatio: Option<f32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub androidMinimumWindowHeight: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub androidMinimumWindowWidth: Option<i32>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub androidRenderOutsideSafeArea: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub androidResizableWindow: Option<bool>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub androidShowActivityIndicatorOnLoading: Option<i32>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub androidStartInFullscreen: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub androidSupportedAspectRatio: Option<i32>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub androidUseSwappy: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub bakeCollisionMeshes: Option<bool>,
    /// String: (5.6.0b2 - 5.6.0b2)
    pub bundleIdentifier: Option<String>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub bundleVersion: Option<String>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub cloudEnabled: Option<bool>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub cloudProjectId: Option<String>,
    /// Vector2f: (5.6.0b2 - 2022.2.0b16)
    pub cursorHotspot: Option<Vector2f>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub d3d11FullscreenMode: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub d3d9FullscreenMode: Option<i32>,
    /// PPtr/*<Texture2D>*/: (5.6.0b2 - 2022.2.0b16)
    pub defaultCursor: Option<PPtr /*<Texture2D>*/>,
    /// bool: (3.4.0 - 2017.4.33f1)
    pub defaultIsFullScreen: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub defaultIsNativeResolution: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub deferSystemGesturesMode: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub disableDepthAndStencilBuffers: Option<bool>,
    /// bool: (2017.4.33f1 - 2020.1.0a20)
    pub disableOldInputManagerSupport: Option<bool>,
    /// i32: (3.4.0 - 2018.4.15f1)
    pub displayResolutionDialog: Option<i32>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub enableFrameTimingStats: Option<bool>,
    /// bool: (2017.4.33f1 - 2020.1.0a20)
    pub enableNativePlatformBackendsForNewInputSystem: Option<bool>,
    /// bool: (5.6.0b2 - 5.6.0b2)
    pub enableNewInputSystem: Option<bool>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub enableOpenGLProfilerGPURecorders: Option<bool>,
    /// bool: (2021.2.16f1 - 2021.2.16f1)
    pub forceSRGBBlit: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub forceSingleInstance: Option<bool>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub framebufferDepthMemorylessMode: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub fullscreenMode: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub gpuSkinning: Option<bool>,
    /// i32: (2017.4.33f1 - 2018.4.15f1)
    pub graphicsJobMode: Option<i32>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub graphicsJobs: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub hideHomeButton: Option<bool>,
    /// PPtr/*<Texture2D>*/: (2022.2.0b16 - 2022.2.0b16)
    pub hmiLoadingImage: Option<PPtr /*<Texture2D>*/>,
    /// bool: (5.6.0b2 - 5.6.0b2)
    pub ignoreAlphaClear: Option<bool>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub insecureHttpOption: Option<i32>,
    /// bool: (5.6.0b2 - 2021.2.16f1)
    pub iosAllowHTTPDownload: Option<bool>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub iosAppInBackgroundBehavior: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub iosShowActivityIndicatorOnLoading: Option<i32>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub iosUseCustomAppBackgroundBehavior: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub isWsaHolographicRemotingEnabled: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub legacyClampBlendShapeWeights: Option<bool>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub loadStoreDebugModeEnabled: Option<bool>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_ActiveColorSpace: Option<i32>,
    /// Vec<i32>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ColorGamuts: Option<Vec<i32>>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_HolographicPauseOnTrackingLoss: Option<bool>,
    /// PPtr/*<Texture2D>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_HolographicTrackingLossScreen: Option<PPtr /*<Texture2D>*/>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_MTRendering: Option<bool>,
    /// bool: (5.6.0b2 - 5.6.0b2)
    pub m_MobileMTRendering: Option<bool>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_RenderingPath: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_ShowUnitySplashLogo: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_ShowUnitySplashScreen: Option<bool>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenAnimation: Option<i32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenBackgroundAnimationZoom: Option<f32>,
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenBackgroundColor: Option<ColorRGBA>,
    /// PPtr/*<Texture2D>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenBackgroundLandscape: Option<PPtr /*<Texture2D>*/>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenBackgroundLandscapeAspect: Option<f32>,
    /// Rectf: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenBackgroundLandscapeUvs: Option<Rectf>,
    /// PPtr/*<Texture2D>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenBackgroundPortrait: Option<PPtr /*<Texture2D>*/>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenBackgroundPortraitAspect: Option<f32>,
    /// Rectf: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenBackgroundPortraitUvs: Option<Rectf>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenDrawMode: Option<i32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenLogoAnimationZoom: Option<f32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenLogoStyle: Option<i32>,
    /// Vec<SplashScreenLogo>: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenLogos: Option<Vec<SplashScreenLogo>>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_SplashScreenOverlayOpacity: Option<f32>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_SpriteBatchVertexThreshold: Option<i32>,
    /// Vec<i32>: (5.6.0b2 - 2022.2.0b16)
    pub m_StackTraceTypes: Option<Vec<i32>>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_StereoRenderingPath: Option<i32>,
    /// AspectRatios: (3.4.0 - 2021.2.16f1)
    pub m_SupportedAspectRatios: Option<AspectRatios>,
    /// PPtr/*<Sprite>*/: (2022.2.0b16 - 2022.2.0b16)
    pub m_UnitySplashLogo: Option<PPtr /*<Sprite>*/>,
    /// PPtr/*<Texture2D>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_VirtualRealitySplashScreen: Option<PPtr /*<Texture2D>*/>,
    /// String: (2017.4.33f1 - 2022.2.0b16)
    pub macAppStoreCategory: Option<String>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub macFullscreenMode: Option<i32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub macRetinaSupport: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub metalFramebufferOnly: Option<bool>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub metroInputSource: Option<i32>,
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub mipStripping: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub mobileMTRenderingBaked: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub muteOtherAudioSources: Option<bool>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub n3dsDisableStereoscopicView: Option<bool>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub n3dsEnableSharedListOpt: Option<bool>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub n3dsEnableVSync: Option<bool>,
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub numberOfMipsStripped: Option<i32>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub organizationId: Option<String>,
    /// String: (2021.2.16f1 - 2021.2.16f1)
    pub playerDataPath: Option<String>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub playerMinOpenGLESVersion: Option<i32>,
    /// Vec<PPtr/*<Object>*/>: (5.6.0b2 - 2022.2.0b16)
    pub preloadedAssets: Option<Vec<PPtr /*<Object>*/>>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub preserveFramebufferAlpha: Option<bool>,
    /// GUID: (5.6.0b2 - 2022.2.0b16)
    pub productGUID: Option<GUID>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub projectName: Option<String>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub protectGraphicsMemory: Option<bool>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub psp2AcquireBGM: Option<bool>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub psp2PowerMode: Option<i32>,
    /// Vec<String>: (2020.3.42f1 - 2022.2.0b16)
    pub qualitySettingsNames: Option<Vec<String>>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub resetResolutionOnWindowResize: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub resizableWindow: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub resolutionScalingMode: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub stadiaPresentMode: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub stadiaTargetFramerate: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub submitAnalytics: Option<bool>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub switchAllowGpuScratchShrinking: Option<bool>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub switchGpuScratchPoolGranularity: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub switchNVNDefaultPoolsGranularity: Option<i32>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub switchNVNGraphicsFirmwareMemory: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub switchNVNMaxPublicSamplerIDCount: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub switchNVNMaxPublicTextureIDCount: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub switchNVNOtherPoolsGranularity: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub switchNVNShaderPoolsGranularity: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub switchQueueCommandMemory: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub switchQueueComputeMemory: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub switchQueueControlMemory: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub targetPixelDensity: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub targetPlatform: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub targetResolution: Option<i32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub tizenShowActivityIndicatorOnLoading: Option<i32>,
    /// bool: (5.6.0b2 - 5.6.0b2)
    pub uiUse16BitDepthBuffer: Option<bool>,
    /// bool: (3.4.0 - 3.4.0)
    pub useAlphaInDashboard: Option<bool>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub useFlipModelSwapchain: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub useHDRDisplay: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub useOnDemandResources: Option<bool>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub videoMemoryForVertexBuffers: Option<i32>,
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub virtualTexturingSupportEnabled: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub visibleInBackground: Option<bool>,
    /// VRSettings: (5.6.0b2 - 2022.2.0b16)
    pub vrSettings: Option<VRSettings>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub vulkanEnableCommandBufferRecycling: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub vulkanEnableLateAcquireNextImage: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub vulkanEnablePreTransform: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub vulkanEnableSetSRGBWrite: Option<bool>,
    /// u32: (2019.3.0f4 - 2022.2.0b16)
    pub vulkanNumSwapchainBuffers: Option<u32>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub wiiUAllowScreenCapture: Option<bool>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub wiiUControllerCount: Option<i32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub wiiUGamePadMSAA: Option<i32>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub wiiUSupportsBalanceBoard: Option<bool>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub wiiUSupportsClassicController: Option<bool>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub wiiUSupportsMotionPlus: Option<bool>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub wiiUSupportsNunchuk: Option<bool>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub wiiUSupportsProController: Option<bool>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub wiiUTVResolution: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub windowsGamepadBackendHint: Option<i32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub wsaTransparentSwapchain: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub xboxEnableAvatar: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub xboxEnableFitness: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub xboxEnableGuest: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub xboxEnableHeadOrientation: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub xboxEnableKinect: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub xboxEnableKinectAutoTracking: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub xboxEnablePIXSampling: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub xboxOneDisableEsram: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub xboxOneDisableKinectGpuReservation: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub xboxOneEnable7thCore: Option<bool>,
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub xboxOneEnableTypeOptimization: Option<bool>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub xboxOneLoggingLevel: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub xboxOneMonoLoggingLevel: Option<i32>,
    /// u32: (2017.4.33f1 - 2022.2.0b16)
    pub xboxOnePresentImmediateThreshold: Option<u32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub xboxOneResolution: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub xboxOneSResolution: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub xboxOneXResolution: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub xboxPIXTextureCapture: Option<bool>,
    /// bool: (3.4.0 - 3.4.0)
    pub xboxSkinOnGPU: Option<bool>,
    /// u32: (5.6.0b2 - 2022.2.0b16)
    pub xboxSpeechDB: Option<u32>,
}

/// PluginBuildInfo is a  class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginBuildInfo {
    pub m_EditorPlugins: Vec<String>,
    pub m_RuntimePlugins: Vec<String>,
}

/// PluginImportOutput is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginImportOutput {
    pub dllType: i32,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub scriptingRuntimeVersion: Option<i32>,
}

/// PluginImporter is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PluginImporter.html):
/**
Represents a plugin importer.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginImporter {
    /**Get or set the AssetBundle name.*/
    pub m_AssetBundleName: String,
    /**Get or set the AssetBundle variant.*/
    pub m_AssetBundleVariant: String,
    pub m_ExecutionOrder: Vec<(String, i32)>,
    pub m_IconMap: Vec<(String, PPtr /*<Texture2D>*/)>,
    pub m_IsOverridable: bool,
    /**Is a native plugin loaded during startup or on demand?*/
    pub m_IsPreloaded: bool,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Output: PluginImportOutput,
    pub m_PlatformData: Vec<((String, String), PlatformSettingsData)>,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /**Allows you to specify a list of #define directives which controls whether your plug-in should be included.*/
    /// Vec<String>: (2018.4.15f1 - 2022.2.0b16)
    pub m_DefineConstraints: Option<Vec<String>>,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_IsExplicitlyReferenced: Option<bool>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_ValidateReferences: Option<bool>,
}

/// PointEffector2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PointEffector2D.html):
/**
Applies forces to attract/repulse against a point.
When the source Collider2D is a trigger, the effector will apply forces whenever the target Collider2D overlaps the source.  When the source Collider isn't a trigger, the effector will apply forces whenever the target Collider2D is in contact with the source only.This effector is designed primarily to work with source Collider2D that are set as triggers so that target Collider2D can overlap the defined area.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PointEffector2D {
    /**The angular drag to apply to rigid-bodies.*/
    pub m_AngularDrag: f32,
    /**The mask used to select specific layers allowed to interact with the effector.*/
    pub m_ColliderMask: BitField,
    /**The scale applied to the calculated distance between source and target.*/
    pub m_DistanceScale: f32,
    /**The linear drag to apply to rigid-bodies.*/
    pub m_Drag: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The magnitude of the force to be applied.*/
    pub m_ForceMagnitude: f32,
    /**The mode used to apply the effector force.*/
    pub m_ForceMode: i32,
    /**The source which is used to calculate the centroid point of the effector.  The distance from the target is defined from this point.*/
    pub m_ForceSource: i32,
    /**The target for where the effector applies any force.*/
    pub m_ForceTarget: i32,
    /**The variation of the magnitude of the force to be applied.*/
    pub m_ForceVariation: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Should the collider-mask be used or the global collision matrix?*/
    pub m_UseColliderMask: bool,
}

/// Polygon2D is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct Polygon2D {
    pub m_Paths: Vec<Vec<Vector2f>>,
}

/// PolygonCollider2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PolygonCollider2D.html):
/**
Collider for 2D physics representing an arbitrary polygon defined by its vertices.
See Also: BoxCollider2D, CircleCollider2D, EdgeCollider2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonCollider2D {
    /**The density of the collider used to calculate its mass (when auto mass is enabled).*/
    pub m_Density: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    pub m_Material: PPtr, /*<PhysicsMaterial2D>*/
    /**The local offset of the collider geometry.*/
    pub m_Offset: Vector2f,
    /**Corner points that define the collider's shape in local space.*/
    pub m_Points: Polygon2D,
    /**Sets whether the Collider will be used or not used by a CompositeCollider2D.*/
    pub m_UsedByComposite: bool,
    /**Whether the collider is used by an attached effector or not.*/
    pub m_UsedByEffector: bool,
    /**Determines whether the PolygonCollider2D's shape is automatically updated based on a SpriteRenderer's tiling properties.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_AutoTiling: Option<bool>,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_CallbackLayers: Option<BitField>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
    /// SpriteTilingProperty: (2017.4.33f1 - 2022.2.0b16)
    pub m_SpriteTilingProperty: Option<SpriteTilingProperty>,
    /**When the value is true, the Collider uses an additional Delaunay triangulation step to produce the Collider mesh. When the value is false, this additional step does not occur.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_UseDelaunayMesh: Option<bool>,
}

/// PositionConstraint is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.PositionConstraint.html):
/**
Constrains the position of an object relative to the position of one or more source objects.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PositionConstraint {
    pub m_AffectTranslationX: bool,
    pub m_AffectTranslationY: bool,
    pub m_AffectTranslationZ: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_Sources: Vec<ConstraintSource>,
    /**The translation used when the sources have a total weight of 0.*/
    pub m_TranslationAtRest: Vector3f,
    /**The offset from the constrained position.*/
    pub m_TranslationOffset: Vector3f,
    /**The weight of the constraint component.*/
    pub m_Weight: f32,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_Active: Option<bool>,
    /// bool: (2018.4.15f1 - 2021.2.16f1)
    pub m_IsContraintActive: Option<bool>,
}

/// Prefab is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct Prefab {
    pub m_RootGameObject: PPtr, /*<GameObject>*/
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub m_ContainsMissingSerializeReferenceTypes: Option<bool>,
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub m_HideFlagsBehaviour: Option<i32>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub m_IsPrefabParent: Option<bool>,
    /// PrefabModification: (5.6.0b2 - 2017.4.33f1)
    pub m_Modification: Option<PrefabModification>,
    /// PPtr/*<Prefab>*/: (5.6.0b2 - 2017.4.33f1)
    pub m_ParentPrefab: Option<PPtr /*<Prefab>*/>,
}

/// PrefabImporter is a  class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PrefabImporter {
    pub m_AddedObjectFileIDs: Vec<i64>,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_IsPrefabVariant: bool,
    pub m_Name: String,
    pub m_UserData: String,
    /// bool: (2018.4.15f1 - 2020.1.0a20)
    pub m_UnableToImportOnPreviousDomainReload: Option<bool>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /// GUID: (2022.2.0b16 - 2022.2.0b16)
    pub m_VariantParentGUID: Option<GUID>,
}

/// PrefabInstance is a  class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PrefabInstance {
    pub m_Modification: PrefabModification,
    pub m_RootGameObject: PPtr, /*<GameObject>*/
    pub m_SourcePrefab: PPtr,   /*<Prefab>*/
}

/// PrefabModification is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct PrefabModification {
    pub m_Modifications: Vec<PropertyModification>,
    pub m_RemovedComponents: Vec<Enum_PPtr___Component_____PPtr___Object___>,
    pub m_TransformParent: PPtr, /*<Transform>*/
    /// Vec<AddedComponent>: (2022.2.0b16 - 2022.2.0b16)
    pub m_AddedComponents: Option<Vec<AddedComponent>>,
    /// Vec<AddedGameObject>: (2022.2.0b16 - 2022.2.0b16)
    pub m_AddedGameObjects: Option<Vec<AddedGameObject>>,
    /// Vec<PPtr/*<GameObject>*/>: (2022.2.0b16 - 2022.2.0b16)
    pub m_RemovedGameObjects: Option<Vec<PPtr /*<GameObject>*/>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_PPtr___Component_____PPtr___Object___ {
    PPtr___Component___(PPtr /*<Component>*/),
    PPtr___Object___(PPtr /*<Object>*/),
}

/// PreloadData is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct PreloadData {
    pub m_Assets: Vec<PPtr /*<Object>*/>,
    pub m_Name: String,
    /// Vec<String>: (5.6.0b2 - 2022.2.0b16)
    pub m_Dependencies: Option<Vec<String>>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_ExplicitDataLayout: Option<bool>,
}

/// Preset is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Presets.Preset.html):
/**
A Preset contains default values for an Object.
The Preset class contains the type of the Object used to create it and a list of each serialized property/value pair of this Object.
It can be used to store informations from any serializable Object in the Editor and apply them back to this Object or any other Object of the same type.
Presets can also be saved as Assets using the .preset extension in order to.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Preset {
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Properties: Vec<PropertyModification>,
    pub m_TargetType: PresetType,
    /**List of properties to ignore when applying the Preset to an object.*/
    /// Vec<String>: (2020.1.0a20 - 2022.2.0b16)
    pub m_ExcludedProperties: Option<Vec<String>>,
}

/// PresetManager is a  class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PresetManager {
    /// Vec<DefaultPresetList>: (2018.4.15f1 - 2018.4.15f1)
    pub m_DefaultList: Option<Vec<DefaultPresetList>>,
    /// Vec<(PresetType, Vec<DefaultPreset>)>: (2019.3.0f4 - 2022.2.0b16)
    pub m_DefaultPresets: Option<Vec<(PresetType, Vec<DefaultPreset>)>>,
}

/// PresetType is a sub class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Presets.PresetType.html):
/**
Stores a type to which a Preset can be applied.
Only classes that inherit from UnityEngine.Object are represented by a PresetType.This structure is used instead of Type because some native C++ types in Unity are not exposed to managed C# for optimization reasons.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PresetType {
    pub m_ManagedTypeFallback: String,
    pub m_ManagedTypePPtr: PPtr, /*<MonoScript>*/
    pub m_NativeTypeID: i32,
}

/// PreviewAnimationClip is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewAnimationClip {
    pub m_Bounds: AABB,
    pub m_ClipBindingConstant: AnimationClipBindingConstant,
    pub m_Compressed: bool,
    pub m_CompressedRotationCurves: Vec<CompressedAnimationCurve>,
    pub m_EulerCurves: Vec<Vector3Curve>,
    pub m_Events: Vec<AnimationEvent>,
    pub m_FloatCurves: Vec<FloatCurve>,
    pub m_Legacy: bool,
    pub m_MuscleClip: ClipMuscleConstant,
    pub m_MuscleClipSize: u32,
    pub m_Name: String,
    pub m_PPtrCurves: Vec<PPtrCurve>,
    pub m_PositionCurves: Vec<Vector3Curve>,
    pub m_RotationCurves: Vec<QuaternionCurve>,
    pub m_SampleRate: f32,
    pub m_ScaleCurves: Vec<Vector3Curve>,
    pub m_UseHighQualityCurve: bool,
    pub m_WrapMode: i32,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_HasGenericRootTransform: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_HasMotionFloatCurves: Option<bool>,
}

/// PreviewData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewData {
    pub m_CompSize: i32,
    pub m_OrigSize: i32,
    pub m_PreviewData: Vec<f32>,
}

/// PreviewImporter is a  class of the Unity engine since version 2020.3.42f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// ProbeSetIndex is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProbeSetIndex {
    pub m_Hash: Hash128,
    pub m_Offset: i32,
    pub m_Size: i32,
}

/// ProbeSetTetrahedralization is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProbeSetTetrahedralization {
    pub m_HullRays: Vec<Vector3f>,
    pub m_Tetrahedra: Vec<Tetrahedron>,
}

/// ProceduralMaterial is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ProceduralMaterial.html):
/**
Deprecated feature, no longer available
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ProceduralMaterial {
    /**The name of the object.*/
    pub m_Name: String,
    pub m_SavedProperties: UnityPropertySheet,
    /**The shader used by the material.*/
    pub m_Shader: PPtr, /*<Shader>*/
    /// Vec<String>: (2017.4.33f1 - 2022.2.0b16)
    pub disabledShaderPasses: Option<Vec<String>>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_AnimationUpdateRate: Option<i32>,
    /// Vec<BuildTextureStackReference>: (2020.1.0a20 - 2022.2.0b16)
    pub m_BuildTextureStacks: Option<Vec<BuildTextureStackReference>>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_CacheSize: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_CustomRenderQueue: Option<i32>,
    /**Gets and sets whether the Double Sided Global Illumination setting is enabled for this material.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_DoubleSidedGI: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_EnableInstancingVariants: Option<bool>,
    /// u32: (3.4.0 - 2017.4.33f1)
    pub m_Flags: Option<u32>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub m_GenerateMipmaps: Option<bool>,
    /// Hash128: (5.6.0b2 - 2017.4.33f1)
    pub m_Hash: Option<Hash128>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_Height: Option<i32>,
    /// Vec<SubstanceInput>: (3.4.0 - 2017.4.33f1)
    pub m_Inputs: Option<Vec<SubstanceInput>>,
    /// Vec<String>: (2022.2.0b16 - 2022.2.0b16)
    pub m_InvalidKeywords: Option<Vec<String>>,
    /// u32: (5.6.0b2 - 2022.2.0b16)
    pub m_LightmapFlags: Option<u32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_LoadingBehavior: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_MaximumSize: Option<i32>,
    /// String: (5.6.0b2 - 2017.4.33f1)
    pub m_PrototypeName: Option<String>,
    /**An array containing names of the local shader keywords that are currently enabled for this material.*/
    /// String: (5.6.0b2 - 2021.2.16f1)
    pub m_ShaderKeywords: Option<String>,
    /// PPtr/*<SubstanceArchive>*/: (3.4.0 - 2017.4.33f1)
    pub m_SubstancePackage: Option<PPtr /*<SubstanceArchive>*/>,
    /// Vec<PPtr/*<ProceduralTexture>*/>: (3.4.0 - 2017.4.33f1)
    pub m_Textures: Option<Vec<PPtr /*<ProceduralTexture>*/>>,
    /// Vec<String>: (2022.2.0b16 - 2022.2.0b16)
    pub m_ValidKeywords: Option<Vec<String>>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_Width: Option<i32>,
    /// Vec<(String, String)>: (5.6.0b2 - 2022.2.0b16)
    pub stringTagMap: Option<Vec<(String, String)>>,
}

/// ProceduralMaterialInformation is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProceduralMaterialInformation {
    pub m_Offset: Vector2f,
    pub m_Scale: Vector2f,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_AnimationUpdateRate: Option<i32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_GenerateAllOutputs: Option<i32>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub m_GenerateMipmaps: Option<bool>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_GeneratedAtLoading: Option<i32>,
}

/// ProceduralTexture is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ProceduralTexture.html):
/**
Deprecated feature, no longer available
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ProceduralTexture {
    /**The name of the object.*/
    pub m_Name: String,
    /// i32: (3.4.0 - 2017.4.33f1)
    pub AlphaSource: Option<i32>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub AlphaSourceIsGrayscale: Option<bool>,
    /// i32: (3.4.0 - 2017.4.33f1)
    pub Format: Option<i32>,
    /// i32: (3.4.0 - 2017.4.33f1)
    pub Type: Option<i32>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub m_AlphaSourceIsInverted: Option<bool>,
    /// u64: (5.6.0b2 - 2017.4.33f1)
    pub m_AlphaSourceUID: Option<u64>,
    /// Vec<u8>: (3.4.0 - 2017.4.33f1)
    pub m_BakedData: Option<Vec<u8>>,
    /// TextureParameters: (3.4.0 - 2017.4.33f1)
    pub m_BakedParameters: Option<TextureParameters>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_ColorSpace: Option<i32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// i32: (3.4.0 - 2017.4.33f1)
    pub m_LightmapFormat: Option<i32>,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_Mipmaps: Option<i32>,
    /// PPtr/*<ProceduralMaterial>*/: (3.4.0 - 2017.4.33f1)
    pub m_SubstanceMaterial: Option<PPtr /*<ProceduralMaterial>*/>,
    /// u64: (3.4.0 - 2017.4.33f1)
    pub m_SubstanceTextureUID: Option<u64>,
    /// TextureParameters: (3.4.0 - 2017.4.33f1)
    pub m_TextureParameters: Option<TextureParameters>,
    /// GLTextureSettings: (3.4.0 - 2017.4.33f1)
    pub m_TextureSettings: Option<GLTextureSettings>,
}

/// ProceduralTextureAssignment is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProceduralTextureAssignment {
    pub baseUID: u32,
    pub material: PPtr, /*<ProceduralMaterial>*/
    pub shaderProp: String,
}

/// Projector is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Projector.html):
/**
A script interface for a projector component.
The Projector can be used to project any material onto the Scene - just like a real world projector.

The properties exposed by this class are an exact match for the values in the Projector's inspector.It can be used to implement blob or projected shadows. You could also project an animated texture or

a render texture that films another part of the Scene. The projector will render all objects in

its view frustum with the provided material.There is no shortcut property in GameObject or Component to access the Projector, so you must

use GetComponent to do it:
See Also: projector component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Projector {
    /**The aspect ratio of the projection.*/
    pub m_AspectRatio: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The far clipping plane distance.*/
    pub m_FarClipPlane: f32,
    /**The field of view of the projection in degrees.*/
    pub m_FieldOfView: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Which object layers are ignored by the projector.*/
    pub m_IgnoreLayers: BitField,
    /**The material that will be projected onto every object.*/
    pub m_Material: PPtr, /*<Material>*/
    /**The near clipping plane distance.*/
    pub m_NearClipPlane: f32,
    /**Is the projection orthographic (true) or perspective (false)?*/
    pub m_Orthographic: bool,
    /**Projection's half-size when in orthographic mode.*/
    pub m_OrthographicSize: f32,
}

/// PropertyModification is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PropertyModification.html):
/**
Defines a single modified property.
Used by the Prefab system to track any changes applied to an instance.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyModification {
    /**The value being applied when it is an object reference (which can not be represented as a string).*/
    pub objectReference: PPtr, /*<Object>*/
    /**Property path of the property being modified (Matches as SerializedProperty.propertyPath).*/
    pub propertyPath: String,
    /**Object that will be modified.*/
    pub target: PPtr, /*<Object>*/
    /**The value being applied.*/
    pub value: String,
}

/// PropertyModificationsTargetTestNativeObject is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyModificationsTargetTestNativeObject {
    pub m_FloatValue: f32,
    pub m_IntegerValue: i32,
}

/// PropertyModificationsTargetTestObject is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyModificationsTargetTestObject {
    pub m_Array: Vec<PropertyModificationsTargetTestNativeObject>,
    pub m_Bytes: Vec<u8>,
    pub m_BytesSize: u32,
    pub m_Data: PropertyModificationsTargetTestNativeObject,
    pub m_FloatTestValue: f32,
    pub m_Floats: Vec<f32>,
    /// Vec<u8>: (2019.3.0f4 - 2020.1.0a20)
    #[serde(alias = "byte data")]
    pub byte_data: Option<Vec<u8>>,
}

/// QualitySetting is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySetting {
    pub anisotropicTextures: i32,
    pub antiAliasing: i32,
    pub pixelLightCount: i32,
    pub shadowCascades: i32,
    pub shadowDistance: f32,
    pub shadowProjection: i32,
    pub shadowResolution: i32,
    pub shadows: i32,
    pub softParticles: bool,
    pub softVegetation: bool,
    pub vSyncCount: i32,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub asyncUploadBufferSize: Option<i32>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub asyncUploadPersistentBuffer: Option<bool>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub asyncUploadTimeSlice: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub billboardsFaceCameraPosition: Option<bool>,
    /// i32: (3.4.0 - 2018.4.15f1)
    pub blendWeights: Option<i32>,
    /// PPtr/*<MonoBehaviour>*/: (2019.3.0f4 - 2022.2.0b16)
    pub customRenderPipeline: Option<PPtr /*<MonoBehaviour>*/>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub enableLODCrossFade: Option<bool>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub globalTextureMipmapLimit: Option<i32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub lodBias: Option<f32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub maximumLODLevel: Option<i32>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub name: Option<String>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub particleRaycastBudget: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub realtimeReflectionProbes: Option<bool>,
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub resolutionScalingFixedDPIFactor: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub shadowCascade2Split: Option<f32>,
    /// Vector3f: (5.6.0b2 - 2022.2.0b16)
    pub shadowCascade4Split: Option<Vector3f>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub shadowNearPlaneOffset: Option<f32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub shadowmaskMode: Option<i32>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub skinWeights: Option<i32>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub streamingMipmapsActive: Option<bool>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub streamingMipmapsAddAllCameras: Option<bool>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub streamingMipmapsMaxFileIORequests: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub streamingMipmapsMaxLevelReduction: Option<i32>,
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub streamingMipmapsMemoryBudget: Option<f32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub streamingMipmapsRenderersPerFrame: Option<i32>,
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub terrainBasemapDistance: Option<f32>,
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub terrainBillboardStart: Option<f32>,
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub terrainDetailDensityScale: Option<f32>,
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub terrainDetailDistance: Option<f32>,
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub terrainFadeLength: Option<f32>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub terrainMaxTrees: Option<i32>,
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub terrainPixelError: Option<f32>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub terrainQualityOverrides: Option<i32>,
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub terrainTreeDistance: Option<f32>,
    /// Vec<MipmapLimitSettings>: (2022.2.0b16 - 2022.2.0b16)
    pub textureMipmapLimitSettings: Option<Vec<MipmapLimitSettings>>,
    /// i32: (3.4.0 - 2021.2.16f1)
    pub textureQuality: Option<i32>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub useLegacyDetailDistribution: Option<bool>,
}

/// QualitySettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/QualitySettings.html):
/**
Script interface for Quality Settings.
There can be an arbitrary number of quality settings. The details of each are set up

in the project's Quality Settings. At run time, the

current quality level can be changed using this class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySettings {
    /// QualitySetting: (3.4.0 - 3.4.0)
    pub Beautiful: Option<QualitySetting>,
    /// QualitySetting: (3.4.0 - 3.4.0)
    pub Fantastic: Option<QualitySetting>,
    /// QualitySetting: (3.4.0 - 3.4.0)
    pub Fast: Option<QualitySetting>,
    /// QualitySetting: (3.4.0 - 3.4.0)
    pub Fastest: Option<QualitySetting>,
    /// QualitySetting: (3.4.0 - 3.4.0)
    pub Good: Option<QualitySetting>,
    /// QualitySetting: (3.4.0 - 3.4.0)
    pub Simple: Option<QualitySetting>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_CurrentQuality: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_DefaultMobileQuality: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_DefaultStandaloneQuality: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_DefaultWebPlayerQuality: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_EditorQuality: Option<i32>,
    /// Vec<QualitySetting>: (5.6.0b2 - 2022.2.0b16)
    pub m_QualitySettings: Option<Vec<QualitySetting>>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_StrippedMaximumLODLevel: Option<i32>,
    /// Vec<String>: (2022.2.0b16 - 2022.2.0b16)
    pub m_TextureMipmapLimitGroupNames: Option<Vec<String>>,
}

/// QuaternionCurve is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct QuaternionCurve {
    pub curve: AnimationCurve,
    pub path: String,
}

/// Quaternionf is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Quaternionf {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// RayTracingShader is a  class of the Unity engine since version 2019.3.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Experimental.Rendering.RayTracingShader.html):
/**
A shader for GPU ray tracing.
This shader should contain at least a raygeneration shader.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShader {
    /**The maximum number of ray bounces this shader can trace (Read Only).*/
    pub m_MaxRecursionDepth: u32,
    /**The name of the object.*/
    pub m_Name: String,
    pub variants: Vec<RayTracingShaderVariant>,
}

/// RayTracingShaderBuiltinSampler is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderBuiltinSampler {
    pub bindPoint: i32,
    pub sampler: u32,
}

/// RayTracingShaderConstantBuffer is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderConstantBuffer {
    pub byteSize: i32,
    pub name: String,
    pub params: Vec<RayTracingShaderParam>,
    /// u32: (2020.3.42f1 - 2022.2.0b16)
    pub hash: Option<u32>,
}

/// RayTracingShaderFunctionDesc is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderFunctionDesc {
    pub attributeSizeInBytes: u32,
    pub identifier: RayTracingShaderID,
    pub payloadSizeInBytes: u32,
}

/// RayTracingShaderID is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderID {
    pub name: String,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// RayTracingShaderImporter is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
    /// u32: (2019.3.0f4 - 2020.3.42f1)
    pub m_CurrentAPIMask: Option<u32>,
}

/// RayTracingShaderParam is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderParam {
    pub arraySize: i64,
    pub colCount: i32,
    pub name: String,
    pub offset: i64,
    pub rowCount: i32,
    /// i32: (2019.3.0f4 - 2019.3.0f4)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// u32: (2020.1.0a20 - 2022.2.0b16)
    pub dataSize: Option<u32>,
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub dataType: Option<i32>,
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub propertySheetType: Option<i32>,
}

/// RayTracingShaderReflectionData is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderReflectionData {
    pub code: Vec<u8>,
    pub functions: Vec<RayTracingShaderFunctionDesc>,
    pub globalResources: RayTracingShaderResources,
    pub hasErrors: bool,
    pub localResources: RayTracingShaderResources,
    /// Vec<u8>: (2022.2.0b16 - 2022.2.0b16)
    pub precompiled: Option<Vec<u8>>,
}

/// RayTracingShaderResource is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderResource {
    pub bindPoint: i32,
    pub name: String,
    pub rayGenMask: u64,
    pub samplerBindPoint: i32,
    pub texDimension: i32,
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub arraySize: Option<i32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub multisampled: Option<bool>,
}

/// RayTracingShaderResources is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderResources {
    pub builtinSamplers: Vec<RayTracingShaderBuiltinSampler>,
    pub constantBuffers: Vec<RayTracingShaderResource>,
    pub constantBuffersDesc: Vec<RayTracingShaderConstantBuffer>,
    pub inputBuffers: Vec<RayTracingShaderResource>,
    pub outputBuffers: Vec<RayTracingShaderResource>,
    pub textures: Vec<RayTracingShaderResource>,
}

/// RayTracingShaderVariant is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderVariant {
    pub resourceReflectionData: RayTracingShaderReflectionData,
    pub targetRenderer: i32,
}

/// RaycastCollider is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct RaycastCollider {
    pub m_Center: Vector3f,
    pub m_Enabled: bool,
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_IsTrigger: bool,
    pub m_Length: f32,
    pub m_Material: PPtr, /*<PhysicMaterial>*/
}

/// RectTransform is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/RectTransform.html):
/**
Position, size, anchor and pivot information for a rectangle.
RectTransforms are used for GUI but can also be used for other things.

It's used to store and manipulate the position, size, and anchoring of a rectangle and supports various forms of scaling based on a parent RectTransform.Note: The Inspector changes which properties are exposed based on which anchor preset is in use. For more information see Rect Transform and Basic Layout.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RectTransform {
    /**The normalized position in the parent RectTransform that the upper right corner is anchored to.*/
    pub m_AnchorMax: Vector2f,
    /**The normalized position in the parent RectTransform that the lower left corner is anchored to.*/
    pub m_AnchorMin: Vector2f,
    /**The position of the pivot of this RectTransform relative to the anchor reference point.*/
    pub m_AnchoredPosition: Vector2f,
    pub m_Children: Vec<PPtr /*<Transform>*/>,
    pub m_Father: PPtr, /*<Transform>*/
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Position of the transform relative to the parent transform.*/
    pub m_LocalPosition: Vector3f,
    /**The rotation of the transform relative to the transform rotation of the parent.*/
    pub m_LocalRotation: Quaternionf,
    /**The scale of the transform relative to the GameObjects parent.*/
    pub m_LocalScale: Vector3f,
    /**The normalized position in this RectTransform that it rotates around.*/
    pub m_Pivot: Vector2f,
    /**The size of this RectTransform relative to the distances between the anchors.*/
    pub m_SizeDelta: Vector2f,
}

/// Rectf is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Rectf {
    pub height: f32,
    pub width: f32,
    pub x: f32,
    pub y: f32,
}

/// ReferencesArtifactGenerator is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferencesArtifactGenerator {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// ReflectionProbe is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ReflectionProbe.html):
/**
The reflection probe is used to capture the surroundings into a texture which is passed to the shaders and used for reflections.
The properties are an exact match for the values shown in the Inspector.This class is a script interface for a reflection probe component.

Reflection probes are usually just created in the Editor, but sometimes you might want to create a reflection probe from a script:
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ReflectionProbe {
    /**The color with which the texture of reflection probe will be cleared.*/
    pub m_BackGroundColor: ColorRGBA,
    /**Reference to the baked texture of the reflection probe's surrounding.*/
    pub m_BakedTexture: PPtr, /*<Texture>*/
    /**Distance around probe used for blending (used in deferred probes).*/
    pub m_BlendDistance: f32,
    pub m_BoxOffset: Vector3f,
    /**Should this reflection probe use box projection?*/
    pub m_BoxProjection: bool,
    pub m_BoxSize: Vector3f,
    /**How the reflection probe clears the background.*/
    pub m_ClearFlags: u32,
    /**This is used to render parts of the reflecion probe's surrounding selectively.*/
    pub m_CullingMask: BitField,
    /**Reference to the baked texture of the reflection probe's surrounding. Use this to assign custom reflection texture.*/
    pub m_CustomBakedTexture: PPtr, /*<Texture>*/
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    pub m_FarClip: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Should this reflection probe use HDR rendering?*/
    pub m_HDR: bool,
    /**Reflection probe importance.*/
    pub m_Importance: i16,
    pub m_IntensityMultiplier: f32,
    /**Should reflection probe texture be generated in the Editor (ReflectionProbeMode.Baked) or should probe use custom specified texure (ReflectionProbeMode.Custom)?*/
    pub m_Mode: i32,
    pub m_NearClip: f32,
    /**Sets the way the probe will refresh.See Also: ReflectionProbeRefreshMode.*/
    pub m_RefreshMode: i32,
    /**Specifies whether Unity should render non-static GameObjects into the Reflection Probe. If you set this to true, Unity renders non-static GameObjects into the Reflection Probe. If you set this to false, Unity does not render non-static GameObjects into the Reflection Probe. Unity only takes this property into account if the Reflection Probe's Type is Custom.*/
    pub m_RenderDynamicObjects: bool,
    /**Resolution of the underlying reflection texture in pixels.*/
    pub m_Resolution: i32,
    /**Shadow drawing distance when rendering the probe.*/
    pub m_ShadowDistance: f32,
    /**Sets this probe time-slicing modeSee Also: ReflectionProbeTimeSlicingMode.*/
    pub m_TimeSlicingMode: i32,
    pub m_Type: i32,
    pub m_UpdateFrequency: i32,
    pub m_UseOcclusionCulling: bool,
}

/// RelativeJoint2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/RelativeJoint2D.html):
/**
Keeps two Rigidbody2D at their relative orientations.
Two Rigidbody2D connected together with this joint will have forces applied to them to keep them both at their relative linear and angular offsets.  If the joint is not connected to another Rigidbody2D then the body with the joint will stay at its current linear and angular offset in world-space i.e. it will be anchored to the implicit static ground-body.You control the maximum linear force applied to maintain the linearOffset by using maxForce.You control the maximum torque applied to maintain the angularOffset by using maxTorqueSee Also: linearOffset, angularOffset, maxForce, maxTorque.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RelativeJoint2D {
    /**The current angular offset between the Rigidbody2D that the joint connects.*/
    pub m_AngularOffset: f32,
    /**Should both the linearOffset and angularOffset be calculated automatically?*/
    pub m_AutoConfigureOffset: bool,
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break.*/
    pub m_BreakTorque: f32,
    pub m_ConnectedRigidBody: PPtr, /*<Rigidbody2D>*/
    /**Scales both the linear and angular forces used to correct the required relative orientation.*/
    pub m_CorrectionScale: f32,
    /**Should the two rigid bodies connected with this joint collide with each other?*/
    pub m_EnableCollision: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The current linear offset between the Rigidbody2D that the joint connects.*/
    pub m_LinearOffset: Vector2f,
    /**The maximum force that can be generated when trying to maintain the relative joint constraint.*/
    pub m_MaxForce: f32,
    /**The maximum torque that can be generated when trying to maintain the relative joint constraint.*/
    pub m_MaxTorque: f32,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_BreakAction: Option<i32>,
}

/// RenderPassAttachment is a  class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderPassAttachment {}

/// RenderSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Experimental.GlobalIllumination.RenderSettings.html):
/**
Experimental render settings features.
See Also: RenderSettings.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderSettings {
    pub m_FlareStrength: f32,
    pub m_Fog: bool,
    pub m_FogColor: ColorRGBA,
    pub m_FogDensity: f32,
    pub m_FogMode: i32,
    pub m_HaloStrength: f32,
    pub m_HaloTexture: PPtr, /*<Texture2D>*/
    pub m_LinearFogEnd: f32,
    pub m_LinearFogStart: f32,
    pub m_SkyboxMaterial: PPtr, /*<Material>*/
    pub m_SpotCookie: PPtr,     /*<Texture2D>*/
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub m_AmbientEquatorColor: Option<ColorRGBA>,
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub m_AmbientGroundColor: Option<ColorRGBA>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_AmbientIntensity: Option<f32>,
    /// ColorRGBA: (3.4.0 - 3.4.0)
    pub m_AmbientLight: Option<ColorRGBA>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_AmbientMode: Option<i32>,
    /// SphericalHarmonicsL2: (5.6.0b2 - 2022.2.0b16)
    pub m_AmbientProbe: Option<SphericalHarmonicsL2>,
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub m_AmbientSkyColor: Option<ColorRGBA>,
    /// PPtr/*<Cubemap>*/: (5.6.0b2 - 2020.3.42f1); PPtr/*<Texture>*/: (2021.2.16f1 - 2022.2.0b16)
    pub m_CustomReflection: Option<Enum_PPtr___Cubemap_____PPtr___Texture___>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_DefaultReflectionMode: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_DefaultReflectionResolution: Option<i32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_FlareFadeSpeed: Option<f32>,
    /// PPtr/*<Cubemap>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_GeneratedSkyboxReflection: Option<PPtr /*<Cubemap>*/>,
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub m_IndirectSpecularColor: Option<ColorRGBA>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_ReflectionBounces: Option<i32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_ReflectionIntensity: Option<f32>,
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub m_SubtractiveShadowColor: Option<ColorRGBA>,
    /// PPtr/*<Light>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_Sun: Option<PPtr /*<Light>*/>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_UseRadianceAmbientProbe: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_PPtr___Cubemap_____PPtr___Texture___ {
    PPtr___Cubemap___(PPtr /*<Cubemap>*/),
    PPtr___Texture___(PPtr /*<Texture>*/),
}

/// RenderTexture is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/RenderTexture.html):
/**
Render textures are textures that can be rendered to.
They can be used to implement image based rendering effects, dynamic shadows,

projectors, reflections or surveillance cameras.One typical usage of render textures is setting them as the "target texture" property

of a Camera (Camera.targetTexture), this will make a camera render into a texture

instead of rendering to the screen.Keep in mind that render texture contents can become "lost" on certain events, like loading a new level, system going to a screensaver mode, in and out of fullscreen and so on.

When that happens, your existing render textures will become "not yet created" again,

you can check for that with IsCreated function.As with other "native engine object" types, it is important to pay attention to the lifetime of

any render textures and release them when you are finished using them with the Release function,

as they will not be garbage collected like normal managed types.A render texture only has a data representation on the GPU and you need to use Texture2D.ReadPixels to transfer its contents to CPU memory.The initial contents of a newly created render texture are undefined. On some platforms and APIs the contents will default to black, but you shouldn't depend on this. You can use LoadStoreActionDebugModeSettings to highlight undefined areas of the display, to help you debug rendering problems in your built application.See Also: Camera.targetTexture.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderTexture {
    pub m_ColorFormat: i32,
    /**Height of the Texture in pixels (Read Only).*/
    pub m_Height: i32,
    pub m_MipMap: bool,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_TextureSettings: GLTextureSettings,
    /**Width of the Texture in pixels (Read Only).*/
    pub m_Width: i32,
    /**The antialiasing level for the RenderTexture.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_AntiAliasing: Option<i32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_BindMS: Option<bool>,
    /// i32: (3.4.0 - 2020.3.42f1)
    pub m_DepthFormat: Option<i32>,
    /**The format of the depth/stencil buffer.*/
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_DepthStencilFormat: Option<i32>,
    /**Dimensionality (type) of the Texture (Read Only).*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_Dimension: Option<i32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_DownscaleFallback: Option<bool>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_EnableCompatibleFormat: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_GenerateMips: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_IsCubemap: Option<bool>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_IsPowerOfTwo: Option<bool>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_MipCount: Option<i32>,
    /**Does this render texture use sRGB read/write conversions? (Read Only).*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_SRGB: Option<bool>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_ShadowSamplingMode: Option<i32>,
    /**Is the render texture marked to be scaled by the Dynamic Resolution system.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_UseDynamicScale: Option<bool>,
    /**Volume extent of a 3D render texture or number of slices of array texture.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_VolumeDepth: Option<i32>,
}

/// Renderer is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Renderer.html):
/**
General functionality for all renderers.
A renderer is what makes an object appear on the screen. Use this class to access the renderer of any object, mesh or Particle System.
Renderers can be disabled to make objects invisible (see enabled), and the materials can be accessed
and modified through them (see material).See Also: Renderer components for meshes, particles,
lines and trails.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Renderer {
    pub m_CastShadows: bool,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u8,
    pub m_LightmapTilingOffset: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    pub m_Materials: Vec<PPtr /*<Material>*/>,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: bool,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    pub m_SubsetIndices: Vec<u32>,
}

/// RendererData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct RendererData {
    pub lightmapIndex: u16,
    pub lightmapIndexDynamic: u16,
    pub lightmapST: Vector4f,
    pub lightmapSTDynamic: Vector4f,
    pub terrainChunkDynamicUVST: Vector4f,
    pub terrainDynamicUVST: Vector4f,
    pub uvMesh: PPtr, /*<Mesh>*/
    /// Hash128: (2018.4.15f1 - 2022.2.0b16)
    pub explicitProbeSetHash: Option<Hash128>,
}

/// ResourceManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceManager {
    pub m_Container: Vec<(String, PPtr /*<Object>*/)>,
    /// Vec<ResourceManager_Dependency>: (5.6.0b2 - 2022.2.0b16)
    pub m_DependentAssets: Option<Vec<ResourceManager_Dependency>>,
}

/// ResourceManager_Dependency is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceManager_Dependency {
    pub m_Dependencies: Vec<PPtr /*<Object>*/>,
    pub m_Object: PPtr, /*<Object>*/
}

/// Rigidbody is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rigidbody.html):
/**
Control of an object's position through physics simulation.
Adding a Rigidbody component to an object will put its motion under the control of Unity's physics engine. Even without adding any code, a Rigidbody object will be pulled downward by gravity and will react to collisions with incoming objects if the right Collider component is also present.The Rigidbody also has a scripting API that lets you apply forces to the object and control it in a physically realistic way. For example, a car's behaviour can be specified in terms of the forces applied by the wheels. Given this information, the physics engine can handle most other aspects of the car's motion, so it will accelerate realistically and respond correctly to collisions.In a script, the FixedUpdate function is recommended as the place to apply forces and change Rigidbody settings (as opposed to Update, which is used for most other frame update tasks). The reason for this is that physics updates are carried out in measured time steps that don't coincide with the frame update. FixedUpdate is called immediately before each physics update and so any changes made there will be processed directly.A common problem when starting out with Rigidbodies is that the game physics appears to run in "slow motion". This is actually due to the scale used for your models. The default gravity settings assume that one world unit corresponds to one metre of distance. With non-physical games, it doesn't make much difference if your models are all 100 units long but when using physics, they will be treated as very large objects. If a large scale is used for objects that are supposed to be small, they will appear to fall very slowly - the physics engine thinks they are very large objects falling over very large distances. With this in mind, be sure to keep your objects more or less at their scale in real life (so a car should be about 4 units = 4 metres, for example).
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Rigidbody {
    /**The angular drag of the object.*/
    pub m_AngularDrag: f32,
    pub m_CollisionDetection: i32,
    /**Controls which degrees of freedom are allowed for the simulation of this Rigidbody.*/
    pub m_Constraints: i32,
    /**The drag of the object.*/
    pub m_Drag: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_Interpolate: u8,
    /**Controls whether physics affects the rigidbody.*/
    pub m_IsKinematic: bool,
    /**The mass of the rigidbody.*/
    pub m_Mass: f32,
    /**Controls whether gravity affects this rigidbody.*/
    pub m_UseGravity: bool,
    /**The center of mass relative to the transform's origin.*/
    /// Vector3f: (2022.2.0b16 - 2022.2.0b16)
    pub m_CenterOfMass: Option<Vector3f>,
    /**The additional layers that all Colliders attached to this Rigidbody should exclude when deciding if the Collider can come into contact with another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ImplicitCom: Option<bool>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ImplicitTensor: Option<bool>,
    /**The additional layers that all Colliders attached to this Rigidbody should include when deciding if the Collider can come into contact with another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /// Quaternionf: (2022.2.0b16 - 2022.2.0b16)
    pub m_InertiaRotation: Option<Quaternionf>,
    /**The inertia tensor of this body, defined as a diagonal matrix in a reference frame positioned at this body's center of mass and rotated by Rigidbody.inertiaTensorRotation.*/
    /// Vector3f: (2022.2.0b16 - 2022.2.0b16)
    pub m_InertiaTensor: Option<Vector3f>,
}

/// Rigidbody2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rigidbody2D.html):
/**
Rigidbody physics component for 2D sprites.
The Rigidbody2D class essentially provides the same functionality in 2D that the Rigidbody class provides in 3D. Adding a Rigidbody2D component to a sprite puts it under the control of the physics engine. By itself, this means that the sprite will be affected by gravity and can be controlled from scripts using forces. By adding the appropriate collider component, the sprite will also respond to collisions with other sprites. This behaviour comes entirely from Unity's physics system; very little code is required to get impressive and authentic physical behaviour and allows for "emergent" gameplay that was not explicitly coded into the game.See Also: Rigidbody class, SpriteRenderer class, Collider2D class, Joint2D class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Rigidbody2D {
    /**Coefficient of angular drag.*/
    pub m_AngularDrag: f32,
    /**The physical behaviour type of the Rigidbody2D.*/
    pub m_BodyType: i32,
    pub m_CollisionDetection: i32,
    /**Controls which degrees of freedom are allowed for the simulation of this Rigidbody2D.*/
    pub m_Constraints: i32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The degree to which this object is affected by gravity.*/
    pub m_GravityScale: f32,
    pub m_Interpolate: i32,
    pub m_LinearDrag: f32,
    /**Mass of the Rigidbody.*/
    pub m_Mass: f32,
    pub m_Material: PPtr, /*<PhysicsMaterial2D>*/
    /**Indicates whether the rigid body should be simulated or not by the physics system.*/
    pub m_Simulated: bool,
    pub m_SleepingMode: i32,
    /**Should the total rigid-body mass be automatically calculated from the [[Collider2D.density]] of attached colliders?*/
    pub m_UseAutoMass: bool,
    /**Should kinematic/kinematic and kinematic/static collisions be allowed?*/
    pub m_UseFullKinematicContacts: bool,
    /**The additional Layers that all Collider2D attached to this Rigidbody2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The additional Layers that all Collider2D attached to this Rigidbody2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
}

/// RoslynAdditionalFileAsset is a  class of the Unity engine since version 2022.2.0b16.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoslynAdditionalFileAsset {
    pub m_Name: String,
}

/// RoslynAdditionalFileImporter is a  class of the Unity engine since version 2022.2.0b16.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoslynAdditionalFileImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// RoslynAnalyzerConfigAsset is a  class of the Unity engine since version 2022.2.0b16.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoslynAnalyzerConfigAsset {
    pub m_Name: String,
}

/// RoslynAnalyzerConfigImporter is a  class of the Unity engine since version 2022.2.0b16.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoslynAnalyzerConfigImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// RotationBySpeedModule is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.RotationBySpeedModule.html):
/**
Script interface for the RotationBySpeedModule.
Rotate particles based on their speed.See Also: ParticleSystem, ParticleSystem.rotationBySpeed.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RotationBySpeedModule {
    pub curve: MinMaxCurve,
    /**ESpecifies whether the RotationBySpeedModule is enabled or disabled.*/
    pub enabled: bool,
    /**Set the minimum and maximum speeds that this module applies the rotation curve between.*/
    pub range: Vector2f,
    /**Set the rotation by speed on each axis separately.*/
    pub separateAxes: bool,
    /**Rotation by speed curve for the x-axis.*/
    pub x: MinMaxCurve,
    /**Rotation by speed curve for the y-axis.*/
    pub y: MinMaxCurve,
}

/// RotationConstraint is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.RotationConstraint.html):
/**
Constrains the rotation of an object relative to the rotation of one or more source objects.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RotationConstraint {
    pub m_AffectRotationX: bool,
    pub m_AffectRotationY: bool,
    pub m_AffectRotationZ: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The rotation used when the sources have a total weight of 0.*/
    pub m_RotationAtRest: Vector3f,
    /**The offset from the constrained rotation.*/
    pub m_RotationOffset: Vector3f,
    pub m_Sources: Vec<ConstraintSource>,
    /**The weight of the constraint component.*/
    pub m_Weight: f32,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_Active: Option<bool>,
    /// bool: (2018.4.15f1 - 2021.2.16f1)
    pub m_IsContraintActive: Option<bool>,
}

/// RotationModule is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct RotationModule {
    pub curve: MinMaxCurve,
    pub enabled: bool,
    pub separateAxes: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
}

/// RuleSetFileAsset is a  class of the Unity engine since version 2020.3.42f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleSetFileAsset {
    pub m_Name: String,
    pub m_Script: String,
}

/// RuleSetFileImporter is a  class of the Unity engine since version 2020.3.42f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleSetFileImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// RuntimeInitializeOnLoadManager is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeInitializeOnLoadManager {
    /// Vec<i32>: (2019.3.0f4 - 2020.1.0a20)
    pub m_AfterAssembliesLoadedMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (2019.3.0f4 - 2020.1.0a20)
    pub m_AfterAssembliesLoadedUnityMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (5.6.0b2 - 2020.1.0a20)
    pub m_AfterMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (5.6.0b2 - 2020.1.0a20)
    pub m_AfterUnityMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<String>: (5.6.0b2 - 2020.1.0a20)
    pub m_AssemblyNames: Option<Vec<String>>,
    /// Vec<i32>: (5.6.0b2 - 2020.1.0a20)
    pub m_BeforeMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (2019.3.0f4 - 2020.1.0a20)
    pub m_BeforeSplashScreenMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (2019.3.0f4 - 2020.1.0a20)
    pub m_BeforeSplashScreenUnityMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (5.6.0b2 - 2020.1.0a20)
    pub m_BeforeUnityMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<ClassInfo>: (5.6.0b2 - 2020.1.0a20)
    pub m_ClassInfos: Option<Vec<ClassInfo>>,
    /// Vec<ClassMethodInfo>: (5.6.0b2 - 2020.1.0a20)
    pub m_ClassMethodInfos: Option<Vec<ClassMethodInfo>>,
    /// Vec<String>: (5.6.0b2 - 2020.1.0a20)
    pub m_NamespaceNames: Option<Vec<String>>,
    /// Vec<i32>: (2019.3.0f4 - 2020.1.0a20)
    pub m_SubsystemRegistrationMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (2019.3.0f4 - 2020.1.0a20)
    pub m_SubsystemRegistrationUnityMethodExecutionOrders: Option<Vec<i32>>,
}

/// SBranchWindLevel is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SBranchWindLevel {
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

/// SParams is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SParams {
    pub BranchLevel1: SBranchWindLevel,
    pub BranchLevel2: SBranchWindLevel,
    pub LeafGroup1: SWindGroup,
    pub LeafGroup2: SWindGroup,
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
    pub m_fAnchorDistanceScale: f32,
    pub m_fAnchorOffset: f32,
    pub m_fDirectionResponse: f32,
    pub m_fFrondRippleLightingScalar: f32,
    pub m_fFrondRippleTile: f32,
    pub m_fGlobalHeight: f32,
    pub m_fGlobalHeightExponent: f32,
    pub m_fGustDurationMax: f32,
    pub m_fGustDurationMin: f32,
    pub m_fGustFallScalar: f32,
    pub m_fGustFrequency: f32,
    pub m_fGustRiseScalar: f32,
    pub m_fGustStrengthMax: f32,
    pub m_fGustStrengthMin: f32,
    pub m_fRollingBranchFieldMin: f32,
    pub m_fRollingBranchLightingAdjust: f32,
    pub m_fRollingBranchVerticalOffset: f32,
    pub m_fRollingLeafRippleMin: f32,
    pub m_fRollingLeafTumbleMin: f32,
    pub m_fRollingNoisePeriod: f32,
    pub m_fRollingNoiseSize: f32,
    pub m_fRollingNoiseSpeed: f32,
    pub m_fRollingNoiseTurbulence: f32,
    pub m_fRollingNoiseTwist: f32,
    pub m_fStrengthResponse: f32,
}

/// SWindGroup is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
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
    pub m_fLeewardScalar: f32,
    pub m_fRollMaxScale: f32,
    pub m_fRollMinScale: f32,
    pub m_fRollSeparation: f32,
    pub m_fRollSpeed: f32,
    pub m_fTwitchSharpness: f32,
}

/// SampleClip is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SampleClip {
    pub m_Name: String,
}

/// SampleSettings is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SampleSettings {
    pub compressionFormat: i32,
    pub conversionMode: i32,
    pub loadType: i32,
    pub quality: f32,
    pub sampleRateOverride: u32,
    pub sampleRateSetting: i32,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub preloadAudioData: Option<bool>,
}

/// SamplerParameter is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SamplerParameter {
    pub bindPoint: i32,
    pub sampler: u32,
}

/// ScaleConstraint is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.ScaleConstraint.html):
/**
Constrains the scale of an object relative to the scale of one or more source objects.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleConstraint {
    pub m_AffectScalingX: bool,
    pub m_AffectScalingY: bool,
    pub m_AffectScalingZ: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The scale used when the sources have a total weight of 0.*/
    pub m_ScaleAtRest: Vector3f,
    /**The offset from the constrained scale.*/
    pub m_ScaleOffset: Vector3f,
    pub m_Sources: Vec<ConstraintSource>,
    /**The weight of the constraint component.*/
    pub m_Weight: f32,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_Active: Option<bool>,
    /// bool: (2018.4.15f1 - 2021.2.16f1)
    pub m_IsContraintActive: Option<bool>,
}

/// Scene is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SceneManagement.Scene.html):
/**
Run-time data structure for *.unity file.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub enabled: Option<bool>,
    /// GUID: (2017.4.33f1 - 2022.2.0b16)
    pub guid: Option<GUID>,
    /// Vec<u8>: (3.4.0 - 3.4.0)
    pub m_PVSData: Option<Vec<u8>>,
    /// Vec<PPtr/*<Renderer>*/>: (3.4.0 - 3.4.0)
    pub m_PVSObjectsArray: Option<Vec<PPtr /*<Renderer>*/>>,
    /**Returns the relative path of the Scene. For example: "Assets/MyScenes/MyScene.unity".*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub path: Option<String>,
}

/// SceneDataContainer is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneDataContainer {
    pub m_SceneData: Vec<(SceneIdentifier, HierarchicalSceneData)>,
}

/// SceneIdentifier is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneIdentifier {
    pub guid: GUID,
    pub handle: i32,
}

/// SceneObjectIdentifier is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneObjectIdentifier {
    pub targetObject: i64,
    pub targetPrefab: i64,
}

/// SceneVisibilityState is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneVisibilityState {
    pub m_IsolationMode: bool,
    pub m_ScenePickingData: SceneDataContainer,
    pub m_SceneVisibilityData: SceneDataContainer,
    pub m_SceneVisibilityDataIsolated: SceneDataContainer,
}

/// ScenesUsingAssets is a  class of the Unity engine since version 2020.1.0a20.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Build.Reporting.ScenesUsingAssets.html):
/**
An extension to the BuildReport class that tracks which scenes in the build have references to a specific asset in the build.
The build process generates this information when BuildOptions.DetailedBuildReport is used during a build.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ScenesUsingAssets {
    pub m_ListOfScenesUsingEachAsset: Vec<(String, Vec<String>)>,
    pub m_ScenesUsingAssets: Vec<BuildReportScenesUsingAsset>,
}

/// ScriptMapper is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptMapper {
    pub m_Shaders: NameToObjectMap,
    /// bool: (5.6.0b2 - 2020.3.42f1)
    pub m_PreloadShaders: Option<bool>,
}

/// ScriptedImporter is a  class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AssetImporters.ScriptedImporter.html):
/**
Abstract base class for custom Asset importers.
Scripted importers are scripts that are associated with specific file extensions. They are invoked by Unity's Asset pipeline to convert the contents of associated files into Assets.Use the ScriptedImporterAttribute class to register custom importers with the Asset pipeline.The C# fields of a ScriptedImporter are serialized, exactly like fields on a MonoBehaviour. See Script Serialization for details.
You can see these properties in the Inspector and use them to control the behaviour of the importer for each asset.
To programmatically access the value of an asset's properties, use AssetImporter.GetAtPath and type cast the return value to the correct class derived from ScriptedImporter.
After changing values, trigger a fresh import by calling EditorUtility.SetDirty and then AssetImporter.SaveAndReimport.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptedImporter {
    /**Get or set the AssetBundle name.*/
    pub m_AssetBundleName: String,
    /**Get or set the AssetBundle variant.*/
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Script: PPtr, /*<MonoScript>*/
    /**Get or set any user data.*/
    pub m_UserData: String,
    /// Vec<(i64, String)>: (2017.4.33f1 - 2018.4.15f1)
    pub m_FileIDToRecycleName: Option<Vec<(i64, String)>>,
    /// Vec<((i32, i64), String)>: (2019.3.0f4 - 2022.2.0b16)
    pub m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// SecondarySpriteTexture is a sub class of the Unity engine since version 2019.3.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SecondarySpriteTexture.html):
/**
Encapsulates a Texture2D and its shader property name to give Sprite-based renderers access to a secondary texture, in addition to the main Sprite texture.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SecondarySpriteTexture {
    /**The shader property name of the secondary Sprite texture. Use this name to identify and sample the texture in the shader.*/
    pub name: String,
    /**The texture to be used as a secondary Sprite texture.*/
    pub texture: PPtr, /*<Texture2D>*/
}

/// SecondaryTextureSettings is a sub class of the Unity engine since version 2020.3.42f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecondaryTextureSettings {
    pub platformSettings: Vec<TextureImporterPlatformSettings>,
    pub sRGB: bool,
}

/// SerializableManagedHost is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableManagedHost {
    pub m_Script: PPtr, /*<MonoScript>*/
}

/// SerializableManagedRefTestClass is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableManagedRefTestClass {
    pub m_Script: PPtr, /*<MonoScript>*/
}

/// SerializedCustomEditorForRenderPipeline is a sub class of the Unity engine since version 2021.2.16f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedCustomEditorForRenderPipeline {
    pub customEditorName: String,
    pub renderPipelineType: String,
}

/// SerializedPass is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedPass {
    pub m_HasInstancingVariant: bool,
    pub m_Name: String,
    pub m_NameIndices: Vec<(String, i32)>,
    pub m_ProgramMask: u32,
    pub m_State: SerializedShaderState,
    pub m_Tags: SerializedTagMap,
    pub m_TextureName: String,
    pub m_Type: i32,
    pub m_UseName: String,
    pub progDomain: SerializedProgram,
    pub progFragment: SerializedProgram,
    pub progGeometry: SerializedProgram,
    pub progHull: SerializedProgram,
    pub progVertex: SerializedProgram,
    /// Vec<Hash128>: (2020.3.42f1 - 2022.2.0b16)
    pub m_EditorDataHash: Option<Vec<Hash128>>,
    /// Vec<u16>: (2020.3.42f1 - 2020.3.42f1)
    pub m_GlobalKeywordMask: Option<Vec<u16>>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_HasProceduralInstancingVariant: Option<bool>,
    /// Vec<u16>: (2020.3.42f1 - 2020.3.42f1)
    pub m_LocalKeywordMask: Option<Vec<u16>>,
    /// Vec<u8>: (2020.3.42f1 - 2022.2.0b16)
    pub m_Platforms: Option<Vec<u8>>,
    /// Vec<u16>: (2021.2.16f1 - 2021.2.16f1)
    pub m_SerializedKeywordStateMask: Option<Vec<u16>>,
    /// SerializedProgram: (2019.3.0f4 - 2022.2.0b16)
    pub progRayTracing: Option<SerializedProgram>,
}

/// SerializedPlayerSubProgram is a sub class of the Unity engine since version 2022.2.0b16.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedPlayerSubProgram {
    pub m_BlobIndex: u32,
    pub m_GpuProgramType: i8,
    pub m_KeywordIndices: Vec<u16>,
    pub m_ShaderRequirements: i64,
}

/// SerializedProgram is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedProgram {
    pub m_SubPrograms: Vec<SerializedSubProgram>,
    /// SerializedProgramParameters: (2020.3.42f1 - 2022.2.0b16)
    pub m_CommonParameters: Option<SerializedProgramParameters>,
    /// Vec<Vec<u32>>: (2022.2.0b16 - 2022.2.0b16)
    pub m_ParameterBlobIndices: Option<Vec<Vec<u32>>>,
    /// Vec<Vec<SerializedPlayerSubProgram>>: (2022.2.0b16 - 2022.2.0b16)
    pub m_PlayerSubPrograms: Option<Vec<Vec<SerializedPlayerSubProgram>>>,
    /// Vec<u16>: (2022.2.0b16 - 2022.2.0b16)
    pub m_SerializedKeywordStateMask: Option<Vec<u16>>,
}

/// SerializedProgramParameters is a sub class of the Unity engine since version 2020.3.42f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedProgramParameters {
    pub m_BufferParams: Vec<BufferBinding>,
    pub m_ConstantBufferBindings: Vec<BufferBinding>,
    pub m_ConstantBuffers: Vec<ConstantBuffer>,
    pub m_MatrixParams: Vec<MatrixParameter>,
    pub m_Samplers: Vec<SamplerParameter>,
    pub m_TextureParams: Vec<TextureParameter>,
    pub m_UAVParams: Vec<UAVParameter>,
    pub m_VectorParams: Vec<VectorParameter>,
}

/// SerializedProperties is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedProperties {
    pub m_Props: Vec<SerializedProperty>,
}

/// SerializedProperty is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SerializedProperty.html):
/**
SerializedProperty and SerializedObject are classes for editing properties on objects in a completely generic way that automatically handles undo, multi-object editing and Prefab overrides.
SerializedProperty is primarily used to read or change the value of a property.  It can also iterate through the properties of an object using Next.
See Also: SerializedObject class, Editor class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedProperty {
    pub m_Attributes: Vec<String>,
    pub m_DefTexture: SerializedTextureProperty,
    pub m_Description: String,
    pub m_Flags: u32,
    /**Name of the property. (Read Only)*/
    pub m_Name: String,
    /**Type name of the property. (Read Only)*/
    pub m_Type: i32,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "m_DefValue[0]")]
    pub m_DefValue_0_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "m_DefValue[1]")]
    pub m_DefValue_1_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "m_DefValue[2]")]
    pub m_DefValue_2_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "m_DefValue[3]")]
    pub m_DefValue_3_: Option<f32>,
}

/// SerializedShader is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedShader {
    pub m_CustomEditorName: String,
    pub m_Dependencies: Vec<SerializedShaderDependency>,
    pub m_DisableNoSubshadersMessage: bool,
    pub m_FallbackName: String,
    pub m_Name: String,
    pub m_PropInfo: SerializedProperties,
    pub m_SubShaders: Vec<SerializedSubShader>,
    /// Vec<SerializedCustomEditorForRenderPipeline>: (2021.2.16f1 - 2022.2.0b16)
    pub m_CustomEditorForRenderPipelines: Option<Vec<SerializedCustomEditorForRenderPipeline>>,
    /// Vec<u8>: (2021.2.16f1 - 2022.2.0b16)
    pub m_KeywordFlags: Option<Vec<u8>>,
    /// Vec<String>: (2021.2.16f1 - 2022.2.0b16)
    pub m_KeywordNames: Option<Vec<String>>,
}

/// SerializedShaderDependency is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedShaderDependency {
    pub from: String,
    pub to: String,
}

/// SerializedShaderFloatValue is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedShaderFloatValue {
    pub name: String,
    pub val: f32,
}

/// SerializedShaderRTBlendState is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedShaderRTBlendState {
    pub blendOp: SerializedShaderFloatValue,
    pub blendOpAlpha: SerializedShaderFloatValue,
    pub colMask: SerializedShaderFloatValue,
    pub destBlend: SerializedShaderFloatValue,
    pub destBlendAlpha: SerializedShaderFloatValue,
    pub srcBlend: SerializedShaderFloatValue,
    pub srcBlendAlpha: SerializedShaderFloatValue,
}

/// SerializedShaderState is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedShaderState {
    pub alphaToMask: SerializedShaderFloatValue,
    pub culling: SerializedShaderFloatValue,
    pub fogColor: SerializedShaderVectorValue,
    pub fogDensity: SerializedShaderFloatValue,
    pub fogEnd: SerializedShaderFloatValue,
    pub fogMode: i32,
    pub fogStart: SerializedShaderFloatValue,
    pub gpuProgramID: i32,
    pub lighting: bool,
    pub m_LOD: i32,
    pub m_Name: String,
    pub m_Tags: SerializedTagMap,
    pub offsetFactor: SerializedShaderFloatValue,
    pub offsetUnits: SerializedShaderFloatValue,
    pub rtBlend0: SerializedShaderRTBlendState,
    pub rtBlend1: SerializedShaderRTBlendState,
    pub rtBlend2: SerializedShaderRTBlendState,
    pub rtBlend3: SerializedShaderRTBlendState,
    pub rtBlend4: SerializedShaderRTBlendState,
    pub rtBlend5: SerializedShaderRTBlendState,
    pub rtBlend6: SerializedShaderRTBlendState,
    pub rtBlend7: SerializedShaderRTBlendState,
    pub rtSeparateBlend: bool,
    pub stencilOp: SerializedStencilOp,
    pub stencilOpBack: SerializedStencilOp,
    pub stencilOpFront: SerializedStencilOp,
    pub stencilReadMask: SerializedShaderFloatValue,
    pub stencilRef: SerializedShaderFloatValue,
    pub stencilWriteMask: SerializedShaderFloatValue,
    pub zTest: SerializedShaderFloatValue,
    pub zWrite: SerializedShaderFloatValue,
    /// SerializedShaderFloatValue: (2020.1.0a20 - 2022.2.0b16)
    pub conservative: Option<SerializedShaderFloatValue>,
    /// SerializedShaderFloatValue: (2017.4.33f1 - 2022.2.0b16)
    pub zClip: Option<SerializedShaderFloatValue>,
}

/// SerializedShaderVectorValue is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedShaderVectorValue {
    pub name: String,
    pub w: SerializedShaderFloatValue,
    pub x: SerializedShaderFloatValue,
    pub y: SerializedShaderFloatValue,
    pub z: SerializedShaderFloatValue,
}

/// SerializedStencilOp is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedStencilOp {
    pub comp: SerializedShaderFloatValue,
    pub fail: SerializedShaderFloatValue,
    pub pass: SerializedShaderFloatValue,
    pub zFail: SerializedShaderFloatValue,
}

/// SerializedSubProgram is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedSubProgram {
    pub m_BlobIndex: u32,
    pub m_Channels: ParserBindChannels,
    pub m_GpuProgramType: i8,
    pub m_ShaderHardwareTier: i8,
    /// Vec<BufferBinding>: (5.6.0b2 - 2020.1.0a20)
    pub m_BufferParams: Option<Vec<BufferBinding>>,
    /// Vec<BufferBinding>: (5.6.0b2 - 2020.1.0a20)
    pub m_ConstantBufferBindings: Option<Vec<BufferBinding>>,
    /// Vec<ConstantBuffer>: (5.6.0b2 - 2020.1.0a20)
    pub m_ConstantBuffers: Option<Vec<ConstantBuffer>>,
    /// Vec<u16>: (2019.3.0f4 - 2020.3.42f1)
    pub m_GlobalKeywordIndices: Option<Vec<u16>>,
    /// Vec<u16>: (5.6.0b2 - 2022.2.0b16)
    pub m_KeywordIndices: Option<Vec<u16>>,
    /// Vec<u16>: (2019.3.0f4 - 2020.3.42f1)
    pub m_LocalKeywordIndices: Option<Vec<u16>>,
    /// Vec<MatrixParameter>: (5.6.0b2 - 2020.1.0a20)
    pub m_MatrixParams: Option<Vec<MatrixParameter>>,
    /// SerializedProgramParameters: (2020.3.42f1 - 2022.2.0b16)
    pub m_Parameters: Option<SerializedProgramParameters>,
    /// Vec<SamplerParameter>: (2017.4.33f1 - 2020.1.0a20)
    pub m_Samplers: Option<Vec<SamplerParameter>>,
    /// i32: (2017.4.33f1 - 2020.3.42f1); i64: (2021.2.16f1 - 2022.2.0b16)
    pub m_ShaderRequirements: Option<i64>,
    /// Vec<TextureParameter>: (5.6.0b2 - 2020.1.0a20)
    pub m_TextureParams: Option<Vec<TextureParameter>>,
    /// Vec<UAVParameter>: (5.6.0b2 - 2020.1.0a20)
    pub m_UAVParams: Option<Vec<UAVParameter>>,
    /// Vec<VectorParameter>: (5.6.0b2 - 2020.1.0a20)
    pub m_VectorParams: Option<Vec<VectorParameter>>,
}

/// SerializedSubShader is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedSubShader {
    pub m_LOD: i32,
    pub m_Passes: Vec<SerializedPass>,
    pub m_Tags: SerializedTagMap,
}

/// SerializedTagMap is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedTagMap {
    pub tags: Vec<(String, String)>,
}

/// SerializedTextureProperty is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedTextureProperty {
    pub m_DefaultName: String,
    pub m_TexDim: i32,
}

/// Shader is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Shader.html):
/**
Shader scripts used for all rendering.
Most of the advanced rendering is controlled via Material class. Shader class is mostly

used just to check whether a shader can run on the user's hardware (isSupported property), setting up

global shader properties and keywords, and finding shaders by name (Find method).See Also: Material class, Materials, ShaderLab documentation.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Shader {
    /**The name of the object.*/
    pub m_Name: String,
    /// Vec<u8>: (5.6.0b2 - 2022.2.0b16)
    pub compressedBlob: Option<Vec<u8>>,
    /// Vec<u32>: (5.6.0b2 - 2018.4.15f1); Vec<Vec<u32>>: (2019.3.0f4 - 2022.2.0b16)
    pub compressedLengths: Option<Vec<Enum_Vec_u32___u32>>,
    /// Vec<u32>: (5.6.0b2 - 2018.4.15f1); Vec<Vec<u32>>: (2019.3.0f4 - 2022.2.0b16)
    pub decompressedLengths: Option<Vec<Enum_Vec_u32___u32>>,
    /// Vec<PPtr/*<Shader>*/>: (5.6.0b2 - 2022.2.0b16)
    pub m_Dependencies: Option<Vec<PPtr /*<Shader>*/>>,
    /// Vec<(String, PPtr/*<Texture>*/)>: (2018.4.15f1 - 2022.2.0b16)
    pub m_NonModifiableTextures: Option<Vec<(String, PPtr /*<Texture>*/)>>,
    /// SerializedShader: (5.6.0b2 - 2022.2.0b16)
    pub m_ParsedForm: Option<SerializedShader>,
    /// String: (3.4.0 - 3.4.0)
    pub m_PathName: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    pub m_Script: Option<String>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_ShaderIsBaked: Option<bool>,
    /// Vec<u32>: (5.6.0b2 - 2018.4.15f1); Vec<Vec<u32>>: (2019.3.0f4 - 2022.2.0b16)
    pub offsets: Option<Vec<Enum_Vec_u32___u32>>,
    /// Vec<u32>: (5.6.0b2 - 2022.2.0b16)
    pub platforms: Option<Vec<u32>>,
    /// Vec<u32>: (2022.2.0b16 - 2022.2.0b16)
    pub stageCounts: Option<Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_Vec_u32___u32 {
    Vec(Vec<u32>),
    u32(u32),
}

/// ShaderBindChannel is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderBindChannel {
    pub source: i8,
    pub target: i8,
}

/// ShaderContainer is a  class of the Unity engine since version 2022.2.0b16.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderContainer {}

/// ShaderImporter is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ShaderImporter.html):
/**
Shader importer lets you modify shader import settings from Editor scripts.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderImporter {
    /**Get or set the AssetBundle name.*/
    pub m_AssetBundleName: String,
    /**Get or set the AssetBundle variant.*/
    pub m_AssetBundleVariant: String,
    pub m_DefaultTextures: Vec<(String, PPtr /*<Texture>*/)>,
    /**The name of the object.*/
    pub m_Name: String,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /// Vec<(String, PPtr/*<Texture>*/)>: (2018.4.15f1 - 2022.2.0b16)
    pub m_NonModifiableTextures: Option<Vec<(String, PPtr /*<Texture>*/)>>,
    /**This property has no effect.*/
    /// i32: (2020.3.42f1 - 2021.2.16f1)
    pub m_PreprocessorOverride: Option<i32>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// ShaderIncludeImporter is a  class of the Unity engine since version 2021.2.16f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderIncludeImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// ShaderInfo is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ShaderInfo.html):
/**
Contains the following information about a shader:
-If the shader has compilation errors or warnings.
-If the shader is supported on the currently selected platform.
-The name of the shader.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderInfo {
    pub variants: Vec<VariantInfo>,
}

/// ShaderNameRegistry is a  class of the Unity engine since version 2021.2.16f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderNameRegistry {
    pub m_PreloadShaders: bool,
    pub m_Shaders: NameToObjectMap,
}

/// ShaderVariantCollection is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ShaderVariantCollection.html):
/**
ShaderVariantCollection records which shader variants are actually used in each shader.
This is used for shader preloading ("warmup"), so that a game can make sure "actually required"
shader variants are loaded at startup (or level load time), to avoid shader compilation related hiccups later on in the game.In Unity, many shaders internally have multiple "variants", to account for different light modes, lightmaps, shadows and so on. These variants are identified by a shader pass type, and a set of shader keywords. See ShaderVariant.Typical use of ShaderVariantCollection is to record the shader variants used during a play session from the editor (under Graphics Settings), save them out as an asset, and add to the list of preloaded shaders (again in Graphics Settings). Additionally, you could call WarmUp on a ShaderVariantCollection object manually.ShaderVariantCollection generally replaces the old Shader.WarmupAllShaders function.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderVariantCollection {
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Shaders: Vec<(PPtr /*<Shader>*/, ShaderInfo)>,
}

/// ShadowSettings is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShadowSettings {
    pub m_Bias: f32,
    pub m_Resolution: i32,
    pub m_Strength: f32,
    pub m_Type: i32,
    /// Matrix4x4f: (2019.3.0f4 - 2022.2.0b16)
    pub m_CullingMatrixOverride: Option<Matrix4x4f>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_CustomResolution: Option<i32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_NearPlane: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_NormalBias: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    pub m_Softness: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    pub m_SoftnessFade: Option<f32>,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_UseCullingMatrixOverride: Option<bool>,
}

/// ShapeModule is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.ShapeModule.html):
/**
Script interface for the ShapeModule.
Configures the initial positions and directions of particles.See Also: ParticleSystem, ParticleSystem.shape.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeModule {
    /**Align particles based on their initial direction of travel.*/
    pub alignToDirection: bool,
    /**Angle of the cone to emit particles from.*/
    pub angle: f32,
    /**Angle of the circle arc to emit particles from.*/
    pub arc: Enum_f32__MultiModeParameter,
    /**Specifies whether the ShapeModule is enabled or disabled.*/
    pub enabled: bool,
    /**Length of the cone to emit particles from.*/
    pub length: f32,
    /**Mesh to emit particles from.*/
    pub m_Mesh: PPtr, /*<Mesh>*/
    /**Emit particles from a single Material of a Mesh.*/
    pub m_MeshMaterialIndex: i32,
    pub m_MeshNormalOffset: f32,
    /**MeshRenderer to emit particles from.*/
    pub m_MeshRenderer: PPtr, /*<MeshRenderer>*/
    /**SkinnedMeshRenderer to emit particles from.*/
    pub m_SkinnedMeshRenderer: PPtr, /*<SkinnedMeshRenderer>*/
    /**Modulate the particle colors with the vertex colors, or the Material color if no vertex colors exist.*/
    pub m_UseMeshColors: bool,
    /**Emit particles from a single Material, or the whole Mesh.*/
    pub m_UseMeshMaterialIndex: bool,
    pub placementMode: i32,
    /**Radius of the shape to emit particles from.*/
    pub radius: Enum_f32__MultiModeParameter,
    /**Randomizes the starting direction of particles.*/
    pub randomDirectionAmount: f32,
    /**Makes particles move in a spherical direction from their starting point.*/
    pub sphericalDirectionAmount: f32,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /**Thickness of the box to emit particles from.*/
    /// Vector3f: (2017.4.33f1 - 2022.2.0b16)
    pub boxThickness: Option<Vector3f>,
    /// f32: (5.6.0b2 - 5.6.0b2)
    pub boxX: Option<f32>,
    /// f32: (5.6.0b2 - 5.6.0b2)
    pub boxY: Option<f32>,
    /// f32: (5.6.0b2 - 5.6.0b2)
    pub boxZ: Option<f32>,
    /**The thickness of the Donut shape to emit particles from.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub donutRadius: Option<f32>,
    /// f32: (5.6.0b2 - 5.6.0b2)
    pub m_MeshScale: Option<f32>,
    /// MultiModeParameter: (2018.4.15f1 - 2022.2.0b16)
    pub m_MeshSpawn: Option<MultiModeParameter>,
    /**Apply an offset to the position from which the system emits particles.*/
    /// Vector3f: (2017.4.33f1 - 2022.2.0b16)
    pub m_Position: Option<Vector3f>,
    /**Apply a rotation to the shape from which the system emits particles.*/
    /// Vector3f: (2017.4.33f1 - 2022.2.0b16)
    pub m_Rotation: Option<Vector3f>,
    /**Apply scale to the shape from which the system emits particles.*/
    /// Vector3f: (2017.4.33f1 - 2022.2.0b16)
    pub m_Scale: Option<Vector3f>,
    /**Sprite to emit particles from.*/
    /// PPtr/*<Sprite>*/: (2018.4.15f1 - 2022.2.0b16)
    pub m_Sprite: Option<PPtr /*<Sprite>*/>,
    /**SpriteRenderer to emit particles from.*/
    /// PPtr/*<SpriteRenderer>*/: (2018.4.15f1 - 2022.2.0b16)
    pub m_SpriteRenderer: Option<PPtr /*<SpriteRenderer>*/>,
    /**Specifies a Texture to tint the particle's start colors.*/
    /// PPtr/*<Texture2D>*/: (2018.4.15f1 - 2022.2.0b16)
    pub m_Texture: Option<PPtr /*<Texture2D>*/>,
    /**When enabled, the system applies the alpha channel of the Texture to the particle alpha when the particle spawns.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_TextureAlphaAffectsParticles: Option<bool>,
    /**When enabled, the system takes four neighboring samples from the Texture then combines them to give the final particle value.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_TextureBilinearFiltering: Option<bool>,
    /**Selects which channel of the Texture to use for discarding particles.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_TextureClipChannel: Option<i32>,
    /**Discards particles when they spawn on an area of the Texture with a value lower than this threshold.*/
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub m_TextureClipThreshold: Option<f32>,
    /**When enabled, the system applies the RGB channels of the Texture to the particle color when the particle spawns.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_TextureColorAffectsParticles: Option<bool>,
    /**When using a Mesh as a source shape type, this option controls which UV channel on the Mesh to use for reading the source Texture.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_TextureUVChannel: Option<i32>,
    /**Radius thickness of the shape's edge from which to emit particles.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub radiusThickness: Option<f32>,
    /**Randomizes the starting position of particles.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub randomPositionAmount: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_f32__MultiModeParameter {
    f32(f32),
    MultiModeParameter(MultiModeParameter),
}

/// SiblingDerived is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct SiblingDerived {}

/// SizeBySpeedModule is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.SizeBySpeedModule.html):
/**
Script interface for the SizeBySpeedModule.
This module controls the size of particles based on their speeds.See Also: ParticleSystem, ParticleSystem.sizeBySpeed.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SizeBySpeedModule {
    pub curve: MinMaxCurve,
    /**Specifies whether the SizeBySpeedModule is enabled or disabled.*/
    pub enabled: bool,
    /**Set the minimum and maximum speed that this modules applies the size curve between.*/
    pub range: Vector2f,
    /**Set the size by speed on each axis separately.*/
    pub separateAxes: bool,
    /**Size by speed curve for the y-axis.*/
    pub y: MinMaxCurve,
    /**Size by speed curve for the z-axis.*/
    pub z: MinMaxCurve,
}

/// SizeModule is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SizeModule {
    pub curve: MinMaxCurve,
    pub enabled: bool,
    pub separateAxes: bool,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
}

/// SkeletonBone is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SkeletonBone.html):
/**
Details of the Transform name mapped to the skeleton bone of a model and its default position and rotation in the T-pose.
The skeleton models used in Unity have multiple bones.  The SkeletonBone struct has properties that are used to describe the position, rotation and scale of each bone.  The bones are not shown.  A MonoBehaviour.OnDrawGizmosSelected tool can be created to view the skeleton. An array of SkeletonBone positions can be used to make a line model using Gizmos.DrawLine.An array of SkeletonBones are used in HumanDescription.skeleton.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SkeletonBone {
    /**The name of the Transform mapped to the bone.*/
    pub m_Name: String,
    pub m_ParentName: String,
    /**The T-pose position of the bone in local space.*/
    pub m_Position: Vector3f,
    /**The T-pose rotation of the bone in local space.*/
    pub m_Rotation: Quaternionf,
    /**The T-pose scaling of the bone in local space.*/
    pub m_Scale: Vector3f,
}

/// SkeletonBoneLimit is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SkeletonBoneLimit {
    pub m_Length: f32,
    pub m_Max: Vector3f,
    pub m_Min: Vector3f,
    pub m_Modified: bool,
    pub m_Value: Vector3f,
}

/// SketchUpImportCamera is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SketchUpImportCamera.html):
/**
Structure to hold camera data extracted from a SketchUp file.
When importing a SketchUp file, Unity retrieves the current camera view the file is saved with and the camera view of all the scenes in the SketchUp file. The camera data of each Scene is stored in SketchUpImportSceneThis can be extracted later from SketchUpImporter.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SketchUpImportCamera {
    /**Aspect ratio of the camera.*/
    pub aspectRatio: f32,
    pub fov: f32,
    /**Indicate if the camera is using a perspective or orthogonal projection.*/
    pub isPerspective: i32,
    /**The position the camera is looking at.*/
    pub lookAt: Vector3f,
    /**The orthogonal projection size of the camera. This value only make sense if SketchUpImportCamera.isPerspective is false.*/
    pub orthoSize: f32,
    /**The position of the camera.*/
    pub position: Vector3f,
    /**Up vector of the camera.*/
    pub up: Vector3f,
    /**The near clipping plane distance.*/
    /// f32: (2019.3.0f4 - 2022.2.0b16)
    pub farPlane: Option<f32>,
    /**The far clipping plane distance.*/
    /// f32: (2019.3.0f4 - 2022.2.0b16)
    pub nearPlane: Option<f32>,
}

/// SketchUpImportData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SketchUpImportData {
    pub defaultCamera: SketchUpImportCamera,
    pub scenes: Vec<SketchUpImportScene>,
}

/// SketchUpImportScene is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SketchUpImportScene.html):
/**
Structure to hold scene data extracted from a SketchUp file.
When importing a SketchUp file, Unity retrieves all the scenes in the SketchUp file.This can be extracted later from SketchUpImporter with SketchUpImporter.GetScenes.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SketchUpImportScene {
    /**The camera data of the SketchUp scene.*/
    pub camera: SketchUpImportCamera,
    /**The name of the SketchUp scene.*/
    pub name: String,
}

/// SketchUpImporter is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SketchUpImporter.html):
/**
Derives from AssetImporter to handle importing of SketchUp files.
From the SketchUpImporter, you can access certain properties that are extracted from the SketchUp file.The following is an example of showing the geo coordinate extracted from the SketchUp file.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SketchUpImporter {
    /**Generate secondary UV set for lightmapping.*/
    pub generateSecondaryUV: bool,
    /**If this is true, any quad faces that exist in the mesh data before it is imported are kept as quads instead of being split into two triangles, for the purposes of tessellation. Set this to false to disable this behavior.*/
    pub keepQuads: bool,
    pub m_AddColliders: bool,
    pub m_AdditionalBone: bool,
    /**Animation compression setting.*/
    pub m_AnimationCompression: i32,
    pub m_AnimationDoRetargetingWarnings: bool,
    pub m_AnimationImportErrors: String,
    pub m_AnimationImportWarnings: String,
    /**Allowed error of animation position compression.*/
    pub m_AnimationPositionError: f32,
    pub m_AnimationRetargetingWarnings: String,
    /**Allowed error of animation rotation compression.*/
    pub m_AnimationRotationError: f32,
    /**Allowed error of animation scale compression.*/
    pub m_AnimationScaleError: f32,
    /**Animator generation mode.*/
    pub m_AnimationType: i32,
    /**The default wrap mode for the generated animation clips.*/
    pub m_AnimationWrapMode: i32,
    /**Get or set the AssetBundle name.*/
    pub m_AssetBundleName: String,
    /**Get or set the AssetBundle variant.*/
    pub m_AssetBundleVariant: String,
    pub m_AssetHash: Hash128,
    pub m_BakeSimulation: bool,
    /**Animation clips to split animation into. See Also: ModelImporterClipAnimation.*/
    pub m_ClipAnimations: Vec<ClipAnimationInfo>,
    /**Animation optimization setting.*/
    pub m_ExtraExposedTransformPaths: Vec<String>,
    /**Scaling factor used when useFileScale is set to true (Read-only).*/
    pub m_FileScale: f32,
    pub m_FileUnit: i32,
    pub m_GenerateBackFace: bool,
    /**Global scale factor for importing.*/
    pub m_GlobalScale: f32,
    pub m_HasExtraRoot: bool,
    /**The human description that is used to generate an Avatar during the import process.*/
    pub m_HumanDescription: HumanDescription,
    /**Controls how much oversampling is used when importing humanoid animations for retargeting.*/
    pub m_HumanoidOversampling: i32,
    /**Import animation from file.*/
    pub m_ImportAnimation: bool,
    /**Controls import of BlendShapes.*/
    pub m_ImportBlendShapes: bool,
    pub m_ImportedRoots: Vec<PPtr /*<GameObject>*/>,
    /**Generates the list of all imported take.*/
    pub m_ImportedTakeInfos: Vec<TakeInfo>,
    /**Are mesh vertices and indices accessible from script?*/
    pub m_IsReadable: bool,
    pub m_LODScreenPercentages: Vec<f32>,
    pub m_LastHumanDescriptionAvatarSource: PPtr, /*<Avatar>*/
    /**Retrieves the latitude Geo Coordinate imported from the SketchUp file.*/
    pub m_Latitude: f64,
    pub m_LegacyGenerateAnimations: i32,
    /**Retrieves the longitude Geo Coordinate imported from the SketchUp file.*/
    pub m_Longitude: f64,
    /**Material naming setting.*/
    pub m_MaterialName: i32,
    /**Existing material search setting.*/
    pub m_MaterialSearch: i32,
    pub m_MergeCoplanarFaces: bool,
    /**Mesh compression setting.*/
    pub m_MeshCompression: i32,
    /**The path of the transform used to generation the motion of the animation.*/
    pub m_MotionNodeName: String,
    /**The name of the object.*/
    pub m_Name: String,
    /**Retrieves the north correction value imported from the SketchUp file.*/
    pub m_NorthCorrection: f64,
    /**Animation optimization setting.*/
    pub m_OptimizeGameObjects: bool,
    /**Generates the list of all imported Animations.*/
    pub m_ReferencedClips: Vec<GUID>,
    /**If set to false, the importer will not resample curves when possible.Read more about animation curve resampling.Notes:- Some unsupported FBX features (such as PreRotation or PostRotation on transforms) will override this setting. In these situations, animation curves will still be resampled even if the setting is disabled. For best results, avoid using PreRotation, PostRotation and GetRotationPivot.- This option was introduced in Version 5.3. Prior to this version, Unity's import behaviour was as if this option was always enabled. Therefore enabling the option gives the same behaviour as pre-5.3 animation import.*/
    pub m_ResampleCurves: bool,
    pub m_RigImportErrors: String,
    pub m_RigImportWarnings: String,
    pub m_SelectedNodes: Vec<i32>,
    pub m_SketchUpImportData: SketchUpImportData,
    /**Use FileScale when importing.*/
    pub m_UseFileScale: bool,
    /**Detect file units and import as 1FileUnit=1UnityUnit, otherwise it will import as 1cm=1UnityUnit.*/
    pub m_UseFileUnits: bool,
    /**Get or set any user data.*/
    pub m_UserData: String,
    pub normalImportMode: i32,
    pub normalSmoothAngle: f32,
    /**Threshold for angle distortion (in degrees) when generating secondary UV.*/
    pub secondaryUVAngleDistortion: f32,
    /**Threshold for area distortion when generating secondary UV.*/
    pub secondaryUVAreaDistortion: f32,
    /**Hard angle (in degrees) for generating secondary UV.*/
    pub secondaryUVHardAngle: f32,
    /**Margin to be left between charts when packing secondary UV.*/
    pub secondaryUVPackMargin: f32,
    /**Swap primary and secondary UV channels when importing.*/
    pub swapUVChannels: bool,
    pub tangentImportMode: i32,
    /**Combine vertices that share the same position in space.*/
    pub weldVertices: bool,
    /**Computes the axis conversion on geometry and animation for Models defined in an axis system that differs from Unity's (left handed, Z forward, Y-up).                     When enabled, Unity transforms the geometry and animation data in order to convert the axis.                     When disabled, Unity transforms the root GameObject of the hierarchy in order to convert the axis.*/
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub bakeAxisConversion: Option<bool>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub blendShapeNormalImportMode: Option<i32>,
    /**Format of the imported mesh index buffer data.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub indexFormat: Option<i32>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub legacyComputeAllNormalsFromSmoothingGroupsWhenMeshHasBlendShapes: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_AddHumanoidExtraRootOnlyWhenUsingAvatar: Option<bool>,
    /**Generate auto mapping if no avatarSetup is provided when importing humanoid animation.*/
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_AutoGenerateAvatarMappingIfUnspecified: Option<bool>,
    /**The Avatar generation of the imported model.*/
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_AvatarSetup: Option<i32>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub m_CopyAvatar: Option<bool>,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /**A list of default FBX properties to treat as user properties during OnPostprocessGameObjectWithUserProperties.*/
    /// Vec<String>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExtraUserProperties: Option<Vec<String>>,
    /// Vec<(i64, String)>: (5.6.0b2 - 2018.4.15f1)
    pub m_FileIDToRecycleName: Option<Vec<(i64, String)>>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_FileIdsGeneration: Option<i32>,
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub m_FileScaleFactor: Option<f32>,
    /// String: (2018.4.15f1 - 2022.2.0b16)
    pub m_FileScaleUnit: Option<String>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_HasEmbeddedTextures: Option<bool>,
    /// bool: (2018.4.15f1 - 2018.4.15f1)
    pub m_HasPreviousCalculatedGlobalScale: Option<bool>,
    /**Import animated custom properties from file.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_ImportAnimatedCustomProperties: Option<bool>,
    /**Import BlendShapes deform percent.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ImportBlendShapeDeformPercent: Option<bool>,
    /**Controls import of cameras. Basic properties like field of view, near plane distance and far plane distance can be animated.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_ImportCameras: Option<bool>,
    /**Import animation constraints.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_ImportConstraints: Option<bool>,
    /**Controls import of lights. Note that because light are defined differently in DCC tools, some light types or properties may not be exported. Basic properties like color and intensity can be animated.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_ImportLights: Option<bool>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub m_ImportMaterials: Option<bool>,
    /**Use visibility properties to enable or disable MeshRenderer components.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_ImportVisibility: Option<bool>,
    /// Vec<((i32, i64), String)>: (2019.3.0f4 - 2022.2.0b16)
    pub m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
    /**Material creation options.*/
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_MaterialImportMode: Option<i32>,
    /**Material import location options.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_MaterialLocation: Option<i32>,
    /// Vec<SourceAssetIdentifier>: (2017.4.33f1 - 2022.2.0b16)
    pub m_Materials: Option<Vec<SourceAssetIdentifier>>,
    /// i32: (2021.2.16f1 - 2022.2.0b16)
    pub m_NodeNameCollisionStrategy: Option<i32>,
    /**If true, always create an explicit Prefab root. Otherwise, if the model has a single root, it is reused as the Prefab root.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_PreserveHierarchy: Option<bool>,
    /// f32: (2018.4.15f1 - 2018.4.15f1)
    pub m_PreviousCalculatedGlobalScale: Option<f32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_RemapMaterialsIfMaterialImportModeIsNone: Option<bool>,
    /**Removes constant animation curves with values identical to the object initial scale value.*/
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub m_RemoveConstantScaleCurves: Option<bool>,
    /**Sorts the gameObject hierarchy by name.*/
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_SortHierarchyByName: Option<bool>,
    /**Enables strict checks on imported vertex data.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_StrictVertexDataChecks: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_SupportsEmbeddedMaterials: Option<bool>,
    /**When disabled, imported material albedo colors are converted to gamma space. This property should be disabled when using linear color space in Player rendering settings.The default value is true.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_UseSRGBMaterialColor: Option<bool>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /**The maximum number of bones per vertex stored in this mesh data.*/
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub maxBonesPerVertex: Option<i32>,
    /**Options to control the optimization of mesh data during asset import.*/
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub meshOptimizationFlags: Option<i32>,
    /**Minimum bone weight to keep.*/
    /// f32: (2019.3.0f4 - 2022.2.0b16)
    pub minBoneWeight: Option<f32>,
    /**Normal generation options for ModelImporter.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub normalCalculationMode: Option<i32>,
    /**Source of smoothing information for calculation of normals.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub normalSmoothingSource: Option<i32>,
    /**Only import bones where they are connected to vertices.*/
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub optimizeBones: Option<bool>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub optimizeMeshForGPU: Option<bool>,
    /**Method to use for handling margins when generating secondary UV.*/
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub secondaryUVMarginMethod: Option<i32>,
    /**The minimum lightmap resolution in texels per unit that the associated model is expected to have.*/
    /// f32: (2020.1.0a20 - 2022.2.0b16)
    pub secondaryUVMinLightmapResolution: Option<f32>,
    /**The minimum object scale that the associated model is expected to have.*/
    /// f32: (2020.1.0a20 - 2022.2.0b16)
    pub secondaryUVMinObjectScale: Option<f32>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub skinWeightsMode: Option<i32>,
}

/// SkinnedCloth is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SkinnedCloth {
    pub m_BendingStiffness: f32,
    pub m_Coefficients: Vec<ClothConstrainCoefficients>,
    pub m_Damping: f32,
    pub m_Enabled: u8,
    pub m_ExternalAcceleration: Vector3f,
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_RandomAcceleration: Vector3f,
    pub m_SelfCollision: bool,
    pub m_StretchingStiffness: f32,
    pub m_Thickness: f32,
    pub m_UseGravity: bool,
    pub m_WorldAccelerationScale: f32,
    pub m_WorldVelocityScale: f32,
}

/// SkinnedMeshRenderer is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SkinnedMeshRenderer.html):
/**
The Skinned Mesh filter.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SkinnedMeshRenderer {
    pub m_AABB: AABB,
    /**The bones used to skin the mesh.*/
    pub m_Bones: Vec<PPtr /*<Transform>*/>,
    pub m_CastShadows: Enum_bool__u8,
    pub m_DirtyAABB: bool,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapTilingOffset: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    pub m_Materials: Vec<PPtr /*<Material>*/>,
    pub m_Mesh: PPtr, /*<Mesh>*/
    /**The maximum number of bones per vertex that are taken into account during skinning.*/
    pub m_Quality: i32,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: Enum_bool__u8,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    /**If enabled, the Skinned Mesh will be updated when offscreen. If disabled, this also disables updating animations.*/
    pub m_UpdateWhenOffscreen: bool,
    /// Vec<f32>: (5.6.0b2 - 2022.2.0b16)
    pub m_BlendShapeWeights: Option<Vec<f32>>,
    /// u8: (2017.4.33f1 - 2022.2.0b16)
    pub m_DynamicOccludee: Option<u8>,
    /**The light probe interpolation type.*/
    /// u8: (5.6.0b2 - 2022.2.0b16)
    pub m_LightProbeUsage: Option<u8>,
    /// PPtr/*<GameObject>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_LightProbeVolumeOverride: Option<PPtr /*<GameObject>*/>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub m_LightmapIndexDynamic: Option<u16>,
    /// Vector4f: (5.6.0b2 - 2022.2.0b16)
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    pub m_MotionVectors: Option<u8>,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr/*<Transform>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_ProbeAnchor: Option<PPtr /*<Transform>*/>,
    /// u8: (2020.1.0a20 - 2022.2.0b16)
    pub m_RayTraceProcedural: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0f4 - 2022.2.0b16)
    pub m_RayTracingMode: Option<u8>,
    /**Should reflection probes be used for this Renderer?*/
    /// u8: (5.6.0b2 - 2022.2.0b16)
    pub m_ReflectionProbeUsage: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on.*/
    /// u32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RenderingLayerMask: Option<u32>,
    /// PPtr/*<Transform>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_RootBone: Option<PPtr /*<Transform>*/>,
    /**Specifies whether skinned motion vectors should be used for this renderer.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_SkinnedMotionVectors: Option<bool>,
    /// i16: (5.6.0b2 - 2022.2.0b16)
    pub m_SortingLayer: Option<i16>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_SortingLayerID: Option<i32>,
    /**Renderer's order within a sorting layer.*/
    /// i16: (5.6.0b2 - 2022.2.0b16)
    pub m_SortingOrder: Option<i16>,
    /// StaticBatchInfo: (5.6.0b2 - 2022.2.0b16)
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.2.16f1 - 2022.2.0b16)
    pub m_StaticShadowCaster: Option<u8>,
    /// Vec<u32>: (3.4.0 - 3.4.0)
    pub m_SubsetIndices: Option<Vec<u32>>,
}

/// Skybox is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Skybox.html):
/**
A script interface for the skybox component.
The skybox class has only the material property.See Also: skybox component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Skybox {
    pub m_CustomSkybox: PPtr, /*<Material>*/
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
}

/// SliderJoint2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SliderJoint2D.html):
/**
Joint that restricts the motion of a Rigidbody2D object to a single line.
See Also: Rigidbody2D, DistanceJoint2D, HingeJoint2D, SpringJoint2D, JointTranslationLimits2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SliderJoint2D {
    /**The joint's anchor point on the object that has the joint component.*/
    pub m_Anchor: Vector2f,
    /**The angle of the line in space (in degrees).*/
    pub m_Angle: f32,
    /**Should the angle be calculated automatically?*/
    pub m_AutoConfigureAngle: bool,
    /**Should the connectedAnchor be calculated automatically?*/
    pub m_AutoConfigureConnectedAnchor: bool,
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break.*/
    pub m_BreakTorque: f32,
    /**The joint's anchor point on the second object (ie, the one which doesn't have the joint component).*/
    pub m_ConnectedAnchor: Vector2f,
    pub m_ConnectedRigidBody: PPtr, /*<Rigidbody2D>*/
    /**Should the two rigid bodies connected with this joint collide with each other?*/
    pub m_EnableCollision: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Parameters for a motor force that is applied automatically to the Rigibody2D along the line.*/
    pub m_Motor: JointMotor2D,
    pub m_TranslationLimits: JointTranslationLimits2D,
    /**Should motion limits be used?*/
    pub m_UseLimits: bool,
    /**Should a motor force be applied automatically to the Rigidbody2D?*/
    pub m_UseMotor: bool,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_BreakAction: Option<i32>,
}

/// SnapshotConstant is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotConstant {
    pub nameHash: u32,
    pub transitionIndices: Vec<u32>,
    pub transitionTypes: Vec<u32>,
    pub values: Vec<f32>,
}

/// SoftJointLimit is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SoftJointLimit.html):
/**
The limits defined by the CharacterJoint.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SoftJointLimit {
    /**When the joint hits the limit, it can be made to bounce off it.*/
    pub bounciness: f32,
    /**The limit position/angle of the joint (in degrees).*/
    pub limit: f32,
    /**Determines how far ahead in space the solver can "see" the joint limit.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub contactDistance: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    pub damper: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    pub spring: Option<f32>,
}

/// SoftJointLimitSpring is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SoftJointLimitSpring.html):
/**
The configuration of the spring attached to the joint's limits: linear and angular. Used by CharacterJoint and ConfigurableJoint.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SoftJointLimitSpring {
    /**The damping of the spring limit. In effect when the stiffness of the sprint limit is not zero.*/
    pub damper: f32,
    /**The stiffness of the spring limit. When stiffness is zero the limit is hard, otherwise soft.*/
    pub spring: f32,
}

/// SortingGroup is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rendering.SortingGroup.html):
/**
Adding a SortingGroup component to a GameObject will ensure that all Renderers within the GameObject's descendants will be sorted and rendered together.
A common use case for having a SortingGroup is to create complex 2D characters that are made up of multiple SpriteRenderers. When several clones of such a character overlap, their individual body parts might not be sorted properly resulting in a visual glitch where the the body parts interleave. For example, the hands of two characters might be sorted in front of their bodies, where you would expect one entire character to be drawn in front of the other character. The SortingGroup component solves this by ensuring the entire branch of the character are sorted and rendered together.The descendants of the SortingGroup are sorted using the same SortingLayer and Renderer.sortingOrder. However, they are only sorted against other descendants of the SortingGroup and not with any renderers outside of it. This allows you to reuse the same SortingLayers (for example, "Hands", "Torso"...) to sort body parts while ensuring they never interleave with other clones of the character.The SortingGroups, together with other renderers, are sorted using the SortingLayer and Renderer.sortingOrder. Additionally, they can be nested within other SortingGroups. This is useful if you have branches of descendants that should not be mixed up i.e. the "Left Hand" vs the "Right Hand" hierarchy branches.
.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SortingGroup {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_SortingLayer: i16,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    /**Ignores any parent SortingGroup and sorts this and its descendant Renderers against other Renderers at the root level.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_SortAtRoot: Option<bool>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_SortingLayerID: Option<i32>,
}

/// SortingLayerEntry is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SortingLayerEntry {
    pub name: String,
    pub uniqueID: u32,
}

/// SourceAssetIdentifier is a sub class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AssetImporter.SourceAssetIdentifier.html):
/**
Represents a unique identifier for a sub-asset embedded in an imported Asset (such as an FBX file).
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceAssetIdentifier {
    pub assembly: String,
    /**The name of the Asset.*/
    pub name: String,
    /**The type of the Asset.*/
    /// String: (2017.4.33f1 - 2022.2.0b16)
    #[serde(alias = "type")]
    pub _type: Option<String>,
}

/// SourceTextureInformation is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AssetImporters.SourceTextureInformation.html):
/**
Original texture data information.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceTextureInformation {
    pub doesTextureContainAlpha: bool,
    /**Height of the image data.*/
    pub height: i32,
    /**Width of the image data.*/
    pub width: i32,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub sourceWasHDR: Option<bool>,
}

/// SparseTexture is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SparseTexture.html):
/**
Class for handling Sparse Textures.
Sparse textures are textures where not the whole texture data can be present in memory at once. They are also commonly called "tiled textures" or "mega textures".Imagine a 16384x16384 texture at 32 bits per pixel - it would take 1GB of memory. The texture is broken down into rectangular "tiles", and each tile can either be present in memory or not. You can load & unload these tiles as needed based on distance from the camera, sectors of the world that the player has to see, etc.Otherwise, the sparse textures behave just like any other texture in shaders - they can have mipmaps, can use all texture filtering modes, etc. If you happen to read from a tile that's not present, you can get undefined result (on many GPUs the result is a black color, but that's not guaranteed).Not all hardware and platforms support sparse textures, so you should check SystemInfo.supportsSparseTextures before using them. For example, on DirectX systems they require DX11.2 (Windows 8.1) and a fairly recent GPU; and on OpenGL they require ARB_sparse_texture extension support. Sparse textures only support non-compressed texture formats.After creating the sparse texture, query the tile size with tileWidth & tileHeight. Tile sizes are platform and GPU dependent.Use UpdateTile or UpdateTileRaw to make a tile resident in memory and update its color data. Use UnloadTile to unload a tile.See Also:  Sparse Textures.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SparseTexture {
    pub m_ColorSpace: i32,
    pub m_Format: i32,
    /**Height of the Texture in pixels (Read Only).*/
    pub m_Height: i32,
    pub m_MipCount: i32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_TextureSettings: GLTextureSettings,
    /**Width of the Texture in pixels (Read Only).*/
    pub m_Width: i32,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_IsAlphaChannelOptional: Option<bool>,
}

/// SpeedTreeImporter is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SpeedTreeImporter.html):
/**
AssetImportor for importing SpeedTree model assets.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpeedTreeImporter {
    /**Gets and sets a default alpha test reference values.*/
    pub m_AlphaTestRef: f32,
    /**Indicates if the cross-fade LOD transition, applied to the last mesh LOD and the billboard, should be animated.*/
    pub m_AnimateCrossFading: bool,
    /**Get or set the AssetBundle name.*/
    pub m_AssetBundleName: String,
    /**Get or set the AssetBundle variant.*/
    pub m_AssetBundleVariant: String,
    /**Returns the best-possible wind quality on this asset (configured in SpeedTree modeler).*/
    pub m_BestWindQuality: i32,
    /**Proportion of the last 3D mesh LOD region width which is used for cross-fading to billboard tree.*/
    pub m_BillboardTransitionCrossFadeWidth: f32,
    /**Enables smooth LOD transitions.*/
    pub m_EnableSmoothLODTransition: bool,
    /**Proportion of the billboard LOD region width which is used for fading out the billboard.*/
    pub m_FadeOutWidth: f32,
    /**Tells if there is a billboard LOD.*/
    pub m_HasBillboard: bool,
    /**Gets and sets a default Hue variation color and amount (in alpha).*/
    pub m_HueVariation: ColorRGBA,
    pub m_LODSettings: Vec<PerLODSettings>,
    /**Gets and sets a default main color.*/
    pub m_MainColor: ColorRGBA,
    pub m_MaterialVersion: i32,
    /**The name of the object.*/
    pub m_Name: String,
    /**How much to scale the tree model compared to what is in the .spm file.*/
    pub m_ScaleFactor: f32,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_FileIDType: Option<i32>,
    /**Material import location options.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_MaterialLocation: Option<i32>,
    /// Vec<SourceAssetIdentifier>: (2018.4.15f1 - 2022.2.0b16)
    pub m_Materials: Option<Vec<SourceAssetIdentifier>>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_SupportsEmbeddedMaterials: Option<bool>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// SpeedTreeWind is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpeedTreeWind {
    pub BRANCH_DIRECTIONAL_1: bool,
    pub BRANCH_DIRECTIONAL_2: bool,
    pub BRANCH_DIRECTIONAL_FROND_1: bool,
    pub BRANCH_DIRECTIONAL_FROND_2: bool,
    pub BRANCH_OSC_COMPLEX_1: bool,
    pub BRANCH_OSC_COMPLEX_2: bool,
    pub BRANCH_SIMPLE_1: bool,
    pub BRANCH_SIMPLE_2: bool,
    pub BRANCH_TURBULENCE_1: bool,
    pub BRANCH_TURBULENCE_2: bool,
    pub BRANCH_WHIP_1: bool,
    pub BRANCH_WHIP_2: bool,
    pub BranchWindAnchor0: f32,
    pub BranchWindAnchor1: f32,
    pub BranchWindAnchor2: f32,
    pub FROND_RIPPLE_ADJUST_LIGHTING: bool,
    pub FROND_RIPPLE_ONE_SIDED: bool,
    pub FROND_RIPPLE_TWO_SIDED: bool,
    pub GLOBAL_PRESERVE_SHAPE: bool,
    pub GLOBAL_WIND: bool,
    pub LEAF_OCCLUSION_1: bool,
    pub LEAF_OCCLUSION_2: bool,
    pub LEAF_RIPPLE_COMPUTED_1: bool,
    pub LEAF_RIPPLE_COMPUTED_2: bool,
    pub LEAF_RIPPLE_VERTEX_NORMAL_1: bool,
    pub LEAF_RIPPLE_VERTEX_NORMAL_2: bool,
    pub LEAF_TUMBLE_1: bool,
    pub LEAF_TUMBLE_2: bool,
    pub LEAF_TWITCH_1: bool,
    pub LEAF_TWITCH_2: bool,
    pub ROLLING: bool,
    pub m_fMaxBranchLevel1Length: f32,
    pub m_sParams: SParams,
}

/// SpeedTreeWindAsset is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpeedTreeWindAsset {
    pub m_Name: String,
    pub m_Wind: SpeedTreeWind,
}

/// SphereCollider is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SphereCollider.html):
/**
A sphere-shaped primitive collider.
See Also: BoxCollider, CapsuleCollider, PhysicMaterial, Rigidbody.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SphereCollider {
    /**The center of the sphere in the object's local space.*/
    pub m_Center: Vector3f,
    /**Enabled Colliders will collide with other Colliders, disabled Colliders won't.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Specify if this collider is configured as a trigger.*/
    pub m_IsTrigger: bool,
    /**The material used by the collider.*/
    pub m_Material: PPtr, /*<PhysicMaterial>*/
    /**The radius of the sphere measured in the object's local space.*/
    pub m_Radius: f32,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ProvidesContacts: Option<bool>,
}

/// SphericalHarmonicsL2 is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rendering.SphericalHarmonicsL2.html):
/**
Spherical harmonics up to the second order (3 bands, 9 coefficients).
Spherical harmonics (SH) represent a function or signal over directions, and are commonly used in computer graphics to efficiently evaluate smooth lighting. Unity uses them for LightProbes and environment lighting.L2 spherical harmonics is composed of 9 coefficients for each color channel.See Also: RenderSettings.ambientMode, RenderSettings.ambientProbe, LightProbes.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SphericalHarmonicsL2 {
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[10]")]
    pub sh_10_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[11]")]
    pub sh_11_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[12]")]
    pub sh_12_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[13]")]
    pub sh_13_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[14]")]
    pub sh_14_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[15]")]
    pub sh_15_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[16]")]
    pub sh_16_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[17]")]
    pub sh_17_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[18]")]
    pub sh_18_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[19]")]
    pub sh_19_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[20]")]
    pub sh_20_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[21]")]
    pub sh_21_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[22]")]
    pub sh_22_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[23]")]
    pub sh_23_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[24]")]
    pub sh_24_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[25]")]
    pub sh_25_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[26]")]
    pub sh_26_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[ 0]")]
    pub sh__0_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[ 1]")]
    pub sh__1_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[ 2]")]
    pub sh__2_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[ 3]")]
    pub sh__3_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[ 4]")]
    pub sh__4_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[ 5]")]
    pub sh__5_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[ 6]")]
    pub sh__6_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[ 7]")]
    pub sh__7_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[ 8]")]
    pub sh__8_: Option<f32>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "sh[ 9]")]
    pub sh__9_: Option<f32>,
}

/// SplashScreenLogo is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PlayerSettings.SplashScreenLogo.html):
/**
A single logo that is shown during the Splash Screen. Controls the Sprite that is displayed and its duration.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SplashScreenLogo {
    /**The total time in seconds for which the logo is shown. The minimum duration is 2 seconds.*/
    pub duration: f32,
    /**The Sprite that is shown during this logo. If this is null, then no logo will be displayed for the duration.*/
    pub logo: PPtr, /*<Sprite>*/
}

/// SplatDatabase is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SplatDatabase {
    pub m_AlphaTextures: Vec<PPtr /*<Texture2D>*/>,
    pub m_AlphamapResolution: i32,
    pub m_BaseMapResolution: i32,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_ColorSpace: Option<i32>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub m_MaterialRequiresMetallic: Option<bool>,
    /// bool: (5.6.0b2 - 2017.4.33f1)
    pub m_MaterialRequiresSmoothness: Option<bool>,
    /// Vec<SplatPrototype>: (3.4.0 - 2017.4.33f1)
    pub m_Splats: Option<Vec<SplatPrototype>>,
    /// Vec<PPtr/*<TerrainLayer>*/>: (2018.4.15f1 - 2022.2.0b16)
    pub m_TerrainLayers: Option<Vec<PPtr /*<TerrainLayer>*/>>,
}

/// SplatPrototype is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SplatPrototype.html):
/**
A Splat prototype is just a texture that is used by the TerrainData.
A class on a Terrain GameObject.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SplatPrototype {
    /**Texture of the splat applied to the Terrain.*/
    pub texture: PPtr, /*<Texture2D>*/
    /**Offset of the tile texture of the SplatPrototype.*/
    pub tileOffset: Vector2f,
    /**Size of the tile used in the texture of the SplatPrototype.*/
    pub tileSize: Vector2f,
    /**Normal map of the splat applied to the Terrain.*/
    /// PPtr/*<Texture2D>*/: (5.6.0b2 - 2017.4.33f1)
    pub normalMap: Option<PPtr /*<Texture2D>*/>,
    /**The smoothness value of the splat layer when the main texture has no alpha channel.*/
    /// f32: (5.6.0b2 - 2017.4.33f1)
    pub smoothness: Option<f32>,
    /// Vector4f: (5.6.0b2 - 2017.4.33f1)
    pub specularMetallic: Option<Vector4f>,
}

/// SpringJoint is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SpringJoint.html):
/**
The spring joint ties together 2 rigid bodies, spring forces will be automatically applied to keep the object at the given distance.
The Spring attempts to maintain the distance it has when it starts out.

So if your joint's start at a rest position where the two rigidbodies are far apart, then the joint will attempt to maintain that distance.

The minDistance and maxDistance properties add on top of this implicit distance.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpringJoint {
    /**The Position of the anchor around which the joints motion is constrained.*/
    pub m_Anchor: Vector3f,
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break. To be able to break, a joint must be _Locked_ or _Limited_ on the axis of rotation where the torque is being applied. This means that some joints cannot break, such as an unconstrained Configurable Joint.*/
    pub m_BreakTorque: f32,
    /**A reference to another rigidbody this joint connects to.*/
    pub m_ConnectedBody: PPtr, /*<Rigidbody>*/
    /**The damper force used to dampen the spring force.*/
    pub m_Damper: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The maximum distance between the bodies relative to their initial distance.*/
    pub m_MaxDistance: f32,
    /**The minimum distance between the bodies relative to their initial distance.*/
    pub m_MinDistance: f32,
    /**The spring force used to keep the two objects together.*/
    pub m_Spring: f32,
    /**Should the connectedAnchor be calculated automatically?*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    /**Position of the anchor relative to the connected Rigidbody.*/
    /// Vector3f: (5.6.0b2 - 2022.2.0b16)
    pub m_ConnectedAnchor: Option<Vector3f>,
    /**A reference to an articulation body this joint connects to.*/
    /// PPtr/*<ArticulationBody>*/: (2020.3.42f1 - 2022.2.0b16)
    pub m_ConnectedArticulationBody: Option<PPtr /*<ArticulationBody>*/>,
    /**The scale to apply to the inverse mass and inertia tensor of the connected body prior to solving the constraints.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ConnectedMassScale: Option<f32>,
    /**Enable collision between bodies connected with the joint.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_EnableCollision: Option<bool>,
    /**Toggle preprocessing for this joint.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_EnablePreprocessing: Option<bool>,
    /**The scale to apply to the inverse mass and inertia tensor of the body prior to solving the constraints.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_MassScale: Option<f32>,
    /**The maximum allowed error between the current spring length and the length defined by minDistance and maxDistance.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_Tolerance: Option<f32>,
}

/// SpringJoint2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SpringJoint2D.html):
/**
Joint that attempts to keep two Rigidbody2D objects a set distance apart by applying a force between them.
Note that unlike DistanceJoint2D, the length of the joint can stretch and oscillate.See Also: DistanceJoint2D, HingeJoint2D, SliderJoint2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpringJoint2D {
    /**The joint's anchor point on the object that has the joint component.*/
    pub m_Anchor: Vector2f,
    /**Should the connectedAnchor be calculated automatically?*/
    pub m_AutoConfigureConnectedAnchor: bool,
    /**Should the distance be calculated automatically?*/
    pub m_AutoConfigureDistance: bool,
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break.*/
    pub m_BreakTorque: f32,
    /**The joint's anchor point on the second object (ie, the one which doesn't have the joint component).*/
    pub m_ConnectedAnchor: Vector2f,
    pub m_ConnectedRigidBody: PPtr, /*<Rigidbody2D>*/
    /**The amount by which the spring force is reduced in proportion to the movement speed.*/
    pub m_DampingRatio: f32,
    /**The distance the spring will try to keep between the two objects.*/
    pub m_Distance: f32,
    /**Should the two rigid bodies connected with this joint collide with each other?*/
    pub m_EnableCollision: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The frequency at which the spring oscillates around the distance distance between the objects.*/
    pub m_Frequency: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_BreakAction: Option<i32>,
}

/// Sprite is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Sprite.html):
/**
Represents a Sprite object for use in 2D gameplay.
Sprites are 2D graphic objects used for characters, props, projectiles and other elements of 2D gameplay. The graphics are obtained from bitmap images - Texture2D. The Sprite class primarily identifies the section of the image that should be used for a specific Sprite. This information can then be used by a SpriteRenderer component on a GameObject to actually display the graphic.See Also: SpriteRenderer class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Sprite {
    /**Returns the border sizes of the Sprite.*/
    pub m_Border: Vector4f,
    pub m_Extrude: u32,
    pub m_IsPolygon: bool,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Offset: Vector2f,
    /**Location of the Sprite's center point in the Rect on the original Texture, specified in pixels.*/
    pub m_Pivot: Vector2f,
    pub m_PixelsToUnits: f32,
    pub m_RD: SpriteRenderData,
    /**Location of the Sprite on the original Texture, specified in pixels.*/
    pub m_Rect: Rectf,
    /// Vec<String>: (2017.4.33f1 - 2022.2.0b16)
    pub m_AtlasTags: Option<Vec<String>>,
    /// Vec<SpriteBone>: (2018.4.15f1 - 2022.2.0b16)
    pub m_Bones: Option<Vec<SpriteBone>>,
    /// Vec<Vec<Vector2f>>: (2017.4.33f1 - 2022.2.0b16)
    pub m_PhysicsShape: Option<Vec<Vec<Vector2f>>>,
    /// (GUID, i64): (2017.4.33f1 - 2022.2.0b16)
    pub m_RenderDataKey: Option<(GUID, i64)>,
    /// PPtr/*<SpriteAtlas>*/: (2017.4.33f1 - 2022.2.0b16)
    pub m_SpriteAtlas: Option<PPtr /*<SpriteAtlas>*/>,
}

/// SpriteAtlas is a  class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/U2D.SpriteAtlas.html):
/**
Sprite Atlas is an asset created within Unity. It is part of the built-in sprite packing solution.
A Sprite Atlas stores a list of packable assets. A packable asset is either a Sprite, Texture2D of TextureImporterType.Sprite or Folder. Before the packing process begins, these packable assets will be grouped and traversed to gather all the sprites from them. These will be used in the packing process. At runtime, these sprites can be enumerated via the Sprite Atlas (See Also: SpriteAtlas.GetSprites).It also provides dedicated texture settings in the inspector for the packed texture. The original texture settings of the sprite will have no effect on the packed texture.By default, Sprite Atlas will be referenced by the sprite and be available at runtime. This means that the sprite will be able to acquire the packed texture from the Sprite Atlas when loaded. A Sprite can be loaded without referencing any Sprite Atlas. A Sprite loaded this way will render as invisible since there is no texture. A reference to a Sprite Atlas can be added later. See Also: SpriteAtlasManager.Sprite Atlas variants can be created by assigning another Sprite Atlas object as the master. Variants will not repack a new texture from the packable list. Instead of this, variants will duplicate the master's packed texture and downscale it according to a user-defined ratio and save this scaled texture.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlas {
    /**Return true if this SpriteAtlas is a variant.*/
    pub m_IsVariant: bool,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_PackedSpriteNamesToIndex: Vec<String>,
    pub m_PackedSprites: Vec<PPtr /*<Sprite>*/>,
    pub m_RenderDataMap: Vec<((GUID, i64), SpriteAtlasData)>,
    /**Get the tag of this SpriteAtlas.*/
    pub m_Tag: String,
}

/// SpriteAtlasAsset is a  class of the Unity engine since version 2020.1.0a20.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/U2D.SpriteAtlasAsset.html):
/**
SpriteAtlasAsset stores inputs for generating SpriteAtlas and generates atlas textures on Import.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlasAsset {
    pub m_ImporterData: Enum_SpriteAtlasEditorData__SpriteAtlasAssetData,
    /**Checks whether the Sprite Atlas Importer set the Sprite Atlas as a Variant.*/
    pub m_IsVariant: bool,
    pub m_MasterAtlas: PPtr, /*<SpriteAtlas>*/
    /**The name of the object.*/
    pub m_Name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_SpriteAtlasEditorData__SpriteAtlasAssetData {
    SpriteAtlasEditorData(SpriteAtlasEditorData),
    SpriteAtlasAssetData(SpriteAtlasAssetData),
}

/// SpriteAtlasAssetData is a sub class of the Unity engine since version 2022.2.0b16.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlasAssetData {
    pub packables: Vec<PPtr /*<Object>*/>,
}

/// SpriteAtlasData is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlasData {
    pub alphaTexture: PPtr, /*<Texture2D>*/
    pub atlasRectOffset: Vector2f,
    pub downscaleMultiplier: f32,
    pub settingsRaw: u32,
    pub texture: PPtr, /*<Texture2D>*/
    pub textureRect: Rectf,
    pub textureRectOffset: Vector2f,
    pub uvTransform: Vector4f,
    /// Vec<SecondarySpriteTexture>: (2020.3.42f1 - 2022.2.0b16)
    pub secondaryTextures: Option<Vec<SecondarySpriteTexture>>,
}

/// SpriteAtlasDatabase is a  class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlasDatabase {}

/// SpriteAtlasEditorData is a sub class of the Unity engine since version 2020.1.0a20.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlasEditorData {
    pub bindAsDefault: bool,
    pub cachedData: PPtr, /*<CachedSpriteAtlasRuntimeData>*/
    pub isAtlasV2: bool,
    pub packables: Vec<PPtr /*<Object>*/>,
    pub packingSettings: PackingSettings,
    pub platformSettings: Vec<TextureImporterPlatformSettings>,
    pub textureSettings: TextureSettings,
    pub variantMultiplier: f32,
    /// Vec<(String, SecondaryTextureSettings)>: (2020.3.42f1 - 2021.2.16f1)
    pub secondaryTextureSettings: Option<Vec<(String, SecondaryTextureSettings)>>,
    /// u32: (2020.1.0a20 - 2020.1.0a20)
    pub totalSpriteSurfaceArea: Option<u32>,
}

/// SpriteAtlasImporter is a  class of the Unity engine since version 2020.1.0a20.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/U2D.SpriteAtlasImporter.html):
/**
SpriteAtlasImporter imports SpriteAtlasAsset and generates SpriteAtlas.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlasImporter {
    /**Get or set the AssetBundle name.*/
    pub m_AssetBundleName: String,
    /**Get or set the AssetBundle variant.*/
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_BindAsDefault: Option<bool>,
    /**SpriteAtlasPackingSettings to use when packing this SpriteAtlas.*/
    /// PackingSettings: (2022.2.0b16 - 2022.2.0b16)
    pub m_PackingSettings: Option<PackingSettings>,
    /// Vec<TextureImporterPlatformSettings>: (2022.2.0b16 - 2022.2.0b16)
    pub m_PlatformSettings: Option<Vec<TextureImporterPlatformSettings>>,
    /// Vec<(String, SecondaryTextureSettings)>: (2022.2.0b16 - 2022.2.0b16)
    pub m_SecondaryTextureSettings: Option<Vec<(String, SecondaryTextureSettings)>>,
    /**SpriteAtlasTextureSettings of the packed Texture generated by this SpriteAtlas.*/
    /// TextureSettings: (2022.2.0b16 - 2022.2.0b16)
    pub m_TextureSettings: Option<TextureSettings>,
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub m_VariantMultiplier: Option<f32>,
}

/// SpriteBone is a sub class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/U2D.SpriteBone.html):
/**
Stores a set of information that describes the bind pose of this Sprite.
This struct describes the hierarchy and other spatial relationships between the bones of this Sprite.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteBone {
    /**The length of the bone. This is important for the leaf bones to describe their length without needing another bone as the terminal bone.*/
    pub length: f32,
    /**The name of the bone. This is useful when recreating bone hierarchy at editor or runtime. You can also use this as a way of resolving the bone path when a Sprite is bound to a more complex or richer hierarchy.*/
    pub name: String,
    /**The ID of the parent of this bone.*/
    pub parentId: i32,
    /**The position in local space of this bone.*/
    pub position: Vector3f,
    /**The rotation of this bone in local space.*/
    pub rotation: Quaternionf,
    /**Shows the color set for the bone in the Editor.*/
    /// ColorRGBA: (2021.2.16f1 - 2022.2.0b16)
    pub color: Option<ColorRGBA>,
    /**The Unique GUID of this bone.*/
    /// String: (2021.2.16f1 - 2022.2.0b16)
    pub guid: Option<String>,
}

/// SpriteData is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteData {
    pub sprite: PPtr, /*<Object>*/
}

/// SpriteMask is a  class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SpriteMask.html):
/**
A component for masking Sprites and Particles.
By default it will mask all Sorting Layers. A custom range of Sorting Layers can be set. If a SortingGroup is present, it will act local to the SortingGroup.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteMask {
    pub m_BackSortingLayer: i16,
    /**Order within the back sorting layer defining the end of the custom range.*/
    pub m_BackSortingOrder: i16,
    pub m_CastShadows: u8,
    pub m_DynamicOccludee: u8,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    pub m_FrontSortingLayer: i16,
    /**Order within the front sorting layer defining the start of the custom range.*/
    pub m_FrontSortingOrder: i16,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Mask sprites from front to back sorting values only.*/
    pub m_IsCustomRangeActive: bool,
    /**The light probe interpolation type.*/
    pub m_LightProbeUsage: u8,
    pub m_LightProbeVolumeOverride: PPtr, /*<GameObject>*/
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    pub m_MaskAlphaCutoff: f32,
    /**Returns all the instantiated materials of this object.*/
    pub m_Materials: Vec<PPtr /*<Material>*/>,
    pub m_MotionVectors: u8,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    pub m_ProbeAnchor: PPtr, /*<Transform>*/
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: u8,
    /**Should reflection probes be used for this Renderer?*/
    pub m_ReflectionProbeUsage: u8,
    pub m_SortingLayer: i16,
    /**Unique ID of the Renderer's sorting layer.*/
    pub m_SortingLayerID: i32,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    /**The Sprite used to define the mask.*/
    pub m_Sprite: PPtr, /*<Sprite>*/
    pub m_StaticBatchInfo: StaticBatchInfo,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    /**Unique ID of the sorting layer defining the end of the custom range.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_BackSortingLayerID: Option<i32>,
    /**Unique ID of the sorting layer defining the start of the custom range.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_FrontSortingLayerID: Option<i32>,
    /// u8: (2020.1.0a20 - 2022.2.0b16)
    pub m_RayTraceProcedural: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0f4 - 2022.2.0b16)
    pub m_RayTracingMode: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on.*/
    /// u32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RenderingLayerMask: Option<u32>,
    /**Determines the position of the Sprite used for sorting the SpriteMask.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_SpriteSortPoint: Option<i32>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.2.16f1 - 2022.2.0b16)
    pub m_StaticShadowCaster: Option<u8>,
}

/// SpriteMetaData is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SpriteMetaData.html):
/**
Editor data used in producing a Sprite.
See Also: TextureImporter.spritesheet.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteMetaData {
    /**Edge-relative alignment of the sprite graphic.*/
    pub m_Alignment: i32,
    /**Edge border size for a sprite (in pixels).*/
    pub m_Border: Vector4f,
    /**Name of the Sprite.*/
    pub m_Name: String,
    pub m_Outline: Vec<Vec<Vector2f>>,
    /**The pivot point of the Sprite, relative to its bounding rectangle.*/
    pub m_Pivot: Vector2f,
    /**Bounding rectangle of the sprite's graphic within the atlas image.*/
    pub m_Rect: Rectf,
    pub m_TessellationDetail: f32,
    /// Vec<SpriteBone>: (2018.4.15f1 - 2022.2.0b16)
    pub m_Bones: Option<Vec<SpriteBone>>,
    /// Vec<int2_storage>: (2018.4.15f1 - 2022.2.0b16)
    pub m_Edges: Option<Vec<int2_storage>>,
    /// Vec<i32>: (2018.4.15f1 - 2022.2.0b16)
    pub m_Indices: Option<Vec<i32>>,
    /// i64: (2019.3.0f4 - 2022.2.0b16)
    pub m_InternalID: Option<i64>,
    /// Vec<Vec<Vector2f>>: (2017.4.33f1 - 2022.2.0b16)
    pub m_PhysicsShape: Option<Vec<Vec<Vector2f>>>,
    /// String: (2018.4.15f1 - 2022.2.0b16)
    pub m_SpriteID: Option<String>,
    /// Vec<Vector2f>: (2018.4.15f1 - 2022.2.0b16)
    pub m_Vertices: Option<Vec<Vector2f>>,
    /// Vec<BoneWeights4>: (2018.4.15f1 - 2022.2.0b16)
    pub m_Weights: Option<Vec<BoneWeights4>>,
}

/// SpriteRenderData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteRenderData {
    pub alphaTexture: PPtr, /*<Texture2D>*/
    pub m_IndexBuffer: Vec<u8>,
    pub m_SubMeshes: Vec<SubMesh>,
    pub m_VertexData: VertexData,
    pub settingsRaw: u32,
    pub texture: PPtr, /*<Texture2D>*/
    pub textureRect: Rectf,
    pub textureRectOffset: Vector2f,
    pub uvTransform: Vector4f,
    /// Vector2f: (2017.4.33f1 - 2022.2.0b16)
    pub atlasRectOffset: Option<Vector2f>,
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub downscaleMultiplier: Option<f32>,
    /// Vec<Matrix4x4f>: (2018.4.15f1 - 2022.2.0b16)
    pub m_Bindpose: Option<Vec<Matrix4x4f>>,
    /// Vec<SecondarySpriteTexture>: (2019.3.0f4 - 2022.2.0b16)
    pub secondaryTextures: Option<Vec<SecondarySpriteTexture>>,
}

/// SpriteRenderer is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SpriteRenderer.html):
/**
Renders a Sprite for 2D graphics.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteRenderer {
    pub m_CastShadows: u8,
    /**Rendering color for the Sprite graphic.*/
    pub m_Color: ColorRGBA,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**Flips the sprite on the X axis.*/
    pub m_FlipX: bool,
    /**Flips the sprite on the Y axis.*/
    pub m_FlipY: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The light probe interpolation type.*/
    pub m_LightProbeUsage: u8,
    pub m_LightProbeVolumeOverride: PPtr, /*<GameObject>*/
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    pub m_Materials: Vec<PPtr /*<Material>*/>,
    pub m_MotionVectors: u8,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    pub m_ProbeAnchor: PPtr, /*<Transform>*/
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: u8,
    /**Should reflection probes be used for this Renderer?*/
    pub m_ReflectionProbeUsage: u8,
    pub m_SortingLayer: i16,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    /**The Sprite to render.*/
    pub m_Sprite: PPtr, /*<Sprite>*/
    pub m_StaticBatchInfo: StaticBatchInfo,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    /**The current threshold for Sprite Renderer tiling.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_AdaptiveModeThreshold: Option<f32>,
    /**The current draw mode of the Sprite Renderer.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_DrawMode: Option<i32>,
    /// u8: (2017.4.33f1 - 2022.2.0b16)
    pub m_DynamicOccludee: Option<u8>,
    /**Specifies how the sprite interacts with the masks.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_MaskInteraction: Option<i32>,
    /// u8: (2020.1.0a20 - 2022.2.0b16)
    pub m_RayTraceProcedural: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0f4 - 2022.2.0b16)
    pub m_RayTracingMode: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on.*/
    /// u32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RenderingLayerMask: Option<u32>,
    /**Property to set or get the size to render when the SpriteRenderer.drawMode is set to SpriteDrawMode.Sliced or SpriteDrawMode.Tiled.*/
    /// Vector2f: (2017.4.33f1 - 2022.2.0b16)
    pub m_Size: Option<Vector2f>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_SortingLayerID: Option<i32>,
    /**Determines the position of the Sprite used for sorting the SpriteRenderer.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_SpriteSortPoint: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_SpriteTileMode: Option<i32>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.2.16f1 - 2022.2.0b16)
    pub m_StaticShadowCaster: Option<u8>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_WasSpriteAssigned: Option<bool>,
}

/// SpriteShapeRenderer is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/U2D.SpriteShapeRenderer.html):
/**
Renders SpriteShapes defined through the SpriteShapeUtility.GenerateSpriteShape API.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteShapeRenderer {
    pub m_CastShadows: u8,
    /**Rendering color for the SpriteShape.*/
    pub m_Color: ColorRGBA,
    pub m_DynamicOccludee: u8,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The light probe interpolation type.*/
    pub m_LightProbeUsage: u8,
    pub m_LightProbeVolumeOverride: PPtr, /*<GameObject>*/
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    pub m_LocalAABB: AABB,
    /**Specifies how the SpriteShape interacts with the masks.*/
    pub m_MaskInteraction: i32,
    /**Returns all the instantiated materials of this object.*/
    pub m_Materials: Vec<PPtr /*<Material>*/>,
    pub m_MotionVectors: u8,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    pub m_ProbeAnchor: PPtr, /*<Transform>*/
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: u8,
    /**Should reflection probes be used for this Renderer?*/
    pub m_ReflectionProbeUsage: u8,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    pub m_RendererPriority: i32,
    /**Determines which rendering layer this renderer lives on.*/
    pub m_RenderingLayerMask: u32,
    pub m_ShapeTexture: PPtr, /*<Texture2D>*/
    pub m_SortingLayer: i16,
    /**Unique ID of the Renderer's sorting layer.*/
    pub m_SortingLayerID: i32,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    pub m_Sprites: Vec<PPtr /*<Sprite>*/>,
    pub m_StaticBatchInfo: StaticBatchInfo,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    /// u8: (2020.1.0a20 - 2022.2.0b16)
    pub m_RayTraceProcedural: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0f4 - 2022.2.0b16)
    pub m_RayTracingMode: Option<u8>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_SpriteSortPoint: Option<i32>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.2.16f1 - 2022.2.0b16)
    pub m_StaticShadowCaster: Option<u8>,
}

/// SpriteSheetMetaData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteSheetMetaData {
    pub m_Outline: Vec<Vec<Vector2f>>,
    pub m_Sprites: Vec<SpriteMetaData>,
    /// Vec<SpriteBone>: (2018.4.15f1 - 2022.2.0b16)
    pub m_Bones: Option<Vec<SpriteBone>>,
    /// Vec<int2_storage>: (2018.4.15f1 - 2022.2.0b16)
    pub m_Edges: Option<Vec<int2_storage>>,
    /// Vec<i32>: (2018.4.15f1 - 2022.2.0b16)
    pub m_Indices: Option<Vec<i32>>,
    /// i64: (2019.3.0f4 - 2022.2.0b16)
    pub m_InternalID: Option<i64>,
    /// Vec<(String, i64)>: (2021.2.16f1 - 2022.2.0b16)
    pub m_NameFileIdTable: Option<Vec<(String, i64)>>,
    /// Vec<Vec<Vector2f>>: (2017.4.33f1 - 2022.2.0b16)
    pub m_PhysicsShape: Option<Vec<Vec<Vector2f>>>,
    /// Vec<SecondarySpriteTexture>: (2019.3.0f4 - 2022.2.0b16)
    pub m_SecondaryTextures: Option<Vec<SecondarySpriteTexture>>,
    /// String: (2018.4.15f1 - 2022.2.0b16)
    pub m_SpriteID: Option<String>,
    /// Vec<Vector2f>: (2018.4.15f1 - 2022.2.0b16)
    pub m_Vertices: Option<Vec<Vector2f>>,
    /// Vec<BoneWeights4>: (2018.4.15f1 - 2022.2.0b16)
    pub m_Weights: Option<Vec<BoneWeights4>>,
}

/// SpriteTilingProperty is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteTilingProperty {
    pub adaptiveTiling: bool,
    pub adaptiveTilingThreshold: f32,
    pub border: Vector4f,
    pub drawMode: i32,
    pub newSize: Vector2f,
    pub oldSize: Vector2f,
    pub pivot: Vector2f,
}

/// StateKey is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct StateKey {
    pub m_LayerIndex: i32,
    pub m_StateID: u32,
}

/// StateMachineBehaviourVectorDescription is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct StateMachineBehaviourVectorDescription {
    pub m_StateMachineBehaviourIndices: Vec<u32>,
    pub m_StateMachineBehaviourRanges: Vec<(StateKey, StateRange)>,
}

/// StateRange is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct StateRange {
    pub m_Count: u32,
    pub m_StartIndex: u32,
}

/// StaticBatchInfo is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct StaticBatchInfo {
    pub firstSubMesh: u16,
    pub subMeshCount: u16,
}

/// StreamedClip is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamedClip {
    pub curveCount: u32,
    pub data: Vec<u32>,
}

/// StreamedResource is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamedResource {
    pub m_Offset: i128,
    pub m_Size: u64,
    pub m_Source: String,
}

/// StreamingController is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/StreamingController.html):
/**
A StreamingController controls the streaming settings for an individual camera location.
The StreamingController component is used to control texture streaming settings for a camera location.

This component supports the preloading of textures in advance of a Camera becoming enabled. See SetPreloadingThe QualitySettings.streamingMipmapsFeature must be enabled and active for this feature to work.The Camera is not considered for texture streaming when this component is disabled.

When this component is enabled the Camera is considered for texture streaming if the Camera is enabled or the StreamingController is in the preloading state.A mipmap bias can be applied for texture streaming calculations. See streamingMipmapBias for details.See Also: camera component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamingController {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Offset applied to the mipmap level chosen by the texture streaming system for any textures visible from this camera. This Offset can take either a positive or negative value.*/
    pub m_StreamingMipmapBias: f32,
}

/// StreamingInfo is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamingInfo {
    pub offset: u64,
    pub path: String,
    pub size: u32,
}

/// StreamingManager is a  class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamingManager {}

/// StructParameter is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct StructParameter {
    pub m_ArraySize: i32,
    pub m_Index: i32,
    pub m_MatrixMembers: Vec<MatrixParameter>,
    pub m_NameIndex: i32,
    pub m_StructSize: i32,
    pub m_VectorMembers: Vec<VectorParameter>,
}

/// SubCollider is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubCollider {
    pub m_Collider: PPtr, /*<Collider2D>*/
    pub m_ColliderPaths: Vec<Vec<IntPoint>>,
}

/// SubDerived is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubDerived {}

/// SubEmitterData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubEmitterData {
    pub emitter: PPtr, /*<ParticleSystem>*/
    pub properties: i32,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub emitProbability: Option<f32>,
}

/// SubMesh is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubMesh {
    pub firstByte: u32,
    pub firstVertex: u32,
    pub indexCount: u32,
    pub localAABB: AABB,
    pub vertexCount: u32,
    /// u32: (2017.4.33f1 - 2022.2.0b16)
    pub baseVertex: Option<u32>,
    /// u32: (3.4.0 - 3.4.0)
    pub isTriStrip: Option<u32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub topology: Option<i32>,
    /// u32: (3.4.0 - 3.4.0)
    pub triangleCount: Option<u32>,
}

/// SubModule is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubModule {
    pub enabled: bool,
    pub subEmitters: Vec<SubEmitterData>,
}

/// SubstanceArchive is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubstanceArchive {
    pub m_Name: String,
    /// Vec<u8>: (3.4.0 - 2017.4.33f1)
    pub m_PackageData: Option<Vec<u8>>,
}

/// SubstanceEnumItem is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubstanceEnumItem {
    pub text: String,
    pub value: i32,
}

/// SubstanceImporter is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubstanceImporter {
    pub m_Name: String,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AssetBundleName: Option<String>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AssetBundleVariant: Option<String>,
    /// Vec<String>: (5.6.0b2 - 2017.4.33f1)
    pub m_DeletedPrototypes: Option<Vec<String>>,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.0)
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_IsFirstImport: Option<i32>,
    /// Vec<MaterialImportOutput>: (5.6.0b2 - 2017.4.33f1)
    pub m_MaterialImportOutputs: Option<Vec<MaterialImportOutput>>,
    /// Vec<MaterialInstanceSettings>: (3.4.0 - 2017.4.33f1)
    pub m_MaterialInstances: Option<Vec<MaterialInstanceSettings>>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_NewHashIdentity: Option<MdFour>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_OldHashIdentity: Option<MdFour>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_UserData: Option<String>,
}

/// SubstanceInput is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubstanceInput {
    pub alteredTexturesUID: Vec<u32>,
    pub enumValues: Vec<SubstanceEnumItem>,
    pub flags: u32,
    pub internalIndex: u32,
    pub internalType: i32,
    pub maximum: f32,
    pub minimum: f32,
    pub name: String,
    pub step: f32,
    pub value: SubstanceValue,
    /// i32: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// Vec<String>: (5.6.0b2 - 2017.4.33f1)
    pub componentLabels: Option<Vec<String>>,
    /// String: (5.6.0b2 - 2017.4.33f1)
    pub group: Option<String>,
    /// u32: (5.6.0b2 - 2017.4.33f1)
    pub internalIdentifier: Option<u32>,
    /// String: (5.6.0b2 - 2017.4.33f1)
    pub label: Option<String>,
    /// String: (5.6.0b2 - 2017.4.33f1)
    pub visibleIf: Option<String>,
}

/// SubstanceValue is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubstanceValue {
    pub texture: PPtr, /*<Texture2D>*/
    /// f32: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "scalar[0]")]
    pub scalar_0_: Option<f32>,
    /// f32: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "scalar[1]")]
    pub scalar_1_: Option<f32>,
    /// f32: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "scalar[2]")]
    pub scalar_2_: Option<f32>,
    /// f32: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "scalar[3]")]
    pub scalar_3_: Option<f32>,
    /// String: (2017.4.33f1 - 2017.4.33f1)
    pub stringvalue: Option<String>,
}

/// SurfaceEffector2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SurfaceEffector2D.html):
/**
Applies tangent forces along the surfaces of colliders.
When the source Collider2D is a trigger, the effector will apply forces whenever the target Collider2D overlaps the source.  When the source Collider isn't a trigger, the effector will apply forces whenever the target Collider2D is in contact with the source only.This effector can be used to create constant speed elevators and moving surfaces.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceEffector2D {
    /**The mask used to select specific layers allowed to interact with the effector.*/
    pub m_ColliderMask: BitField,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The scale of the impulse force applied while attempting to reach the surface speed.*/
    pub m_ForceScale: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The speed to be maintained along the surface.*/
    pub m_Speed: f32,
    /**The speed variation (from zero to the variation) added to base speed to be applied.*/
    pub m_SpeedVariation: f32,
    /**Should bounce be used for any contact with the surface?*/
    pub m_UseBounce: bool,
    /**Should the collider-mask be used or the global collision matrix?*/
    pub m_UseColliderMask: bool,
    /**Should the impulse force but applied to the contact point?*/
    pub m_UseContactForce: bool,
    /**Should friction be used for any contact with the surface?*/
    pub m_UseFriction: bool,
}

/// TagManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct TagManager {
    pub tags: Vec<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "Builtin Layer 0")]
    pub Builtin_Layer_0: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "Builtin Layer 1")]
    pub Builtin_Layer_1: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "Builtin Layer 2")]
    pub Builtin_Layer_2: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "Builtin Layer 3")]
    pub Builtin_Layer_3: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "Builtin Layer 4")]
    pub Builtin_Layer_4: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "Builtin Layer 5")]
    pub Builtin_Layer_5: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "Builtin Layer 6")]
    pub Builtin_Layer_6: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "Builtin Layer 7")]
    pub Builtin_Layer_7: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 10")]
    pub User_Layer_10: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 11")]
    pub User_Layer_11: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 12")]
    pub User_Layer_12: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 13")]
    pub User_Layer_13: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 14")]
    pub User_Layer_14: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 15")]
    pub User_Layer_15: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 16")]
    pub User_Layer_16: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 17")]
    pub User_Layer_17: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 18")]
    pub User_Layer_18: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 19")]
    pub User_Layer_19: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 20")]
    pub User_Layer_20: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 21")]
    pub User_Layer_21: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 22")]
    pub User_Layer_22: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 23")]
    pub User_Layer_23: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 24")]
    pub User_Layer_24: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 25")]
    pub User_Layer_25: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 26")]
    pub User_Layer_26: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 27")]
    pub User_Layer_27: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 28")]
    pub User_Layer_28: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 29")]
    pub User_Layer_29: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 30")]
    pub User_Layer_30: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 31")]
    pub User_Layer_31: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 8")]
    pub User_Layer_8: Option<String>,
    /// String: (3.4.0 - 3.4.0)
    #[serde(alias = "User Layer 9")]
    pub User_Layer_9: Option<String>,
    /// Vec<String>: (5.6.0b2 - 2022.2.0b16)
    pub layers: Option<Vec<String>>,
    /// Vec<SortingLayerEntry>: (5.6.0b2 - 2022.2.0b16)
    pub m_SortingLayers: Option<Vec<SortingLayerEntry>>,
}

/// TakeInfo is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TakeInfo.html):
/**
A Takeinfo object contains all the information needed to describe a take.
See Also: ModelImporter.importedTakeInfos.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TakeInfo {
    /**Start time in second.*/
    pub bakeStartTime: f32,
    /**Stop time in second.*/
    pub bakeStopTime: f32,
    pub clip: PPtr, /*<AnimationClip>*/
    /**This is the default clip name for the clip generated for this take.*/
    pub defaultClipName: String,
    /**Take name as define from imported file.*/
    pub name: String,
    /**Sample rate of the take.*/
    pub sampleRate: f32,
    /**Start time in second.*/
    pub startTime: f32,
    /**Stop time in second.*/
    pub stopTime: f32,
    /// i64: (2019.3.0f4 - 2022.2.0b16)
    pub internalID: Option<i64>,
}

/// TargetJoint2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TargetJoint2D.html):
/**
The joint attempts to move a Rigidbody2D to a specific target position.
This joint is the only joint that doesn't connect two Rigidbody2D together.  Instead, it only operates on the single body it is connected to.When connected, it will attempt to move the body to a specified target position.  When setting a target you can also set the anchor position which is a point relative to the Rigidbody2D where forces will be applied.The joint moves the body using a configurable spring that has a force limit.An example usage for this joint might be to enable Collider2D to be dragged, selecting an anchor point and moving the body to the position under the mouse.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TargetJoint2D {
    /**The local-space anchor on the rigid-body the joint is attached to.*/
    pub m_Anchor: Vector2f,
    /**Should the target be calculated automatically?*/
    pub m_AutoConfigureTarget: bool,
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break.*/
    pub m_BreakTorque: f32,
    pub m_ConnectedRigidBody: PPtr, /*<Rigidbody2D>*/
    /**The amount by which the target spring force is reduced in proportion to the movement speed.*/
    pub m_DampingRatio: f32,
    /**Should the two rigid bodies connected with this joint collide with each other?*/
    pub m_EnableCollision: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The frequency at which the target spring oscillates around the target position.*/
    pub m_Frequency: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The maximum force that can be generated when trying to maintain the target joint constraint.*/
    pub m_MaxForce: f32,
    /**The world-space position that the joint will attempt to move the body to.*/
    pub m_Target: Vector2f,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_BreakAction: Option<i32>,
}

/// Terrain is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Terrain.html):
/**
The Terrain component renders the terrain.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Terrain {
    /**Specifies if an array of internal light probes should be baked for terrain trees. Available only in editor.*/
    pub m_BakeLightProbesForTrees: bool,
    pub m_ChunkDynamicUVST: Vector4f,
    /**Density of detail objects.*/
    pub m_DetailObjectDensity: f32,
    /**Detail objects will be displayed up to this distance.*/
    pub m_DetailObjectDistance: f32,
    /**Indicates whether Unity draws the Terrain geometry itself.*/
    pub m_DrawHeightmap: bool,
    /**Specify if terrain trees and details should be drawn.*/
    pub m_DrawTreesAndFoliage: bool,
    pub m_DynamicUVST: Vector4f,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Lets you essentially lower the heightmap resolution used for rendering.*/
    pub m_HeightmapMaximumLOD: i32,
    /**An approximation of how many pixels the terrain will pop in the worst case when switching lod.*/
    pub m_HeightmapPixelError: f32,
    /**The index of the baked lightmap applied to this terrain.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    /**The custom material Unity uses to render the Terrain.*/
    pub m_MaterialTemplate: PPtr, /*<Material>*/
    /**How reflection probes are used for terrain. See ReflectionProbeUsage.*/
    pub m_ReflectionProbeUsage: i32,
    pub m_SplatMapDistance: f32,
    /**The Terrain Data that stores heightmaps, terrain textures, detail meshes and trees.*/
    pub m_TerrainData: PPtr, /*<TerrainData>*/
    /**Distance from the camera where trees will be rendered as billboards only.*/
    pub m_TreeBillboardDistance: f32,
    /**Total distance delta that trees will use to transition from billboard orientation to mesh orientation.*/
    pub m_TreeCrossFadeLength: f32,
    /**The maximum distance at which trees are rendered.*/
    pub m_TreeDistance: f32,
    /**Maximum number of trees rendered at full LOD.*/
    pub m_TreeMaximumFullLODCount: i32,
    /**Specifies if the terrain tile will be automatically connected to adjacent tiles.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_AllowAutoConnect: Option<bool>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub m_CastShadows: Option<bool>,
    /**Set to true to enable the terrain instance renderer. The default value is false.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_DrawInstanced: Option<bool>,
    /**When this options is enabled, Terrain heightmap geometries will be added in acceleration structures used for Ray Tracing.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_EnableHeightmapRayTracing: Option<bool>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_EnableTreesAndDetailsRayTracing: Option<bool>,
    /// Hash128: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExplicitProbeSetHash: Option<Hash128>,
    /**Grouping ID for auto connect.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_GroupingID: Option<i32>,
    /**When enabled, the terrain ignores the terrain overrides set in the QualitySettings.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_IgnoreQualitySettings: Option<bool>,
    /// f32: (5.6.0b2 - 2018.4.15f1)
    pub m_LegacyShininess: Option<f32>,
    /// ColorRGBA: (5.6.0b2 - 2018.4.15f1)
    pub m_LegacySpecular: Option<ColorRGBA>,
    /// i32: (5.6.0b2 - 2018.4.15f1)
    pub m_MaterialType: Option<i32>,
    /**Allows you to specify how Unity chooses the layer for tree instances.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_PreserveTreePrototypeLayers: Option<bool>,
    /**Determines which rendering layers the Terrain renderer lives on.*/
    /// u32: (2019.3.0f4 - 2022.2.0b16)
    pub m_RenderingLayerMask: Option<u32>,
    /**Allows you to set the shadow casting mode for the terrain.*/
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_ShadowCastingMode: Option<i32>,
    /// bool: (2021.2.16f1 - 2022.2.0b16)
    pub m_StaticShadowCaster: Option<bool>,
}

/// TerrainCollider is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TerrainCollider.html):
/**
A heightmap based collider.
See Also: SphereCollider, CapsuleCollider, PhysicMaterial, Rigidbody.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainCollider {
    /**Enabled Colliders will collide with other Colliders, disabled Colliders won't.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The material used by the collider.*/
    pub m_Material: PPtr, /*<PhysicMaterial>*/
    /**The terrain that stores the heightmap.*/
    pub m_TerrainData: PPtr, /*<TerrainData>*/
    /// bool: (3.4.0 - 3.4.0)
    pub m_CreateTreeColliders: Option<bool>,
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_EnableTreeColliders: Option<bool>,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**Specify if this collider is configured as a trigger.*/
    /// bool: (3.4.0 - 3.4.0)
    pub m_IsTrigger: Option<bool>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ProvidesContacts: Option<bool>,
}

/// TerrainData is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TerrainData.html):
/**
The TerrainData class stores heightmaps, detail mesh positions, tree instances, and terrain texture alpha maps.
The Terrain component links to the terrain data and renders it.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainData {
    pub m_DetailDatabase: DetailDatabase,
    pub m_Heightmap: Heightmap,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_SplatDatabase: SplatDatabase,
    /// Vec<PPtr/*<Shader>*/>: (2018.4.15f1 - 2022.2.0b16)
    pub m_PreloadShaders: Option<Vec<PPtr /*<Shader>*/>>,
}

/// TerrainLayer is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TerrainLayer.html):
/**
Description of a terrain layer.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainLayer {
    /**A Vector4 value specifying the maximum RGBA value that the diffuse texture maps to when the value of the channel is 1.*/
    pub m_DiffuseRemapMax: Vector4f,
    /**A Vector4 value specifying the minimum RGBA value that the diffuse texture maps to when the value of the channel is 0.*/
    pub m_DiffuseRemapMin: Vector4f,
    /**The diffuse texture used by the terrain layer.*/
    pub m_DiffuseTexture: PPtr, /*<Texture2D>*/
    /**A Vector4 value specifying the maximum RGBA value that the mask map texture maps to when the value of the channel is 1.*/
    pub m_MaskMapRemapMax: Vector4f,
    /**A Vector4 value specifying the minimum RGBA value that the mask map texture maps to when the value of the channel is 0.*/
    pub m_MaskMapRemapMin: Vector4f,
    /**The mask map texture used by the terrain layer.*/
    pub m_MaskMapTexture: PPtr, /*<Texture2D>*/
    /**Metallic factor used by the terrain layer.*/
    pub m_Metallic: f32,
    /**The name of the object.*/
    pub m_Name: String,
    /**Normal map texture used by the terrain layer.*/
    pub m_NormalMapTexture: PPtr, /*<Texture2D>*/
    /**A float value that scales the normal vector. The minimum value is 0, the maximum value is 1.*/
    pub m_NormalScale: f32,
    /**Smoothness of the specular reflection.*/
    pub m_Smoothness: f32,
    /**Specular color.*/
    pub m_Specular: ColorRGBA,
    /**UV tiling offset.*/
    pub m_TileOffset: Vector2f,
    /**UV Tiling size.*/
    pub m_TileSize: Vector2f,
}

/// TestObjectVectorPairStringBool is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectVectorPairStringBool {
    pub m_Map: Vec<(String, bool)>,
    pub m_String: String,
}

/// TestObjectWithSerializedAnimationCurve is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectWithSerializedAnimationCurve {
    pub m_Curve: AnimationCurve,
}

/// TestObjectWithSerializedArray is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectWithSerializedArray {
    pub m_ClampTestValue: f32,
    pub m_IntegerArray: Vec<i32>,
}

/// TestObjectWithSerializedMapStringBool is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectWithSerializedMapStringBool {
    pub m_Map: Vec<(String, bool)>,
    pub m_String: String,
}

/// TestObjectWithSerializedMapStringNonAlignedStruct is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectWithSerializedMapStringNonAlignedStruct {
    pub m_Map: Vec<(String, NonAlignedStruct)>,
    pub m_String: String,
}

/// TestObjectWithSpecialLayoutOne is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectWithSpecialLayoutOne {
    pub differentLayout: LayoutDataOne,
    pub sameLayout: LayoutDataOne,
}

/// TestObjectWithSpecialLayoutTwo is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectWithSpecialLayoutTwo {
    pub differentLayout: LayoutDataTwo,
    pub sameLayout: LayoutDataThree,
}

/// Tetrahedron is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tetrahedron {
    pub matrix: Matrix3x4f,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "indices[0]")]
    pub indices_0_: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "indices[1]")]
    pub indices_1_: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "indices[2]")]
    pub indices_2_: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "indices[3]")]
    pub indices_3_: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "neighbors[0]")]
    pub neighbors_0_: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "neighbors[1]")]
    pub neighbors_1_: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "neighbors[2]")]
    pub neighbors_2_: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "neighbors[3]")]
    pub neighbors_3_: Option<i32>,
}

/// TextAsset is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TextAsset.html):
/**
Represents a raw text or binary file asset.
You can use raw text files in your project as assets and get their contents through

this class. For more information, see text.You can access the file as a raw byte array to access data from binary files. For more

information on how to access data from binary files, see bytes and GetData.For more information about importing text or binary files into your project as Text Assets,

see Text Asset.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TextAsset {
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Script: String,
    /// String: (3.4.0 - 5.6.0b2)
    pub m_PathName: Option<String>,
}

/// TextMesh is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TextMesh.html):
/**
A script interface for the text mesh component.
See Also: text mesh component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TextMesh {
    /**How lines of text are aligned (Left, Right, Center).*/
    pub m_Alignment: i16,
    /**Which point of the text shares the position of the Transform.*/
    pub m_Anchor: i16,
    /**The size of each character (This scales the whole text).*/
    pub m_CharacterSize: f32,
    /**The Font used.*/
    pub m_Font: PPtr, /*<Font>*/
    /**The font size to use (for dynamic fonts).*/
    pub m_FontSize: i32,
    /**The font style to use (for dynamic fonts).*/
    pub m_FontStyle: i32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**How much space will be in-between lines of text.*/
    pub m_LineSpacing: f32,
    /**How far should the text be offset from the transform.position.z when drawing.*/
    pub m_OffsetZ: f32,
    /**How much space will be inserted for a tab '\t' character. This is a multiplum of the 'spacebar' character offset.*/
    pub m_TabSize: f32,
    /**The text that is displayed.*/
    pub m_Text: String,
    /**The color used to render the text.*/
    /// ColorRGBA: (5.6.0b2 - 2022.2.0b16)
    pub m_Color: Option<ColorRGBA>,
    /**Enable HTML-style tags for Text Formatting Markup.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_RichText: Option<bool>,
}

/// TextScriptImporter is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextScriptImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_Name: String,
    pub m_UserData: String,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// Texture is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Texture.html):
/**
Base class for Texture handling.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Texture {
    /**The name of the object.*/
    pub m_Name: String,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_IsAlphaChannelOptional: Option<bool>,
}

/// Texture2D is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Texture2D.html):
/**
Class that represents textures in C# code.
Use this class to create textures, or to modify existing texture assets.The ImageConversion class provides extension methods to this class that handle image encoding functionality. For details on those methods, see the ImageConversion documentation.Do not assume that the texture will be created and available in Awake. All texture uploads are synchronized on the Main thread at Start. Perform texture operations in Start.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Texture2D {
    pub m_CompleteImageSize: i64,
    /**Height of the Texture in pixels (Read Only).*/
    pub m_Height: i32,
    pub m_ImageCount: i32,
    /**Whether Unity stores an additional copy of this texture's pixel data in CPU-addressable memory.*/
    pub m_IsReadable: bool,
    pub m_LightmapFormat: i32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_TextureDimension: i32,
    pub m_TextureFormat: i32,
    pub m_TextureSettings: GLTextureSettings,
    /**Width of the Texture in pixels (Read Only).*/
    pub m_Width: i32,
    /// Vec<u8>: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "image data")]
    pub image_data: Option<Vec<u8>>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_ColorSpace: Option<i32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.1.0a20 - 2021.2.16f1)
    pub m_IgnoreMasterTextureLimit: Option<bool>,
    /**This property causes a texture to ignore all texture mipmap limit settings.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_IgnoreMipmapLimit: Option<bool>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// bool: (2020.1.0a20 - 2022.2.0b16)
    pub m_IsPreProcessed: Option<bool>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_MipCount: Option<i32>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_MipMap: Option<bool>,
    /// String: (2022.2.0b16 - 2022.2.0b16)
    pub m_MipmapLimitGroupName: Option<String>,
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub m_MipsStripped: Option<i32>,
    /// Vec<u8>: (2020.3.42f1 - 2022.2.0b16)
    pub m_PlatformBlob: Option<Vec<u8>>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_ReadAllowed: Option<bool>,
    /// StreamingInfo: (5.6.0b2 - 2022.2.0b16)
    pub m_StreamData: Option<StreamingInfo>,
    /**Determines whether mipmap streaming is enabled for this Texture.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_StreamingMipmaps: Option<bool>,
    /**Sets the relative priority for this Texture when reducing memory size to fit within the memory budget.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_StreamingMipmapsPriority: Option<i32>,
}

/// Texture2DArray is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Texture2DArray.html):
/**
Class for handling 2D texture arrays.
Modern graphics APIs (e.g. D3D10 and later, OpenGL ES 3.0 and later, Metal etc.) support "texture arrays", which is an array of same size & format textures.

From the shader side, they are treated as a single resource, and sampling them needs an extra coordinate that indicates which array element to sample from.Typically texture arrays are useful as an alternative for texture atlases, or in other cases where objects use a set of same-sized textures (e.g. terrains).Currently in Unity texture arrays do not have an import pipeline for them, and must be created from code, either at runtime or in editor scripts.

Using Graphics.CopyTexture is useful for fast copying of pixel data from regular 2D textures into elements of a texture array. From editor scripts,

a common way of creating serialized texture array is to create it, fill with data (either via Graphics.CopyTexture from regular 2D textures, or via SetPixels or

SetPixels32) and save it as an asset via AssetDatabase.CreateAsset.Note that not all platforms and GPUs support texture arrays; for example Direct3D9 and OpenGL ES 2.0 do not. Use SystemInfo.supports2DArrayTextures to check. Also, this class does not support Texture2DArray creation with a Crunch compression TextureFormat.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Texture2DArray {
    pub m_ColorSpace: i32,
    pub m_DataSize: u32,
    /**Number of elements in a texture array (Read Only).*/
    pub m_Depth: i32,
    /**Texture format (Read Only).*/
    pub m_Format: i32,
    /**Height of the Texture in pixels (Read Only).*/
    pub m_Height: i32,
    /**Whether Unity stores an additional copy of this texture's pixel data in CPU-addressable memory.*/
    pub m_IsReadable: bool,
    pub m_MipCount: i32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_StreamData: StreamingInfo,
    pub m_TextureSettings: GLTextureSettings,
    /**Width of the Texture in pixels (Read Only).*/
    pub m_Width: i32,
    /// Vec<u8>: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "image data")]
    pub image_data: Option<Vec<u8>>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_UsageMode: Option<i32>,
}

/// Texture3D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Texture3D.html):
/**
Class for handling 3D Textures, Use this to create 3D texture assets.
3D textures are commonly used as lookup tables by shaders, or to represent volumetric data.Typically you'd create a 3D texture, fill it up with data (SetPixels or SetPixels32) and call

Apply to upload the data to the GPU.Note that this class does not support Texture3D creation with a Crunch compression TextureFormat.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Texture3D {
    pub m_DataSize: u32,
    /**The depth of the texture (Read Only).*/
    pub m_Depth: i32,
    /**The format of the pixel data in the texture (Read Only).*/
    pub m_Format: i32,
    /**Height of the Texture in pixels (Read Only).*/
    pub m_Height: i32,
    /**Whether Unity stores an additional copy of this texture's pixel data in CPU-addressable memory.*/
    pub m_IsReadable: bool,
    pub m_MipCount: i32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_StreamData: StreamingInfo,
    pub m_TextureSettings: GLTextureSettings,
    /**Width of the Texture in pixels (Read Only).*/
    pub m_Width: i32,
    /// Vec<u8>: (5.6.0b2 - 2022.2.0b16)
    #[serde(alias = "image data")]
    pub image_data: Option<Vec<u8>>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_ColorSpace: Option<i32>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_UsageMode: Option<i32>,
}

/// TextureImportInstructions is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextureImportInstructions {
    pub colorSpace: i32,
    pub compressedFormat: i32,
    pub compressionQuality: i32,
    pub desiredFormat: i32,
    pub height: i32,
    pub uncompressedFormat: i32,
    pub usageMode: i32,
    pub width: i32,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub androidETC2FallbackDownscale: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub androidETC2FallbackFormat: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub cubeIntermediateSize: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub cubeLayout: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub cubeMode: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub depth: Option<i32>,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub vtOnly: Option<bool>,
}

/// TextureImportOutput is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextureImportOutput {
    pub importInspectorWarnings: String,
    pub sourceTextureInformation: SourceTextureInformation,
    pub textureImportInstructions: TextureImportInstructions,
}

/// TextureImporter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TextureImporter.html):
/**
Texture importer lets you modify Texture2D import settings from editor scripts.
Settings of this class cover most of the settings exposed in Texture Import Settings. Some settings require you to use TextureImporterSettings. Refer to TextureImporter.SetTextureSettings).
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TextureImporter {
    /**Keeps texture borders the same when generating mipmaps.*/
    pub m_BorderMipMap: i32,
    /**Convert heightmap to normal map*/
    pub m_ConvertToNormalMap: i32,
    pub m_EnableMipMap: i32,
    pub m_ExternalNormalMap: i32,
    /**Fade out mip levels to gray color.*/
    pub m_FadeOut: i32,
    /**Cubemap generation mode.*/
    pub m_GenerateCubemap: i32,
    pub m_GrayScaleToAlpha: i32,
    pub m_HeightScale: f32,
    /**Whether Unity stores an additional copy of the imported texture's pixel data in CPU-addressable memory.*/
    pub m_IsReadable: i32,
    pub m_Lightmap: i32,
    /**Maximum texture size.*/
    pub m_MaxTextureSize: i32,
    /**Mip level where texture is faded out completely.*/
    pub m_MipMapFadeDistanceEnd: i32,
    /**Mip level where texture begins to fade out.*/
    pub m_MipMapFadeDistanceStart: i32,
    pub m_MipMapMode: i32,
    /**Scaling mode for non power of two textures.*/
    pub m_NPOTScale: i32,
    /**The name of the object.*/
    pub m_Name: String,
    /**Normal map filtering mode.*/
    pub m_NormalMapFilter: i32,
    pub m_TextureFormat: i32,
    pub m_TextureSettings: GLTextureSettings,
    /**Which type of texture are we dealing with here.*/
    pub m_TextureType: i32,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_Alignment: Option<i32>,
    /**If the alpha channel of your texture represents transparency, enable this property to dilate the color channels of visible texels into fully transparent areas. This effectively adds padding around transparent areas that prevents filtering artifacts from forming on their edges. Unity does not support this property for HDR textures.This property makes the color data of invisible texels undefined. Disable this property to preserve invisible texels' original color data.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_AlphaIsTransparency: Option<i32>,
    /**Returns or assigns the alpha test reference value.*/
    /// f32: (2017.4.33f1 - 2022.2.0b16)
    pub m_AlphaTestReferenceValue: Option<f32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_AlphaUsage: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_ApplyGammaDecoding: Option<i32>,
    /**Get or set the AssetBundle name.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AssetBundleName: Option<String>,
    /**Get or set the AssetBundle variant.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AssetBundleVariant: Option<String>,
    /// Vec<BuildTargetSettings>: (3.4.0 - 3.4.0)
    pub m_BuildTargetSettings: Option<Vec<BuildTargetSettings>>,
    /**The quality of Crunch texture compression. The range is 0 through 100. A higher quality means larger textures and longer compression times.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_CompressionQuality: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_CompressionQualitySet: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_CookieLightType: Option<i32>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_CorrectGamma: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_CubemapConvolution: Option<i32>,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.0); Vec<(i64, String)>: (5.6.0b2 - 2018.4.15f1)
    pub m_FileIDToRecycleName: Option<Vec<(i64, String)>>,
    /**Indicates whether to invert the green channel values of a normal map.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_FlipGreenChannel: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_FlipbookColumns: Option<i32>,
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub m_FlipbookRows: Option<i32>,
    /// i32: (2021.2.16f1 - 2021.2.16f1)
    pub m_IgnoreMasterTextureLimit: Option<i32>,
    /**Enable this flag for textures that should ignore mipmap limit settings.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_IgnoreMipmapLimit: Option<i32>,
    /**Ignore the Gamma attribute in PNG files. This property does not effect other file formats.*/
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub m_IgnorePngGamma: Option<i32>,
    /// Vec<((i32, i64), String)>: (2019.3.0f4 - 2022.2.0b16)
    pub m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_LinearTexture: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_MaxTextureSizeSet: Option<i32>,
    /**Enables or disables coverage-preserving alpha mipmapping.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_MipMapsPreserveCoverage: Option<i32>,
    /**Name of the texture mipmap limit group to which this texture belongs.*/
    /// String: (2022.2.0b16 - 2022.2.0b16)
    pub m_MipmapLimitGroupName: Option<String>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_NewHashIdentity: Option<MdFour>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_OldHashIdentity: Option<MdFour>,
    /// TextureImportOutput: (5.6.0b2 - 2022.2.0b16)
    pub m_Output: Option<TextureImportOutput>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_PSDRemoveMatte: Option<bool>,
    /// bool: (2018.4.15f1 - 2021.2.16f1)
    pub m_PSDShowRemoveMatteOption: Option<bool>,
    /// Vec<PlatformSettings>: (5.6.0b2 - 5.6.0b2); Vec<TextureImporterPlatformSettings>: (2017.4.33f1 - 2022.2.0b16)
    pub m_PlatformSettings: Option<Vec<Enum_TextureImporterPlatformSettings__PlatformSettings>>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_RecommendedTextureFormat: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SeamlessCubemap: Option<i32>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_SingleChannelComponent: Option<i32>,
    /// SourceTextureInformation: (3.4.0 - 3.4.0)
    pub m_SourceTextureInformation: Option<SourceTextureInformation>,
    /**Border sizes of the generated sprites.*/
    /// Vector4f: (5.6.0b2 - 2022.2.0b16)
    pub m_SpriteBorder: Option<Vector4f>,
    /// u32: (5.6.0b2 - 2022.2.0b16)
    pub m_SpriteExtrude: Option<u32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_SpriteGenerateFallbackPhysicsShape: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SpriteMeshType: Option<i32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_SpriteMode: Option<i32>,
    /// String: (5.6.0b2 - 2021.2.16f1)
    pub m_SpritePackingTag: Option<String>,
    /**The point in the Sprite object's coordinate space where the graphic is located.*/
    /// Vector2f: (5.6.0b2 - 2022.2.0b16)
    pub m_SpritePivot: Option<Vector2f>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_SpritePixelsToUnits: Option<f32>,
    /// SpriteSheetMetaData: (5.6.0b2 - 2022.2.0b16)
    pub m_SpriteSheet: Option<SpriteSheetMetaData>,
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_SpriteTessellationDetail: Option<f32>,
    /**Enable mipmap streaming for this texture.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_StreamingMipmaps: Option<i32>,
    /**Relative priority for this texture when reducing memory size in order to hit the memory budget.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_StreamingMipmapsPriority: Option<i32>,
    /// u32: (2022.2.0b16 - 2022.2.0b16)
    pub m_Swizzle: Option<u32>,
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_TextureFormatSet: Option<i32>,
    /**The shape of the imported texture.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_TextureShape: Option<i32>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /**Get or set any user data.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_UserData: Option<String>,
    /**When enabled, this texture can solely be used in combination with a Texture Stack for Virtual Texturing. When enabled the texture is not guaranteed to be available as a Texture2D in the Player (e.g., not accessible from a script). When disabled, the Player includes the texture both as a Texture2D (e.g., accessible from script) and as a streamable texture in a Texture Stack.*/
    /// i32: (2020.1.0a20 - 2022.2.0b16)
    pub m_VTOnly: Option<i32>,
    /**Whether this texture stores data in sRGB (also called gamma) color space.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_sRGBTexture: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_TextureImporterPlatformSettings__PlatformSettings {
    TextureImporterPlatformSettings(TextureImporterPlatformSettings),
    PlatformSettings(PlatformSettings),
}

/// TextureImporterPlatformSettings is a sub class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TextureImporterPlatformSettings.html):
/**
Stores platform specifics settings of a TextureImporter.
See Also: TextureImporter.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TextureImporterPlatformSettings {
    /**Allows Alpha splitting on the imported texture when needed (for example ETC1 compression for textures with transparency).*/
    pub m_AllowsAlphaSplitting: bool,
    /**Override for ETC2 decompression fallback on Android devices that don't support ETC2.*/
    pub m_AndroidETC2FallbackOverride: i32,
    pub m_BuildTarget: String,
    /**The quality of Crunch texture compression. The range is 0 through 100. A higher quality means larger textures and longer compression times.*/
    pub m_CompressionQuality: i32,
    /**Use crunch compression when available.*/
    pub m_CrunchedCompression: bool,
    /**Maximum texture size.*/
    pub m_MaxTextureSize: i32,
    /**Set to true in order to override the Default platform parameters by those provided in the TextureImporterPlatformSettings structure.*/
    pub m_Overridden: bool,
    /**For Texture to be scaled down choose resize algorithm. ( Applyed only when Texture dimension is bigger than Max Size ).*/
    pub m_ResizeAlgorithm: i32,
    /**Compression of imported texture.*/
    pub m_TextureCompression: i32,
    pub m_TextureFormat: i32,
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_ForceMaximumCompressionQuality_BC6H_BC7: Option<bool>,
}

/// TextureParameter is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextureParameter {
    pub m_Dim: i8,
    pub m_Index: i32,
    pub m_NameIndex: i32,
    pub m_SamplerIndex: i32,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_MultiSampled: Option<bool>,
}

/// TextureParameters is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextureParameters {
    pub height: i32,
    pub mipLevels: i32,
    pub textureFormat: i32,
    pub width: i32,
}

/// TextureSettings is a sub class of the Unity engine since version 2020.1.0a20.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextureSettings {
    pub anisoLevel: i32,
    pub compressionQuality: i32,
    pub crunchedCompression: bool,
    pub filterMode: i32,
    pub generateMipMaps: bool,
    pub maxTextureSize: i32,
    pub readable: bool,
    pub sRGB: bool,
    pub textureCompression: i32,
}

/// TierGraphicsSettings is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct TierGraphicsSettings {
    pub hdrMode: i32,
    pub renderingPath: i32,
    pub useCascadedShadowMaps: bool,
    pub useHDR: bool,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub enableLPPV: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub prefer32BitShadowMaps: Option<bool>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub realtimeGICPUUsage: Option<i32>,
}

/// Tile is a sub class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Tilemaps.Tile.html):
/**
Class for a default tile in the Tilemap.
This inherits from TileBase and represents a default tile to be placed in a Tilemap. It implements TileBase.GetTileData for simple rendering of a Sprite in the tile map.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    pub m_TileColorIndex: u32,
    pub m_TileIndex: u32,
    pub m_TileMatrixIndex: u32,
    pub m_TileSpriteIndex: u32,
    /// u16: (2020.3.42f1 - 2022.2.0b16)
    pub dummyAlignment: Option<u16>,
    /// u32: (2019.3.0f4 - 2022.2.0b16)
    pub m_AllTileFlags: Option<u32>,
    /// i32: (2017.4.33f1 - 2018.4.15f1)
    pub m_ColliderType: Option<i32>,
    /// PPtr/*<GameObject>*/: (2017.4.33f1 - 2018.4.15f1)
    pub m_ObjectToInstantiate: Option<PPtr /*<GameObject>*/>,
    /// i32: (2017.4.33f1 - 2018.4.15f1)
    pub m_TileFlags: Option<i32>,
    /// u16: (2019.3.0f4 - 2022.2.0b16)
    pub m_TileObjectToInstantiateIndex: Option<u16>,
}

/// TileAnimationData is a sub class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Tilemaps.TileAnimationData.html):
/**
A Struct for the required data for animating a Tile.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TileAnimationData {
    /**The array of sprites that are ordered by appearance in the animation.*/
    pub m_AnimatedSprites: Vec<PPtr /*<Sprite>*/>,
    /**The animation speed.*/
    pub m_AnimationSpeed: f32,
    pub m_AnimationTimeOffset: f32,
    /**TileAnimationFlags for controlling the Tile Animation.*/
    /// u32: (2022.2.0b16 - 2022.2.0b16)
    pub m_Flags: Option<u32>,
    /// bool: (2017.4.33f1 - 2021.2.16f1)
    pub m_IsLooping: Option<bool>,
}

/// Tilemap is a  class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Tilemaps.Tilemap.html):
/**
The Tilemap stores Sprites in a layout marked by a Grid component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Tilemap {
    pub m_AnimatedTiles: Vec<(int3_storage, TileAnimationData)>,
    /**The frame rate for all Tile animations in the Tilemap.*/
    pub m_AnimationFrameRate: f32,
    /**The color of the Tilemap layer.*/
    pub m_Color: ColorRGBA,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The origin of the Tilemap in cell position.*/
    pub m_Origin: int3_storage,
    /**The size of the Tilemap in cells.*/
    pub m_Size: int3_storage,
    /**Gets the anchor point of Tiles in the Tilemap.*/
    pub m_TileAnchor: Vector3f,
    pub m_TileAssetArray: Vec<TilemapRefCountedData>,
    pub m_TileColorArray: Vec<TilemapRefCountedData>,
    pub m_TileMatrixArray: Vec<TilemapRefCountedData>,
    pub m_TileOrientation: i32,
    pub m_TileOrientationMatrix: Matrix4x4f,
    pub m_TileSpriteArray: Vec<TilemapRefCountedData>,
    pub m_Tiles: Vec<(int3_storage, Tile)>,
    /// Vec<TilemapRefCountedData>: (2019.3.0f4 - 2022.2.0b16)
    pub m_TileObjectToInstantiateArray: Option<Vec<TilemapRefCountedData>>,
}

/// TilemapCollider2D is a  class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Tilemaps.TilemapCollider2D.html):
/**
Collider for 2D physics representing shapes defined by the corresponding Tilemap.
See Also: BoxCollider2D, CircleCollider2D, EdgeCollider2D, PolygonCollider2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TilemapCollider2D {
    /**The density of the collider used to calculate its mass (when auto mass is enabled).*/
    pub m_Density: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    pub m_Material: PPtr, /*<PhysicsMaterial2D>*/
    /**The local offset of the collider geometry.*/
    pub m_Offset: Vector2f,
    /**Sets whether the Collider will be used or not used by a CompositeCollider2D.*/
    pub m_UsedByComposite: bool,
    /**Whether the collider is used by an attached effector or not.*/
    pub m_UsedByEffector: bool,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_CallbackLayers: Option<BitField>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**The amount of Collider shapes each Tile extrudes to facilitate compositing with neighboring Tiles. This eliminates fine gaps between Tiles when using a CompositeCollider2D. This is calculated in Unity units within world space.*/
    /// f32: (2019.3.0f4 - 2022.2.0b16)
    pub m_ExtrusionFactor: Option<f32>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
    /**Maximum number of Tile Changes accumulated before doing a full collider rebuild instead of an incremental rebuild.*/
    /// u32: (2019.3.0f4 - 2022.2.0b16)
    pub m_MaximumTileChangeCount: Option<u32>,
    /**When the value is true, the Collider uses an additional Delaunay triangulation step to produce the Collider mesh. When the value is false, this additional step does not occur.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_UseDelaunayMesh: Option<bool>,
}

/// TilemapEditorUserSettings is a  class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct TilemapEditorUserSettings {
    pub m_FocusMode: i32,
    pub m_LastUsedPalette: PPtr, /*<GameObject>*/
}

/// TilemapRefCountedData is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct TilemapRefCountedData {
    pub m_Data: PPtr, /*<Object>*/
    pub m_RefCount: u32,
}

/// TilemapRenderer is a  class of the Unity engine since version 2017.4.33f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Tilemaps.TilemapRenderer.html):
/**
The tile map renderer is used to render the tile map marked out by a tile map component and a grid component.
This class is a script interface for a tile map renderer component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TilemapRenderer {
    pub m_CastShadows: u8,
    /**Bounds used for culling of Tilemap chunks.*/
    pub m_ChunkCullingBounds: Vector3f,
    /**Size in number of tiles of each chunk created by the TilemapRenderer.*/
    pub m_ChunkSize: int3_storage,
    pub m_DynamicOccludee: u8,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The light probe interpolation type.*/
    pub m_LightProbeUsage: u8,
    pub m_LightProbeVolumeOverride: PPtr, /*<GameObject>*/
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    /**Specifies how the Tilemap interacts with the masks.*/
    pub m_MaskInteraction: i32,
    /**Returns all the instantiated materials of this object.*/
    pub m_Materials: Vec<PPtr /*<Material>*/>,
    /**Maximum number of chunks the TilemapRenderer caches in memory.*/
    pub m_MaxChunkCount: u32,
    /**Maximum number of frames the TilemapRenderer keeps unused chunks in memory.*/
    pub m_MaxFrameAge: u32,
    pub m_MotionVectors: u8,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    pub m_ProbeAnchor: PPtr, /*<Transform>*/
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: u8,
    /**Should reflection probes be used for this Renderer?*/
    pub m_ReflectionProbeUsage: u8,
    /**Active sort order for the TilemapRenderer.*/
    pub m_SortOrder: i32,
    pub m_SortingLayer: i16,
    /**Unique ID of the Renderer's sorting layer.*/
    pub m_SortingLayerID: i32,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    pub m_StaticBatchInfo: StaticBatchInfo,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    /**Returns whether the TilemapRenderer automatically detects the bounds to extend chunk culling by.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_DetectChunkCullingBounds: Option<i32>,
    /**The mode in which the TileMapRenderer batches the tiles for rendering.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_Mode: Option<i32>,
    /// u8: (2020.1.0a20 - 2022.2.0b16)
    pub m_RayTraceProcedural: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0f4 - 2022.2.0b16)
    pub m_RayTracingMode: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on.*/
    /// u32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RenderingLayerMask: Option<u32>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.2.16f1 - 2022.2.0b16)
    pub m_StaticShadowCaster: Option<u8>,
}

/// TimeManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeManager {
    pub m_TimeScale: f32,
    /// f32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "Fixed Timestep")]
    pub Fixed_Timestep: Option<f32>,
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    #[serde(alias = "Maximum Particle Timestep")]
    pub Maximum_Particle_Timestep: Option<f32>,
    /// f32: (3.4.0 - 2022.2.0b16)
    #[serde(alias = "Maximum Allowed Timestep")]
    pub Maximum_Allowed_Timestep: Option<f32>,
}

/// TrailModule is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.TrailModule.html):
/**
Script interface for the TrailsModule.
This module adds trails to your particles. For example, you can make the trails stay in the wake of particles as they move, or make them connect each particle in the system together.See Also: ParticleSystem, ParticleSystem.trails.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TrailModule {
    /**The gradient that controls the trail colors during the lifetime of the attached particle.*/
    pub colorOverLifetime: MinMaxGradient,
    /**The gradient that controls the trail colors over the length of the trail.*/
    pub colorOverTrail: MinMaxGradient,
    /**Specifies whether trails disappear immediately when their owning particle dies. When false, each trail persists until all its points have naturally expired, based on its lifetime.*/
    pub dieWithParticles: bool,
    /**Specifies whether the TrailModule is enabled or disabled.*/
    pub enabled: bool,
    /**Toggle whether the trail inherits the particle color as its starting color.*/
    pub inheritParticleColor: bool,
    /**The curve describing the trail lifetime, throughout the lifetime of the particle.*/
    pub lifetime: MinMaxCurve,
    /**Set the minimum distance each trail can travel before the system adds a new vertex to it.*/
    pub minVertexDistance: f32,
    /**Choose what proportion of particles receive a trail.*/
    pub ratio: f32,
    /**Set whether the particle size acts as a multiplier on top of the trail lifetime.*/
    pub sizeAffectsLifetime: bool,
    /**Set whether the particle size acts as a multiplier on top of the trail width.*/
    pub sizeAffectsWidth: bool,
    /**Choose whether the U coordinate of the trail Texture is tiled or stretched.*/
    pub textureMode: i32,
    /**The curve describing the width of each trail point.*/
    pub widthOverTrail: MinMaxCurve,
    /**Drop new trail points in world space, regardless of Particle System Simulation Space.*/
    pub worldSpace: bool,
    /**Adds an extra position to each ribbon, connecting it to the location of the Transform Component.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub attachRibbonsToTransform: Option<bool>,
    /**Configures the trails to generate Normals and Tangents. With this data, Scene lighting can affect the trails via Normal Maps and the Unity Standard Shader, or your own custom-built Shaders.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub generateLightingData: Option<bool>,
    /**Choose how the system generates the particle trails.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub mode: Option<i32>,
    /**Select how many lines to create through the Particle System.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub ribbonCount: Option<i32>,
    /**Apply a shadow bias to prevent self-shadowing artifacts. The specified value is the proportion of the trail width at each segment.*/
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub shadowBias: Option<f32>,
    /**Specifies whether, if you use this system as a sub-emitter, ribbons connect particles from each parent particle independently.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub splitSubEmitterRibbons: Option<bool>,
    /**A multiplier for the UV coordinates of the trail texture.*/
    /// Vector2f: (2022.2.0b16 - 2022.2.0b16)
    pub textureScale: Option<Vector2f>,
}

/// TrailRenderer is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TrailRenderer.html):
/**
The trail renderer is used to make trails behind objects in the Scene as they move about.
This class is a script interface for a trail renderer component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TrailRenderer {
    /**Does the GameObject of this Trail Renderer auto destruct?*/
    pub m_Autodestruct: bool,
    pub m_CastShadows: Enum_bool__u8,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapTilingOffset: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    pub m_Materials: Vec<PPtr /*<Material>*/>,
    /**Set the minimum distance the trail can travel before a new vertex is added to it.*/
    pub m_MinVertexDistance: f32,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: Enum_bool__u8,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    /**How long does the trail take to fade out.*/
    pub m_Time: f32,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ApplyActiveColorSpace: Option<bool>,
    /// Gradient: (3.4.0 - 3.4.0)
    pub m_Colors: Option<Gradient>,
    /// u8: (2017.4.33f1 - 2022.2.0b16)
    pub m_DynamicOccludee: Option<u8>,
    /**Creates trails when the GameObject moves.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_Emitting: Option<bool>,
    /**The width of the trail at the end of the trail.*/
    /// f32: (3.4.0 - 3.4.0)
    pub m_EndWidth: Option<f32>,
    /**The light probe interpolation type.*/
    /// u8: (5.6.0b2 - 2022.2.0b16)
    pub m_LightProbeUsage: Option<u8>,
    /// PPtr/*<GameObject>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_LightProbeVolumeOverride: Option<PPtr /*<GameObject>*/>,
    /// u16: (5.6.0b2 - 2022.2.0b16)
    pub m_LightmapIndexDynamic: Option<u16>,
    /// Vector4f: (5.6.0b2 - 2022.2.0b16)
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    /**Specifies how the TrailRenderer interacts with SpriteMask.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_MaskInteraction: Option<i32>,
    /// u8: (5.6.0b2 - 2022.2.0b16)
    pub m_MotionVectors: Option<u8>,
    /// LineParameters: (5.6.0b2 - 2022.2.0b16)
    pub m_Parameters: Option<LineParameters>,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr/*<Transform>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_ProbeAnchor: Option<PPtr /*<Transform>*/>,
    /// u8: (2020.1.0a20 - 2022.2.0b16)
    pub m_RayTraceProcedural: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0f4 - 2022.2.0b16)
    pub m_RayTracingMode: Option<u8>,
    /**Should reflection probes be used for this Renderer?*/
    /// u8: (5.6.0b2 - 2022.2.0b16)
    pub m_ReflectionProbeUsage: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on.*/
    /// u32: (2018.4.15f1 - 2022.2.0b16)
    pub m_RenderingLayerMask: Option<u32>,
    /// i16: (5.6.0b2 - 2022.2.0b16)
    pub m_SortingLayer: Option<i16>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_SortingLayerID: Option<i32>,
    /**Renderer's order within a sorting layer.*/
    /// i16: (5.6.0b2 - 2022.2.0b16)
    pub m_SortingOrder: Option<i16>,
    /**The width of the trail at the spawning point.*/
    /// f32: (3.4.0 - 3.4.0)
    pub m_StartWidth: Option<f32>,
    /// StaticBatchInfo: (5.6.0b2 - 2022.2.0b16)
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.2.16f1 - 2022.2.0b16)
    pub m_StaticShadowCaster: Option<u8>,
    /// Vec<u32>: (3.4.0 - 3.4.0)
    pub m_SubsetIndices: Option<Vec<u32>>,
}

/// Transform is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Transform.html):
/**
Position, rotation and scale of an object.
Every object in a Scene has a Transform.

It's used to store and manipulate the position, rotation and scale of the object.

Every Transform can have a parent, which allows you to apply position, rotation and scale hierarchically. This is the hierarchy seen in the Hierarchy pane.

They also support enumerators so you can loop through children using:
See Also: The component reference, Physics class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Transform {
    pub m_Children: Vec<PPtr /*<Transform>*/>,
    pub m_Father: PPtr, /*<Transform>*/
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Position of the transform relative to the parent transform.*/
    pub m_LocalPosition: Vector3f,
    /**The rotation of the transform relative to the transform rotation of the parent.*/
    pub m_LocalRotation: Quaternionf,
    /**The scale of the transform relative to the GameObjects parent.*/
    pub m_LocalScale: Vector3f,
}

/// TransformMaskElement is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransformMaskElement {
    pub m_Path: String,
    pub m_Weight: f32,
}

/// Tree is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Tree.html):
/**
Tree Component for the tree creator.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /// PPtr/*<SpeedTreeWindAsset>*/: (5.6.0b2 - 2022.2.0b16)
    pub m_SpeedTreeWindAsset: Option<PPtr /*<SpeedTreeWindAsset>*/>,
}

/// TreeInstance is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TreeInstance.html):
/**
Contains information about a tree placed in the Terrain game object.
This struct can be accessed from the TerrainData Object.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TreeInstance {
    /**Color of this instance.*/
    pub color: ColorRGBA,
    /**Height scale of this instance (compared to the prototype's size).*/
    pub heightScale: f32,
    pub index: i32,
    /**Lightmap color calculated for this instance.*/
    pub lightmapColor: ColorRGBA,
    /**Position of the tree.*/
    pub position: Vector3f,
    /**Width scale of this instance (compared to the prototype's size).*/
    pub widthScale: f32,
    /**Read-only.Rotation of the tree on X-Z plane (in radians).*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub rotation: Option<f32>,
}

/// TreePrototype is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TreePrototype.html):
/**
Simple class that contains a pointer to a tree prototype.
This class is used by the TerrainData gameObject.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TreePrototype {
    /**Bend factor of the tree prototype.*/
    pub bendFactor: f32,
    /**Retrieves the actual GameObject used by the tree.*/
    pub prefab: PPtr, /*<GameObject>*/
    /**The LOD index of a Tree LODGroup that Unity uses to generate a NavMesh. It uses this value only for Trees with a LODGroup, and ignores this value for regular Trees.*/
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub navMeshLod: Option<i32>,
}

/// TriggerModule is a sub class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.TriggerModule.html):
/**
Script interface for the TriggerModule.
This module is useful for killing particles when they touch a set of collision shapes, or for calling a script command to let you apply custom particle behaviors when the trigger is activated.The example code for MonoBehaviour.OnParticleTrigger shows how the callback type action works.See Also: ParticleSystem, ParticleSystem.trigger.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerModule {
    /**Specifies whether the TriggerModule is enabled or disabled.*/
    pub enabled: bool,
    /**Choose what action to perform when particles enter the trigger volume.*/
    pub enter: i32,
    /**Choose what action to perform when particles leave the trigger volume.*/
    pub exit: i32,
    /**Choose what action to perform when particles are inside the trigger volume.*/
    pub inside: i32,
    /**Choose what action to perform when particles are outside the trigger volume.*/
    pub outside: i32,
    /**A multiplier Unity applies to the size of each particle before it processes overlaps.*/
    pub radiusScale: f32,
    /**Determines whether collider information is available when calling [[ParticleSystem::GetTriggerParticles]].*/
    /// i32: (2020.3.42f1 - 2022.2.0b16)
    pub colliderQueryMode: Option<i32>,
    /// PPtr/*<Component>*/: (5.6.0b2 - 2020.1.0a20)
    pub collisionShape0: Option<PPtr /*<Component>*/>,
    /// PPtr/*<Component>*/: (5.6.0b2 - 2020.1.0a20)
    pub collisionShape1: Option<PPtr /*<Component>*/>,
    /// PPtr/*<Component>*/: (5.6.0b2 - 2020.1.0a20)
    pub collisionShape2: Option<PPtr /*<Component>*/>,
    /// PPtr/*<Component>*/: (5.6.0b2 - 2020.1.0a20)
    pub collisionShape3: Option<PPtr /*<Component>*/>,
    /// PPtr/*<Component>*/: (5.6.0b2 - 2020.1.0a20)
    pub collisionShape4: Option<PPtr /*<Component>*/>,
    /// PPtr/*<Component>*/: (5.6.0b2 - 2020.1.0a20)
    pub collisionShape5: Option<PPtr /*<Component>*/>,
    /// Vec<PPtr/*<Component>*/>: (2020.3.42f1 - 2022.2.0b16)
    pub primitives: Option<Vec<PPtr /*<Component>*/>>,
}

/// TrueTypeFontImporter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TrueTypeFontImporter.html):
/**
AssetImporter for importing Fonts.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TrueTypeFontImporter {
    /**An array of font names, to be used when includeFontData is set to false.*/
    pub m_FontNames: Vec<String>,
    /**Font size to use for importing the characters.*/
    pub m_FontSize: i32,
    pub m_ForceTextureCase: i32,
    /**If this is enabled, the actual font will be embedded into the asset for Dynamic fonts.*/
    pub m_IncludeFontData: bool,
    /**The name of the object.*/
    pub m_Name: String,
    /**Calculation mode for determining font's ascent.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_AscentCalculationMode: Option<i32>,
    /**Get or set the AssetBundle name.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AssetBundleName: Option<String>,
    /**Get or set the AssetBundle variant.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_AssetBundleVariant: Option<String>,
    /**Border pixels added to character images for padding. This is useful if you want to render text using a shader which needs to render outside of the character area (like an outline shader).*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_CharacterPadding: Option<i32>,
    /**Spacing between character images in the generated texture in pixels. This is useful if you want to render text using a shader which samples pixels outside of the character area (like an outline shader).*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_CharacterSpacing: Option<i32>,
    /**A custom set of characters to be included in the Font Texture.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_CustomCharacters: Option<String>,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /// Vec<PPtr/*<Font>*/>: (5.6.0b2 - 2022.2.0b16)
    pub m_FallbackFontReferences: Option<Vec<PPtr /*<Font>*/>>,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.0)
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_FontName: Option<String>,
    /**Font rendering mode to use for this font.*/
    /// i32: (5.6.0b2 - 2022.2.0b16)
    pub m_FontRenderingMode: Option<i32>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_NewHashIdentity: Option<MdFour>,
    /// MdFour: (3.4.0 - 3.4.0)
    pub m_OldHashIdentity: Option<MdFour>,
    /// Output: (5.6.0b2 - 2022.2.0b16)
    pub m_Output: Option<Output>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_RenderMode: Option<i32>,
    /**Set this property to true if you want to round the internal advance width of the font to the nearest integer.*/
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub m_ShouldRoundAdvanceValue: Option<bool>,
    /// i32: (3.4.0 - 3.4.0)
    pub m_Style: Option<i32>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_Use2xBehaviour: Option<bool>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_UseLegacyBoundsCalculation: Option<bool>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /**Get or set any user data.*/
    /// String: (5.6.0b2 - 2022.2.0b16)
    pub m_UserData: Option<String>,
}

/// UAVParameter is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct UAVParameter {
    pub m_Index: i32,
    pub m_NameIndex: i32,
    pub m_OriginalIndex: i32,
}

/// UVAnimation is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct UVAnimation {
    pub cycles: f32,
    /// i32: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "x Tile")]
    pub x_Tile: Option<i32>,
    /// i32: (3.4.0 - 2017.4.33f1)
    #[serde(alias = "y Tile")]
    pub y_Tile: Option<i32>,
}

/// UVModule is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct UVModule {
    pub animationType: i32,
    pub cycles: f32,
    pub enabled: bool,
    pub flipU: f32,
    pub flipV: f32,
    pub frameOverTime: MinMaxCurve,
    pub rowIndex: i32,
    pub startFrame: MinMaxCurve,
    pub tilesX: i32,
    pub tilesY: i32,
    pub uvChannelMask: i32,
    /// f32: (2018.4.15f1 - 2022.2.0b16)
    pub fps: Option<f32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub mode: Option<i32>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub randomRow: Option<bool>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub rowMode: Option<i32>,
    /// Vector2f: (2018.4.15f1 - 2022.2.0b16)
    pub speedRange: Option<Vector2f>,
    /// Vec<SpriteData>: (2017.4.33f1 - 2022.2.0b16)
    pub sprites: Option<Vec<SpriteData>>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    pub timeMode: Option<i32>,
}

/// UnityAdsManager is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityAdsManager {}

/// UnityAdsSettings is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityAdsSettings {
    pub m_Enabled: bool,
    pub m_InitializeOnStartup: bool,
    pub m_TestMode: bool,
    /// String: (5.6.0b2 - 5.6.0b2)
    pub m_AndroidGameId: Option<String>,
    /// u32: (5.6.0b2 - 5.6.0b2)
    pub m_EnabledPlatforms: Option<u32>,
    /// String: (2017.4.33f1 - 2022.2.0b16)
    pub m_GameId: Option<String>,
    /// String: (5.6.0b2 - 5.6.0b2)
    pub m_IosGameId: Option<String>,
}

/// UnityAnalyticsManager is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityAnalyticsManager {}

/// UnityAnalyticsSettings is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityAnalyticsSettings {
    pub m_Enabled: bool,
    pub m_InitializeOnStartup: bool,
    pub m_TestMode: bool,
    /// bool: (2020.3.42f1 - 2022.2.0b16)
    pub m_PackageRequiringCoreStatsPresent: Option<bool>,
    /// String: (5.6.0b2 - 2017.4.33f1)
    pub m_TestConfigUrl: Option<String>,
    /// String: (5.6.0b2 - 2017.4.33f1)
    pub m_TestEventUrl: Option<String>,
}

/// UnityConnectSettings is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityConnectSettings {
    pub CrashReportingSettings: CrashReportingSettings,
    pub PerformanceReportingSettings: PerformanceReportingSettings,
    pub UnityAdsSettings: UnityAdsSettings,
    pub UnityAnalyticsSettings: UnityAnalyticsSettings,
    pub UnityPurchasingSettings: UnityPurchasingSettings,
    pub m_Enabled: bool,
    pub m_TestMode: bool,
    /// String: (2018.4.15f1 - 2022.2.0b16)
    pub m_ConfigUrl: Option<String>,
    /// String: (2020.3.42f1 - 2022.2.0b16)
    pub m_DashboardUrl: Option<String>,
    /// String: (2018.4.15f1 - 2022.2.0b16)
    pub m_EventOldUrl: Option<String>,
    /// String: (2018.4.15f1 - 2022.2.0b16)
    pub m_EventUrl: Option<String>,
    /// String: (5.6.0b2 - 2017.4.33f1)
    pub m_TestConfigUrl: Option<String>,
    /// String: (5.6.0b2 - 2017.4.33f1)
    pub m_TestEventUrl: Option<String>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_TestInitMode: Option<i32>,
}

/// UnityPropertySheet is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityPropertySheet {
    pub m_Colors: Vec<(Enum_FastPropertyName__String, ColorRGBA)>,
    pub m_Floats: Vec<(Enum_FastPropertyName__String, f32)>,
    pub m_TexEnvs: Vec<(Enum_FastPropertyName__String, UnityTexEnv)>,
    /// Vec<(String, i32)>: (2021.2.16f1 - 2022.2.0b16)
    pub m_Ints: Option<Vec<(String, i32)>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_FastPropertyName__String {
    FastPropertyName(FastPropertyName),
    String(String),
}

/// UnityPurchasingSettings is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityPurchasingSettings {
    pub m_Enabled: bool,
    pub m_TestMode: bool,
}

/// UnityTexEnv is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityTexEnv {
    pub m_Offset: Vector2f,
    pub m_Scale: Vector2f,
    pub m_Texture: PPtr, /*<Texture>*/
}

/// UpdateZoneInfo is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateZoneInfo {
    pub needSwap: bool,
    pub passIndex: i32,
    pub rotation: f32,
    pub updateZoneCenter: Vector3f,
    pub updateZoneSize: Vector3f,
}

/// VFXCPUBufferData is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXCPUBufferData {
    pub data: Vec<u32>,
}

/// VFXCPUBufferDesc is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXCPUBufferDesc {
    pub capacity: u32,
    pub initialData: VFXCPUBufferData,
    pub layout: Vec<VFXLayoutElementDesc>,
    pub stride: u32,
}

/// VFXEditorSystemDesc is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXEditorSystemDesc {
    pub buffers: Vec<VFXMapping>,
    pub capacity: u32,
    pub flags: i32,
    pub layer: u32,
    pub tasks: Vec<VFXEditorTaskDesc>,
    pub values: Vec<VFXMapping>,
    /// i32: (2018.4.15f1 - 2019.3.0f4)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// VFXEditorTaskDesc is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXEditorTaskDesc {
    pub buffers: Vec<VFXMapping>,
    pub params: Vec<VFXMapping>,
    pub processor: PPtr, /*<NamedObject>*/
    pub shaderSourceIndex: i32,
    pub values: Vec<VFXMapping>,
    /// i32: (2018.4.15f1 - 2019.3.0f4)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// Vec<VFXMappingTemporary>: (2019.3.0f4 - 2019.3.0f4)
    pub temporaryBuffers: Option<Vec<VFXMappingTemporary>>,
}

/// VFXEntryExpressionValue is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXEntryExpressionValue {
    pub m_ExpressionIndex: u32,
    pub m_Value: f32,
}

/// VFXEventDesc is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXEventDesc {
    pub name: String,
    pub playSystems: Vec<u32>,
    pub stopSystems: Vec<u32>,
    /// Vec<u32>: (2021.2.16f1 - 2022.2.0b16)
    pub initSystems: Option<Vec<u32>>,
}

/// VFXExpressionContainer is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXExpressionContainer {
    pub m_Expressions: Vec<Expression>,
    pub m_NeedsLocalToWorld: bool,
    pub m_NeedsWorldToLocal: bool,
    /// u32: (2022.2.0b16 - 2022.2.0b16)
    pub m_ConstantBakeCurveCount: Option<u32>,
    /// u32: (2022.2.0b16 - 2022.2.0b16)
    pub m_ConstantBakeGradientCount: Option<u32>,
    /// u32: (2022.2.0b16 - 2022.2.0b16)
    pub m_DynamicBakeCurveCount: Option<u32>,
    /// u32: (2022.2.0b16 - 2022.2.0b16)
    pub m_DynamicBakeGradientCount: Option<u32>,
    /// u32: (2020.3.42f1 - 2022.2.0b16)
    pub m_MaxCommonExpressionsIndex: Option<u32>,
    /// i32: (2019.3.0f4 - 2022.2.0b16)
    pub m_NeededMainCameraBuffers: Option<i32>,
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_NeedsMainCamera: Option<bool>,
}

/// VFXField is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXField {
    pub m_Array: Vec<VFXEntryExpressionValue>,
}

/// VFXGPUBufferDesc is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXGPUBufferDesc {
    pub capacity: u32,
    pub layout: Vec<VFXLayoutElementDesc>,
    pub size: u32,
    pub stride: u32,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// VFXLayoutElementDesc is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXLayoutElementDesc {
    pub name: String,
    pub offset: VFXLayoutOffset,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// VFXLayoutOffset is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXLayoutOffset {
    pub bucket: u32,
    pub element: u32,
    pub structure: u32,
}

/// VFXManager is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/VFX.VFXManager.html):
/**
Use this class to set a number of properties that control VisualEffect behavior within your Unity Project.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXManager {
    pub m_CopyBufferShader: PPtr, /*<ComputeShader>*/
    pub m_FixedTimeStep: f32,
    pub m_IndirectShader: PPtr, /*<ComputeShader>*/
    pub m_MaxDeltaTime: f32,
    pub m_RenderPipeSettingsPath: String,
    pub m_SortShader: PPtr, /*<ComputeShader>*/
    /// u32: (2022.2.0b16 - 2022.2.0b16)
    pub m_BatchEmptyLifetime: Option<u32>,
    /// u32: (2020.1.0a20 - 2022.2.0b16)
    pub m_CompiledVersion: Option<u32>,
    /// f32: (2022.2.0b16 - 2022.2.0b16)
    pub m_MaxScrubTime: Option<f32>,
    /// PPtr/*<MonoBehaviour>*/: (2021.2.16f1 - 2022.2.0b16)
    pub m_RuntimeResources: Option<PPtr /*<MonoBehaviour>*/>,
    /// u32: (2020.1.0a20 - 2022.2.0b16)
    pub m_RuntimeVersion: Option<u32>,
    /// PPtr/*<ComputeShader>*/: (2019.3.0f4 - 2022.2.0b16)
    pub m_StripUpdateShader: Option<PPtr /*<ComputeShader>*/>,
}

/// VFXMapping is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXMapping {
    pub index: i32,
    pub nameId: String,
}

/// VFXMappingTemporary is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXMappingTemporary {
    pub mapping: VFXMapping,
    pub pastFrameIndex: u32,
    pub perCameraBuffer: bool,
}

/// VFXPropertySheetSerializedBase is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXPropertySheetSerializedBase {
    pub m_AnimationCurve: VFXField,
    pub m_Bool: VFXField,
    pub m_Float: VFXField,
    pub m_Gradient: VFXField,
    pub m_Int: VFXField,
    pub m_Matrix4x4f: VFXField,
    pub m_NamedObject: VFXField,
    pub m_Uint: VFXField,
    pub m_Vector2f: VFXField,
    pub m_Vector3f: VFXField,
    pub m_Vector4f: VFXField,
}

/// VFXRenderer is a  class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXRenderer {
    pub m_CastShadows: u8,
    pub m_DynamicOccludee: u8,
    pub m_Enabled: bool,
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_LightProbeUsage: u8,
    pub m_LightProbeVolumeOverride: PPtr, /*<GameObject>*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    pub m_MotionVectors: u8,
    pub m_ProbeAnchor: PPtr, /*<Transform>*/
    pub m_ReceiveShadows: u8,
    pub m_ReflectionProbeUsage: u8,
    pub m_RendererPriority: i32,
    pub m_RenderingLayerMask: u32,
    pub m_SortingLayer: i16,
    pub m_SortingLayerID: i32,
    pub m_SortingOrder: i16,
    pub m_StaticBatchInfo: StaticBatchInfo,
    pub m_StaticBatchRoot: PPtr, /*<Transform>*/
    /// Vec<PPtr/*<Material>*/>: (2018.4.15f1 - 2020.1.0a20)
    pub m_Materials: Option<Vec<PPtr /*<Material>*/>>,
    /// u8: (2020.1.0a20 - 2022.2.0b16)
    pub m_RayTraceProcedural: Option<u8>,
    /// u8: (2019.3.0f4 - 2022.2.0b16)
    pub m_RayTracingMode: Option<u8>,
    /// u8: (2021.2.16f1 - 2022.2.0b16)
    pub m_StaticShadowCaster: Option<u8>,
}

/// VFXRendererSettings is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXRendererSettings {
    pub lightProbeUsage: i32,
    pub motionVectorGenerationMode: i32,
    pub receiveShadows: bool,
    pub reflectionProbeUsage: i32,
    pub shadowCastingMode: i32,
}

/// VFXShaderSourceDesc is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXShaderSourceDesc {
    pub compute: bool,
    pub name: String,
    pub source: String,
}

/// VFXSystemDesc is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXSystemDesc {
    pub buffers: Vec<VFXMapping>,
    pub capacity: u32,
    pub flags: i32,
    pub layer: u32,
    pub tasks: Vec<VFXTaskDesc>,
    pub values: Vec<VFXMapping>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// String: (2020.1.0a20 - 2022.2.0b16)
    pub name: Option<String>,
}

/// VFXTaskDesc is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXTaskDesc {
    pub buffers: Vec<VFXMapping>,
    pub params: Vec<VFXMapping>,
    pub processor: PPtr, /*<NamedObject>*/
    pub values: Vec<VFXMapping>,
    /// i32: (2018.4.15f1 - 2022.2.0b16)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// Vec<VFXMappingTemporary>: (2019.3.0f4 - 2022.2.0b16)
    pub temporaryBuffers: Option<Vec<VFXMappingTemporary>>,
}

/// VFXTemporaryGPUBufferDesc is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXTemporaryGPUBufferDesc {
    pub desc: VFXGPUBufferDesc,
    pub frameCount: u32,
}

/// VRSettings is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct VRSettings {
    /// Google: (5.6.0b2 - 2020.1.0a20)
    pub cardboard: Option<Google>,
    /// Google: (5.6.0b2 - 2020.1.0a20)
    pub daydream: Option<Google>,
    /// bool: (2018.4.15f1 - 2022.2.0b16)
    pub enable360StereoCapture: Option<bool>,
    /// HoloLens: (2017.4.33f1 - 2020.1.0a20)
    pub hololens: Option<HoloLens>,
    /// Lumin: (2019.3.0f4 - 2020.1.0a20)
    pub lumin: Option<Lumin>,
    /// DeviceNone: (5.6.0b2 - 2020.1.0a20)
    pub none: Option<DeviceNone>,
    /// Oculus: (2017.4.33f1 - 2020.1.0a20)
    pub oculus: Option<Oculus>,
}

/// ValueArrayConstant is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ValueArrayConstant {
    pub m_ValueArray: Vec<ValueConstant>,
}

/// ValueConstant is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ValueConstant {
    pub m_ID: u32,
    pub m_Index: u32,
    pub m_Type: u32,
}

/// ValueDelta is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ValueDelta {
    pub m_Start: f32,
    pub m_Stop: f32,
}

/// VariableBoneCountWeights is a sub class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct VariableBoneCountWeights {
    pub m_Data: Vec<u32>,
}

/// VariantInfo is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct VariantInfo {
    pub keywords: String,
    pub passType: i32,
}

/// Vector2f is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

/// Vector3Curve is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Vector3Curve {
    pub curve: AnimationCurve,
    pub path: String,
}

/// Vector3f is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Vector4f is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Vector4f {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// VectorParameter is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct VectorParameter {
    pub m_ArraySize: i32,
    pub m_Dim: i8,
    pub m_Index: i32,
    pub m_NameIndex: i32,
    pub m_Type: i8,
}

/// VelocityModule is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct VelocityModule {
    pub enabled: bool,
    pub inWorldSpace: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    /// MinMaxCurve: (2018.4.15f1 - 2022.2.0b16)
    pub orbitalOffsetX: Option<MinMaxCurve>,
    /// MinMaxCurve: (2018.4.15f1 - 2022.2.0b16)
    pub orbitalOffsetY: Option<MinMaxCurve>,
    /// MinMaxCurve: (2018.4.15f1 - 2022.2.0b16)
    pub orbitalOffsetZ: Option<MinMaxCurve>,
    /// MinMaxCurve: (2018.4.15f1 - 2022.2.0b16)
    pub orbitalX: Option<MinMaxCurve>,
    /// MinMaxCurve: (2018.4.15f1 - 2022.2.0b16)
    pub orbitalY: Option<MinMaxCurve>,
    /// MinMaxCurve: (2018.4.15f1 - 2022.2.0b16)
    pub orbitalZ: Option<MinMaxCurve>,
    /// MinMaxCurve: (2018.4.15f1 - 2022.2.0b16)
    pub radial: Option<MinMaxCurve>,
    /// MinMaxCurve: (2017.4.33f1 - 2022.2.0b16)
    pub speedModifier: Option<MinMaxCurve>,
}

/// VersionControlSettings is a  class of the Unity engine since version 2020.1.0a20.
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionControlSettings {
    pub m_CollabEditorSettings: CollabEditorSettings,
    pub m_Mode: String,
}

/// VertexData is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct VertexData {
    pub m_Channels: Vec<ChannelInfo>,
    pub m_DataSize: Vec<u8>,
    pub m_VertexCount: u32,
    /// i32: (5.6.0b2 - 2017.4.33f1)
    pub m_CurrentChannels: Option<i32>,
}

/// VideoBuildInfo is a  class of the Unity engine since version 2020.3.42f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoBuildInfo {
    pub m_IsVideoModuleDisabled: bool,
    pub m_VideoClipCount: i32,
}

/// VideoClip is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Video.VideoClip.html):
/**
A container for video data.
A VideoClip stores the video portion of a movie file using a codec that is appropriate for the target platform.  VideoClips are referenced by VideoPlayers to play videos.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoClip {
    /**The height of the images in the video clip in pixels. (Read Only).*/
    pub Height: u32,
    /**The width of the images in the video clip in pixels. (Read Only).*/
    pub Width: u32,
    pub m_AudioChannelCount: Vec<u16>,
    pub m_AudioLanguage: Vec<String>,
    pub m_AudioSampleRate: Vec<u32>,
    pub m_ExternalResources: StreamedResource,
    pub m_Format: i32,
    /**The length of the VideoClip in frames. (Read Only).*/
    pub m_FrameCount: u64,
    /**The frame rate of the clip in frames/second. (Read Only).*/
    pub m_FrameRate: f64,
    pub m_HasSplitAlpha: bool,
    /**The name of the object.*/
    pub m_Name: String,
    /**The video clip path in the project's assets. (Read Only).*/
    pub m_OriginalPath: String,
    pub m_ProxyHeight: u32,
    pub m_ProxyWidth: u32,
    /// u32: (2017.4.33f1 - 2022.2.0b16)
    pub m_PixelAspecRatioDen: Option<u32>,
    /// u32: (2017.4.33f1 - 2022.2.0b16)
    pub m_PixelAspecRatioNum: Option<u32>,
    /// Vec<PPtr/*<Shader>*/>: (2020.1.0a20 - 2022.2.0b16)
    pub m_VideoShaders: Option<Vec<PPtr /*<Shader>*/>>,
    /**Whether the imported clip contains sRGB color data (Read Only).*/
    /// bool: (2019.3.0f4 - 2022.2.0b16)
    pub m_sRGB: Option<bool>,
}

/// VideoClipImporter is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/VideoClipImporter.html):
/**
VideoClipImporter lets you modify VideoClip import settings from Editor scripts.
See the Movie File Format Support Notes section in the VideoPlayer class documentation for supported movie file formats and encoding guidelines.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoClipImporter {
    /**Get or set the AssetBundle name.*/
    pub m_AssetBundleName: String,
    /**Get or set the AssetBundle variant.*/
    pub m_AssetBundleVariant: String,
    pub m_ColorSpace: i32,
    pub m_Deinterlace: i32,
    pub m_EncodeAlpha: bool,
    pub m_EndFrame: i32,
    /**Apply a horizontal flip during import.*/
    pub m_FlipHorizontal: bool,
    /**Apply a vertical flip during import.*/
    pub m_FlipVertical: bool,
    pub m_FrameRange: i32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Output: VideoClipImporterOutput,
    pub m_StartFrame: i32,
    pub m_TargetSettings: Vec<(i32, VideoClipImporterTargetSettings)>,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub m_AudioImportMode: Option<i32>,
    /// Vec<(SourceAssetIdentifier, PPtr/*<Object>*/)>: (2017.4.33f1 - 2022.2.0b16)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>>,
    /**Number of frames in the clip.*/
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub m_FrameCount: Option<i32>,
    /**Frame rate of the clip.*/
    /// f64: (5.6.0b2 - 5.6.0b2)
    pub m_FrameRate: Option<f64>,
    /**Import audio tracks from source file.*/
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub m_ImportAudio: Option<bool>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub m_IsColorLinear: Option<bool>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub m_OriginalHeight: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub m_OriginalWidth: Option<i32>,
    /// f32: (5.6.0b2 - 2018.4.15f1)
    pub m_Quality: Option<f32>,
    /// Vec<u16>: (5.6.0b2 - 5.6.0b2)
    pub m_SourceAudioChannelCount: Option<Vec<u16>>,
    /// Vec<u32>: (5.6.0b2 - 5.6.0b2)
    pub m_SourceAudioSampleRate: Option<Vec<u32>>,
    /**Size in bytes of the file before importing.*/
    /// u64: (5.6.0b2 - 5.6.0b2)
    pub m_SourceFileSize: Option<u64>,
    /**True if the source file has a channel for per-pixel transparency.*/
    /// bool: (5.6.0b2 - 5.6.0b2)
    pub m_SourceHasAlpha: Option<bool>,
    /// bool: (5.6.0b2 - 2018.4.15f1)
    pub m_UseLegacyImporter: Option<bool>,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// VideoClipImporterOutput is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoClipImporterOutput {
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub encodedEndFrame: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub encodedHeight: Option<i32>,
    /// VideoClipImporterTargetSettings: (2017.4.33f1 - 2022.2.0b16)
    pub encodedSettings: Option<VideoClipImporterTargetSettings>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub encodedStartFrame: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub encodedWidth: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b2)
    pub format: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub originalFrameCount: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub originalHeight: Option<i32>,
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub originalWidth: Option<i32>,
    /// VideoClipImporterTargetSettings: (5.6.0b2 - 5.6.0b2)
    pub settings: Option<VideoClipImporterTargetSettings>,
    /// Vec<u16>: (2017.4.33f1 - 2022.2.0b16)
    pub sourceAudioChannelCount: Option<Vec<u16>>,
    /// Vec<u32>: (2017.4.33f1 - 2022.2.0b16)
    pub sourceAudioSampleRate: Option<Vec<u32>>,
    /// u64: (2017.4.33f1 - 2022.2.0b16)
    pub sourceFileSize: Option<u64>,
    /// f64: (2017.4.33f1 - 2022.2.0b16)
    pub sourceFrameRate: Option<f64>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub sourceHasAlpha: Option<bool>,
    /// u32: (2017.4.33f1 - 2022.2.0b16)
    pub sourcePixelAspectRatioDenominator: Option<u32>,
    /// u32: (2017.4.33f1 - 2022.2.0b16)
    pub sourcePixelAspectRatioNumerator: Option<u32>,
    /// StreamedResource: (5.6.0b2 - 5.6.0b2)
    pub streamedResource: Option<StreamedResource>,
    /// bool: (2017.4.33f1 - 2022.2.0b16)
    pub transcodeSkipped: Option<bool>,
}

/// VideoClipImporterTargetSettings is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoClipImporterTargetSettings {
    pub aspectRatio: i32,
    pub bitrateMode: i32,
    pub codec: i32,
    pub customHeight: i32,
    pub customWidth: i32,
    pub enableTranscoding: bool,
    pub resizeFormat: i32,
    pub spatialQuality: i32,
}

/// VideoPlayer is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Video.VideoPlayer.html):
/**
Plays video content onto a target.
Content can be either a VideoClip imported asset or a URL such as file:// or http://. Video content will be projected onto one of the supported targets, such as camera background or RenderTexture.
If the video content includes transparency, this transparency will be present in the target, allowing objects behind the video target to be visible. When the data VideoPlayer.source is set to URL, the audio and video description of what is being played will only be initialized once the VideoPlayer preparation is completed. You can test this with VideoPlayer.isPrepared.The following demonstrates a few features of the VideoPlayer:
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoPlayer {
    /**Defines how the video content will be stretched to fill the target area.*/
    pub m_AspectRatio: i32,
    /**Destination for the audio embedded in the video.*/
    pub m_AudioOutputMode: i32,
    /**Number of audio tracks that this VideoPlayer will take control of.*/
    pub m_ControlledAudioTrackCount: u16,
    pub m_DataSource: i32,
    pub m_DirectAudioMutes: Vec<bool>,
    pub m_DirectAudioVolumes: Vec<f32>,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    pub m_EnabledAudioTracks: Vec<bool>,
    pub m_FrameReadyEventEnabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_Looping: bool,
    /**Whether the content will start playing back as soon as the component awakes.*/
    pub m_PlayOnAwake: bool,
    /**Factor by which the basic playback rate will be multiplied.*/
    pub m_PlaybackSpeed: f32,
    /**Where the video content will be drawn.*/
    pub m_RenderMode: i32,
    /**Whether the VideoPlayer is allowed to skip frames to catch up with current time.*/
    pub m_SkipOnDrop: bool,
    pub m_TargetAudioSources: Vec<PPtr /*<AudioSource>*/>,
    /**Camera component to draw to when VideoPlayer.renderMode is set to either VideoRenderMode.CameraFarPlane or VideoRenderMode.CameraNearPlane.*/
    pub m_TargetCamera: PPtr, /*<Camera>*/
    /**Overall transparency level of the target camera plane video.*/
    pub m_TargetCameraAlpha: f32,
    /**Material texture property which is targeted when VideoPlayer.renderMode is set to Video.VideoTarget.MaterialOverride.*/
    pub m_TargetMaterialProperty: String,
    /**Renderer which is targeted when VideoPlayer.renderMode is set to Video.VideoTarget.MaterialOverride*/
    pub m_TargetMaterialRenderer: PPtr, /*<Renderer>*/
    /**RenderTexture to draw to when VideoPlayer.renderMode is set to Video.VideoTarget.RenderTexture.*/
    pub m_TargetTexture: PPtr, /*<RenderTexture>*/
    /**The file or HTTP URL that the VideoPlayer reads content from.*/
    pub m_Url: String,
    pub m_VideoClip: PPtr, /*<VideoClip>*/
    /**Determines whether the VideoPlayer will wait for the first frame to be loaded into the texture before starting playback when VideoPlayer.playOnAwake is on.*/
    pub m_WaitForFirstFrame: bool,
    /**Type of 3D content contained in the source video media.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_TargetCamera3DLayout: Option<i32>,
    /// String: (5.6.0b2 - 5.6.0b2)
    pub m_TargetMaterialName: Option<String>,
    /**The clock that the VideoPlayer observes to detect and correct drift.*/
    /// i32: (2017.4.33f1 - 2022.2.0b16)
    pub m_TimeReference: Option<i32>,
    /**The clock source used by the VideoPlayer to derive its current time.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_TimeUpdateMode: Option<i32>,
    /// Vec<PPtr/*<Shader>*/>: (2020.1.0a20 - 2022.2.0b16)
    pub m_VideoShaders: Option<Vec<PPtr /*<Shader>*/>>,
}

/// VisualEffect is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/VFX.VisualEffect.html):
/**
The visual effect class that references an VisualEffectAsset instance within the Scene.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffect {
    pub m_Asset: PPtr, /*<VisualEffectAsset>*/
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_PropertySheet: VFXPropertySheetSerializedBase,
    /**This property controls whether the visual effect generates a new seed for the random number generator with each call to VisualEffect.Play function.*/
    pub m_ResetSeedOnPlay: Enum_bool__u8,
    /**The initial seed used for internal random number generator.*/
    pub m_StartSeed: u32,
    /// u8: (2022.2.0b16 - 2022.2.0b16)
    pub m_AllowInstancing: Option<u8>,
    /**The default event name. Unity calls this event when the VisualEffect awakes, or when you call VisualEffect.Reinit.*/
    /// String: (2019.3.0f4 - 2022.2.0b16)
    pub m_InitialEventName: Option<String>,
    /// u8: (2019.3.0f4 - 2022.2.0b16)
    pub m_InitialEventNameOverriden: Option<u8>,
}

/// VisualEffectAsset is a  class of the Unity engine since version 2018.4.15f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/VFX.VisualEffectAsset.html):
/**
This class contains a graph of the elements needed to describe a visual effect. These include: the visual effects system, generated shaders, and compiled data.
See Also: VisualEffect.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectAsset {
    pub m_Infos: VisualEffectInfo,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Systems: Vec<VFXSystemDesc>,
}

/// VisualEffectImporter is a  class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr /*<Object>*/)>,
    pub m_Name: String,
    pub m_UserData: String,
    /// Vec<i64>: (2019.3.0f4 - 2022.2.0b16)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// VisualEffectInfo is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectInfo {
    pub m_Buffers: Vec<VFXGPUBufferDesc>,
    pub m_CPUBuffers: Vec<VFXCPUBufferDesc>,
    pub m_CullingFlags: i32,
    pub m_Events: Vec<VFXEventDesc>,
    pub m_ExposedExpressions: Vec<VFXMapping>,
    pub m_Expressions: VFXExpressionContainer,
    pub m_PropertySheet: VFXPropertySheetSerializedBase,
    pub m_RendererSettings: VFXRendererSettings,
    pub m_RuntimeVersion: u32,
    pub m_UpdateMode: i32,
    /// u32: (2021.2.16f1 - 2022.2.0b16)
    pub m_CompilationVersion: Option<u32>,
    /// String: (2019.3.0f4 - 2022.2.0b16)
    pub m_InitialEventName: Option<String>,
    /// u32: (2022.2.0b16 - 2022.2.0b16)
    pub m_InstancingCapacity: Option<u32>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_InstancingDisabledReason: Option<i32>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_InstancingMode: Option<i32>,
    /// f32: (2019.3.0f4 - 2022.2.0b16)
    pub m_PreWarmDeltaTime: Option<f32>,
    /// u32: (2019.3.0f4 - 2022.2.0b16)
    pub m_PreWarmStepCount: Option<u32>,
    /// Vec<VFXTemporaryGPUBufferDesc>: (2019.3.0f4 - 2022.2.0b16)
    pub m_TemporaryBuffers: Option<Vec<VFXTemporaryGPUBufferDesc>>,
}

/// VisualEffectResource is a  class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectResource {
    pub m_Graph: PPtr, /*<MonoBehaviour>*/
    pub m_Infos: Enum_VisualEffectInfo__VisualEffectSettings,
    pub m_Name: String,
    /// Vec<VFXShaderSourceDesc>: (2018.4.15f1 - 2019.3.0f4)
    pub m_ShaderSources: Option<Vec<VFXShaderSourceDesc>>,
    /// Vec<VFXEditorSystemDesc>: (2018.4.15f1 - 2019.3.0f4)
    pub m_Systems: Option<Vec<VFXEditorSystemDesc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_VisualEffectInfo__VisualEffectSettings {
    VisualEffectInfo(VisualEffectInfo),
    VisualEffectSettings(VisualEffectSettings),
}

/// VisualEffectSettings is a sub class of the Unity engine since version 2020.1.0a20.
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectSettings {
    pub m_CullingFlags: i32,
    pub m_InitialEventName: String,
    pub m_PreWarmDeltaTime: f32,
    pub m_PreWarmStepCount: u32,
    pub m_RendererSettings: VFXRendererSettings,
    pub m_UpdateMode: i32,
    /// u32: (2022.2.0b16 - 2022.2.0b16)
    pub m_InstancingCapacity: Option<u32>,
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_InstancingMode: Option<i32>,
}

/// VisualEffectSubgraphBlock is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectSubgraphBlock {
    pub m_Name: String,
}

/// VisualEffectSubgraphOperator is a  class of the Unity engine since version 2019.3.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectSubgraphOperator {
    pub m_Name: String,
}

/// WheelCollider is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/WheelCollider.html):
/**
A special collider for vehicle wheels.
Wheel collider is used to model vehicle wheels. It simulates a spring and damper suspension setup,

and uses a slip based tire friction model to calculate wheel contact forces.Wheel's collision detection is performed by casting a ray from center downwards the local

y-axis. The wheel has a radius and can extend downwards by suspensionDistance

amount.The wheel is controlled with motorTorque, brakeTorque and steerAngle properties.Wheel collider computes friction separately from the rest of physics engine, using a slip based

friction model. This allows for more realistic behaviour, but makes

wheel colliders ignore standard PhysicMaterial settings. Simulation of different road materials

is done by changing the forwardFriction and sidewaysFriction

based on what material the wheel is hitting. See Also: GetGroundHit and WheelFrictionCurve.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct WheelCollider {
    /**The center of the wheel, measured in the object's local space.*/
    pub m_Center: Vector3f,
    /**Properties of tire friction in the direction the wheel is pointing in.*/
    pub m_ForwardFriction: WheelFrictionCurve,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**The mass of the wheel, expressed in kilograms. Must be larger than zero. Typical values would be in range (20,80).*/
    pub m_Mass: f32,
    /**The radius of the wheel, measured in local space.*/
    pub m_Radius: f32,
    /**Properties of tire friction in the sideways direction.*/
    pub m_SidewaysFriction: WheelFrictionCurve,
    /**Maximum extension distance of wheel suspension, measured in local space.*/
    pub m_SuspensionDistance: f32,
    /**The parameters of wheel's suspension. The suspension attempts to reach a target position by applying a linear force and a damping force.*/
    pub m_SuspensionSpring: JointSpring,
    /**Enabled Colliders will collide with other Colliders, disabled Colliders won't.*/
    /// bool: (5.6.0b2 - 2022.2.0b16)
    pub m_Enabled: Option<bool>,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_ExcludeLayers: Option<BitField>,
    /**Application point of the suspension and tire forces measured from the base of the resting wheel.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_ForceAppPointDistance: Option<f32>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b16 - 2022.2.0b16)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_LayerOverridePriority: Option<i32>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b16 - 2022.2.0b16)
    pub m_ProvidesContacts: Option<bool>,
    /**The damping rate of the wheel. Must be larger than zero.*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_WheelDampingRate: Option<f32>,
}

/// WheelFrictionCurve is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/WheelFrictionCurve.html):
/**
WheelFrictionCurve is used by the WheelCollider to describe friction properties of the wheel tire.
The curve takes a measure of tire slip as an input and gives a force as output. The curve is approximated by
a two-piece spline. The first section goes from (0,0) to (extremumSlip,extremumValue), at which
point the curve's tangent is zero. The second section goes from (extremumSlip,extremumValue)
to (asymptoteSlip,asymptoteValue), where curve's tangent is again zero:Wheel collider computes friction separately from the rest of physics engine, using a slip based
friction model. It separates the overall friction force into a "forwards" component (in the
direction of rolling, and responsible for acceleration and braking) and "sideways" component
(orthogonal to rolling, responsible for keeping the car oriented). Tire friction is described
separately in these directions using WheelCollider.forwardFriction and WheelCollider.sidewaysFriction.
In both directions it is first determined how much the tire is slipping (what is the speed difference between
the rubber and the road). Then this slip value is used to find out tire force exerted on the contact.The property of real tires is that for low slip they can exert high forces as the rubber compensates
for the slip by stretching. Later when the slip gets really high, the forces are reduced as the tire
starts to slide or spin. Thus tire friction curves have a shape like in the image above.Because the friction for the tires is computed separately, the PhysicMaterial of the ground
does not affect the wheels. Simulation of different road materials is done by changing
the WheelCollider.forwardFriction and WheelCollider.sidewaysFriction of the wheel,
based on what material the wheel is hitting.See Also: WheelCollider, WheelCollider.forwardFriction, WheelCollider.sidewaysFriction.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct WheelFrictionCurve {
    /**Asymptote point slip (default 2).*/
    /// f32: (3.4.0 - 3.4.0)
    pub asymptoteSlip: Option<f32>,
    /**Force at the asymptote slip (default 10000).*/
    /// f32: (3.4.0 - 3.4.0)
    pub asymptoteValue: Option<f32>,
    /**Extremum point slip (default 1).*/
    /// f32: (3.4.0 - 3.4.0)
    pub extremumSlip: Option<f32>,
    /**Force at the extremum slip (default 20000).*/
    /// f32: (3.4.0 - 3.4.0)
    pub extremumValue: Option<f32>,
    /**Asymptote point slip (default 2).*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_AsymptoteSlip: Option<f32>,
    /**Force at the asymptote slip (default 10000).*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_AsymptoteValue: Option<f32>,
    /**Extremum point slip (default 1).*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_ExtremumSlip: Option<f32>,
    /**Force at the extremum slip (default 20000).*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_ExtremumValue: Option<f32>,
    /**Multiplier for the extremumValue and asymptoteValue values (default 1).*/
    /// f32: (5.6.0b2 - 2022.2.0b16)
    pub m_Stiffness: Option<f32>,
    /// f32: (3.4.0 - 3.4.0)
    pub stiffnessFactor: Option<f32>,
}

/// WheelJoint2D is a  class of the Unity engine since version 5.6.0b2.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/WheelJoint2D.html):
/**
The wheel joint allows the simulation of wheels by providing a constraining suspension motion with an optional motor.
See Also: JointSuspension2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct WheelJoint2D {
    /**The joint's anchor point on the object that has the joint component.*/
    pub m_Anchor: Vector2f,
    /**Should the connectedAnchor be calculated automatically?*/
    pub m_AutoConfigureConnectedAnchor: bool,
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break.*/
    pub m_BreakTorque: f32,
    /**The joint's anchor point on the second object (ie, the one which doesn't have the joint component).*/
    pub m_ConnectedAnchor: Vector2f,
    pub m_ConnectedRigidBody: PPtr, /*<Rigidbody2D>*/
    /**Should the two rigid bodies connected with this joint collide with each other?*/
    pub m_EnableCollision: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Parameters for a motor force that is applied automatically to the Rigibody2D along the line.*/
    pub m_Motor: JointMotor2D,
    /**Set the joint suspension configuration.*/
    pub m_Suspension: JointSuspension2D,
    /**Should a motor force be applied automatically to the Rigidbody2D?*/
    pub m_UseMotor: bool,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b16 - 2022.2.0b16)
    pub m_BreakAction: Option<i32>,
}

/// WindZone is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/WindZone.html):
/**
Wind Zones add realism to the trees you create by making them wave their branches and leaves as if blown by the wind.
Note: This only works with trees created by the tree creator or imported from SpeedTree Modeler.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct WindZone {
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    pub m_GameObject: PPtr, /*<GameObject>*/
    /**Defines the type of wind zone to be used (Spherical or Directional).*/
    pub m_Mode: i32,
    /**Radius of the Spherical Wind Zone (only active if the WindZoneMode is set to Spherical).*/
    pub m_Radius: f32,
    /**The primary wind force.*/
    pub m_WindMain: f32,
    /**Defines the frequency of the wind changes.*/
    pub m_WindPulseFrequency: f32,
    /**Defines how much the wind changes over time.*/
    pub m_WindPulseMagnitude: f32,
    /**The turbulence wind force.*/
    pub m_WindTurbulence: f32,
}

/// WorldAnchor is a  class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorldAnchor {
    pub m_GameObject: PPtr, /*<GameObject>*/
}

/// WorldParticleCollider is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorldParticleCollider {
    pub m_BounceFactor: f32,
    pub m_CollidesWith: BitField,
    pub m_CollisionEnergyLoss: f32,
    pub m_GameObject: PPtr, /*<GameObject>*/
    pub m_MinKillVelocity: f32,
    pub m_SendCollisionMessage: bool,
}

/// bitset is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct bitset {
    pub bitCount: i32,
    pub bitblocks: Vec<u8>,
}

/// float3 is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// float4 is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct float4 {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// int2_storage is a sub class of the Unity engine since version 2018.4.15f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct int2_storage {
    pub x: i32,
    pub y: i32,
}

/// int3_storage is a sub class of the Unity engine since version 2017.4.33f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct int3_storage {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// xform is a sub class of the Unity engine since version 5.6.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct xform {
    pub q: float4,
    pub s: float3,
    pub t: float3,
}
