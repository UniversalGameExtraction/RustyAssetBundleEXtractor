#![allow(warnings)]
#![cfg(feature = "generated")]
use crate::objects::PPtr;
use serde_derive::{Deserialize, Serialize};

/// AABB is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AABB {
    pub m_Center: Vector3f,
    pub m_Extent: Vector3f,
}

/// ASTCImporter is a  class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct ASTCImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_Name: String,
    pub m_UserData: String,
}

/// AddedComponent is a sub class of the Unity engine since version 2022.2.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SceneManagement.AddedComponent.html):
/**
Class with information about a component that has been added to a Prefab instance.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AddedComponent {
    /// PPtr<[`Component`]>: (2022.2.0b1 - 6000.2.0a6)
    pub addedObject: PPtr,
    pub insertIndex: i32,
    /// PPtr<[`GameObject`]>: (2022.2.0b1 - 6000.2.0a6)
    pub targetCorrespondingSourceObject: PPtr,
}

/// AddedGameObject is a sub class of the Unity engine since version 2022.1.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SceneManagement.AddedGameObject.html):
/**
Class with information about a GameObject that has been added as a child under a Prefab instance.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AddedGameObject {
    /// PPtr<[`Transform`]>: (2022.1.0b1 - 6000.2.0a6)
    pub addedObject: PPtr,
    pub insertIndex: i32,
    /// PPtr<[`Transform`]>: (2022.1.0b1 - 6000.2.0a6)
    pub targetCorrespondingSourceObject: PPtr,
}

/// AimConstraint is a  class of the Unity engine since version 2018.1.0f1.
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
    /// PPtr<[`GameObject`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /// PPtr<[`Transform`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_WorldUpObject: PPtr,
    /**The world up Vector used when the world up type is AimConstraint.WorldUpType.Vector or AimConstraint.WorldUpType.ObjectRotationUp.*/
    pub m_WorldUpVector: Vector3f,
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_Active: Option<bool>,
    /// bool: (2018.1.0f1 - 2022.1.0a9)
    pub m_IsContraintActive: Option<bool>,
}

/// AndroidAssetPackImporter is a  class of the Unity engine since version 2019.4.29f1.
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
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2019.4.29f1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    /**Get or set any user data.*/
    pub m_UserData: String,
}

/// AndroidDeviceFilterData is a sub class of the Unity engine since version 6000.0.5f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AndroidDeviceFilterData.html):
/**
Set of parameters used to define an Android device or group of Android devices.
Specify a list of parameters to identify an Android device or set of Android devices. Enter values for one or multiple parameters. The parameter values are processed using logical AND operation to check if the device properties match with all the specified values.Unity ignores the filter if all parameters are empty.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidDeviceFilterData {
    pub androidOsVersionString: String,
    pub brandName: String,
    pub deviceName: String,
    pub driverVersionString: String,
    pub productName: String,
    pub vendorName: String,
    pub vulkanApiVersionString: String,
}

/// Animation is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animation.html):
/**
The animation component is used to play back animations.
You can assign animation clips to the animation component and control playback from your script.
The animation system in Unity is weight-based and supports Animation Blending, Additive animations, Animation Mixing, Layers and full control over all aspects of playback.For an overview of animation scripting in Unity please read this introduction.AnimationState can be used to change the layer of an animation, modify playback speed, and for direct control over blending and mixing.Also Animation supports enumerators. Looping through all AnimationStates is performed like this:
Additional resources: An overview of animation scripting in Unity is here.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Animation {
    /**When enabled, the physics system uses animated transforms from GameObjects with kinematic Rigidbody components to influence other GameObjects.*/
    pub m_AnimatePhysics: bool,
    /// PPtr<[`AnimationClip`]>: (3.4.0 - 6000.2.0a6)
    pub m_Animation: PPtr,
    /// Vec<PPtr<[`AnimationClip`]>>: (3.4.0 - 6000.2.0a6)
    pub m_Animations: Vec<PPtr>,
    /**Controls culling of this Animation component.*/
    pub m_CullingType: i32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Should the default animation clip (the Animation.clip property) automatically start playing on startup?*/
    pub m_PlayAutomatically: bool,
    /**How should time beyond the playback range of the clip be treated?*/
    pub m_WrapMode: i32,
    /**Specifies the update mode of the Animation.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_UpdateMode: Option<i32>,
    /// AABB: (3.4.0 - 4.2.2)
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
    /// i32: (4.0.0 - 4.7.2)
    pub m_AnimationType: Option<i32>,
    /// AnimationClipBindingConstant: (4.3.0 - 6000.2.0a6)
    pub m_ClipBindingConstant: Option<AnimationClipBindingConstant>,
    /// Vec<Vector3Curve>: (5.3.0f1 - 6000.2.0a6)
    pub m_EulerCurves: Option<Vec<Vector3Curve>>,
    /**Returns true if the Animation has animation on the root transform.*/
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_HasGenericRootTransform: Option<bool>,
    /**Returns true if the AnimationClip has editor curves for its root motion.*/
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_HasMotionFloatCurves: Option<bool>,
    /**Set to true if the AnimationClip will be used with the Legacy Animation component ( instead of the Animator ).*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_Legacy: Option<bool>,
    /// ClipMuscleConstant: (4.0.0 - 6000.2.0a6)
    pub m_MuscleClip: Option<ClipMuscleConstant>,
    /// u32: (4.0.0 - 6000.2.0a6)
    pub m_MuscleClipSize: Option<u32>,
    /// Vec<PPtrCurve>: (4.3.0 - 6000.2.0a6)
    pub m_PPtrCurves: Option<Vec<PPtrCurve>>,
    /// bool: (4.3.0 - 6000.2.0a6)
    pub m_UseHighQualityCurve: Option<bool>,
}

/// AnimationClipBindingConstant is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationClipBindingConstant {
    pub genericBindings: Vec<GenericBinding>,
    /// Vec<PPtr<[`Object`]>>: (4.3.0 - 6000.2.0a6)
    pub pptrCurveMapping: Vec<PPtr>,
}

/// AnimationClipOverride is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationClipOverride {
    /// PPtr<[`AnimationClip`]>: (4.3.0 - 6000.2.0a6)
    pub m_OriginalClip: PPtr,
    /// PPtr<[`AnimationClip`]>: (4.3.0 - 6000.2.0a6)
    pub m_OverrideClip: PPtr,
}

/// AnimationCurve is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AnimationCurve.html):
/**
Represents the variation of a value over time. AnimationCurves are typically used to animate the value of Component properties in AnimationClip, but you can use them to dynamically drive any float value.
The AnimationCurve class is a core component of Unity's Animation System.
To construct a simple AnimationCurve through code, use one of the following static utility methods:
- Use AnimationCurve.EaseInOut for a curve that smoothly transitions from one value to another.
- Use AnimationCurve.Linear for a curve that linearly transitions from one value to another.
- Use AnimationCurve.Constant for a curve that holds a constant value over its duration.
To construct a complex AnimationCurve, use AnimationCurve.AnimationCurve and AnimationCurve.AddKey.Once constructed, use an AnimationCurve to animate the following properties:
- The GameObject position, rotation, scale, or component properties in an AnimationClip.
- The properties of ParticleSystem or VisualEffect.
- The properties of your own MonoBehaviour over time.
The following example illustrates how to use an AnimationCurve to gradually change the intensity of a Light.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationCurve {
    pub m_Curve: Vec<Keyframe>,
    pub m_PostInfinity: i32,
    pub m_PreInfinity: i32,
    /// i32: (5.3.0f1 - 6000.2.0a6)
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
    /// PPtr<[`Object`]>: (3.4.0 - 6000.2.0a6)
    pub objectReferenceParameter: PPtr,
    /**The time at which the event will be fired off.*/
    pub time: f32,
}

/// AnimationManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationManager {}

/// Animator is a  class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animator.html):
/**
Interface to control the Mecanim animation system.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Animator {
    /**Should root motion be applied?*/
    pub m_ApplyRootMotion: bool,
    /**Gets/Sets the current Avatar.*/
    /// PPtr<[`Avatar`]>: (4.0.0 - 6000.2.0a6)
    pub m_Avatar: PPtr,
    /// PPtr<[`AnimatorController`]>: (4.0.0 - 4.2.2); PPtr<[`RuntimeAnimatorController`]>: (4.3.0 - 6000.2.0a6)
    pub m_Controller: PPtr,
    /**Controls culling of this Animator component.*/
    pub m_CullingMode: i32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.0.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /// bool: (4.5.3 - 6000.2.0a6)
    pub m_AllowConstantClipSamplingOptimization: Option<bool>,
    /**When enabled, the physics system uses animated transforms from GameObjects with kinematic Rigidbody components to influence other GameObjects.*/
    /// bool: (4.0.0 - 6000.2.0a6)
    pub m_AnimatePhysics: Option<bool>,
    /**Returns true if the object has a transform hierarchy.*/
    /// bool: (4.3.0 - 6000.2.0a6)
    pub m_HasTransformHierarchy: Option<bool>,
    /// bool: (2018.1.0f1 - 2023.1.0a6)
    pub m_KeepAnimatorControllerStateOnDisable: Option<bool>,
    /**Controls the behaviour of the Animator component when a GameObject is inactive.*/
    /// bool: (2020.3.43f1 - 6000.2.0a6)
    pub m_KeepAnimatorStateOnDisable: Option<bool>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_LinearVelocityBlending: Option<bool>,
    /**Automatic stabilization of feet during transition and blending.*/
    /// bool: (2020.3.43f1 - 6000.2.0a6)
    pub m_StabilizeFeet: Option<bool>,
    /**Specifies the update mode of the Animator.*/
    /// i32: (4.5.0 - 6000.2.0a6)
    pub m_UpdateMode: Option<i32>,
    /**Specifies whether playable graph values are reset or preserved when the Animator is disabled.*/
    /// bool: (2020.3.46f1 - 6000.2.0a6)
    pub m_WriteDefaultValuesOnDisable: Option<bool>,
}

/// AnimatorCondition is a sub class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AnimatorCondition.html):
/**
Condition that determines if a transition is taken.
Animator conditions represent a comparison between an animator parameter and a threshold value.
When a condition is assigned to a transition, the animator evaluates the condition to determine if the transition is taken.
When the condition is true, the transition is taken.A condition is made of three parts: a comparison mode, a parameter name, and a threshold. The parameter is compared to the threshold using the comparison. The parameter is always to the left of the comparison.
For example, a condition with a comparison of Greater evaluates to true if the parameter is greater than the threshold.Not all parameter types are compatible with every comparison modes. If you attempt to use a parameter type with an incompatible comparison mode, an error occurs. Parameters types and their compatible comparison modes are as follows:
- For float parameters, the Greater and Less modes are compatible.
- For int parameters, the Greater, Less, Equals, and NotEquals modes are compatible.
- For boolean parameters, the If and IfNot modes are compatible.
- For trigger parameters, the If mode is compatible.
Note that when the comparison mode is If or IfNot, the threshold value is ignored.The following example adds a menu that creates a state machine in the Editor. It uses animator conditions to control when a transition is taken.
Additional resources: AnimatorStateMachine, AnimatorTransition.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorCondition {
    pub m_ConditionEvent: String,
    pub m_ConditionMode: i32,
    pub m_EventTreshold: f32,
}

/// AnimatorController is a  class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AnimatorController.html):
/**
The Animator Controller controls animation through layers with state machines, controlled by parameters.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorController {
    /**Retrieves all AnimationClip used by the controller.*/
    /// Vec<PPtr<[`AnimationClip`]>>: (4.0.0 - 6000.2.0a6)
    pub m_AnimationClips: Vec<PPtr>,
    pub m_Controller: ControllerConstant,
    pub m_ControllerSize: u32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_TOS: Vec<(u32, String)>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_MultiThreadedStateMachine: Option<bool>,
    /// StateMachineBehaviourVectorDescription: (5.0.0f4 - 6000.2.0a6)
    pub m_StateMachineBehaviourVectorDescription: Option<StateMachineBehaviourVectorDescription>,
    /// Vec<PPtr<[`MonoBehaviour`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_StateMachineBehaviours: Option<Vec<PPtr>>,
}

/// AnimatorOverrideController is a  class of the Unity engine since version 4.3.0.
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
    /// PPtr<[`RuntimeAnimatorController`]>: (4.3.0 - 6000.2.0a6)
    pub m_Controller: PPtr,
    /**The name of the object.*/
    pub m_Name: String,
}

/// AnimatorState is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AnimatorState.html):
/**
Represents a state in a state machine.
Each state contains an (AnimationClip or a BlendTree) which plays while the character is in the state. When a state transition is triggered, the character adopts a new state. The (AnimationClip or BlendTree) for the new state takes over.The following example illustrates how to create a simple state machine using an editor script. It creates three states, "Idle", "Run", and "Jump", and transitions between them. This example does not set the Motion. You can create a new state with AnimatorController.AddMotion, or you can assign a motion to an existing state using AnimatorState.motion.
Additional resources: AnimatorController.AddMotion, AnimatorStateMachine.AddState, AnimatorStateMachine.states.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorState {
    /**Offset at which the animation loop starts. Useful for synchronizing looped animations.Units is normalized time.*/
    pub m_CycleOffset: f32,
    /**Should Foot IK be respected for this state.*/
    pub m_IKOnFeet: bool,
    /**Whether the animation state is mirrored.*/
    pub m_Mirror: bool,
    /**The motion assigned to this state.*/
    /// PPtr<[`Motion`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_Motion: PPtr,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Position: Vector3f,
    /**The default speed of the motion.*/
    pub m_Speed: f32,
    /// Vec<PPtr<[`MonoBehaviour`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_StateMachineBehaviours: Vec<PPtr>,
    /**A tag can be used to identify a state.*/
    pub m_Tag: String,
    /**The transitions that are going out of the state.*/
    /// Vec<PPtr<[`AnimatorStateTransition`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_Transitions: Vec<PPtr>,
    /**Whether or not the AnimatorStates writes back the default values for properties that are not animated by its Motion.*/
    pub m_WriteDefaultValues: bool,
    /**The animator controller parameter that drives the cycle offset value.*/
    /// String: (5.1.0f1 - 6000.2.0a6)
    pub m_CycleOffsetParameter: Option<String>,
    /**Define if the cycle offset value is driven by an Animator controller parameter or by the value set in the editor.*/
    /// bool: (5.1.0f1 - 6000.2.0a6)
    pub m_CycleOffsetParameterActive: Option<bool>,
    /**The animator controller parameter that drives the mirror value.*/
    /// String: (5.1.0f1 - 6000.2.0a6)
    pub m_MirrorParameter: Option<String>,
    /**Define if the mirror value is driven by an Animator controller parameter or by the value set in the editor.*/
    /// bool: (5.1.0f1 - 6000.2.0a6)
    pub m_MirrorParameterActive: Option<bool>,
    /**The animator controller parameter that drives the speed value.*/
    /// String: (5.1.0f1 - 6000.2.0a6)
    pub m_SpeedParameter: Option<String>,
    /**Define if the speed value is driven by an Animator controller parameter or by the value set in the editor.*/
    /// bool: (5.1.0f1 - 6000.2.0a6)
    pub m_SpeedParameterActive: Option<bool>,
    /**If timeParameterActive is true, the value of this Parameter will be used instead of normalized time.*/
    /// String: (2017.2.0f1 - 6000.2.0a6)
    pub m_TimeParameter: Option<String>,
    /**If true, use value of given Parameter as normalized time.*/
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub m_TimeParameterActive: Option<bool>,
}

/// AnimatorStateMachine is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AnimatorStateMachine.html):
/**
A graph controlling the interaction of states. Each state references a motion.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorStateMachine {
    /**The position of the AnyState node.*/
    pub m_AnyStatePosition: Vector3f,
    /**The list of AnyState transitions.*/
    /// Vec<PPtr<[`AnimatorStateTransition`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_AnyStateTransitions: Vec<PPtr>,
    pub m_ChildStateMachines: Vec<ChildAnimatorStateMachine>,
    pub m_ChildStates: Vec<ChildAnimatorState>,
    /**The state that the state machine will be in when it starts.*/
    /// PPtr<[`AnimatorState`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_DefaultState: PPtr,
    /**The position of the entry node.*/
    pub m_EntryPosition: Vector3f,
    /**The list of entry transitions in the state machine.*/
    /// Vec<PPtr<[`AnimatorTransition`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_EntryTransitions: Vec<PPtr>,
    /**The position of the exit node.*/
    pub m_ExitPosition: Vector3f,
    /**The name of the object.*/
    pub m_Name: String,
    /**The position of the parent state machine node. Only valid when in a hierachic state machine.*/
    pub m_ParentStateMachinePosition: Vector3f,
    /// Vec<PPtr<[`MonoBehaviour`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_StateMachineBehaviours: Vec<PPtr>,
    /// Vec<(PPtr<[`AnimatorStateMachine`]>, Vec<PPtr<[`AnimatorTransition`]>>)>: (5.0.0f4 - 6000.2.0a6)
    pub m_StateMachineTransitions: Vec<(PPtr, Vec<PPtr>)>,
}

/// AnimatorStateTransition is a  class of the Unity engine since version 5.0.0f4.
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
    /// PPtr<[`AnimatorState`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_DstState: PPtr,
    /// PPtr<[`AnimatorStateMachine`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_DstStateMachine: PPtr,
    /**If AnimatorStateTransition.hasExitTime is true, exitTime represents the exact time at which the transition can take effect.This is represented in normalized time, so for example an exit time of 0.75 means that on the first frame where 75% of the animation has played, the Exit Time condition will be true. On the next frame, the condition will be false.For looped animations, transitions with exit times smaller than 1 will be evaluated every loop, so you can use this to time your transition with the proper timing in the animation, every loop.Transitions with exit times greater than one will be evaluated only once, so they can be used to exit at a specific time, after a fixed number of loops. For example, a transition with an exit time of 3.5 will be evaluated once, after three and a half loops.*/
    pub m_ExitTime: f32,
    /**When active the transition will have an exit time condition.*/
    pub m_HasExitTime: bool,
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
    /**Determines whether the duration of the transition is reported in a fixed duration in seconds or as a normalized time.*/
    /// bool: (5.1.0f1 - 6000.2.0a6)
    pub m_HasFixedDuration: Option<bool>,
}

/// AnimatorTransition is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AnimatorTransition.html):
/**
Transitions define when and how the state machine switch from on state to another. AnimatorTransition always originate from a StateMachine or a StateMachine entry. They do not define timing parameters.
A transition happens when all its conditions are met.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorTransition {
    /**AnimatorCondition conditions that need to be met for a transition to happen.*/
    pub m_Conditions: Vec<AnimatorCondition>,
    /// PPtr<[`AnimatorState`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_DstState: PPtr,
    /// PPtr<[`AnimatorStateMachine`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_DstStateMachine: PPtr,
    /**Is the transition destination the exit of the current state machine.*/
    pub m_IsExit: bool,
    /**Mutes the transition. The transition will never occur.*/
    pub m_Mute: bool,
    /**The name of the object.*/
    pub m_Name: String,
    /**Mutes all other transitions in the source state.*/
    pub m_Solo: bool,
}

/// AnimatorTransitionBase is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.AnimatorTransitionBase.html):
/**
Base class for animator transitions. Transitions define when and how the state machine switches from one state to another.
A transition happens when all its conditions are met.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatorTransitionBase {
    /**AnimatorCondition conditions that need to be met for a transition to happen.*/
    pub m_Conditions: Vec<AnimatorCondition>,
    /// PPtr<[`AnimatorState`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_DstState: PPtr,
    /// PPtr<[`AnimatorStateMachine`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_DstStateMachine: PPtr,
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
    /// f32: (2021.1.0b1 - 6000.2.0a6)
    pub m_FadeGizmoSize: Option<f32>,
    /// bool: (2021.1.0b1 - 6000.2.0a6)
    pub m_FadeGizmos: Option<bool>,
    /// f32: (3.4.0 - 3.4.2)
    pub m_IconSize: Option<f32>,
    /// bool: (4.3.0 - 6000.2.0a6)
    pub m_ShowGrid: Option<bool>,
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub m_ShowSelectionOutline: Option<bool>,
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub m_ShowSelectionWire: Option<bool>,
    /// bool: (3.5.0 - 6000.2.0a6)
    pub m_Use3dGizmos: Option<bool>,
    /// f32: (3.5.0 - 6000.2.0a6)
    pub m_WorldIconSize: Option<f32>,
}

/// AreaEffector2D is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AreaEffector2D.html):
/**
Applies forces within an area.
When the source Collider2D is a trigger, the effector will apply forces whenever the target Collider2D overlaps the source.  When the source Collider isn't a trigger, the effector will apply forces whenever the target Collider2D is in contact with the source only.This effector is designed primarily to work with source Collider2D that are set as triggers so that target Collider2D can overlap the defined area.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AreaEffector2D {
    /**The mask used to select specific layers allowed to interact with the effector.*/
    pub m_ColliderMask: BitField,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The magnitude of the force to be applied.*/
    pub m_ForceMagnitude: f32,
    /**The target for where the effector applies any force.*/
    pub m_ForceTarget: i32,
    /**The variation of the magnitude of the force to be applied.*/
    pub m_ForceVariation: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The angular damping to apply to rigid-bodies.*/
    /// f32: (6000.1.0b1 - 6000.2.0a6)
    pub m_AngularDamping: Option<f32>,
    /// f32: (5.0.0f4 - 6000.1.0a5)
    pub m_AngularDrag: Option<f32>,
    /// f32: (5.0.0f4 - 6000.1.0a5)
    pub m_Drag: Option<f32>,
    /**The angle of the force to be applied.*/
    /// f32: (5.0.2f1 - 6000.2.0a6)
    pub m_ForceAngle: Option<f32>,
    /// f32: (5.0.0f4 - 5.0.1f1)
    pub m_ForceDirection: Option<f32>,
    /**The linear damping to apply to rigid-bodies.*/
    /// f32: (6000.1.0b1 - 6000.2.0a6)
    pub m_LinearDamping: Option<f32>,
    /**Should the collider-mask be used or the global collision matrix?*/
    /// bool: (5.0.2f1 - 6000.2.0a6)
    pub m_UseColliderMask: Option<bool>,
    /**Should the forceAngle use global space?*/
    /// bool: (5.0.2f1 - 6000.2.0a6)
    pub m_UseGlobalAngle: Option<bool>,
}

/// ArticulationBody is a  class of the Unity engine since version 2020.1.0b1.
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
    /// PPtr<[`GameObject`]>: (2020.1.0b1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /// Vector3f: (2022.2.0b1 - 6000.2.0a6)
    pub m_CenterOfMass: Option<Vector3f>,
    /**The ArticulationBody's collision detection mode.*/
    /// i32: (2020.3.5f1 - 6000.2.0a6)
    pub m_CollisionDetectionMode: Option<i32>,
    /// bool: (2020.1.0b1 - 2021.2.0a20)
    pub m_ComputeParentAnchor: Option<bool>,
    /**The additional layers that all Colliders attached to this ArticulationBody should exclude when deciding if the Collider can come into contact with another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_ImplicitCom: Option<bool>,
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_ImplicitTensor: Option<bool>,
    /**The additional layers that all Colliders attached to this ArticulationBody should include when deciding if a the Collider can come into contact with another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /// Quaternionf: (2022.2.0b1 - 6000.2.0a6)
    pub m_InertiaRotation: Option<Quaternionf>,
    /**The inertia tensor of this body.*/
    /// Vector3f: (2022.2.0b1 - 6000.2.0a6)
    pub m_InertiaTensor: Option<Vector3f>,
    /**Whether the parent anchor should be computed automatically or not.*/
    /// bool: (2021.2.0b1 - 6000.2.0a6)
    pub m_MatchAnchors: Option<bool>,
    /**Controls whether gravity affects this articulation body.*/
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub m_UseGravity: Option<bool>,
}

/// ArticulationDrive is a sub class of the Unity engine since version 2020.1.0b1.
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
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub driveType: Option<i32>,
}

/// AspectRatios is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AspectRatios {
    pub Others: bool,
    /// bool: (3.4.0 - 2022.1.24f1)
    #[serde(alias = "16:10")]
    pub _16_10: Option<bool>,
    /// bool: (3.4.0 - 2022.1.24f1)
    #[serde(alias = "16:9")]
    pub _16_9: Option<bool>,
    /// bool: (3.4.0 - 2022.1.24f1)
    #[serde(alias = "4:3")]
    pub _4_3: Option<bool>,
    /// bool: (3.4.0 - 2022.1.24f1)
    #[serde(alias = "5:4")]
    pub _5_4: Option<bool>,
}

/// AssemblyDefinitionAsset is a  class of the Unity engine since version 2017.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssemblyDefinitionAsset {
    pub m_Name: String,
    pub m_Script: String,
}

/// AssemblyDefinitionImporter is a  class of the Unity engine since version 2017.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssemblyDefinitionImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.3.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UserData: String,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// AssemblyDefinitionReferenceAsset is a  class of the Unity engine since version 2019.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssemblyDefinitionReferenceAsset {
    pub m_Name: String,
    pub m_Script: String,
}

/// AssemblyDefinitionReferenceImporter is a  class of the Unity engine since version 2019.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssemblyDefinitionReferenceImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2019.2.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// AssemblyJsonAsset is a  class of the Unity engine since version 2017.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssemblyJsonAsset {
    pub m_Name: String,
    pub m_Script: String,
    /// String: (2017.1.0b1 - 2017.1.0b1)
    pub m_PathName: Option<String>,
}

/// AssemblyJsonImporter is a  class of the Unity engine since version 2017.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssemblyJsonImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_Name: String,
    pub m_UserData: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0b2 - 2017.2.0b10)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
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
    /// i32: (3.4.0 - 2020.1.17f1)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// i32: (5.0.0f4 - 2020.1.17f1)
    pub assetBundleIndex: Option<i32>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub digest: Option<MdFour>,
    /// Vec<(GUID, String)>: (2018.1.0f1 - 2018.2.21f1); Vec<(String, GUID)>: (2018.3.0b1 - 2020.1.17f1)
    pub guidOfPathLocationDependencies: Option<Vec<(Enum_GUID__String, Enum_GUID__String)>>,
    /// MdFour: (4.3.0 - 4.7.2); Hash128: (5.0.0f4 - 2020.1.17f1)
    pub hash: Option<Enum_MdFour__Hash128>,
    /// Vec<GUID>: (2018.1.0f1 - 2020.1.17f1)
    pub hashOfImportedAssetDependencies: Option<Vec<GUID>>,
    /// Vec<GUID>: (2018.1.0f1 - 2020.1.17f1)
    pub hashOfSourceAssetDependencies: Option<Vec<GUID>>,
    /// i32: (3.5.0 - 2020.1.17f1)
    pub importerClassId: Option<i32>,
    /// u32: (3.5.0 - 2020.1.17f1)
    pub importerVersionHash: Option<u32>,
    /// u32: (3.4.0 - 3.4.2)
    #[serde(alias = "metaModificationDate[0]")]
    pub metaModificationDate_0_: Option<u32>,
    /// u32: (3.4.0 - 3.4.2)
    #[serde(alias = "metaModificationDate[1]")]
    pub metaModificationDate_1_: Option<u32>,
    /// u32: (3.4.0 - 3.4.2)
    #[serde(alias = "modificationDate[0]")]
    pub modificationDate_0_: Option<u32>,
    /// u32: (3.4.0 - 3.4.2)
    #[serde(alias = "modificationDate[1]")]
    pub modificationDate_1_: Option<u32>,
    /// String: (2017.2.4p1 - 2020.1.17f1)
    pub scriptedImporterClassID: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_GUID__String {
    GUID(GUID),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_MdFour__Hash128 {
    MdFour(MdFour),
    Hash128(Hash128),
}

/// AssetBundle is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AssetBundle.html):
/**
API for accessing the content of AssetBundle files.
This class exposes an API, via static methods, for loading and managing AssetBundles.This same class offers non-static methods and properties that expose the contents of a specific loaded AssetBundle, including the ability to load an Asset from within an AssetBundle.Create AssetBundles by calling BuildPipeline.BuildAssetBundles or using the Addressables package.
The build process generates one or more AssetBundle files, and each AssetBundle file contains a serialized instance of this class.Additional resources: Intro to AssetBundles, UnityWebRequestAssetBundle.GetAssetBundle, BuildPipeline.BuildAssetBundles.
Scenes inside AssetBundles
- An AssetBundle can contain scenes or assets, but not a mix of both types.
- AssetBundle.LoadAsset, and the other Load methods, do not support loading scenes from AssetBundles.
- Scenes can be loaded from AssetBundles using the SceneManager.  When running in the Player, or Play mode in the Editor, first load the AssetBundle containing scenes.  Then call SceneManager.LoadScene or SceneManager.LoadSceneAsync with the scene path or name.
- When the Editor is in Edit mode, it does not support loading scenes from AssetBundles. Calls to EditorSceneManager.OpenScene with the path of a scene inside a loaded AssetBundle fail and log an error stating that the scene file is not found.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetBundle {
    pub m_Container: Vec<(String, AssetInfo)>,
    pub m_MainAsset: AssetInfo,
    /**The name of the object.*/
    pub m_Name: String,
    /// Vec<PPtr<[`Object`]>>: (3.4.0 - 6000.2.0a6)
    pub m_PreloadTable: Vec<PPtr>,
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleName: Option<String>,
    /// Vec<(i32, u32)>: (3.5.0 - 4.7.2)
    pub m_ClassCompatibility: Option<Vec<(i32, u32)>>,
    /// Vec<(i32, i32)>: (5.4.0f3 - 5.4.6f3)
    pub m_ClassVersionMap: Option<Vec<(i32, i32)>>,
    /// Vec<String>: (5.0.0f4 - 6000.2.0a6)
    pub m_Dependencies: Option<Vec<String>>,
    /// i32: (2017.3.0b1 - 6000.2.0a6)
    pub m_ExplicitDataLayout: Option<i32>,
    /**Return true if the AssetBundle contains Unity Scene files*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_IsStreamedSceneAssetBundle: Option<bool>,
    /// i32: (2017.1.0f1 - 6000.2.0a6)
    pub m_PathFlags: Option<i32>,
    /// u32: (4.2.0 - 6000.2.0a6)
    pub m_RuntimeCompatibility: Option<u32>,
    /// Vec<(String, String)>: (2017.3.0b1 - 6000.2.0a6)
    pub m_SceneHashes: Option<Vec<(String, String)>>,
    /// Vec<AssetBundleScriptInfo>: (3.4.0 - 4.7.2)
    pub m_ScriptCompatibility: Option<Vec<AssetBundleScriptInfo>>,
}

/// AssetBundleFullName is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetBundleFullName {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
}

/// AssetBundleInfo is a sub class of the Unity engine since version 5.0.0f4.
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

/// AssetBundleManifest is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AssetBundleManifest.html):
/**
Manifest for all the AssetBundles in the build.
Additional resources: BuildPipeline.BuildAssetBundles, AssetBundle.GetAllAssetNames
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
    /// Vec<(i32, AssetBundleFullName)>: (5.0.0f4 - 5.6.7f1)
    pub m_AssetBundleNames: Option<Vec<(i32, AssetBundleFullName)>>,
    /// Vec<(String, AssetTimeStamp)>: (3.5.0 - 5.6.7f1)
    pub m_AssetTimeStamps: Option<Vec<(String, AssetTimeStamp)>>,
    /// AssetDatabaseMetrics: (5.0.0f4 - 5.6.7f1)
    pub m_Metrics: Option<AssetDatabaseMetrics>,
    /// i32: (3.5.0 - 5.6.7f1)
    pub m_UnityShadersVersion: Option<i32>,
    /// Vec<(i32, u32)>: (4.0.0 - 5.6.7f1)
    pub m_lastValidVersionHashes: Option<Vec<(i32, u32)>>,
}

/// AssetDatabaseMetrics is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetDatabaseMetrics {
    pub totalAssetCount: i32,
    /// i32: (5.0.0f4 - 2017.4.5f1)
    pub nonProAssetCount: Option<i32>,
    /// i32: (5.0.0f4 - 2017.4.5f1)
    pub nonProAssetsCreatedAfterProLicense: Option<i32>,
}

/// AssetDatabaseV1 is a  class of the Unity engine since version 2017.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetDatabaseV1 {
    pub m_AssetBundleNames: Vec<(i32, AssetBundleFullName)>,
    pub m_AssetTimeStamps: Vec<(String, AssetTimeStamp)>,
    pub m_Assets: Vec<(GUID, Asset)>,
    pub m_Metrics: AssetDatabaseMetrics,
    pub m_UnityShadersVersion: i32,
    /// Vec<(i32, u32)>: (2017.1.0b1 - 2017.1.0b1)
    pub m_lastValidVersionHashes: Option<Vec<(i32, u32)>>,
    /// Vec<(AssetImporterHashKey, u32)>: (2017.1.0f1 - 2020.1.17f1)
    pub m_lastValidVersions: Option<Vec<(AssetImporterHashKey, u32)>>,
}

/// AssetImporter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AssetImporter.html):
/**
Provides access to import settings and base functionality for all asset types.
Although this is the base class for all asset importers, the recommended best practice is that you derive from the ScriptedImporter class if you want to write a new, custom importer.Each asset imported into the project is linked to a corresponding asset importer object. This object provides access to the settings applied during the asset import process. These settings are stored in the .meta file, are located adjacent to the source asset file. They encompass asset bundle information, custom user data, and any external objects upon which the asset relies.To obtain the asset importer object associated with an asset, use the AssetImporter.GetAtPath method.To apply and save any changes made to the settings, use the AssetImporter.SaveAndReimport method. This action reimports the asset with the updated configuration.The following example iterates through all assets within the project, identifying those that lack an asset bundle name by examining their respective asset importer objects.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetImporter {
    /**The name of the object.*/
    pub m_Name: String,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.2)
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_NewHashIdentity: Option<MdFour>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_OldHashIdentity: Option<MdFour>,
}

/// AssetImporterHashKey is a sub class of the Unity engine since version 2017.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetImporterHashKey {
    pub ScriptClass: String,
    /// i32: (2017.1.0f1 - 2020.1.17f1)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// AssetImporterLog is a  class of the Unity engine since version 2018.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetImporterLog {
    pub m_Logs: Vec<AssetImporter_ImportError>,
    pub m_Name: String,
}

/// AssetImporter_ImportError is a sub class of the Unity engine since version 2018.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetImporter_ImportError {
    pub error: String,
    pub file: String,
    pub line: i32,
    pub mode: i32,
    /// PPtr<[`Object`]>: (2018.1.0f1 - 2022.2.0a13)
    pub object: PPtr,
}

/// AssetInfo is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetInfo {
    /// PPtr<[`Object`]>: (3.4.0 - 6000.2.0a6)
    pub asset: PPtr,
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
    /// i32: (5.0.0f4 - 2017.4.5f1)
    pub licenseType: Option<i32>,
    /// u32: (3.4.0 - 5.6.7f1)
    pub originalChangeset: Option<u32>,
    /// MdFour: (3.4.0 - 4.7.2); Hash128: (5.0.0f4 - 5.6.7f1)
    pub originalDigest: Option<Enum_MdFour__Hash128>,
    /// GUID: (3.4.0 - 5.6.7f1)
    pub originalParent: Option<GUID>,
    /// u64: (5.0.0f4 - 2017.4.5f1)
    pub timeCreated: Option<u64>,
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

/// AssetTimeStamp is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetTimeStamp {
    /// u32: (3.5.0 - 2020.1.17f1)
    #[serde(alias = "metaModificationDate[0]")]
    pub metaModificationDate_0_: Option<u32>,
    /// u32: (3.5.0 - 2020.1.17f1)
    #[serde(alias = "metaModificationDate[1]")]
    pub metaModificationDate_1_: Option<u32>,
    /// u32: (3.5.0 - 2020.1.17f1)
    #[serde(alias = "modificationDate[0]")]
    pub modificationDate_0_: Option<u32>,
    /// u32: (3.5.0 - 2020.1.17f1)
    #[serde(alias = "modificationDate[1]")]
    pub modificationDate_1_: Option<u32>,
}

/// AttachmentIndexArray is a sub class of the Unity engine since version 6000.0.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rendering.AttachmentIndexArray.html):
/**
Struct encapsulating a fixed list of attachment indices used when declaring native render passes.
This fixed size struct allows for non-GC declaration of the required indices.
Indices point into the attachment list as pass to CommandBuffer.BeginRenderPassAdditional resources: CommandBuffer.BeginRenderPass.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentIndexArray {
    pub activeAttachments: i32,
    pub attachments: Vec<i32>,
}

/// AttachmentInfo is a sub class of the Unity engine since version 6000.0.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentInfo {
    pub format: i32,
    pub needsResolve: bool,
}

/// AudioBuildInfo is a  class of the Unity engine since version 2018.4.13f1.
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
The chorus effect modulates the original sound by a sinusoid low frequency oscillator (LFO). The output sounds like there are multiple sources emitting the same sound with slight variations (resembling a choir).Additional resources: Audio Chorus Filter information.
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Chorus modulation rate in hz. 0.0 to 20.0. Default = 0.8 hz.*/
    pub m_Rate: f32,
    /**Volume of 1st chorus tap. 0.0 to 1.0. Default = 0.5.*/
    pub m_WetMix1: f32,
    /**Volume of 2nd chorus tap. This tap is 90 degrees out of phase of the first tap. 0.0 to 1.0. Default = 0.5.*/
    pub m_WetMix2: f32,
    /**Volume of 3rd chorus tap. This tap is 90 degrees out of phase of the second tap. 0.0 to 1.0. Default = 0.5.*/
    pub m_WetMix3: f32,
    /// f32: (3.4.0 - 4.1.5)
    pub m_FeedBack: Option<f32>,
}

/// AudioClip is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioClip.html):
/**
A container for audio data.
An AudioClip stores the audio file either compressed as ogg vorbis or uncompressed.
AudioClips are referenced and used by AudioSources to play sounds.Additional resources: AudioClip component in the Components Reference.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioClip {
    /**The name of the object.*/
    pub m_Name: String,
    /// bool: (3.4.0 - 4.7.2)
    pub m_3D: Option<bool>,
    /**Returns true if this audio clip is ambisonic (read-only).*/
    /// bool: (2017.1.0b1 - 6000.2.0a6)
    pub m_Ambisonic: Option<bool>,
    /// Vec<u8>: (3.4.0 - 4.7.2)
    pub m_AudioData: Option<Vec<u8>>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_BitsPerSample: Option<i32>,
    /**The number of channels in the audio clip. (Read Only)*/
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_Channels: Option<i32>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_CompressionFormat: Option<i32>,
    /// i32: (3.4.0 - 4.7.2)
    pub m_Format: Option<i32>,
    /**The sample frequency of the clip in Hertz. (Read Only)*/
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_Frequency: Option<i32>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_IsTrackerFormat: Option<bool>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_Legacy3D: Option<bool>,
    /**The length of the audio clip in seconds. (Read Only)*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_Length: Option<f32>,
    /**Enable this property to load the AudioClip asynchronously in the background instead of on the main thread. Set this property in the Inspector (Read Only).*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_LoadInBackground: Option<bool>,
    /**The load type of the clip (read-only).*/
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_LoadType: Option<i32>,
    /**Enable this property in the Inspector to preload audio data from the audio clip when loading the clip Asset (Read Only).*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_PreloadAudioData: Option<bool>,
    /// StreamedResource: (5.0.0f4 - 6000.2.0a6)
    pub m_Resource: Option<StreamedResource>,
    /// i32: (3.4.0 - 4.7.2)
    pub m_Stream: Option<i32>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SubsoundIndex: Option<i32>,
    /// i32: (3.4.0 - 4.7.2)
    pub m_Type: Option<i32>,
    /// bool: (3.4.0 - 4.7.2)
    pub m_UseHardware: Option<bool>,
}

/// AudioContainerElement is a  class of the Unity engine since version 2023.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioContainerElement {
    /// PPtr<[`AudioClip`]>: (2023.2.0b1 - 6000.2.0a6)
    pub m_AudioClip: PPtr,
    pub m_Enabled: bool,
    pub m_Name: String,
    pub m_Volume: f32,
}

/// AudioDistortionFilter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioDistortionFilter.html):
/**
The Audio Distortion Filter distorts the sound from an AudioSource or sounds reaching the AudioListener.
Additional resources: Audio Distortion Filter information.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioDistortionFilter {
    /**Distortion value. 0.0 to 1.0. Default = 0.5.*/
    pub m_DistortionLevel: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
}

/// AudioEchoFilter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioEchoFilter.html):
/**
The Audio Echo Filter repeats a sound after a given Delay, attenuating the repetitions based on the Decay Ratio.
Additional resources: Audio Echo Filter information.
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
Additional resources: Audio High Pass Filter information.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioHighPassFilter {
    /**Highpass cutoff frequency in hz. 10.0 to 22000.0. Default = 5000.0.*/
    pub m_CutoffFrequency: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Determines how much the filter's self-resonance isdampened.*/
    pub m_HighpassResonanceQ: f32,
}

/// AudioImporter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioImporter.html):
/**
Use this class to modify AudioClip import settings from editor scripts.
Use these settings to ensure all or a subset of your audio files have the same import settings. You can also edit these settings in the Audio Clip Inspector. Additional resources: AudioClip
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioImporter {
    pub m_3D: bool,
    /**Force audioclips to mono sound type, so all audio plays through a single channel.*/
    pub m_ForceToMono: bool,
    /**The name of the object.*/
    pub m_Name: String,
    /// Vec<u8>: (3.4.0 - 3.5.7)
    #[serde(alias = "audio preview data")]
    pub audio_preview_data: Option<Vec<u8>>,
    /**Enable this property to treat the audio clip as ambisonic.*/
    /// bool: (2017.1.0b1 - 6000.2.0a6)
    pub m_Ambisonic: Option<bool>,
    /**Get or set the AssetBundle name.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleName: Option<String>,
    /**Get or set the AssetBundle variant.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleVariant: Option<String>,
    /// SampleSettings: (5.0.0f4 - 6000.2.0a6)
    pub m_DefaultSettings: Option<SampleSettings>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.2)
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    /// i32: (3.4.0 - 4.7.2)
    pub m_Format: Option<i32>,
    /**Corresponding to the "Load In Background" flag in the AudioClip inspector, when this flag is set, the loading of the clip will happen delayed without blocking the main thread.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_LoadInBackground: Option<bool>,
    /// bool: (3.4.0 - 4.7.2)
    pub m_Loopable: Option<bool>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_NewHashIdentity: Option<MdFour>,
    /// bool: (5.1.2f1 - 6000.2.0a6)
    pub m_Normalize: Option<bool>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_OldHashIdentity: Option<MdFour>,
    /// Output: (4.0.0 - 4.7.2); AudioImporterOutput: (5.0.0f4 - 6000.2.0a6)
    pub m_Output: Option<Enum_Output__AudioImporterOutput>,
    /// Vec<(i32, SampleSettings)>: (5.0.0f4 - 2023.3.0a8); Vec<(String, SampleSettings)>: (2023.3.0b1 - 6000.2.0a6)
    pub m_PlatformSettingOverrides: Option<Vec<(Enum_i32__String, SampleSettings)>>,
    /// bool: (5.0.0f4 - 2022.2.0a16)
    pub m_PreloadAudioData: Option<bool>,
    /// PreviewData: (5.0.0f4 - 6000.2.0a6)
    pub m_PreviewData: Option<PreviewData>,
    /// u32: (3.4.0 - 3.5.7)
    pub m_PreviewDataLength: Option<u32>,
    /// f32: (3.4.0 - 4.7.2)
    pub m_Quality: Option<f32>,
    /// i32: (3.4.0 - 4.7.2)
    pub m_Stream: Option<i32>,
    /// bool: (3.4.0 - 4.7.2)
    pub m_UseHardware: Option<bool>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /**Get or set any user data.*/
    /// String: (4.0.0 - 6000.2.0a6)
    pub m_UserData: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_Output__AudioImporterOutput {
    Output(Output),
    AudioImporterOutput(AudioImporterOutput),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_i32__String {
    i32(i32),
    String(String),
}

/// AudioImporterOutput is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioImporterOutput {
    pub editorOutputContainerFormat: i32,
    pub editorOutputSettings: SampleSettings,
    pub outputContainerFormat: i32,
    pub outputSettings: SampleSettings,
    /// StreamedResource: (5.0.0f4 - 2017.4.40f1)
    pub playerResource: Option<StreamedResource>,
}

/// AudioListener is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioListener.html):
/**
Representation of a listener in 3D space.
This class implements a microphone-like device. It records the sounds around it and plays that through the player's speakers.
You can only have one listener in a Scene.Additional resources: AudioSource, AudioListener component in the Components Reference.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioListener {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /// Vec<ExtensionPropertyValue>: (2017.2.0b2 - 2017.2.0b11)
    pub m_ExtensionPropertyValues: Option<Vec<ExtensionPropertyValue>>,
}

/// AudioLowPassFilter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioLowPassFilter.html):
/**
The Audio Low Pass Filter passes low frequencies of an AudioSource or all sounds reaching an AudioListener, while removing frequencies higher than the Cutoff Frequency.
Additional resources: Audio Low Pass Filter information.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioLowPassFilter {
    pub lowpassLevelCustomCurve: AnimationCurve,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Determines how much the filter's self-resonance is dampened.*/
    pub m_LowpassResonanceQ: f32,
    /**Lowpass cutoff frequency in hz. 10.0 to 22000.0. Default = 5000.0.*/
    /// f32: (3.4.0 - 5.1.5f1)
    pub m_CutoffFrequency: Option<f32>,
}

/// AudioManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioManager {
    pub m_DSPBufferSize: i32,
    pub m_Volume: f32,
    /// i32: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "Default Speaker Mode")]
    pub Default_Speaker_Mode: Option<i32>,
    /// f32: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "Doppler Factor")]
    pub Doppler_Factor: Option<f32>,
    /// f32: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "Rolloff Scale")]
    pub Rolloff_Scale: Option<f32>,
    /// String: (2017.1.0b1 - 6000.2.0a6)
    pub m_AmbisonicDecoderPlugin: Option<String>,
    /// bool: (4.2.0 - 6000.2.0a6)
    pub m_DisableAudio: Option<bool>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_RealVoiceCount: Option<i32>,
    /// i32: (2018.3.14f1 - 6000.2.0a6)
    pub m_RequestedDSPBufferSize: Option<i32>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SampleRate: Option<i32>,
    /// String: (5.2.0f2 - 6000.2.0a6)
    pub m_SpatializerPlugin: Option<String>,
    /// f32: (3.4.0 - 4.7.2)
    pub m_SpeedOfSound: Option<f32>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_VirtualVoiceCount: Option<i32>,
    /// bool: (5.3.6f1 - 6000.2.0a6)
    pub m_VirtualizeEffects: Option<bool>,
}

/// AudioMixer is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Audio.AudioMixer.html):
/**
AudioMixer asset.
This is a singleton representing a specific audio mixer asset in the project.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixer {
    pub m_EnableSuspend: bool,
    /// PPtr<[`AudioMixerGroup`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_MasterGroup: PPtr,
    pub m_MixerConstant: AudioMixerConstant,
    /**The name of the object.*/
    pub m_Name: String,
    /// PPtr<[`AudioMixerGroup`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_OutputGroup: PPtr,
    /// Vec<PPtr<[`AudioMixerSnapshot`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_Snapshots: Vec<PPtr>,
    /// PPtr<[`AudioMixerSnapshot`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_StartSnapshot: PPtr,
    pub m_SuspendThreshold: f32,
    /**How time should progress for this AudioMixer. Used during Snapshot transitions.*/
    /// i32: (5.3.6f1 - 6000.2.0a6)
    pub m_UpdateMode: Option<i32>,
}

/// AudioMixerConstant is a sub class of the Unity engine since version 5.0.0f4.
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
    /// Vec<GroupConnection>: (2021.2.0b1 - 2021.3.45f1)
    pub groupConnections: Option<Vec<GroupConnection>>,
}

/// AudioMixerController is a  class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixerController {
    pub m_EnableSuspend: bool,
    /// PPtr<[`AudioMixerGroup`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_MasterGroup: PPtr,
    pub m_MixerConstant: AudioMixerConstant,
    pub m_Name: String,
    /// PPtr<[`AudioMixerGroup`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_OutputGroup: PPtr,
    /// Vec<PPtr<[`AudioMixerSnapshot`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_Snapshots: Vec<PPtr>,
    /// PPtr<[`AudioMixerSnapshot`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_StartSnapshot: PPtr,
    pub m_SuspendThreshold: f32,
    /// i32: (5.3.6f1 - 6000.2.0a6)
    pub m_UpdateMode: Option<i32>,
}

/// AudioMixerEffectController is a  class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixerEffectController {
    pub m_Bypass: bool,
    pub m_EffectID: GUID,
    pub m_EffectName: String,
    pub m_EnableWetMix: bool,
    pub m_MixLevel: GUID,
    pub m_Name: String,
    pub m_Parameters: Vec<Parameter>,
    /// PPtr<[`AudioMixerEffectController`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_SendTarget: PPtr,
}

/// AudioMixerGroup is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Audio.AudioMixerGroup.html):
/**
Object representing a group in the mixer.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixerGroup {
    /// PPtr<[`AudioMixer`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_AudioMixer: PPtr,
    /// Vec<PPtr<[`AudioMixerGroup`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_Children: Vec<PPtr>,
    pub m_GroupID: GUID,
    /**The name of the object.*/
    pub m_Name: String,
}

/// AudioMixerGroupController is a  class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixerGroupController {
    /// PPtr<[`AudioMixer`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_AudioMixer: PPtr,
    /// Vec<PPtr<[`AudioMixerGroup`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_Children: Vec<PPtr>,
    pub m_GroupID: GUID,
    pub m_Name: String,
}

/// AudioMixerSnapshot is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Audio.AudioMixerSnapshot.html):
/**
Object representing a snapshot in the mixer.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixerSnapshot {
    /// PPtr<[`AudioMixer`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_AudioMixer: PPtr,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_SnapshotID: GUID,
}

/// AudioMixerSnapshotController is a  class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMixerSnapshotController {
    /// PPtr<[`AudioMixer`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_AudioMixer: PPtr,
    pub m_Name: String,
    pub m_SnapshotID: GUID,
}

/// AudioRandomContainer is a  class of the Unity engine since version 2023.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioRandomContainer {
    pub m_AutomaticTriggerMode: i32,
    pub m_AutomaticTriggerTime: f32,
    pub m_AutomaticTriggerTimeRandomizationEnabled: bool,
    pub m_AutomaticTriggerTimeRandomizationRange: Vector2f,
    pub m_AvoidRepeatingLast: i32,
    /// Vec<PPtr<[`AudioContainerElement`]>>: (2023.2.0b1 - 6000.2.0a6)
    pub m_Elements: Vec<PPtr>,
    pub m_LoopCount: i32,
    pub m_LoopCountRandomizationEnabled: bool,
    pub m_LoopCountRandomizationRange: Vector2f,
    pub m_LoopMode: i32,
    pub m_Name: String,
    pub m_Pitch: f32,
    pub m_PitchRandomizationEnabled: bool,
    pub m_PitchRandomizationRange: Vector2f,
    pub m_PlaybackMode: i32,
    pub m_TriggerMode: i32,
    pub m_Volume: f32,
    pub m_VolumeRandomizationEnabled: bool,
    pub m_VolumeRandomizationRange: Vector2f,
}

/// AudioResource is a  class of the Unity engine since version 2023.2.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Audio.AudioResource.html):
/**
Represents an audio resource asset that you can play through an AudioSource.
Additional resources: AudioClip, AudioRandomContainer
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioResource {
    /**The name of the object.*/
    pub m_Name: String,
}

/// AudioReverbFilter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioReverbFilter.html):
/**
The Audio Reverb Filter takes an Audio Clip and distorts it to create a custom reverb effect.
Additional resources: Audio Reverb Filter information.
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /// f32: (3.4.0 - 5.6.0b9)
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /// f32: (3.4.0 - 5.6.0b9)
    pub m_RoomRolloffFactor: Option<f32>,
}

/// AudioSource is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AudioSource.html):
/**
A representation of audio sources in 3D.
Attach an AudioSource to a GameObject to play back sounds in a 3D environment.
To play 3D sounds you also need to have an AudioListener.
Usually, you can find the audio listener attached to the camera in your scene.
If you set AudioSource.spatialBlend to 0.0f, then Unity will treat the audio clip as a 2D sound. If you set it to 1.0f, the clip is fully 3D. Anything in between is a blend of 2D and 3D.To play, pause, and stop a single audio clip, use Play, Pause and Stop.
To adjust its volume while playing, use the volume property. Use time to seek through the audio track.
To play multiple sounds on one AudioSource, use PlayOneShot.
To play a clip at a static position in 3D space, use PlayClipAtPoint.Additional resources: AudioListener, AudioClip, AudioSource component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioSource {
    /**Bypass effects (Applied from filter components or global listener filters).*/
    pub BypassEffects: bool,
    /**Sets the Doppler scale for this AudioSource.*/
    pub DopplerLevel: f32,
    /**Checks if the audio clip is looping*/
    pub Loop: bool,
    /**The distance where sound either becomes inaudible or stops attenuation, depending on the rolloff mode.*/
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The pitch of the audio source.*/
    pub m_Pitch: f32,
    /**Enable this property to automatically play the audio source when the component or GameObject becomes active.*/
    pub m_PlayOnAwake: bool,
    /**The volume of the audio source (0.0 to 1.0).*/
    pub m_Volume: f32,
    /// PPtr<[`AudioClip`]>: (3.4.0 - 6000.2.0a6)
    pub m_audioClip: PPtr,
    pub panLevelCustomCurve: AnimationCurve,
    pub rolloffCustomCurve: AnimationCurve,
    /**Sets/Gets how the AudioSource attenuates over distance.*/
    pub rolloffMode: i32,
    pub spreadCustomCurve: AnimationCurve,
    /**When set, global effects on the AudioListener doesn't apply to the audio signal generated by the AudioSource. It also does'nt apply, if the AudioSource is playing into a mixer group.*/
    /// bool: (4.2.0 - 6000.2.0a6)
    pub BypassListenerEffects: Option<bool>,
    /**When set, it doesn't route the signal from an AudioSource into the global reverb associated with reverb zones.*/
    /// bool: (4.2.0 - 6000.2.0a6)
    pub BypassReverbZones: Option<bool>,
    /**The target group to which the AudioSource should route its signal.*/
    /// PPtr<[`AudioMixerGroup`]>: (5.0.0f4 - 6000.2.0a6)
    pub OutputAudioMixerGroup: Option<PPtr>,
    /**Enables or disables spatialization.*/
    /// bool: (5.2.0f2 - 6000.2.0a6)
    pub Spatialize: Option<bool>,
    /**Determines if the spatializer effect is inserted before or after the effect filters.*/
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub SpatializePostEffects: Option<bool>,
    /// Vec<ExtensionPropertyValue>: (2017.2.0b2 - 2017.2.0b11)
    pub m_ExtensionPropertyValues: Option<Vec<ExtensionPropertyValue>>,
    /**The default AudioResource to play.*/
    /// PPtr<[`AudioResource`]>: (2023.2.0b1 - 6000.2.0a6)
    pub m_Resource: Option<PPtr>,
    /// AnimationCurve: (5.0.0f4 - 6000.2.0a6)
    pub reverbZoneMixCustomCurve: Option<AnimationCurve>,
}

/// AutoOffMeshLinkData is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct AutoOffMeshLinkData {
    pub m_Area: u8,
    pub m_End: Vector3f,
    pub m_LinkDirection: u8,
    pub m_LinkType: u16,
    pub m_Radius: f32,
    pub m_Start: Vector3f,
}

/// Avatar is a  class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Avatar.html):
/**
The avatar asset describes the mapping of the character in the Animator.
Use the Avatar asset on a character with an Animator to control how the hierarchy animates.
You must create an Avatar to use with humanoid animation. If there is no Avatar on a character
with generic animation, an Avatar internal to the Animator is created instead.
Normally, the Avatar is created by the ModelImporter, but it can be created manually with AvatarBuilder
or through the "Build Generic Avatar" menu item of the Animator context menu.Generic AvatarFor generic animation, use AvatarBuilder.BuildGenericAvatar to specify what is the root of the animated
hierarchy (This can be nested in the hierarchy of the Animator) and the name of the node that holds the
root motion animation.Human AvatarFor humanoid animation, use AvatarBuilder.BuildHumanAvatar to specify what is the root of the animated
hierarchy (this can be nested in the hierarchy of the Animator) and the HumanDescription that provides
the mapping of transforms to human bones.Additional resources: Animator.avatar, AvatarBuilder, HumanDescription, ModelImporter.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Avatar {
    pub m_Avatar: AvatarConstant,
    pub m_AvatarSize: u32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_TOS: Vec<(u32, String)>,
    /**Returns the HumanDescription used to create this Avatar.*/
    /// HumanDescription: (2019.1.0b1 - 6000.2.0a6)
    pub m_HumanDescription: Option<HumanDescription>,
}

/// AvatarBodyMask is a  class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarBodyMask {
    pub m_Mask: Vec<u32>,
    pub m_Name: String,
}

/// AvatarConstant is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarConstant {
    pub m_Human: OffsetPtr,
    pub m_HumanSkeletonIndexArray: Vec<i32>,
    pub m_RootMotionBoneIndex: i32,
    pub m_RootMotionBoneX: xform,
    /// OffsetPtr: (4.3.0 - 6000.2.0a6)
    pub m_AvatarSkeleton: Option<OffsetPtr>,
    /// OffsetPtr: (4.3.0 - 6000.2.0a6)
    pub m_AvatarSkeletonPose: Option<OffsetPtr>,
    /// OffsetPtr: (4.3.0 - 6000.2.0a6)
    pub m_DefaultPose: Option<OffsetPtr>,
    /// Vec<i32>: (4.3.0 - 6000.2.0a6)
    pub m_HumanSkeletonReverseIndexArray: Option<Vec<i32>>,
    /// OffsetPtr: (4.3.0 - 6000.2.0a6)
    pub m_RootMotionSkeleton: Option<OffsetPtr>,
    /// Vec<i32>: (4.3.0 - 6000.2.0a6)
    pub m_RootMotionSkeletonIndexArray: Option<Vec<i32>>,
    /// OffsetPtr: (4.3.0 - 6000.2.0a6)
    pub m_RootMotionSkeletonPose: Option<OffsetPtr>,
    /// OffsetPtr: (4.0.0 - 4.2.2)
    pub m_Skeleton: Option<OffsetPtr>,
    /// Vec<u32>: (4.3.0 - 6000.2.0a6)
    pub m_SkeletonNameIDArray: Option<Vec<u32>>,
    /// OffsetPtr: (4.0.0 - 4.2.2)
    pub m_SkeletonPose: Option<OffsetPtr>,
}

/// AvatarMask is a  class of the Unity engine since version 4.1.0.
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

/// AvatarSkeletonMask is a  class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarSkeletonMask {
    pub elements: Vec<AvatarSkeletonMaskElement>,
    pub m_Name: String,
}

/// AvatarSkeletonMaskElement is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarSkeletonMaskElement {
    pub path: String,
    pub weight: f32,
}

/// Behaviour is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Behaviour.html):
/**
Behaviours are Components that can be enabled or disabled.
Additional resources: MonoBehaviour and Component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Behaviour {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
}

/// BillboardAsset is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/BillboardAsset.html):
/**
BillboardAsset describes how a billboard is rendered.
Billboards are a level-of-detail (LOD) method of drawing complicated 3D objects in a simpler manner if they are further away. Because the object is further away, there is often less requirement to draw the object at full detail due to its size on-screen and low likelihood of being a focal point in the Camera view. Instead, the object can be pre-rendered to a texture, and this texture used on a very simple Camera-facing Mesh of flat geometry (often simply a quadrilateral) known as a billboard. At great distances an object does not significantly change its orientation relative to the camera, so a billboard looks much like the object it represents from frame to frame, without having to be redrawn from the model. The BillboardAsset class allows the creation of billboards that are rendered from several directions, allowing a BillboardAsset to efficiently represent an object at low level of detail from any approximately-horizontal viewpoint.A BillboardAsset is usually created by importing SpeedTree assets. You can also create your own once you know how the billboard is described.SpeedTree billboard geometry is usually more complex than a plain quadrilateral. By using more vertices to cut out the empty part of the billboard image, rendering performance can potentially be improved due to the graphics system not having to draw as many redundant transparent pixels. You have access to the geometry data via BillboardAsset.vertices and BillboardAsset.indices.All vertices are considered in UV-space, because the geometry is due to be textured by the billboard images. UV vertices are easily expanded to 3D-space vertices by knowing the billboard's width, height, bottom, and what direction the billboard is currently facing. Assuming we have a billboard located at (0,0,0) looking at negative Z axis, the 3D-space coordinates are calculated as:X = (u - 0.5) * width
Y = v * height + bottom
Z = 0
UV-space vertices are expanded to 3D-space vertices, with a SpeedTree billboard at (0, 0, 0) in 3D space.In order to display something similar to the real 3D mesh being billboarded, SpeedTree billboards select an appropriate image from several pre-rendered images according to the current view direction.
The eight SpeedTree billboard images are baked. The images are created by capturing the rendered image of the 3D tree at different view angles, evenly distributed around the Y-axis. The first image always starts at positive X axis direction (or 0° if you imagine a unit circle from above).All images should be atlased together in one single texture. Each image is represented as a Vector4 of {left u, top v, width in u, height in v} in the atlas. You have access to all the images via BillboardAsset.imageTexCoords. SpeedTree Modeler always exports a normal texture alongside the diffuse texture for even better approximation of the lighting, and it shares the same atlas layout with the diffuse texture.Once the BillboardAsset is constructed, use BillboardRenderer to render it.
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
    /// PPtr<[`Material`]>: (5.0.0f4 - 6000.2.0a6)
    pub material: PPtr,
    pub vertices: Vec<Vector2f>,
    /**Width of the billboard.*/
    pub width: f32,
    /// Vec<u8>: (5.0.0f4 - 5.2.5f1)
    pub rotated: Option<Vec<u8>>,
}

/// BillboardRenderer is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/BillboardRenderer.html):
/**
Renders a billboard from a BillboardAsset.
BillboardRenderers that share the same BillboardAsset can be rendered in a batch if they are next to each other in the order of rendering. The batching is always enabled regardless of whether dynamic batching is enabled or not.Due to the always-upright nature of a tree billboard, BillboardRenderer can only rotate around Y-axis.Additional resources: BillboardAsset.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BillboardRenderer {
    /**The BillboardAsset to render.*/
    /// PPtr<[`BillboardAsset`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_Billboard: PPtr,
    pub m_CastShadows: u8,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    /// Vec<PPtr<[`Material`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_Materials: Vec<PPtr>,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr<[`Transform`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_ProbeAnchor: PPtr,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: Enum_bool__u8,
    /**Should reflection probes be used for this Renderer?*/
    pub m_ReflectionProbeUsage: i32,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    /// PPtr<[`Transform`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_StaticBatchRoot: PPtr,
    /// u8: (2017.2.0f1 - 6000.2.0a6)
    pub m_DynamicOccludee: Option<u8>,
    /**The light probe interpolation type.*/
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeUsage: Option<u8>,
    /// PPtr<[`GameObject`]>: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_MotionVectors: Option<u8>,
    /// u8: (2020.1.0b1 - 6000.2.0a6)
    pub m_RayTraceProcedural: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlags: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlagsOverride: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0b1 - 6000.2.0a6)
    pub m_RayTracingMode: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on, if you use a scriptable render pipeline.*/
    /// u32: (2018.1.0f1 - 6000.2.0a6)
    pub m_RenderingLayerMask: Option<u32>,
    /// u8: (2023.3.0b1 - 6000.2.0a6)
    pub m_SmallMeshCulling: Option<u8>,
    /// i16: (5.6.0b1 - 6000.2.0a6)
    pub m_SortingLayer: Option<i16>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SortingLayerID: Option<i32>,
    /// StaticBatchInfo: (5.5.0f3 - 6000.2.0a6)
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.1.0b1 - 6000.2.0a6)
    pub m_StaticShadowCaster: Option<u8>,
    /// Vec<u32>: (5.0.0f4 - 5.4.6f3)
    pub m_SubsetIndices: Option<Vec<u32>>,
    /// bool: (5.0.0f4 - 5.3.8f2)
    pub m_UseLightProbes: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_bool__u8 {
    bool(bool),
    u8(u8),
}

/// BitField is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct BitField {
    pub m_Bits: u32,
}

/// BlendShapeData is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendShapeData {
    pub channels: Vec<MeshBlendShapeChannel>,
    pub fullWeights: Vec<f32>,
    pub shapes: Vec<MeshBlendShape>,
    pub vertices: Vec<BlendShapeVertex>,
}

/// BlendShapeVertex is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendShapeVertex {
    pub index: u32,
    pub normal: Vector3f,
    pub tangent: Vector3f,
    pub vertex: Vector3f,
}

/// BlendTree is a  class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.BlendTree.html):
/**
Blend trees are used to blend continuously animation between their children. They can either be 1D or 2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendTree {
    pub m_Childs: Vec<Enum_Child__ChildMotion>,
    /**Sets the maximum threshold that will be used by the ChildMotion. Only used when useAutomaticThresholds is true.*/
    pub m_MaxThreshold: f32,
    /**Sets the minimum threshold that will be used by the ChildMotion. Only used when useAutomaticThresholds is true.*/
    pub m_MinThreshold: f32,
    /**The name of the object.*/
    pub m_Name: String,
    /**When active, the children's thresholds are automatically spread between 0 and 1.*/
    pub m_UseAutomaticThresholds: bool,
    /// String: (4.0.0 - 4.1.5)
    pub m_BlendEvent: Option<String>,
    /// String: (4.1.0 - 4.1.5)
    pub m_BlendEventY: Option<String>,
    /**Parameter that is used to compute the blending weight of the children in 1D blend trees or on the X axis of a 2D blend tree.*/
    /// String: (4.2.0 - 6000.2.0a6)
    pub m_BlendParameter: Option<String>,
    /**Parameter that is used to compute the blending weight of the children on the Y axis of a 2D blend tree.*/
    /// String: (4.2.0 - 6000.2.0a6)
    pub m_BlendParameterY: Option<String>,
    /**The Blending type can be either 1D or different types of 2D.*/
    /// u32: (4.1.0 - 4.1.5); i32: (4.2.0 - 6000.2.0a6)
    pub m_BlendType: Option<i64>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_NormalizedBlendValues: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_Child__ChildMotion {
    Child(Child),
    ChildMotion(ChildMotion),
}

/// BoneInfluence is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct BoneInfluence {
    /// i32: (3.4.0 - 5.6.7f1)
    #[serde(alias = "boneIndex[0]")]
    pub boneIndex_0_: Option<i32>,
    /// i32: (3.4.0 - 5.6.7f1)
    #[serde(alias = "boneIndex[1]")]
    pub boneIndex_1_: Option<i32>,
    /// i32: (3.4.0 - 5.6.7f1)
    #[serde(alias = "boneIndex[2]")]
    pub boneIndex_2_: Option<i32>,
    /// i32: (3.4.0 - 5.6.7f1)
    #[serde(alias = "boneIndex[3]")]
    pub boneIndex_3_: Option<i32>,
    /// f32: (3.4.0 - 5.6.7f1)
    #[serde(alias = "weight[0]")]
    pub weight_0_: Option<f32>,
    /// f32: (3.4.0 - 5.6.7f1)
    #[serde(alias = "weight[1]")]
    pub weight_1_: Option<f32>,
    /// f32: (3.4.0 - 5.6.7f1)
    #[serde(alias = "weight[2]")]
    pub weight_2_: Option<f32>,
    /// f32: (3.4.0 - 5.6.7f1)
    #[serde(alias = "weight[3]")]
    pub weight_3_: Option<f32>,
}

/// BoneWeights4 is a sub class of the Unity engine since version 2017.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct BoneWeights4 {
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    #[serde(alias = "boneIndex[0]")]
    pub boneIndex_0_: Option<i32>,
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    #[serde(alias = "boneIndex[1]")]
    pub boneIndex_1_: Option<i32>,
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    #[serde(alias = "boneIndex[2]")]
    pub boneIndex_2_: Option<i32>,
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    #[serde(alias = "boneIndex[3]")]
    pub boneIndex_3_: Option<i32>,
    /// f32: (2017.1.0b1 - 6000.2.0a6)
    #[serde(alias = "weight[0]")]
    pub weight_0_: Option<f32>,
    /// f32: (2017.1.0b1 - 6000.2.0a6)
    #[serde(alias = "weight[1]")]
    pub weight_1_: Option<f32>,
    /// f32: (2017.1.0b1 - 6000.2.0a6)
    #[serde(alias = "weight[2]")]
    pub weight_2_: Option<f32>,
    /// f32: (2017.1.0b1 - 6000.2.0a6)
    #[serde(alias = "weight[3]")]
    pub weight_3_: Option<f32>,
}

/// BoxCollider is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/BoxCollider.html):
/**
A box-shaped primitive collider.
Additional resources: SphereCollider, CapsuleCollider, PhysicsMaterial, Rigidbody.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxCollider {
    /**The center of the box, measured in the object's local space.*/
    pub m_Center: Vector3f,
    /**Enabled Colliders will collide with other Colliders, disabled Colliders won't.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Specify if this collider is configured as a trigger.*/
    pub m_IsTrigger: bool,
    /**The material used by the collider.*/
    /// PPtr<[`PhysicMaterial`]>: (3.4.0 - 2023.3.0a8); PPtr<[`PhysicsMaterial`]>: (2023.3.0b1 - 6000.2.0a6)
    pub m_Material: PPtr,
    /**The size of the box, measured in the object's local space.*/
    pub m_Size: Vector3f,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_ProvidesContacts: Option<bool>,
}

/// BoxCollider2D is a  class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/BoxCollider2D.html):
/**
Collider for 2D physics representing an axis-aligned rectangle.
Additional resources: CircleCollider2D, PolygonCollider2D, EdgeCollider2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxCollider2D {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.3.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    /// PPtr<[`PhysicsMaterial2D`]>: (4.3.0 - 6000.2.0a6)
    pub m_Material: PPtr,
    /**The width and height of the rectangle.*/
    pub m_Size: Vector2f,
    /**Determines whether the BoxCollider2D's shape is automatically updated based on a SpriteRenderer's tiling properties.*/
    /// bool: (5.6.0f1 - 6000.2.0a6)
    pub m_AutoTiling: Option<bool>,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_CallbackLayers: Option<BitField>,
    /// Vector2f: (4.3.0 - 4.7.2)
    pub m_Center: Option<Vector2f>,
    /**The composite operation to be used by a CompositeCollider2D.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOperation: Option<i32>,
    /**The composite operation order to be used when a CompositeCollider2D is used.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOrder: Option<i32>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**The density of the collider used to calculate its mass (when auto mass is enabled).*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_Density: Option<f32>,
    /**Controls the radius of all edges created by the collider.*/
    /// f32: (5.6.0f1 - 6000.2.0a6)
    pub m_EdgeRadius: Option<f32>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /**The local offset of the collider geometry.*/
    /// Vector2f: (5.0.0f4 - 6000.2.0a6)
    pub m_Offset: Option<Vector2f>,
    /// SpriteTilingProperty: (5.6.0f1 - 6000.2.0a6)
    pub m_SpriteTilingProperty: Option<SpriteTilingProperty>,
    /// bool: (5.6.0b1 - 2023.1.0a21)
    pub m_UsedByComposite: Option<bool>,
    /**Whether the collider is used by an attached effector or not.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_UsedByEffector: Option<bool>,
}

/// BranchWindLevel is a sub class of the Unity engine since version 2023.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct BranchWindLevel {
    pub m_afBend_0: f32,
    pub m_afBend_1: f32,
    pub m_afBend_10: f32,
    pub m_afBend_11: f32,
    pub m_afBend_12: f32,
    pub m_afBend_13: f32,
    pub m_afBend_14: f32,
    pub m_afBend_15: f32,
    pub m_afBend_16: f32,
    pub m_afBend_17: f32,
    pub m_afBend_18: f32,
    pub m_afBend_19: f32,
    pub m_afBend_2: f32,
    pub m_afBend_3: f32,
    pub m_afBend_4: f32,
    pub m_afBend_5: f32,
    pub m_afBend_6: f32,
    pub m_afBend_7: f32,
    pub m_afBend_8: f32,
    pub m_afBend_9: f32,
    pub m_afFlexibility_0: f32,
    pub m_afFlexibility_1: f32,
    pub m_afFlexibility_10: f32,
    pub m_afFlexibility_11: f32,
    pub m_afFlexibility_12: f32,
    pub m_afFlexibility_13: f32,
    pub m_afFlexibility_14: f32,
    pub m_afFlexibility_15: f32,
    pub m_afFlexibility_16: f32,
    pub m_afFlexibility_17: f32,
    pub m_afFlexibility_18: f32,
    pub m_afFlexibility_19: f32,
    pub m_afFlexibility_2: f32,
    pub m_afFlexibility_3: f32,
    pub m_afFlexibility_4: f32,
    pub m_afFlexibility_5: f32,
    pub m_afFlexibility_6: f32,
    pub m_afFlexibility_7: f32,
    pub m_afFlexibility_8: f32,
    pub m_afFlexibility_9: f32,
    pub m_afOscillation_0: f32,
    pub m_afOscillation_1: f32,
    pub m_afOscillation_10: f32,
    pub m_afOscillation_11: f32,
    pub m_afOscillation_12: f32,
    pub m_afOscillation_13: f32,
    pub m_afOscillation_14: f32,
    pub m_afOscillation_15: f32,
    pub m_afOscillation_16: f32,
    pub m_afOscillation_17: f32,
    pub m_afOscillation_18: f32,
    pub m_afOscillation_19: f32,
    pub m_afOscillation_2: f32,
    pub m_afOscillation_3: f32,
    pub m_afOscillation_4: f32,
    pub m_afOscillation_5: f32,
    pub m_afOscillation_6: f32,
    pub m_afOscillation_7: f32,
    pub m_afOscillation_8: f32,
    pub m_afOscillation_9: f32,
    pub m_afSpeed_0: f32,
    pub m_afSpeed_1: f32,
    pub m_afSpeed_10: f32,
    pub m_afSpeed_11: f32,
    pub m_afSpeed_12: f32,
    pub m_afSpeed_13: f32,
    pub m_afSpeed_14: f32,
    pub m_afSpeed_15: f32,
    pub m_afSpeed_16: f32,
    pub m_afSpeed_17: f32,
    pub m_afSpeed_18: f32,
    pub m_afSpeed_19: f32,
    pub m_afSpeed_2: f32,
    pub m_afSpeed_3: f32,
    pub m_afSpeed_4: f32,
    pub m_afSpeed_5: f32,
    pub m_afSpeed_6: f32,
    pub m_afSpeed_7: f32,
    pub m_afSpeed_8: f32,
    pub m_afSpeed_9: f32,
    pub m_afTurbulence_0: f32,
    pub m_afTurbulence_1: f32,
    pub m_afTurbulence_10: f32,
    pub m_afTurbulence_11: f32,
    pub m_afTurbulence_12: f32,
    pub m_afTurbulence_13: f32,
    pub m_afTurbulence_14: f32,
    pub m_afTurbulence_15: f32,
    pub m_afTurbulence_16: f32,
    pub m_afTurbulence_17: f32,
    pub m_afTurbulence_18: f32,
    pub m_afTurbulence_19: f32,
    pub m_afTurbulence_2: f32,
    pub m_afTurbulence_3: f32,
    pub m_afTurbulence_4: f32,
    pub m_afTurbulence_5: f32,
    pub m_afTurbulence_6: f32,
    pub m_afTurbulence_7: f32,
    pub m_afTurbulence_8: f32,
    pub m_afTurbulence_9: f32,
    pub m_fIndependence: f32,
}

/// BrokenPrefabAsset is a  class of the Unity engine since version 2022.2.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/BrokenPrefabAsset.html):
/**
BrokenPrefabAsset is for Prefab files where the file content cannot be loaded without errors.
A Prefab Asset can be broken if the content of the file invalid or if it is a Variant Prefab where the parent Prefab is either invalid or missing.Search for t:BrokenPrefabAsset in the project browser to see which assets are of that type.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BrokenPrefabAsset {
    /// PPtr<[`BrokenPrefabAsset`]>: (2022.2.0b1 - 6000.2.0a6)
    pub m_BrokenParentPrefab: PPtr,
    /**Returns true if the content of the file is valid.*/
    pub m_IsPrefabFileValid: bool,
    /**Returns true if the prefab is a variant.*/
    pub m_IsVariant: bool,
    pub m_IsWarning: bool,
    pub m_Message: String,
    /**The name of the object.*/
    pub m_Name: String,
}

/// BufferBinding is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct BufferBinding {
    pub m_Index: i32,
    pub m_NameIndex: i32,
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub m_ArraySize: Option<i32>,
}

/// BuildArchiveImporter is a  class of the Unity engine since version 2023.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildArchiveImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2023.1.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// BuildInstructionImporter is a  class of the Unity engine since version 2023.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildInstructionImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2023.1.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// BuildMetaDataImporter is a  class of the Unity engine since version 2023.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildMetaDataImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2023.1.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// BuildReport is a  class of the Unity engine since version 5.4.0f3.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Build.Reporting.BuildReport.html):
/**
The BuildReport API gives you information about the Unity build process.
A BuildReport object is returned by BuildPipeline.BuildPlayer and can be used to discover information about the files output, the build steps taken, and other platform-specific information such as native code stripping.For AssetBundle builds the BuildReport is available by calling GetLatestReport immediately after calling BuildPipeline.BuildAssetBundles.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildReport {
    /// Vec<PPtr<[`Object`]>>: (5.4.0f3 - 6000.2.0a6)
    pub m_Appendices: Vec<PPtr>,
    pub m_BuildSteps: Vec<BuildStepInfo>,
    pub m_Files: Vec<BuildReportFile>,
    /**The name of the object.*/
    pub m_Name: String,
    /**A BuildSummary containing overall statistics and data about the build process.*/
    pub m_Summary: BuildSummary,
}

/// BuildReportFile is a sub class of the Unity engine since version 5.4.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildReportFile {
    pub id: u32,
    pub path: String,
    pub role: String,
    pub totalSize: u64,
}

/// BuildReportPackedAssetInfo is a sub class of the Unity engine since version 5.4.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildReportPackedAssetInfo {
    pub classID: i32,
    pub fileID: i64,
    pub packedSize: i128,
    pub sourceAssetGUID: GUID,
    /// String: (5.4.2f2 - 6000.2.0a6)
    pub buildTimeAssetPath: Option<String>,
    /// u64: (2019.3.0b1 - 6000.2.0a6)
    pub offset: Option<u64>,
}

/// BuildReportScenesUsingAsset is a sub class of the Unity engine since version 2020.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildReportScenesUsingAsset {
    pub assetPath: String,
    pub scenePaths: Vec<String>,
}

/// BuildSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rendering.RayTracingAccelerationStructure.BuildSettings.html):
/**
Contains options for building a RayTracingAccelerationStructure.
This struct packages together parameters for the CommandBuffer.BuildRayTracingAccelerationStructure and RayTracingAccelerationStructure.Build methods. Use the different parameters to control or optimize how Unity builds the acceleration structure.
Additional resources:: Build, CommandBuffer.BuildRayTracingAccelerationStructure.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSettings {
    pub enableDynamicBatching: bool,
    pub hasAdvancedVersion: bool,
    pub hasPROVersion: bool,
    pub hasPublishingRights: bool,
    pub hasShadows: bool,
    pub isEducationalBuild: bool,
    pub m_Version: String,
    /// GUID: (5.6.0f1 - 2023.1.0a19); String: (5.6.0b3 - 5.6.0b3)
    pub buildGUID: Option<Enum_GUID__String>,
    /// Vec<String>: (5.6.0b1 - 6000.2.0a6)
    pub buildTags: Option<Vec<String>>,
    /// bool: (5.0.0f4 - 5.2.5f1)
    pub enableMultipleDisplays: Option<bool>,
    /// Vec<String>: (5.4.0f3 - 6000.2.0a6)
    pub enabledVRDevices: Option<Vec<String>>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub hasClusterRendering: Option<bool>,
    /// bool: (4.2.0 - 6000.2.0a6)
    pub hasLocalLightShadows: Option<bool>,
    /// bool: (4.6.2 - 4.7.2)
    pub hasOculusPlugin: Option<bool>,
    /// bool: (3.4.0 - 5.2.5f1)
    pub hasRenderTexture: Option<bool>,
    /// bool: (4.2.0 - 6000.2.0a6)
    pub hasSoftShadows: Option<bool>,
    /// bool: (3.4.0 - 2023.1.0a19)
    pub isDebugBuild: Option<bool>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub isEmbedded: Option<bool>,
    /// bool: (3.5.0 - 6000.2.0a6)
    pub isNoWatermarkBuild: Option<bool>,
    /// bool: (3.5.0 - 6000.2.0a6)
    pub isPrototypingBuild: Option<bool>,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub isTrial: Option<bool>,
    /// bool: (2018.2.0b1 - 2018.3.0b3)
    pub isWsaHolographicRemotingEnabled: Option<bool>,
    /// Vec<String>: (3.4.0 - 5.0.4f1)
    pub levels: Option<Vec<String>>,
    /// String: (3.4.0 - 6000.2.0a6)
    pub m_AuthToken: Option<String>,
    /// Vec<i32>: (5.1.0f1 - 6000.2.0a6)
    pub m_GraphicsAPIs: Option<Vec<i32>>,
    /// Vec<String>: (5.0.0f4 - 6000.2.0a6)
    pub preloadedPlugins: Option<Vec<String>>,
    /// Vec<(i32, u32)>: (3.5.0 - 4.7.2); Vec<(i32, Hash128)>: (5.0.0f4 - 2020.2.0a19)
    pub runtimeClassHashes: Option<Vec<(i32, Enum_Hash128__u32)>>,
    /// Vec<String>: (5.1.0f1 - 6000.2.0a6)
    pub scenes: Option<Vec<String>>,
    /// Vec<(Hash128, Hash128)>: (5.0.0f4 - 2020.2.0a19)
    pub scriptHashes: Option<Vec<(Hash128, Hash128)>>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub usesOnMouseEvents: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_Hash128__u32 {
    Hash128(Hash128),
    u32(u32),
}

/// BuildStepInfo is a sub class of the Unity engine since version 5.4.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildStepInfo {
    pub messages: Vec<BuildStepMessage>,
    pub stepName: String,
    /// i32: (2018.1.0f1 - 6000.2.0a6)
    pub depth: Option<i32>,
    /// u64: (5.4.0f3 - 2017.4.40f1)
    pub duration: Option<u64>,
    /// u64: (2018.1.0f1 - 6000.2.0a6)
    pub durationTicks: Option<u64>,
}

/// BuildStepMessage is a sub class of the Unity engine since version 5.4.0f3.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Build.Reporting.BuildStepMessage.html):
/**
Contains information about a single log message recorded during the build process.
Additional resources: BuildStep.messages
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildStepMessage {
    /**The text content of the log message.*/
    pub content: String,
    /**The LogType of the log message.*/
    /// i32: (5.4.0f3 - 6000.2.0a6)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// BuildSummary is a sub class of the Unity engine since version 5.4.0f3.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Build.Reporting.BuildSummary.html):
/**
Contains overall summary information about a build.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSummary {
    pub assetBundleOptions: i32,
    pub crc: u32,
    /**The BuildOptions used for the build, as passed to BuildPipeline.BuildPlayer.*/
    pub options: i32,
    /**The output path for the build, as provided to BuildPipeline.BuildPlayer.*/
    pub outputPath: String,
    pub platformName: String,
    /**The total number of errors and exceptions recorded during the build process.*/
    pub totalErrors: i32,
    /**The total size of the build output, in bytes.*/
    pub totalSize: u64,
    /**The total number of warnings recorded during the build process.*/
    pub totalWarnings: i32,
    /// GUID: (5.6.0f1 - 6000.2.0a6)
    pub buildGUID: Option<GUID>,
    /// i32: (2017.2.0f1 - 6000.2.0a6)
    pub buildResult: Option<i32>,
    /// DateTime: (2018.1.0f1 - 6000.2.0a6)
    pub buildStartTime: Option<DateTime>,
    /**The type of build.*/
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub buildType: Option<i32>,
    /**Whether the multi-process option was enabled for the build.*/
    /// bool: (2023.1.0b1 - 6000.2.0a6)
    pub multiProcessEnabled: Option<bool>,
    /// String: (5.4.0f3 - 2017.4.40f1)
    pub name: Option<String>,
    /// String: (5.6.0b1 - 6000.2.0a6)
    pub platformGroupName: Option<String>,
    /// i32: (2021.2.0f1 - 6000.2.0a6)
    pub subtarget: Option<i32>,
    /// bool: (5.5.0f3 - 2017.1.5f1)
    pub success: Option<bool>,
    /// u64: (5.4.0f3 - 2017.4.40f1)
    pub totalTimeMS: Option<u64>,
    /// u64: (2018.1.0f1 - 6000.2.0a6)
    pub totalTimeTicks: Option<u64>,
}

/// BuildTargetSettings is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildTargetSettings {
    pub m_BuildTarget: String,
    pub m_TextureFormat: i32,
    /// bool: (5.2.0f2 - 5.4.6f3)
    pub m_AllowsAlphaSplitting: Option<bool>,
    /// i32: (3.5.0 - 5.4.6f3)
    pub m_CompressionQuality: Option<i32>,
    /// i32: (5.5.0f3 - 2017.4.40f1)
    pub m_LoadingBehavior: Option<i32>,
    /// i32: (3.4.0 - 5.4.6f3)
    pub m_MaxTextureSize: Option<i32>,
    /// i32: (5.5.0f3 - 2017.4.40f1)
    pub m_TextureHeight: Option<i32>,
    /// i32: (5.5.0f3 - 2017.4.40f1)
    pub m_TextureWidth: Option<i32>,
}

/// BuildTextureStackReference is a sub class of the Unity engine since version 2020.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildTextureStackReference {
    pub groupName: String,
    pub itemName: String,
}

/// BuiltAssetBundleInfo is a sub class of the Unity engine since version 5.5.4f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuiltAssetBundleInfo {
    pub bundleArchiveFile: u32,
    pub bundleName: String,
    pub packagedFileIndices: Vec<u32>,
}

/// BuiltAssetBundleInfoSet is a  class of the Unity engine since version 5.5.4f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuiltAssetBundleInfoSet {
    pub bundleInfos: Vec<BuiltAssetBundleInfo>,
}

/// BuiltinShaderSettings is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuiltinShaderSettings {
    pub m_Mode: i32,
    /// PPtr<[`Shader`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_Shader: PPtr,
}

/// BuoyancyEffector2D is a  class of the Unity engine since version 5.3.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/BuoyancyEffector2D.html):
/**
Applies forces to simulate buoyancy, fluid-flow and fluid damping.
When any Collider2D overlap the area defined by the effector, calculations are made to determine if they are below the surfaceLevel.  If they are not, no forces are applied.  If they are then the effector will apply buoyancy forces in an attempt to move the Collider2D to the surfaceLevel i.e. they will float.This effector is designed primarily to work with Collider2D that are set as triggers so that Collider2D can overlap the defined area and have buoyancy forces applied to them.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct BuoyancyEffector2D {
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
    /// PPtr<[`GameObject`]>: (5.3.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Defines an arbitrary horizontal line that represents the fluid surface level.*/
    pub m_SurfaceLevel: f32,
    /**Should the collider-mask be used or the global collision matrix?*/
    pub m_UseColliderMask: bool,
    /**A force applied to slow angular movement of any Collider2D in contact with the effector.*/
    /// f32: (6000.1.0b1 - 6000.2.0a6)
    pub m_AngularDamping: Option<f32>,
    /// f32: (5.3.0f1 - 6000.1.0a5)
    pub m_AngularDrag: Option<f32>,
    /**A force applied to slow linear movement of any Collider2D in contact with the effector.*/
    /// f32: (6000.1.0b1 - 6000.2.0a6)
    pub m_LinearDamping: Option<f32>,
    /// f32: (5.3.0f1 - 6000.1.0a5)
    pub m_LinearDrag: Option<f32>,
}

/// C4DImporter is a  class of the Unity engine since version 2021.3.30f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct C4DImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2021.3.30f1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
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

/// CachedSpriteAtlas is a  class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct CachedSpriteAtlas {
    pub frames: Vec<((GUID, i64), SpriteRenderData)>,
    /// Vec<PPtr<[`Texture2D`]>>: (4.3.0 - 6000.2.0a6)
    pub textures: Vec<PPtr>,
    /// Vec<PPtr<[`Texture2D`]>>: (5.2.0f2 - 6000.2.0a6)
    pub alphaTextures: Option<Vec<PPtr>>,
}

/// CachedSpriteAtlasRuntimeData is a  class of the Unity engine since version 2017.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct CachedSpriteAtlasRuntimeData {
    /// Vec<PPtr<[`Texture2D`]>>: (2017.1.0b1 - 6000.2.0a6)
    pub alphaTextures: Vec<PPtr>,
    pub frames: Vec<((GUID, i64), SpriteAtlasData)>,
    /// Vec<PPtr<[`Texture2D`]>>: (2017.1.0b1 - 6000.2.0a6)
    pub textures: Vec<PPtr>,
    /// Hash128: (2020.1.0b1 - 6000.2.0a6)
    pub currentPackingHash: Option<Hash128>,
}

/// Camera is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Camera.html):
/**
A Camera is a device through which the player views the world.
A screen space point is defined in pixels. The bottom-left of the screen is (0,0); the right-top
is (pixelWidth,pixelHeight). The z position is in world units from the Camera.A viewport space point is normalized and relative to the Camera. The bottom-left of the Camera is
(0,0); the top-right is (1,1). The z position is in world units from the Camera.A world space point is defined in global coordinates (for example, Transform.position).Additional resources: camera component.
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub m_NormalizedViewPortRect: Rectf,
    /**The rendering path that should be used, if possible.*/
    pub m_RenderingPath: i32,
    /**Destination render texture.*/
    /// PPtr<[`RenderTexture`]>: (3.4.0 - 6000.2.0a6)
    pub m_TargetTexture: PPtr,
    /**Is the camera orthographic (true) or perspective (false)?*/
    pub orthographic: bool,
    /**The distance of the far clipping plane from the Camera, in world units.*/
    /// f32: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "far clip plane")]
    pub far_clip_plane: Option<f32>,
    /**The vertical field of view of the Camera, in degrees.*/
    /// f32: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "field of view")]
    pub field_of_view: Option<f32>,
    /**Dynamic Resolution Scaling.*/
    /// bool: (2017.3.0b1 - 6000.2.0a6)
    pub m_AllowDynamicResolution: Option<bool>,
    /**MSAA rendering.*/
    /// bool: (5.6.0b1 - 6000.2.0a6)
    pub m_AllowMSAA: Option<bool>,
    /**The camera anamorphism. To use this property, enable UsePhysicalProperties.*/
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub m_Anamorphism: Option<f32>,
    /**The camera aperture. To use this property, enable UsePhysicalProperties.*/
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub m_Aperture: Option<f32>,
    /**The camera barrel clipping. To use this property, enable UsePhysicalProperties.*/
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub m_BarrelClipping: Option<f32>,
    /**The blade count in the lens of the camera. To use this property, enable UsePhysicalProperties.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_BladeCount: Option<i32>,
    /**The curvature of the blades. To use this property, enable UsePhysicalProperties.*/
    /// Vector2f: (2022.2.0b1 - 6000.2.0a6)
    pub m_Curvature: Option<Vector2f>,
    /**The camera focal length, expressed in millimeters. To use this property, enable UsePhysicalProperties.*/
    /// f32: (2018.2.0b1 - 6000.2.0a6)
    pub m_FocalLength: Option<f32>,
    /**The focus distance of the lens. To use this property, enable UsePhysicalProperties.*/
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub m_FocusDistance: Option<f32>,
    /// bool: (5.6.0b1 - 6000.2.0a6)
    pub m_ForceIntoRT: Option<bool>,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_GateFitMode: Option<i32>,
    /// bool: (3.5.0 - 6000.2.0a6)
    pub m_HDR: Option<bool>,
    /**The sensor sensitivity of the camera. To use this property, enable UsePhysicalProperties.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_Iso: Option<i32>,
    /**The lens offset of the camera. The lens shift is relative to the sensor size. For example, a lens shift of 0.5 offsets the sensor by half its horizontal size.*/
    /// Vector2f: (2018.2.0b1 - 6000.2.0a6)
    pub m_LensShift: Option<Vector2f>,
    /// bool: (4.3.0 - 6000.2.0a6)
    pub m_OcclusionCulling: Option<bool>,
    /**The size of the camera sensor, expressed in millimeters.*/
    /// Vector2f: (2018.2.0b1 - 6000.2.0a6)
    pub m_SensorSize: Option<Vector2f>,
    /**The exposure time of the camera, in seconts. To use this property, enable UsePhysicalProperties.*/
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub m_ShutterSpeed: Option<f32>,
    /**Distance to a point where virtual eyes converge.*/
    /// f32: (4.5.0 - 6000.2.0a6)
    pub m_StereoConvergence: Option<f32>,
    /// bool: (5.1.0f1 - 2017.2.0b10)
    pub m_StereoMirrorMode: Option<bool>,
    /**The distance between the virtual eyes. Use this to query or set the current eye separation. Note that most VR devices provide this value, in which case setting the value will have no effect.*/
    /// f32: (4.5.0 - 6000.2.0a6)
    pub m_StereoSeparation: Option<f32>,
    /**Set the target display for this Camera.*/
    /// u32: (4.5.0 - 5.2.5f1); i32: (5.3.0f1 - 6000.2.0a6)
    pub m_TargetDisplay: Option<i64>,
    /// i32: (5.1.3f1 - 6000.2.0a6)
    pub m_TargetEye: Option<i32>,
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub m_projectionMatrixMode: Option<i32>,
    /**The distance of the near clipping plane from the Camera, in world units.*/
    /// f32: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "near clip plane")]
    pub near_clip_plane: Option<f32>,
    /**Camera's half-size when in orthographic mode.*/
    /// f32: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "orthographic size")]
    pub orthographic_size: Option<f32>,
}

/// Canvas is a  class of the Unity engine since version 4.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Canvas.html):
/**
Element that can be used for screen rendering.
Elements on a canvas are rendered AFTER Scene rendering, either from an attached camera or using overlay mode.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Canvas {
    /// PPtr<[`Camera`]>: (4.5.0 - 6000.2.0a6)
    pub m_Camera: PPtr,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.5.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Forces pixel alignment for elements in the canvas. It only applies when Canvas.renderMode is set to Screen Space.*/
    pub m_PixelPerfect: bool,
    /**Is the Canvas in World or Overlay mode?*/
    pub m_RenderMode: i32,
    /// i32: (5.6.0f1 - 6000.2.0a6)
    pub m_AdditionalShaderChannelsFlag: Option<i32>,
    /// f32: (4.5.0 - 4.5.5)
    pub m_Alpha: Option<f32>,
    /// bool: (4.5.0 - 4.5.5)
    pub m_Normals: Option<bool>,
    /**Allows for nested canvases to override pixelPerfect settings inherited from parent canvases.*/
    /// bool: (4.6.0 - 6000.2.0a6)
    pub m_OverridePixelPerfect: Option<bool>,
    /**Allows for nested canvases to override the Canvas.sortingOrder from parent canvases.*/
    /// bool: (4.6.0 - 6000.2.0a6)
    pub m_OverrideSorting: Option<bool>,
    /**How far away from the camera is the Canvas generated? It only applies when Canvas.renderMode is set to RenderMode.ScreenSpaceCamera.*/
    /// f32: (4.6.0 - 6000.2.0a6)
    pub m_PlaneDistance: Option<f32>,
    /// bool: (4.5.0 - 4.5.5)
    pub m_PositionUVs: Option<bool>,
    /// bool: (4.6.0 - 6000.2.0a6)
    pub m_ReceivesEvents: Option<bool>,
    /// f32: (5.3.4f1 - 6000.2.0a6)
    pub m_SortingBucketNormalizedSize: Option<f32>,
    /**Unique ID of the Canvas' sorting layer.*/
    /// u32: (4.6.0 - 4.7.2); i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SortingLayerID: Option<i64>,
    /**Canvas' order within a sorting layer.*/
    /// i16: (4.6.0 - 6000.2.0a6)
    pub m_SortingOrder: Option<i16>,
    /**For Overlay mode, display index on which the UI canvas will appear.*/
    /// i8: (5.3.0f1 - 6000.2.0a6)
    pub m_TargetDisplay: Option<i8>,
    /**Should the Canvas size be updated based on the render target when a manual Camera.Render call is performed.*/
    /// i32: (2022.1.0b1 - 6000.2.0a6)
    pub m_UpdateRectTransformForStandalone: Option<i32>,
    /**Should the Canvas vertex color always be in gamma space before passing to the UI shaders in linear color space work flow.*/
    /// bool: (2021.3.34f1 - 6000.2.0a6)
    pub m_VertexColorAlwaysGammaSpace: Option<bool>,
}

/// CanvasGroup is a  class of the Unity engine since version 4.6.0.
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
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.6.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Should the group ignore parent groups?*/
    pub m_IgnoreParentGroups: bool,
    /**Is the group interactable (are the elements beneath the group enabled).*/
    pub m_Interactable: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    /// u8: (4.6.1 - 6000.2.0a6)
    pub m_Enabled: Option<u8>,
}

/// CanvasRenderer is a  class of the Unity engine since version 4.6.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CanvasRenderer.html):
/**
A component that will render to the screen after all normal rendering has completed when attached to a Canvas. Designed for GUI application.
Additional resources:Canvas.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CanvasRenderer {
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.6.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Indicates whether geometry emitted by this renderer can be ignored when the vertex color alpha is close to zero for every vertex of the mesh.*/
    /// bool: (2018.2.0b1 - 6000.2.0a6)
    pub m_CullTransparentMesh: Option<bool>,
}

/// CapsuleCollider is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CapsuleCollider.html):
/**
A capsule-shaped primitive collider.
Capsules are cylinders with a half-sphere at each end.Additional resources: BoxCollider, SphereCollider, PhysicsMaterial, Rigidbody.
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The height of the capsule measured in the object's local space.*/
    pub m_Height: f32,
    /**Specify if this collider is configured as a trigger.*/
    pub m_IsTrigger: bool,
    /**The material used by the collider.*/
    /// PPtr<[`PhysicMaterial`]>: (3.4.0 - 2023.3.0a8); PPtr<[`PhysicsMaterial`]>: (2023.3.0b1 - 6000.2.0a6)
    pub m_Material: PPtr,
    /**The radius of the sphere, measured in the object's local space.*/
    pub m_Radius: f32,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_ProvidesContacts: Option<bool>,
}

/// CapsuleCollider2D is a  class of the Unity engine since version 5.5.0f3.
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
    /// PPtr<[`GameObject`]>: (5.5.0f3 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    /// PPtr<[`PhysicsMaterial2D`]>: (5.5.0f3 - 6000.2.0a6)
    pub m_Material: PPtr,
    /**The local offset of the collider geometry.*/
    pub m_Offset: Vector2f,
    /**The width and height of the capsule area.*/
    pub m_Size: Vector2f,
    /**Whether the collider is used by an attached effector or not.*/
    pub m_UsedByEffector: bool,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_CallbackLayers: Option<BitField>,
    /**The composite operation to be used by a CompositeCollider2D.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOperation: Option<i32>,
    /**The composite operation order to be used when a CompositeCollider2D is used.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOrder: Option<i32>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /// bool: (5.6.0b1 - 2023.1.0a21)
    pub m_UsedByComposite: Option<bool>,
}

/// Channel is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub attributeName: String,
    pub byteOffset: i32,
    pub curve: AnimationCurve,
}

/// ChannelInfo is a sub class of the Unity engine since version 4.0.0.
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
It will then carry out the movement but be constrained by collisions.Additional resources: Character Controller component and Character animation examples
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterController {
    /**The center of the character's capsule relative to the transform's position.*/
    pub m_Center: Vector3f,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_Enabled: Option<bool>,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**Specify if this collider is configured as a trigger.*/
    /// bool: (5.0.0f4 - 2023.3.0a3)
    pub m_IsTrigger: Option<bool>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /**The material used by the collider.*/
    /// PPtr<[`PhysicMaterial`]>: (5.0.0f4 - 2023.3.0a8); PPtr<[`PhysicsMaterial`]>: (2023.3.0b1 - 6000.2.0a6)
    pub m_Material: Option<PPtr>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_ProvidesContacts: Option<bool>,
}

/// CharacterInfo is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CharacterInfo.html):
/**
Specification for how to render a character from the font texture. See Font.characterInfo.
Additional resources: Example at Font.RequestCharactersInTexture.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterInfo {
    /**Unicode value of the character.*/
    pub index: u32,
    pub uv: Rectf,
    pub vert: Rectf,
    /**The horizontal distance, rounded to the nearest integer, from the origin of this character to the origin of the next character.*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub advance: Option<f32>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub flipped: Option<bool>,
    /// f32: (3.4.0 - 5.2.5f1)
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
    /// PPtr<[`Rigidbody`]>: (3.4.0 - 6000.2.0a6)
    pub m_ConnectedBody: PPtr,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /// bool: (4.3.0 - 6000.2.0a6)
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    /**Position of the anchor relative to the connected Rigidbody.*/
    /// Vector3f: (4.3.0 - 6000.2.0a6)
    pub m_ConnectedAnchor: Option<Vector3f>,
    /**A reference to an articulation body this joint connects to.*/
    /// PPtr<[`ArticulationBody`]>: (2020.2.0b1 - 6000.2.0a6)
    pub m_ConnectedArticulationBody: Option<PPtr>,
    /**The scale to apply to the inverse mass and inertia tensor of the connected body prior to solving the constraints.*/
    /// f32: (2017.1.0f1 - 6000.2.0a6)
    pub m_ConnectedMassScale: Option<f32>,
    /**Enable collision between bodies connected with the joint.*/
    /// bool: (4.5.0 - 6000.2.0a6)
    pub m_EnableCollision: Option<bool>,
    /**Toggle preprocessing for this joint.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_EnablePreprocessing: Option<bool>,
    /**Brings violated constraints back into alignment even when the solver fails.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_EnableProjection: Option<bool>,
    /// bool: (2017.1.0b2 - 2017.1.0b5)
    pub m_Enabled: Option<bool>,
    /**The scale to apply to the inverse mass and inertia tensor of the body prior to solving the constraints.*/
    /// f32: (2017.1.0f1 - 6000.2.0a6)
    pub m_MassScale: Option<f32>,
    /**Set the angular tolerance threshold (in degrees) for projection.*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_ProjectionAngle: Option<f32>,
    /**Set the linear tolerance threshold for projection.*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_ProjectionDistance: Option<f32>,
    /**The configuration of the spring attached to the swing limits of the joint.*/
    /// SoftJointLimitSpring: (5.0.0f4 - 6000.2.0a6)
    pub m_SwingLimitSpring: Option<SoftJointLimitSpring>,
    /**The configuration of the spring attached to the twist limits of the joint.*/
    /// SoftJointLimitSpring: (5.0.0f4 - 6000.2.0a6)
    pub m_TwistLimitSpring: Option<SoftJointLimitSpring>,
}

/// Child is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Child {
    pub m_IsAnim: bool,
    /// PPtr<[`Motion`]>: (4.0.0 - 4.7.2)
    pub m_Motion: PPtr,
    pub m_Threshold: f32,
    pub m_TimeScale: f32,
    /// f32: (4.1.0 - 4.7.2)
    pub m_CycleOffset: Option<f32>,
    /// bool: (4.1.0 - 4.7.2)
    pub m_Mirror: Option<bool>,
    /// Vector2f: (4.1.0 - 4.7.2)
    pub m_Position: Option<Vector2f>,
}

/// ChildAnimatorState is a sub class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.ChildAnimatorState.html):
/**
Structure that represents a state in the context of its parent state machine.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ChildAnimatorState {
    /**The position the the state node in the context of its parent state machine.*/
    pub m_Position: Vector3f,
    /**The state.*/
    /// PPtr<[`AnimatorState`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_State: PPtr,
}

/// ChildAnimatorStateMachine is a sub class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.ChildAnimatorStateMachine.html):
/**
Structure that represents a state machine in the context of its parent state machine.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ChildAnimatorStateMachine {
    /**The position of the state machine node in the context of its parent state machine.*/
    pub m_Position: Vector3f,
    /**The state machine.*/
    /// PPtr<[`AnimatorStateMachine`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_StateMachine: PPtr,
}

/// ChildMotion is a sub class of the Unity engine since version 5.0.0f4.
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
    /// PPtr<[`Motion`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_Motion: PPtr,
    /**The position of the child. Used in 2D blend trees.*/
    pub m_Position: Vector2f,
    /**The threshold of the child. Used in 1D blend trees.*/
    pub m_Threshold: f32,
    /**The relative speed of the child.*/
    pub m_TimeScale: f32,
}

/// CircleCollider2D is a  class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CircleCollider2D.html):
/**
Collider for 2D physics representing an circle.
Additional resources: BoxCollider class, PolygonCollider2D class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CircleCollider2D {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.3.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    /// PPtr<[`PhysicsMaterial2D`]>: (4.3.0 - 6000.2.0a6)
    pub m_Material: PPtr,
    /**Radius of the circle.*/
    pub m_Radius: f32,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_CallbackLayers: Option<BitField>,
    /// Vector2f: (4.3.0 - 4.7.2)
    pub m_Center: Option<Vector2f>,
    /**The composite operation to be used by a CompositeCollider2D.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOperation: Option<i32>,
    /**The composite operation order to be used when a CompositeCollider2D is used.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOrder: Option<i32>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**The density of the collider used to calculate its mass (when auto mass is enabled).*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_Density: Option<f32>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /**The local offset of the collider geometry.*/
    /// Vector2f: (5.0.0f4 - 6000.2.0a6)
    pub m_Offset: Option<Vector2f>,
    /// bool: (5.6.0b1 - 2023.1.0a21)
    pub m_UsedByComposite: Option<bool>,
    /**Whether the collider is used by an attached effector or not.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_UsedByEffector: Option<bool>,
}

/// ClampVelocityModule is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClampVelocityModule {
    pub dampen: f32,
    pub enabled: bool,
    pub magnitude: MinMaxCurve,
    pub separateAxis: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    /// MinMaxCurve: (2017.2.0f1 - 6000.2.0a6)
    pub drag: Option<MinMaxCurve>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub inWorldSpace: Option<bool>,
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub multiplyDragByParticleSize: Option<bool>,
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub multiplyDragByParticleVelocity: Option<bool>,
}

/// ClassInfo is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClassInfo {
    pub m_AssemblyNameIndex: i32,
    pub m_ClassName: String,
    pub m_IsUnityClass: bool,
    pub m_MethodIndex: i32,
    pub m_NamespaceIndex: i32,
    pub m_NumOfMethods: i32,
    /// String: (2019.4.29f1 - 2019.4.40f1)
    pub m_NamespaceName: Option<String>,
}

/// ClassMethodInfo is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClassMethodInfo {
    pub m_ClassIndex: i32,
    pub m_MethodName: String,
    pub m_OrderNumber: i32,
}

/// Clip is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Clip {
    pub m_DenseClip: DenseClip,
    pub m_StreamedClip: StreamedClip,
    /// Box<OffsetPtr>: (4.0.0 - 2018.2.21f1)
    pub m_Binding: Option<Box<OffsetPtr>>,
    /// ConstantClip: (4.3.0 - 6000.2.0a6)
    pub m_ConstantClip: Option<ConstantClip>,
}

/// ClipAnimationInfo is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClipAnimationInfo {
    pub firstFrame: Enum_i32__f32,
    pub lastFrame: Enum_i32__f32,
    pub name: String,
    pub wrapMode: i32,
    /// bool: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "loop")]
    pub _loop: Option<bool>,
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub additiveReferencePoseFrame: Option<f32>,
    /// Vec<u32>: (4.0.0 - 6000.2.0a6)
    pub bodyMask: Option<Vec<u32>>,
    /// Vec<ClipAnimationInfoCurve>: (4.0.0 - 6000.2.0a6)
    pub curves: Option<Vec<ClipAnimationInfoCurve>>,
    /// f32: (4.0.0 - 6000.2.0a6)
    pub cycleOffset: Option<f32>,
    /// Vec<AnimationEvent>: (4.3.0 - 6000.2.0a6)
    pub events: Option<Vec<AnimationEvent>>,
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub hasAdditiveReferencePose: Option<bool>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub heightFromFeet: Option<bool>,
    /// i64: (2019.1.0b1 - 6000.2.0a6)
    pub internalID: Option<i64>,
    /// bool: (4.0.0 - 4.0.1)
    pub keepAdditionalBonesAnimation: Option<bool>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub keepOriginalOrientation: Option<bool>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub keepOriginalPositionXZ: Option<bool>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub keepOriginalPositionY: Option<bool>,
    /// f32: (4.0.0 - 6000.2.0a6)
    pub level: Option<f32>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub loopBlend: Option<bool>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub loopBlendOrientation: Option<bool>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub loopBlendPositionXZ: Option<bool>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub loopBlendPositionY: Option<bool>,
    /// bool: (4.3.0 - 6000.2.0a6)
    pub loopTime: Option<bool>,
    /// PPtr<[`AvatarMask`]>: (4.3.0 - 6000.2.0a6)
    pub maskSource: Option<PPtr>,
    /// i32: (4.3.0 - 6000.2.0a6)
    pub maskType: Option<i32>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub mirror: Option<bool>,
    /// f32: (4.0.0 - 6000.2.0a6)
    pub orientationOffsetY: Option<f32>,
    /// Vec<AvatarSkeletonMaskElement>: (4.0.0 - 4.0.1)
    pub skeletonMaskElements: Option<Vec<AvatarSkeletonMaskElement>>,
    /// String: (4.0.0 - 6000.2.0a6)
    pub takeName: Option<String>,
    /// Vec<TransformMaskElement>: (4.1.0 - 6000.2.0a6)
    pub transformMask: Option<Vec<TransformMaskElement>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_i32__f32 {
    i32(i32),
    f32(f32),
}

/// ClipAnimationInfoCurve is a sub class of the Unity engine since version 4.0.0.
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

/// ClipMuscleConstant is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClipMuscleConstant {
    pub m_AverageAngularSpeed: f32,
    pub m_AverageSpeed: Enum_float4__float3,
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
    pub m_Mirror: bool,
    pub m_OrientationOffsetY: f32,
    pub m_RightFootStartX: xform,
    pub m_StartTime: f32,
    pub m_StartX: xform,
    pub m_StopTime: f32,
    pub m_ValueArrayDelta: Vec<ValueDelta>,
    /// Vec<i32>: (4.0.0 - 4.2.2)
    pub m_AdditionalCurveIndexArray: Option<Vec<i32>>,
    /// bool: (4.3.0 - 6000.2.0a6)
    pub m_LoopTime: Option<bool>,
    /// xform: (4.0.0 - 4.7.2)
    pub m_MotionStartX: Option<xform>,
    /// xform: (4.0.0 - 4.7.2)
    pub m_MotionStopX: Option<xform>,
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub m_StartAtOrigin: Option<bool>,
    /// xform: (5.5.0f3 - 6000.2.0a6)
    pub m_StopX: Option<xform>,
    /// Vec<f32>: (5.3.0f1 - 6000.2.0a6)
    pub m_ValueArrayReferencePose: Option<Vec<f32>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_float4__float3 {
    float4(float4),
    float3(float3),
}

/// Cloth is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Cloth.html):
/**
The Cloth class provides an interface to cloth simulation physics.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Cloth {
    /**Bending stiffness of the cloth.*/
    pub m_BendingStiffness: f32,
    /**An array of CapsuleColliders which this Cloth instance should collide with.*/
    /// Vec<PPtr<[`CapsuleCollider`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_CapsuleColliders: Vec<PPtr>,
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
    /// PPtr<[`GameObject`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**A random, external acceleration applied to the cloth.*/
    pub m_RandomAcceleration: Vector3f,
    /**Cloth's sleep threshold.*/
    pub m_SleepThreshold: f32,
    pub m_SolverFrequency: Enum_u32__f32,
    /**An array of ClothSphereColliderPairs which this Cloth instance should collide with.*/
    /// Vec<(PPtr<[`SphereCollider`]>, PPtr<[`SphereCollider`]>)>: (5.0.0f4 - 2019.1.0a10); Vec<ClothSphereColliderPair>: (2019.1.0b1 - 6000.2.0a6)
    pub m_SphereColliders: Vec<Enum_PPtr__PPtr___ClothSphereColliderPair>,
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
    /// Vec<u32>: (2017.3.0b1 - 6000.2.0a6)
    pub m_SelfAndInterCollisionIndices: Option<Vec<u32>>,
    /**Minimum distance at which two cloth particles repel each other (default: 0.0).*/
    /// f32: (2017.3.0b1 - 6000.2.0a6)
    pub m_SelfCollisionDistance: Option<f32>,
    /**Self-collision stiffness defines how strong the separating impulse should be for colliding particles.*/
    /// f32: (2017.3.0b1 - 6000.2.0a6)
    pub m_SelfCollisionStiffness: Option<f32>,
    /**Add one virtual particle per triangle to improve collision stability.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_UseVirtualParticles: Option<bool>,
    /// Vec<u32>: (2017.3.0b1 - 6000.2.0a6)
    pub m_VirtualParticleIndices: Option<Vec<u32>>,
    /// Vec<Vector3f>: (2017.3.0b1 - 6000.2.0a6)
    pub m_VirtualParticleWeights: Option<Vec<Vector3f>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_PPtr__PPtr___ClothSphereColliderPair {
    PPtr__PPtr((PPtr, PPtr)),
    ClothSphereColliderPair(ClothSphereColliderPair),
}

/// ClothAttachment is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClothAttachment {
    /// PPtr<[`Collider`]>: (3.4.0 - 4.7.2)
    pub m_Collider: PPtr,
    pub m_Tearable: bool,
    pub m_TwoWayInteraction: bool,
}

/// ClothConstrainCoefficients is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClothConstrainCoefficients {
    pub collisionSphereDistance: f32,
    pub maxDistance: f32,
    /// f32: (3.4.0 - 4.7.2)
    pub collisionSphereRadius: Option<f32>,
    /// f32: (3.4.0 - 4.7.2)
    pub maxDistanceBias: Option<f32>,
}

/// ClothRenderer is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClothRenderer {
    pub m_CastShadows: bool,
    pub m_Enabled: bool,
    /// PPtr<[`GameObject`]>: (3.4.0 - 4.7.2)
    pub m_GameObject: PPtr,
    pub m_LightmapIndex: u8,
    pub m_LightmapTilingOffset: Vector4f,
    /// Vec<PPtr<[`Material`]>>: (3.4.0 - 4.7.2)
    pub m_Materials: Vec<PPtr>,
    pub m_PauseWhenNotVisible: bool,
    pub m_ReceiveShadows: bool,
    /// PPtr<[`Transform`]>: (3.4.0 - 4.7.2)
    pub m_StaticBatchRoot: PPtr,
    pub m_SubsetIndices: Vec<u32>,
    /// PPtr<[`Transform`]>: (3.5.0 - 4.7.2)
    pub m_LightProbeAnchor: Option<PPtr>,
    /// i16: (4.3.0 - 4.3.4)
    pub m_SortingLayer: Option<i16>,
    /// u32: (4.5.0 - 4.7.2)
    pub m_SortingLayerID: Option<u32>,
    /// i16: (4.3.0 - 4.7.2)
    pub m_SortingOrder: Option<i16>,
    /// bool: (3.5.0 - 4.7.2)
    pub m_UseLightProbes: Option<bool>,
}

/// ClothSphereColliderPair is a sub class of the Unity engine since version 2019.1.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ClothSphereColliderPair.html):
/**
A pair of SphereColliders used to define shapes for Cloth objects to collide against.
A ClothSphereColliderPair can contain either a single valid SphereCollider instance (with the second one being null), or a pair of two SphereColliders. In the former cases the ClothSphereColliderPair just represents a single SphereCollider for the cloth to collide against. In the latter case, it represents a conic capsule shape defined by the two spheres, and the cone connecting the two. Conic capsule shapes are useful for modelling limbs of a character.Select the cloth object to see a visualization of Cloth colliders shapes in the Scene view.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ClothSphereColliderPair {
    /**The first SphereCollider of a ClothSphereColliderPair.*/
    /// PPtr<[`SphereCollider`]>: (2019.1.0b1 - 6000.2.0a6)
    pub first: PPtr,
    /**The second SphereCollider of a ClothSphereColliderPair.*/
    /// PPtr<[`SphereCollider`]>: (2019.1.0b1 - 6000.2.0a6)
    pub second: PPtr,
}

/// CloudServiceHandlerBehaviour is a  class of the Unity engine since version 5.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct CloudServiceHandlerBehaviour {
    pub m_Enabled: u8,
    /// PPtr<[`GameObject`]>: (5.1.0f1 - 5.1.5f1)
    pub m_GameObject: PPtr,
}

/// CloudWebServicesManager is a  class of the Unity engine since version 5.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct CloudWebServicesManager {}

/// ClusterInput is a sub class of the Unity engine since version 5.3.0f1.
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

/// ClusterInputManager is a  class of the Unity engine since version 5.3.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterInputManager {
    pub m_Inputs: Vec<ClusterInput>,
}

/// CollabEditorSettings is a sub class of the Unity engine since version 2017.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct CollabEditorSettings {
    pub inProgressEnabled: bool,
}

/// CollisionModule is a sub class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.CollisionModule.html):
/**
Script interface for the CollisionModule of a Particle System.
Additional resources: ParticleSystem, ParticleSystem.collision.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CollisionModule {
    /**Specifies whether the CollisionModule is enabled or disabled.*/
    pub enabled: bool,
    /**Kill particles whose speed falls below this threshold, after a collision.*/
    pub minKillSpeed: f32,
    /**The type of particle collision to perform.*/
    /// i32: (3.5.0 - 6000.2.0a6)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /**How much force is applied to each particle after a collision.*/
    /// f32: (3.5.0 - 5.2.5f1)
    pub bounce: Option<f32>,
    /**How much force is applied to a Collider when hit by particles from this Particle System.*/
    /// f32: (2017.1.0b1 - 6000.2.0a6)
    pub colliderForce: Option<f32>,
    /**Control which Layers this Particle System collides with.*/
    /// BitField: (4.0.0 - 6000.2.0a6)
    pub collidesWith: Option<BitField>,
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub collidesWithDynamic: Option<bool>,
    /// bool: (4.2.0 - 6000.2.0a6)
    pub collisionMessages: Option<bool>,
    /// i32: (5.3.0f1 - 6000.2.0a6)
    pub collisionMode: Option<i32>,
    /**How much speed does each particle lose after a collision.*/
    /// f32: (3.5.0 - 5.2.5f1)
    pub dampen: Option<f32>,
    /// f32: (3.5.0 - 5.2.5f1)
    pub energyLossOnCollision: Option<f32>,
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub interiorCollisions: Option<bool>,
    /**How much force is applied to each particle after a collision.*/
    /// MinMaxCurve: (5.3.0f1 - 6000.2.0a6)
    pub m_Bounce: Option<MinMaxCurve>,
    /**How much speed does each particle lose after a collision.*/
    /// MinMaxCurve: (5.3.0f1 - 6000.2.0a6)
    pub m_Dampen: Option<MinMaxCurve>,
    /// MinMaxCurve: (5.3.0f1 - 6000.2.0a6)
    pub m_EnergyLossOnCollision: Option<MinMaxCurve>,
    /// Vec<PPtr<[`Transform`]>>: (2020.2.0b1 - 6000.2.0a6)
    pub m_Planes: Option<Vec<PPtr>>,
    /**The maximum number of collision shapes Unity considers for particle collisions. It ignores excess shapes. Terrains take priority.*/
    /// i32: (5.3.0f1 - 6000.2.0a6)
    pub maxCollisionShapes: Option<i32>,
    /**Kill particles whose speed goes above this threshold, after a collision.*/
    /// f32: (5.4.0f3 - 6000.2.0a6)
    pub maxKillSpeed: Option<f32>,
    /**Specifies whether the physics system considers the collision angle when it applies forces from particles to Colliders.*/
    /// bool: (2017.1.0b1 - 6000.2.0a6)
    pub multiplyColliderForceByCollisionAngle: Option<bool>,
    /**Specifies whether the physics system considers particle sizes when it applies forces to Colliders.*/
    /// bool: (2017.1.0b1 - 6000.2.0a6)
    pub multiplyColliderForceByParticleSize: Option<bool>,
    /**Specifies whether the physics system considers particle speeds when it applies forces to Colliders.*/
    /// bool: (2017.1.0b1 - 6000.2.0a6)
    pub multiplyColliderForceByParticleSpeed: Option<bool>,
    /// f32: (4.0.0 - 5.2.5f1)
    pub particleRadius: Option<f32>,
    /// PPtr<[`Transform`]>: (3.5.0 - 2020.2.0a13)
    pub plane0: Option<PPtr>,
    /// PPtr<[`Transform`]>: (3.5.0 - 2020.2.0a13)
    pub plane1: Option<PPtr>,
    /// PPtr<[`Transform`]>: (3.5.0 - 2020.2.0a13)
    pub plane2: Option<PPtr>,
    /// PPtr<[`Transform`]>: (3.5.0 - 2020.2.0a13)
    pub plane3: Option<PPtr>,
    /// PPtr<[`Transform`]>: (3.5.0 - 2020.2.0a13)
    pub plane4: Option<PPtr>,
    /// PPtr<[`Transform`]>: (3.5.0 - 2020.2.0a13)
    pub plane5: Option<PPtr>,
    /**Specifies the accuracy of particle collisions against colliders in the Scene.*/
    /// i32: (4.0.0 - 6000.2.0a6)
    pub quality: Option<i32>,
    /**A multiplier that Unity applies to the size of each particle before collisions are processed.*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub radiusScale: Option<f32>,
    /**Size of voxels in the collision cache.*/
    /// f32: (4.0.0 - 6000.2.0a6)
    pub voxelSize: Option<f32>,
}

/// ColorBySpeedModule is a sub class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.ColorBySpeedModule.html):
/**
Script interface for the ColorBySpeedModule of a Particle System.
Additional resources: ParticleSystem, ParticleSystem.colorBySpeed.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ColorBySpeedModule {
    /**Specifies whether the ColorBySpeedModule is enabled or disabled.*/
    pub enabled: bool,
    pub gradient: MinMaxGradient,
    /**Apply the color gradient between these minimum and maximum speeds.*/
    pub range: Vector2f,
}

/// ColorModule is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ColorModule {
    pub enabled: bool,
    pub gradient: MinMaxGradient,
}

/// ColorRGBA is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ColorRGBA {
    /// f32: (2018.3.0b1 - 6000.2.0a6)
    pub a: Option<f32>,
    /// f32: (2018.3.0b1 - 6000.2.0a6)
    pub b: Option<f32>,
    /// f32: (2018.3.0b1 - 6000.2.0a6)
    pub g: Option<f32>,
    /// f32: (2018.3.0b1 - 6000.2.0a6)
    pub r: Option<f32>,
    /// u32: (3.4.0 - 2018.2.21f1)
    pub rgba: Option<u32>,
}

/// Component is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Component.html):
/**
Base class for everything attached to a GameObject.
Note that your code will never directly create a Component.  Instead, you write script code, and attach the script to a GameObject.
Additional resources: ScriptableObject as a way to create scripts that do not attach to any GameObject.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
}

/// ComponentPair is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentPair {
    /// PPtr<[`Component`]>: (5.5.0f3 - 6000.2.0a6)
    pub component: PPtr,
}

/// CompositeCollider2D is a  class of the Unity engine since version 5.6.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CompositeCollider2D.html):
/**
A Collider that can merge other Colliders together.
A CompositeCollider2D merges other Colliders together when their Collider2D.compositeOperation is anything other than Collider2D.CompositeOperation.None, that is, whenever a composite operation is selected.When a Composite Collider uses a Collider, the Editor will ignore and not show the Collider2D.sharedMaterial, Collider2D.isTrigger & Collider2D.compositeOperation properties. The same properties on the CompositeCollider2D will be used instead. You should set these properties on the Composite Collider to merge all Colliders into the Composite Collider.NOTE: This Collider2D cannot be disabled/enabled with the Behaviour.enabled property. Any changes to that property will be ignored.
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
    /// PPtr<[`GameObject`]>: (5.6.0b1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Specifies when to generate the Composite Collider geometry.*/
    pub m_GenerationType: i32,
    /**Specifies the type of geometry the Composite Collider should generate.*/
    pub m_GeometryType: i32,
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    /// PPtr<[`PhysicsMaterial2D`]>: (5.6.0b1 - 6000.2.0a6)
    pub m_Material: PPtr,
    /**The local offset of the collider geometry.*/
    pub m_Offset: Vector2f,
    /**Whether the collider is used by an attached effector or not.*/
    pub m_UsedByEffector: bool,
    /**Controls the minimum distance allowed between generated vertices.*/
    pub m_VertexDistance: f32,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_CallbackLayers: Option<BitField>,
    /// PPtr<[`GameObject`]>: (2022.1.18f1 - 6000.2.0a6)
    pub m_CompositeGameObject: Option<PPtr>,
    /**The composite operation to be used by a CompositeCollider2D.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOperation: Option<i32>,
    /**The composite operation order to be used when a CompositeCollider2D is used.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOrder: Option<i32>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**Controls the radius of all edges created by the Collider.*/
    /// f32: (5.6.0f1 - 6000.2.0a6)
    pub m_EdgeRadius: Option<f32>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /**Vertices are offset by this distance when compositing multiple physic shapes. Any vertices between shapes within this distance are combined.*/
    /// f32: (2019.1.3f1 - 6000.2.0a6)
    pub m_OffsetDistance: Option<f32>,
    /**When the value is true, the Collider uses an additional Delaunay triangulation step to produce the Collider mesh. When the value is false, this additional step does not occur.*/
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_UseDelaunayMesh: Option<bool>,
    /// bool: (5.6.0b1 - 2023.1.0a21)
    pub m_UsedByComposite: Option<bool>,
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
    /// PackedBitVector: (3.4.0 - 4.7.2)
    pub m_BindPoses: Option<PackedBitVector>,
    /// PackedBitVector: (3.5.0 - 4.7.2)
    pub m_Colors: Option<PackedBitVector>,
    /// PackedBitVector: (5.0.0f4 - 6000.2.0a6)
    pub m_FloatColors: Option<PackedBitVector>,
    /// u32: (5.0.0f4 - 6000.2.0a6)
    pub m_UVInfo: Option<u32>,
}

/// ComputeBufferCounter is a sub class of the Unity engine since version 5.2.3f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeBufferCounter {
    pub bindpoint: i32,
    pub offset: i32,
}

/// ComputeShader is a  class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ComputeShader.html):
/**
Compute Shader asset.
Compute shaders are programs that run on the GPU outside of the normal rendering pipeline.
They correspond to compute shader assets in the project (.compute files).Compute shader support can be queried runtime using SystemInfo.supportsComputeShaders. See Compute Shaders overview for more info about platforms supporting compute shaders.Additional resources: ComputeBuffer class, Compute Shaders overview.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShader {
    /**The name of the object.*/
    pub m_Name: String,
    /// Vec<ComputeShaderCB>: (4.0.0 - 4.7.2)
    pub constantBuffers: Option<Vec<ComputeShaderCB>>,
    /// Vec<ComputeShaderKernel>: (4.0.0 - 4.7.2)
    pub kernels: Option<Vec<ComputeShaderKernel>>,
    /// Vec<ComputeShaderVariant>: (5.0.0f4 - 2020.1.0a8); Vec<ComputeShaderPlatformVariant>: (2020.1.0b1 - 6000.2.0a6)
    pub variants: Option<Vec<Enum_ComputeShaderPlatformVariant__ComputeShaderVariant>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_ComputeShaderPlatformVariant__ComputeShaderVariant {
    ComputeShaderPlatformVariant(ComputeShaderPlatformVariant),
    ComputeShaderVariant(ComputeShaderVariant),
}

/// ComputeShaderBuiltinSampler is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderBuiltinSampler {
    pub bindPoint: i32,
    pub sampler: i64,
}

/// ComputeShaderCB is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderCB {
    pub byteSize: i32,
    pub name: Enum_FastPropertyName__String,
    pub params: Vec<ComputeShaderParam>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_FastPropertyName__String {
    FastPropertyName(FastPropertyName),
    String(String),
}

/// ComputeShaderImporter is a  class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ComputeShaderImporter.html):
/**
Define compute shader import settings in the Unity Editor.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderImporter {
    /**The name of the object.*/
    pub m_Name: String,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /**Get or set the AssetBundle name.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleName: Option<String>,
    /**Get or set the AssetBundle variant.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleVariant: Option<String>,
    /// u32: (5.3.2f1 - 2020.3.48f1)
    pub m_CurrentAPIMask: Option<u32>,
    /// i32: (5.0.0f4 - 5.3.1f1)
    pub m_CurrentBuildTarget: Option<i32>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /**This property has no effect.*/
    /// i32: (2020.2.0b1 - 2022.1.0a9)
    pub m_PreprocessorOverride: Option<i32>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// ComputeShaderKernel is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderKernel {
    pub builtinSamplers: Vec<ComputeShaderBuiltinSampler>,
    pub cbs: Vec<ComputeShaderResource>,
    pub code: Vec<u8>,
    pub inBuffers: Vec<ComputeShaderResource>,
    pub outBuffers: Vec<ComputeShaderResource>,
    pub textures: Vec<ComputeShaderResource>,
    /// Vec<u32>: (2020.1.0b1 - 6000.2.0a6)
    pub cbVariantIndices: Option<Vec<u32>>,
    /// FastPropertyName: (4.0.0 - 5.5.6f1); String: (5.6.0b1 - 2020.1.0a8)
    pub name: Option<Enum_FastPropertyName__String>,
    /// i64: (2021.1.0b1 - 6000.2.0a6)
    pub requirements: Option<i64>,
    /// Vec<u32>: (5.4.0f3 - 6000.2.0a6)
    pub threadGroupSize: Option<Vec<u32>>,
}

/// ComputeShaderKernelParent is a sub class of the Unity engine since version 2020.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderKernelParent {
    pub name: String,
    /// Vec<String>: (2022.1.0b1 - 6000.2.0a6)
    pub dynamicKeywords: Option<Vec<String>>,
    /// Vec<String>: (2020.2.0b1 - 6000.2.0a6)
    pub globalKeywords: Option<Vec<String>>,
    /// Vec<String>: (2020.2.0b1 - 6000.2.0a6)
    pub localKeywords: Option<Vec<String>>,
    /// Vec<ComputeShaderKernel>: (2022.1.0b1 - 6000.2.0a6)
    pub uniqueVariants: Option<Vec<ComputeShaderKernel>>,
    /// Vec<String>: (2020.1.0b1 - 2020.2.0a13)
    pub validKeywords: Option<Vec<String>>,
    /// Vec<(String, u32)>: (2022.1.0b1 - 6000.2.0a6)
    pub variantIndices: Option<Vec<(String, u32)>>,
    /// Vec<(String, ComputeShaderKernel)>: (2020.1.0b1 - 2022.1.0a13)
    pub variantMap: Option<Vec<(String, ComputeShaderKernel)>>,
}

/// ComputeShaderParam is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderParam {
    pub arraySize: i64,
    pub colCount: i64,
    pub name: Enum_FastPropertyName__String,
    pub offset: i64,
    pub rowCount: i64,
    /// i32: (4.0.0 - 6000.2.0a6)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// ComputeShaderPlatformVariant is a sub class of the Unity engine since version 2020.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderPlatformVariant {
    pub constantBuffers: Vec<ComputeShaderCB>,
    pub kernels: Vec<ComputeShaderKernelParent>,
    pub resourcesResolved: bool,
    pub targetLevel: i32,
    pub targetRenderer: i32,
}

/// ComputeShaderResource is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderResource {
    pub bindPoint: i32,
    pub name: Enum_FastPropertyName__String,
    /// ComputeBufferCounter: (5.2.3f1 - 2017.4.40f1)
    pub counter: Option<ComputeBufferCounter>,
    /// FastPropertyName: (5.1.0f1 - 5.5.6f1); String: (5.6.0b1 - 6000.2.0a6)
    pub generatedName: Option<Enum_FastPropertyName__String>,
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub samplerBindPoint: Option<i32>,
    /// i32: (5.1.0f1 - 5.2.2f1)
    pub secondaryBindPoint: Option<i32>,
    /// i32: (2018.2.0f1 - 6000.2.0a6)
    pub texDimension: Option<i32>,
}

/// ComputeShaderVariant is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShaderVariant {
    pub constantBuffers: Vec<ComputeShaderCB>,
    pub kernels: Vec<ComputeShaderKernel>,
    pub targetLevel: i32,
    pub targetRenderer: i32,
    /// bool: (5.1.0f1 - 2020.1.0a8)
    pub resourcesResolved: Option<bool>,
}

/// Condition is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    pub m_ConditionEvent: String,
    pub m_ConditionMode: i32,
    pub m_EventTreshold: f32,
    pub m_ExitTime: f32,
}

/// ConfigSetting is a sub class of the Unity engine since version 5.4.2f2.
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
    /// PPtr<[`Rigidbody`]>: (3.4.0 - 6000.2.0a6)
    pub m_ConnectedBody: PPtr,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /// SoftJointLimitSpring: (5.0.0f4 - 6000.2.0a6)
    pub m_AngularXLimitSpring: Option<SoftJointLimitSpring>,
    /**The configuration of the spring attached to the angular Y and angular Z limits of the joint.*/
    /// SoftJointLimitSpring: (5.0.0f4 - 6000.2.0a6)
    pub m_AngularYZLimitSpring: Option<SoftJointLimitSpring>,
    /**Should the connectedAnchor be calculated automatically?*/
    /// bool: (4.3.0 - 6000.2.0a6)
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    /**Position of the anchor relative to the connected Rigidbody.*/
    /// Vector3f: (4.3.0 - 6000.2.0a6)
    pub m_ConnectedAnchor: Option<Vector3f>,
    /**A reference to an articulation body this joint connects to.*/
    /// PPtr<[`ArticulationBody`]>: (2020.2.0b1 - 6000.2.0a6)
    pub m_ConnectedArticulationBody: Option<PPtr>,
    /**The scale to apply to the inverse mass and inertia tensor of the connected body prior to solving the constraints.*/
    /// f32: (2017.1.0f1 - 6000.2.0a6)
    pub m_ConnectedMassScale: Option<f32>,
    /**Enable collision between bodies connected with the joint.*/
    /// bool: (4.5.0 - 6000.2.0a6)
    pub m_EnableCollision: Option<bool>,
    /**Toggle preprocessing for this joint.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_EnablePreprocessing: Option<bool>,
    /// bool: (2017.1.0b2 - 2017.1.0b5)
    pub m_Enabled: Option<bool>,
    /**The configuration of the spring attached to the linear limit of the joint.*/
    /// SoftJointLimitSpring: (5.0.0f4 - 6000.2.0a6)
    pub m_LinearLimitSpring: Option<SoftJointLimitSpring>,
    /**The scale to apply to the inverse mass and inertia tensor of the body prior to solving the constraints.*/
    /// f32: (2017.1.0f1 - 6000.2.0a6)
    pub m_MassScale: Option<f32>,
    /**Enable this property to swap the order in which the physics engine processes the Rigidbodies involved in the joint. This results in different joint motion but has no impact on Rigidbodies and anchors.*/
    /// bool: (3.5.0 - 6000.2.0a6)
    pub m_SwapBodies: Option<bool>,
}

/// ConstantBuffer is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantBuffer {
    pub m_MatrixParams: Vec<MatrixParameter>,
    pub m_NameIndex: i32,
    pub m_Size: i32,
    pub m_VectorParams: Vec<VectorParameter>,
    /// bool: (2020.3.2f1 - 6000.2.0a6)
    pub m_IsPartialCB: Option<bool>,
    /// Vec<StructParameter>: (2017.3.0b1 - 6000.2.0a6)
    pub m_StructParams: Option<Vec<StructParameter>>,
}

/// ConstantClip is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantClip {
    pub data: Vec<f32>,
}

/// ConstantForce is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ConstantForce.html):
/**
A force applied constantly.
This is a small physics utility class used to apply a continous force to an object.Rigidbody.AddForce applies a force to the Rigidbody only for one frame, thus you have to keep calling the function.
ConstantForce on the other hand will apply the force every frame until you change the force or torque to a new value.Additional resources: Rigidbody.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantForce {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The force applied to the rigidbody every frame.*/
    pub m_Force: Vector3f,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The force - relative to the rigid bodies coordinate system - applied every frame.*/
    pub m_RelativeForce: Vector3f,
    /**The torque - relative to the rigid bodies coordinate system - applied every frame.*/
    pub m_RelativeTorque: Vector3f,
    /**The torque applied to the rigidbody every frame.*/
    pub m_Torque: Vector3f,
}

/// ConstantForce2D is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ConstantForce2D.html):
/**
Applies both linear and angular (torque) forces continuously to the rigidbody each physics update.
This is equivalent of calling Rigidbody2D.AddForce, Rigidbody2D.AddRelativeForce and Rigidbody2D.AddTorque each physics update.
Additional resources: Rigidbody2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantForce2D {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The linear force applied to the rigidbody each physics update.*/
    pub m_Force: Vector2f,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The linear force, relative to the rigid-body coordinate system, applied each physics update.*/
    pub m_RelativeForce: Vector2f,
    /**The torque applied to the rigidbody each physics update.*/
    pub m_Torque: f32,
}

/// ConstraintSource is a sub class of the Unity engine since version 2018.1.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.ConstraintSource.html):
/**
Represents a weighted position that can be used in a constraint.
Use this struct to provide a weighted position to a constraint that implements the IConstraint interface.
You can use many constraint sources in a constraint. To adjust the effect these sources have on the constraint, set the weight parameter.Additional resources: IConstraint PositionConstraint RotationConstraint ScaleConstraint AimConstraint ParentConstraint
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstraintSource {
    /**The transform component of the source object.*/
    /// PPtr<[`Transform`]>: (2018.1.0f1 - 6000.2.0a6)
    pub sourceTransform: PPtr,
    /**The weight of the source in the evaluation of the constraint.*/
    pub weight: f32,
}

/// ControllerConstant is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ControllerConstant {
    pub m_DefaultValues: OffsetPtr,
    pub m_StateMachineArray: Vec<OffsetPtr>,
    pub m_Values: OffsetPtr,
    /// Vec<OffsetPtr>: (4.0.0 - 4.2.2)
    pub m_HumanLayerArray: Option<Vec<OffsetPtr>>,
    /// Vec<OffsetPtr>: (4.3.0 - 6000.2.0a6)
    pub m_LayerArray: Option<Vec<OffsetPtr>>,
}

/// CrashReportManager is a  class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct CrashReportManager {}

/// CrashReportingSettings is a sub class of the Unity engine since version 5.4.0f3.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/CrashReporting.CrashReportingSettings.html):
/**
Editor API for the Unity Services editor feature. Normally CrashReporting is enabled from the Services window, but if writing your own editor extension, this API can be used.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CrashReportingSettings {
    pub m_EventUrl: String,
    /// bool: (6000.1.0b11 - 6000.1.0b11)
    pub m_EnableCloudDiagnosticsReporting: Option<bool>,
    /// bool: (5.4.0f3 - 6000.2.0a6)
    pub m_Enabled: Option<bool>,
    /// u32: (2018.3.0b1 - 6000.2.0a6)
    pub m_LogBufferSize: Option<u32>,
    /// String: (2017.2.0f1 - 2018.2.21f1)
    pub m_NativeEventUrl: Option<String>,
}

/// Cubemap is a  class of the Unity engine since version 4.0.0.
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
    /**The name of the object.*/
    pub m_Name: String,
    /// Vec<PPtr<[`Texture2D`]>>: (4.0.0 - 6000.2.0a6)
    pub m_SourceTextures: Vec<PPtr>,
    pub m_TextureDimension: i32,
    pub m_TextureFormat: i32,
    pub m_TextureSettings: GLTextureSettings,
    /**Width of the Texture in pixels (Read Only).*/
    pub m_Width: i32,
    /// Vec<u8>: (4.0.0 - 6000.2.0a6)
    #[serde(alias = "image data")]
    pub image_data: Option<Vec<u8>>,
    /// bool: (2017.3.0b1 - 2023.2.0a17)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.3.0b1 - 2023.2.0a17)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2019.3.0f5 - 2023.1.0a6)
    pub m_IgnoreMasterTextureLimit: Option<bool>,
    /// bool: (2022.2.0f1 - 6000.2.0a6)
    pub m_IgnoreMipmapLimit: Option<bool>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// bool: (2019.4.9f1 - 6000.2.0a6)
    pub m_IsPreProcessed: Option<bool>,
    /// i32: (5.2.0f2 - 6000.2.0a6)
    pub m_MipCount: Option<i32>,
    /// bool: (4.0.0 - 5.1.5f1)
    pub m_MipMap: Option<bool>,
    /// String: (2022.2.0f1 - 6000.2.0a6)
    pub m_MipmapLimitGroupName: Option<String>,
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub m_MipsStripped: Option<i32>,
    /// Vec<u8>: (2020.2.0b1 - 6000.2.0a6)
    pub m_PlatformBlob: Option<Vec<u8>>,
    /// bool: (4.0.0 - 5.4.6f3)
    pub m_ReadAllowed: Option<bool>,
    /// StreamingInfo: (5.3.0f1 - 6000.2.0a6)
    pub m_StreamData: Option<StreamingInfo>,
    /**Determines whether mipmap streaming is enabled for this Texture.*/
    /// bool: (2018.2.0b1 - 6000.2.0a6)
    pub m_StreamingMipmaps: Option<bool>,
    /**Sets the relative priority for this Texture when reducing memory size to fit within the memory budget.*/
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub m_StreamingMipmapsPriority: Option<i32>,
}

/// CubemapArray is a  class of the Unity engine since version 5.5.0f3.
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
    pub m_TextureSettings: GLTextureSettings,
    /**Width of the Texture in pixels (Read Only).*/
    pub m_Width: i32,
    /// Vec<u8>: (5.5.0f3 - 6000.2.0a6)
    #[serde(alias = "image data")]
    pub image_data: Option<Vec<u8>>,
    /// bool: (2017.3.0b1 - 2023.2.0a17)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.3.0b1 - 2023.2.0a17)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// StreamingInfo: (5.6.0b1 - 6000.2.0a6)
    pub m_StreamData: Option<StreamingInfo>,
    /// i32: (2020.2.0b1 - 6000.2.0a6)
    pub m_UsageMode: Option<i32>,
}

/// CustomCollider2D is a  class of the Unity engine since version 2021.2.0b1.
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
    /// PPtr<[`GameObject`]>: (2021.2.0b1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    /// PPtr<[`PhysicsMaterial2D`]>: (2021.2.0b1 - 6000.2.0a6)
    pub m_Material: PPtr,
    /**The local offset of the collider geometry.*/
    pub m_Offset: Vector2f,
    /**Whether the collider is used by an attached effector or not.*/
    pub m_UsedByEffector: bool,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_CallbackLayers: Option<BitField>,
    /**The composite operation to be used by a CompositeCollider2D.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOperation: Option<i32>,
    /**The composite operation order to be used when a CompositeCollider2D is used.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOrder: Option<i32>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /// bool: (2021.2.0b1 - 2023.1.0a21)
    pub m_UsedByComposite: Option<bool>,
}

/// CustomDataModule is a sub class of the Unity engine since version 5.6.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.CustomDataModule.html):
/**
Script interface for the CustomDataModule of a Particle System.
Additional resources: ParticleSystem, ParticleSystem.customData.
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

/// CustomRenderTexture is a  class of the Unity engine since version 5.6.0b1.
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
    /**Dimensionality (type) of the render texture.*/
    pub m_Dimension: i32,
    /**When this parameter is set to true, Unity double-buffers the Custom Render Texture so that you can access it during its own update.*/
    pub m_DoubleBuffered: bool,
    pub m_GenerateMips: bool,
    /**The height of the render texture in pixels.*/
    pub m_Height: i32,
    pub m_InitColor: ColorRGBA,
    /// PPtr<[`Material`]>: (5.6.0b1 - 6000.2.0a6)
    pub m_InitMaterial: PPtr,
    /// PPtr<[`Texture`]>: (5.6.0b1 - 6000.2.0a6)
    pub m_InitTexture: PPtr,
    /**Determine how Unity initializes a texture.*/
    pub m_InitializationMode: i32,
    /**The Material that Unity uses to initialize the content of a Custom Render Texture.*/
    /// PPtr<[`Material`]>: (5.6.0b1 - 6000.2.0a6)
    pub m_Material: PPtr,
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
    /**The width of the render texture in pixels.*/
    pub m_Width: i32,
    /**When this parameter is set to true, Unity wraps Update zones around the border of the Custom Render Texture. Otherwise, Unity clamps Update zones at the border of the Custom Render Texture.*/
    pub m_WrapUpdateZones: bool,
    /// bool: (2017.3.0b1 - 6000.2.0a6)
    pub m_BindMS: Option<bool>,
    /// i32: (5.6.0b1 - 2021.2.0a17)
    pub m_DepthFormat: Option<i32>,
    /**The format of the depth/stencil buffer.*/
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_DepthStencilFormat: Option<i32>,
    /// bool: (2017.3.0b1 - 2023.2.0a17)
    pub m_DownscaleFallback: Option<bool>,
    /// bool: (2019.1.0b1 - 6000.2.0a6)
    pub m_EnableCompatibleFormat: Option<bool>,
    /**Enable random access write into this render texture on Shader Model 5.0 level shaders.*/
    /// bool: (2021.3.29f1 - 6000.2.0a6)
    pub m_EnableRandomWrite: Option<bool>,
    /// i32: (2017.3.0b1 - 2023.2.0a17)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// i32: (5.6.0f1 - 6000.2.0a6)
    pub m_InitSource: Option<i32>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// i32: (2019.2.0b1 - 6000.2.0a6)
    pub m_MipCount: Option<i32>,
    /// i32: (2021.2.0f1 - 6000.2.0a6)
    pub m_ShadowSamplingMode: Option<i32>,
    /**When this flag is set to true, render texture is set to be used by the Dynamic Resolution system.*/
    /// bool: (2017.3.0b1 - 6000.2.0a6)
    pub m_UseDynamicScale: Option<bool>,
    /**When this flag is set to true, render texture is set to be used by the Dynamic Resolution system. Scale is applied with an explicit call to ApplyDynamicScale*/
    /// bool: (2023.2.0b1 - 6000.2.0a6)
    pub m_UseDynamicScaleExplicit: Option<bool>,
}

/// DDSImporter is a  class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/DDSImporter.html):
/**
Texture importer lets you modify Texture2D import settings for DDS textures from editor scripts.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DDSImporter {
    /**The name of the object.*/
    pub m_Name: String,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /**Get or set the AssetBundle name.*/
    /// String: (5.0.0f4 - 5.5.6f1)
    pub m_AssetBundleName: Option<String>,
    /**Get or set the AssetBundle variant.*/
    /// String: (5.0.0f4 - 5.5.6f1)
    pub m_AssetBundleVariant: Option<String>,
    /// bool: (5.5.0f3 - 5.5.6f1)
    pub m_IsReadable: Option<bool>,
}

/// DataTemplate is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct DataTemplate {
    /// PPtr<[`DataTemplate`]>: (3.4.0 - 3.4.2)
    pub m_Father: PPtr,
    pub m_IsDataTemplate: bool,
    pub m_LastMergeIdentifier: GUID,
    pub m_Name: String,
    /// Vec<PPtr<[`EditorExtension`]>>: (3.4.0 - 3.4.2)
    pub m_Objects: Vec<PPtr>,
}

/// DateTime is a sub class of the Unity engine since version 2018.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct DateTime {
    pub ticks: i64,
}

/// DefaultAsset is a  class of the Unity engine since version 5.0.0f4.
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

/// DefaultImporter is a  class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultImporter {
    pub m_Name: String,
    pub m_UserData: String,
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleName: Option<String>,
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleVariant: Option<String>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// DefaultPreset is a sub class of the Unity engine since version 2018.1.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Presets.DefaultPreset.html):
/**
This structure defines a default Preset.
See Preset.GetDefaultListForType and Preset.SetDefaultListForType for usage.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultPreset {
    /**The Preset applied to an object instance when it matches the search filter defined by DefaultPreset.m_Filter.*/
    /// PPtr<[`Preset`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Preset: PPtr,
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub m_Disabled: Option<bool>,
    /**The search filter that is compared against the object instance. The DefaultPreset.m_Preset is applied to the object instance if it matches the search filter.*/
    /// String: (2019.3.0b1 - 6000.2.0a6)
    pub m_Filter: Option<String>,
}

/// DefaultPresetList is a sub class of the Unity engine since version 2018.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultPresetList {
    pub defaultPresets: Vec<DefaultPreset>,
    /// PresetType: (2018.1.0f1 - 2019.3.0a8)
    #[serde(alias = "type")]
    pub _type: Option<PresetType>,
}

/// DeletedItem is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedItem {
    pub changeset: i32,
    pub digest: Enum_MdFour__Hash128,
    pub fullPath: String,
    pub guid: GUID,
    pub parent: GUID,
    /// i32: (3.4.0 - 5.6.7f1)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// DenseClip is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct DenseClip {
    pub m_BeginTime: f32,
    pub m_CurveCount: u32,
    pub m_FrameCount: i32,
    pub m_SampleArray: Vec<f32>,
    pub m_SampleRate: f32,
}

/// Derived is a  class of the Unity engine since version 2019.1.0b1.
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
    /// Vec<PPtr<[`Texture2D`]>>: (3.4.0 - 6000.2.0a6)
    pub m_PreloadTextureAtlasData: Vec<PPtr>,
    pub m_TreeInstances: Vec<TreeInstance>,
    pub m_TreePrototypes: Vec<TreePrototype>,
    pub m_WavingGrassAmount: f32,
    pub m_WavingGrassSpeed: f32,
    pub m_WavingGrassStrength: f32,
    /// PPtr<[`Shader`]>: (6000.2.0a6 - 6000.2.0a6)
    #[serde(alias = "m_DefaultShaders[0]")]
    pub m_DefaultShaders_0_: Option<PPtr>,
    /// PPtr<[`Shader`]>: (6000.2.0a6 - 6000.2.0a6)
    #[serde(alias = "m_DefaultShaders[1]")]
    pub m_DefaultShaders_1_: Option<PPtr>,
    /// PPtr<[`Shader`]>: (6000.2.0a6 - 6000.2.0a6)
    #[serde(alias = "m_DefaultShaders[2]")]
    pub m_DefaultShaders_2_: Option<PPtr>,
    /// PPtr<[`Shader`]>: (2019.1.0f2 - 2021.1.28f1)
    pub m_DetailBillboardShader: Option<PPtr>,
    /// PPtr<[`Shader`]>: (2019.1.0f2 - 2021.1.28f1)
    pub m_DetailMeshGrassShader: Option<PPtr>,
    /// PPtr<[`Shader`]>: (2019.1.0f2 - 2021.1.28f1)
    pub m_DetailMeshLitShader: Option<PPtr>,
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_DetailScatterMode: Option<i32>,
    /// Vec<Vector3f>: (3.4.0 - 2020.2.0a21)
    pub m_RandomRotations: Option<Vec<Vector3f>>,
}

/// DetailPatch is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct DetailPatch {
    pub layerIndices: Vec<u8>,
    /// AABB: (3.4.0 - 2020.2.0a21)
    pub bounds: Option<AABB>,
    /// Vec<u8>: (2022.2.0b1 - 6000.2.0a6)
    pub coverage: Option<Vec<u8>>,
    /// Vec<u8>: (3.4.0 - 2022.1.24f1)
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub prototype: PPtr,
    /**Texture used by the DetailPrototype.*/
    /// PPtr<[`Texture2D`]>: (3.4.0 - 6000.2.0a6)
    pub prototypeTexture: PPtr,
    /**Render mode for the DetailPrototype.*/
    pub renderMode: i32,
    /**Indicates whether this detail prototype uses the Mesh object from the GameObject specified by prototype.*/
    pub usePrototypeMesh: i32,
    /**Rotate detail axis parallel to the ground's normal direction, so that the detail is perpendicular to the ground.*/
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub alignToGround: Option<f32>,
    /// f32: (3.4.0 - 2020.2.0a21)
    pub bendFactor: Option<f32>,
    /**Controls detail density for this detail prototype, relative to it's size.*/
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub density: Option<f32>,
    /// f32: (2020.2.0f1 - 6000.2.0a6)
    pub holeTestRadius: Option<f32>,
    /// f32: (3.4.0 - 2021.1.28f1)
    pub lightmapFactor: Option<f32>,
    /**Specifies the random seed value for detail object placement.*/
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub noiseSeed: Option<i32>,
    /**Controls how Unity generates the detail positions.*/
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub positionJitter: Option<f32>,
    /// f32: (2022.2.0a13 - 2022.2.0a13)
    pub positionOrderliness: Option<f32>,
    /**Controls the detail's target coverage.*/
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub targetCoverage: Option<f32>,
    /**Indicates the global density scale set in the terrain settings affects this detail prototype.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub useDensityScaling: Option<i32>,
    /**Indicates whether this detail prototype uses  GPU Instancing for rendering.*/
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub useInstancing: Option<i32>,
}

/// DeviceNone is a sub class of the Unity engine since version 5.6.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceNone {}

/// DifferentMarshallingTestObject is a  class of the Unity engine since version 2023.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct DifferentMarshallingTestObject {}

/// DirectorGenericBinding is a sub class of the Unity engine since version 2017.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectorGenericBinding {
    /// PPtr<[`Object`]>: (2017.1.0b1 - 6000.2.0a6)
    pub key: PPtr,
    /// PPtr<[`Object`]>: (2017.1.0b1 - 6000.2.0a6)
    pub value: PPtr,
}

/// DistanceJoint2D is a  class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/DistanceJoint2D.html):
/**
Joint that keeps two Rigidbody2D objects a fixed distance apart.
Note that unlike the SpringJoint2D component, the distance separating the objects is truly fixed and does not allow for any stretching.Additional resources: HingeJoint2D class, SliderJoint2D class, SpringJoint2D class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DistanceJoint2D {
    /**The joint's anchor point on the object that has the joint component.*/
    pub m_Anchor: Vector2f,
    /**The joint's anchor point on the second object (ie, the one which doesn't have the joint component).*/
    pub m_ConnectedAnchor: Vector2f,
    /// PPtr<[`Rigidbody2D`]>: (4.3.0 - 6000.2.0a6)
    pub m_ConnectedRigidBody: PPtr,
    /**The distance separating the two ends of the joint.*/
    pub m_Distance: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.3.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Should the connectedAnchor be calculated automatically?*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    /**Should the distance be calculated automatically?*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_AutoConfigureDistance: Option<bool>,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_BreakAction: Option<i32>,
    /**The force that needs to be applied for this joint to break.*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_BreakForce: Option<f32>,
    /**The torque that needs to be applied for this joint to break.*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_BreakTorque: Option<f32>,
    /// bool: (4.3.0 - 5.0.0f4)
    pub m_CollideConnected: Option<bool>,
    /**Should the two Rigidbody2D connected with this joint collide with each other?*/
    /// bool: (5.0.1f1 - 6000.2.0a6)
    pub m_EnableCollision: Option<bool>,
    /**Whether to maintain a maximum distance only or not.  If not then the absolute distance will be maintained instead.*/
    /// bool: (4.5.0 - 6000.2.0a6)
    pub m_MaxDistanceOnly: Option<bool>,
}

/// EdgeCollider2D is a  class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/EdgeCollider2D.html):
/**
Collider for 2D physics representing an arbitrary set of connected edges (lines) defined by its vertices.
Additional resources: BoxCollider2D, CircleCollider2D, PolygonCollider2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct EdgeCollider2D {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.3.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    /// PPtr<[`PhysicsMaterial2D`]>: (4.3.0 - 6000.2.0a6)
    pub m_Material: PPtr,
    /**Get or set the points defining multiple continuous edges.*/
    pub m_Points: Vec<Vector2f>,
    /**Defines the position of a virtual point adjacent to the end point of the EdgeCollider2D.*/
    /// Vector2f: (2020.1.0b1 - 6000.2.0a6)
    pub m_AdjacentEndPoint: Option<Vector2f>,
    /**Defines the position of a virtual point adjacent to the start point of the EdgeCollider2D.*/
    /// Vector2f: (2020.1.0b1 - 6000.2.0a6)
    pub m_AdjacentStartPoint: Option<Vector2f>,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_CallbackLayers: Option<BitField>,
    /**The composite operation to be used by a CompositeCollider2D.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOperation: Option<i32>,
    /**The composite operation order to be used when a CompositeCollider2D is used.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOrder: Option<i32>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**The density of the collider used to calculate its mass (when auto mass is enabled).*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_Density: Option<f32>,
    /**Controls the radius of all edges created by the collider.*/
    /// f32: (5.6.0f1 - 6000.2.0a6)
    pub m_EdgeRadius: Option<f32>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /**The local offset of the collider geometry.*/
    /// Vector2f: (5.0.0f4 - 6000.2.0a6)
    pub m_Offset: Option<Vector2f>,
    /**Set this to true to use the adjacentEndPoint to form the collision normal that is used to calculate the collision response when a collision occurs at the Edge Collider's end point. Set this to false to not use the adjacentEndPoint, and the collision normal becomes the direction of motion of the collision.*/
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub m_UseAdjacentEndPoint: Option<bool>,
    /**Set this to true to use the adjacentStartPoint to form the collision normal that is used to calculate the collision response when a collision occurs at the Edge Collider's start point. Set this to false to not use the adjacentStartPoint, and the collision normal becomes the direction of motion of the collision.*/
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub m_UseAdjacentStartPoint: Option<bool>,
    /// bool: (5.6.0b1 - 2023.1.0a21)
    pub m_UsedByComposite: Option<bool>,
    /**Whether the collider is used by an attached effector or not.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_UsedByEffector: Option<bool>,
}

/// EditorBuildSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/EditorBuildSettings.html):
/**
This class allows you to modify the Editor Build Settings via script.
EditorBuildSettings is stored in ProjectSettings/EditorBuildSettings.asset. Typically this file should be included in source control.Scene ListThe ProjectSettings/EditorBuildSettings.asset file contains the global scene list, which defines scenes to be included in the Player build.
This list can be overridden by the active BuildProfile.  For more information see Override settings with build profiles.Config ObjectsEditorBuildSettings can be used to persist references to configuration objects.In this context a config object is an asset, typically a ScriptableObject, that contains configuration settings.
The objects listed in EditorBuildSettings are not automatically included in the build, making them ideal for editor-only settings.
These assets could be accessed by custom build scripts, build callbacks, or any other script running in the editor.API for working with config objects:
- EditorBuildSettings.AddConfigObject
- EditorBuildSettings.RemoveConfigObject
- EditorBuildSettings.TryGetConfigObject
- EditorBuildSettings.GetConfigObjectNames
Config Object Example

Consider a package with a ScriptableObject-derived class for quality settings.
You can customize these settings and create multiple assets, with different values for various contexts.
Use EditorBuildSettings.AddConfigObject to track which asset should be considered as the "active" setting, marked by a distinctive name.
Then a build callback reads settings for the active quality settings by calling EditorBuildSettings.TryGetConfigObject with the designated name.Note: A similar feature is available for config objects that need to be included in the Player build, making them accessible to scripts running in the Player.  See PlayerSettings.SetPreloadedAssets and PlayerSettings.GetPreloadedAssets.Additional resources: EditorBuildSettingsScene, EditorUserBuildSettings, BuildPlayerOptions, SceneManager, IPreprocessBuildWithReport.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorBuildSettings {
    pub m_Scenes: Vec<Scene>,
    /// bool: (2023.1.0b1 - 6000.2.0a6)
    pub m_UseUCBPForAssetBundles: Option<bool>,
    /// Vec<(String, PPtr<[`Object`]>)>: (2018.1.0f1 - 6000.2.0a6)
    pub m_configObjects: Option<Vec<(String, PPtr)>>,
}

/// EditorExtensionImpl is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorExtensionImpl {
    /// Vec<u8>: (3.4.0 - 3.4.2)
    pub gFlattenedTypeTree: Option<Vec<u8>>,
    /// PPtr<[`DataTemplate`]>: (3.4.0 - 3.4.2)
    pub m_DataTemplate: Option<PPtr>,
    /// PPtr<[`EditorExtension`]>: (3.4.0 - 3.4.2)
    pub m_Object: Option<PPtr>,
    /// bitset: (3.4.0 - 3.4.2)
    pub m_OverrideVariable: Option<bitset>,
    /// PPtr<[`EditorExtensionImpl`]>: (3.4.0 - 3.4.2)
    pub m_TemplateFather: Option<PPtr>,
}

/// EditorProjectAccess is a  class of the Unity engine since version 2018.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorProjectAccess {
    pub m_Name: String,
}

/// EditorSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/EditorSettings.html):
/**
User settings for Unity Editor.
Use EditorSettings to apply Editor Project Settings to your Unity Project. You can control settings such as version control, streaming settings, and Asset serialization.Additional resources: Editor Project Settings
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorSettings {
    /// bool: (2020.1.0f1 - 6000.2.0a6)
    pub m_AssetNamingUsesSpace: Option<bool>,
    /// i32: (2019.3.0f1 - 6000.2.0a6)
    pub m_AssetPipelineMode: Option<i32>,
    /// bool: (2019.1.0f2 - 6000.2.0a6)
    pub m_AsyncShaderCompilation: Option<bool>,
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_Bc7TextureCompressor: Option<i32>,
    /// i32: (2021.3.10f1 - 6000.2.0a6)
    pub m_CacheServerDownloadBatchSize: Option<i32>,
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub m_CacheServerEnableAuth: Option<bool>,
    /// bool: (2019.3.0f1 - 6000.2.0a6)
    pub m_CacheServerEnableDownload: Option<bool>,
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub m_CacheServerEnableTls: Option<bool>,
    /// bool: (2019.3.0f1 - 6000.2.0a6)
    pub m_CacheServerEnableUpload: Option<bool>,
    /// String: (2019.3.0f1 - 6000.2.0a6)
    pub m_CacheServerEndpoint: Option<String>,
    /// i32: (2019.3.0f1 - 6000.2.0a6)
    pub m_CacheServerMode: Option<i32>,
    /// String: (2019.3.0f1 - 6000.2.0a6)
    pub m_CacheServerNamespacePrefix: Option<String>,
    /// i32: (2019.4.40f1 - 6000.2.0a6)
    pub m_CacheServerValidationMode: Option<i32>,
    /// bool: (2020.1.0b1 - 2022.1.0a9)
    pub m_CachingShaderPreprocessor: Option<bool>,
    /// CollabEditorSettings: (2017.1.0f1 - 2020.1.0a16)
    pub m_CollabEditorSettings: Option<CollabEditorSettings>,
    /// i32: (4.3.0 - 6000.2.0a6)
    pub m_DefaultBehaviorMode: Option<i32>,
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub m_DisableCookiesInLightmapper: Option<bool>,
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_EnableEditorAsyncCPUTextureLoading: Option<bool>,
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_EnableEnlightenBakedGI: Option<bool>,
    /// bool: (2020.2.0a12 - 2020.2.0a16)
    pub m_EnableRoslynAnalyzers: Option<bool>,
    /// bool: (2019.1.0b1 - 6000.2.0a6)
    pub m_EnableTextureStreamingInEditMode: Option<bool>,
    /// bool: (2018.2.0b1 - 6000.2.0a6)
    pub m_EnableTextureStreamingInPlayMode: Option<bool>,
    /// i32: (2019.3.0b1 - 6000.2.0a6)
    pub m_EnterPlayModeOptions: Option<i32>,
    /// bool: (2019.3.0b1 - 6000.2.0a6)
    pub m_EnterPlayModeOptionsEnabled: Option<bool>,
    /// i32: (2017.2.0f1 - 6000.2.0a6)
    pub m_EtcTextureBestCompressor: Option<i32>,
    /// i32: (2017.2.0f1 - 6000.2.0a6)
    pub m_EtcTextureCompressorBehavior: Option<i32>,
    /// i32: (2017.2.0f1 - 6000.2.0a6)
    pub m_EtcTextureFastCompressor: Option<i32>,
    /// i32: (2017.2.0f1 - 6000.2.0a6)
    pub m_EtcTextureNormalCompressor: Option<i32>,
    /// i32: (3.4.0 - 3.5.7); String: (4.0.0 - 2020.1.0a16)
    pub m_ExternalVersionControlSupport: Option<Enum_i32__String>,
    /// i32: (2020.1.0f1 - 6000.2.0a6)
    pub m_GameObjectNamingDigits: Option<i32>,
    /// i32: (2020.1.0f1 - 6000.2.0a6)
    pub m_GameObjectNamingScheme: Option<i32>,
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_InspectorUseIMGUIDefaultInspector: Option<bool>,
    /// i32: (2017.3.0b1 - 6000.2.0a6)
    pub m_LineEndingsForNewScripts: Option<i32>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_PrefabModeAllowAutoSave: Option<bool>,
    /// PPtr<[`SceneAsset`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_PrefabRegularEnvironment: Option<PPtr>,
    /// PPtr<[`SceneAsset`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_PrefabUIEnvironment: Option<PPtr>,
    /// String: (5.2.0f2 - 6000.2.0a6)
    pub m_ProjectGenerationIncludedExtensions: Option<String>,
    /// String: (5.2.0f2 - 6000.2.0a6)
    pub m_ProjectGenerationRootNamespace: Option<String>,
    /// bool: (2023.2.0a6 - 2023.2.0a17)
    pub m_RecalculateEnvironmentLighting: Option<bool>,
    /// bool: (6000.0.0f1 - 6000.2.0a6)
    pub m_ReferencedClipsExactNaming: Option<bool>,
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_RefreshImportMode: Option<i32>,
    /// i32: (3.5.0 - 6000.2.0a6)
    pub m_SerializationMode: Option<i32>,
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub m_SerializeInlineMappingsOnOneLine: Option<bool>,
    /// bool: (6000.0.45f1 - 6000.2.0a6)
    pub m_ShadowmaskStitching: Option<bool>,
    /// bool: (2019.2.0b1 - 2020.1.0a14)
    pub m_ShowLightmapResolutionOverlay: Option<bool>,
    /// i32: (2022.1.0b1 - 6000.2.0a6)
    pub m_SpritePackerCacheSize: Option<i32>,
    /// i32: (4.3.0 - 6000.2.0a6)
    pub m_SpritePackerMode: Option<i32>,
    /// i32: (5.1.0f1 - 6000.2.0a6)
    pub m_SpritePackerPaddingPower: Option<i32>,
    /// bool: (2019.3.0b1 - 6000.2.0a6)
    pub m_UseLegacyProbeSampleCount: Option<bool>,
    /// String: (5.5.0f3 - 2018.2.21f1)
    pub m_UserGeneratedProjectSuffix: Option<String>,
    /// i32: (3.4.0 - 5.4.6f3)
    pub m_WebSecurityEmulationEnabled: Option<i32>,
    /// String: (3.4.0 - 5.4.6f3)
    pub m_WebSecurityEmulationHostUrl: Option<String>,
}

/// EditorUSerSettings is a  class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorUSerSettings {
    pub m_VCAutomaticAdd: bool,
    pub m_VCDebugCmd: bool,
    pub m_VCDebugCom: bool,
    pub m_VCDebugOut: bool,
    /// Vec<(String, String)>: (4.0.0 - 4.0.1)
    pub m_ConfigValues: Option<Vec<(String, String)>>,
    /// String: (3.5.0 - 3.5.7)
    pub m_VCPassword: Option<String>,
    /// String: (3.5.0 - 3.5.7)
    pub m_VCServer: Option<String>,
    /// String: (3.5.0 - 3.5.7)
    pub m_VCUserName: Option<String>,
    /// String: (3.5.0 - 3.5.7)
    pub m_VCWorkspace: Option<String>,
}

/// EditorUserBuildSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/EditorUserBuildSettings.html):
/**
User build settings for the Editor
EditorUserBuildSettings tracks the build configuration for Player or AssetBundles builds.
For example, the active build target is used to determine which platform the Player is built for.This class is used to store build-related settings that are local to the project, as installed on a specific device.
For example, EditorUserBuildSettings.activeBuildTarget is a setting that could have different values for different people who are using the same Unity project and simultaneously building the Player for different devices.EditorUserBuildSettings is stored in Library/EditorUserBuildSettings.asset, a location which is not intended for inclusion in version control.
Some of these settings can be overridden by the active BuildProfile, see Override settings with build profiles for details.When writing a custom build script it is possible to retrieve the current settings using this class, or to provide different settings, see BuildPlayerOptions and BuildAssetBundlesParameters.The related class EditorBuildSettings exposes build-related settings that are suitable to be backed up and shared through version control.
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
    /// String: (2021.2.0b1 - 6000.2.0a6)
    pub m_ActiveBuildPlatformGroupName: Option<String>,
    /// PPtr<[`MonoBehaviour`]>: (6000.0.20f1 - 6000.2.0a6)
    pub m_ActiveBuildProfile: Option<PPtr>,
    /// i32: (5.6.0b1 - 6000.2.0a6)
    pub m_ActiveBuildTargetGroup: Option<i32>,
    /// GUID: (6000.0.4f1 - 6000.2.0a6)
    pub m_ActivePlatformGuid: Option<GUID>,
    /// String: (6000.0.0f1 - 6000.0.19f1)
    pub m_ActiveProfilePath: Option<String>,
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_ActiveStandaloneBuildSubtarget: Option<i32>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_AndroidBuildSystem: Option<i32>,
    /// i32: (5.6.0f1 - 6000.2.0a6)
    pub m_AndroidBuildType: Option<i32>,
    /// i32: (2021.1.0b1 - 6000.2.0a6)
    pub m_AndroidCreateSymbols: Option<i32>,
    /// bool: (2018.4.13f1 - 2021.1.0a6)
    pub m_AndroidCreateSymbolsZip: Option<bool>,
    /// String: (2018.1.0f1 - 6000.2.0a6)
    pub m_AndroidCurrentDeploymentTargetId: Option<String>,
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub m_AndroidDebugMinification: Option<i32>,
    /// String: (2017.2.0f1 - 6000.2.0a6)
    pub m_AndroidDeviceSocketAddress: Option<String>,
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub m_AndroidReleaseMinification: Option<i32>,
    /// bool: (2018.3.0b1 - 2018.4.36f1)
    pub m_AndroidUseLegacySdkTools: Option<bool>,
    /// bool: (2017.4.17f1 - 6000.2.0a6)
    pub m_BuildAppBundle: Option<bool>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_BuildScriptsOnly: Option<bool>,
    /// bool: (2019.3.0b1 - 6000.2.0a6)
    pub m_BuildWithDeepProfilingSupport: Option<bool>,
    /// bool: (5.5.1f1 - 6000.2.0a6)
    pub m_CompressFilesInPackage: Option<bool>,
    /// bool: (5.0.0f4 - 2021.1.28f1)
    pub m_CompressWithPsArc: Option<bool>,
    /// bool: (5.6.0b1 - 6000.2.0a6)
    pub m_CreateRomFileForSwitch: Option<bool>,
    /// bool: (5.6.0b1 - 2020.1.0a21)
    pub m_CreateSolutionFileForSwitch: Option<bool>,
    /// bool: (2018.2.0b1 - 2019.2.21f1)
    pub m_DatalessPlayer: Option<bool>,
    /// bool: (5.6.2f1 - 6000.2.0a6)
    pub m_EnableDebugPadForSwitch: Option<bool>,
    /// bool: (4.2.0 - 2021.2.0a19)
    pub m_EnableHeadlessMode: Option<bool>,
    /// bool: (2018.3.0b1 - 2023.2.0a15)
    pub m_EnableHeapInspectorForSwitch: Option<bool>,
    /// bool: (2023.3.0b5 - 6000.2.0a6)
    pub m_EnableHostIOForSwitch: Option<bool>,
    /// bool: (2021.3.29f1 - 6000.2.0a6)
    pub m_EnableMemoryTrackerForSwitch: Option<bool>,
    /// bool: (2019.4.38f1 - 6000.2.0a6)
    pub m_EnableRomCompressionForSwitch: Option<bool>,
    /// bool: (2022.3.57f1 - 2022.3.62f1)
    pub m_EnableUnpublishableErrorsForSwitch: Option<bool>,
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub m_ExplicitArrayBoundsChecks: Option<bool>,
    /// bool: (5.4.0f3 - 6000.2.0a6)
    pub m_ExplicitDivideByZeroChecks: Option<bool>,
    /// bool: (3.5.0 - 6000.2.0a6)
    pub m_ExplicitNullChecks: Option<bool>,
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub m_ExportAsGoogleAndroidProject: Option<bool>,
    /// String: (5.6.0f1 - 2019.3.0a10)
    pub m_FacebookAccessToken: Option<String>,
    /// bool: (5.6.0b1 - 2017.2.5f1)
    pub m_FacebookCreatePackageForSubmission: Option<bool>,
    /// bool: (5.6.3f1 - 6000.2.0a6)
    pub m_ForceInstallation: Option<bool>,
    /// bool: (5.2.2f1 - 5.6.7f1)
    pub m_ForceOptimizeScriptCompilation: Option<bool>,
    /// bool: (4.5.0 - 4.7.2)
    pub m_GenerateMetroReferenceProjects: Option<bool>,
    /// bool: (2018.4.35f1 - 6000.2.0a6)
    pub m_GenerateNintendoSwitchShaderInfo: Option<bool>,
    /// bool: (5.0.0f4 - 2018.4.36f1)
    pub m_GenerateWSAReferenceProjects: Option<bool>,
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub m_HTCSScriptDebuggingForSwitch: Option<bool>,
    /// i32: (2021.2.0b1 - 2022.1.0a13)
    pub m_Il2CppCodeGeneration: Option<i32>,
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub m_MovePackageToDiscOuterEdge: Option<bool>,
    /// bool: (2022.3.49f1 - 6000.2.0a6)
    pub m_NVNAftermath: Option<bool>,
    /// bool: (2018.3.0b1 - 2020.1.0a9)
    pub m_NVNDrawValidation: Option<bool>,
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub m_NVNDrawValidationHeavy: Option<bool>,
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub m_NVNDrawValidationLight: Option<bool>,
    /// bool: (5.6.0b1 - 6000.2.0a6)
    pub m_NVNGraphicsDebuggerForSwitch: Option<bool>,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_NVNShaderDebugging: Option<bool>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_NeedSubmissionMaterials: Option<bool>,
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_OverrideMaxTextureSize: Option<i32>,
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_OverrideTextureCompression: Option<i32>,
    /// i32: (5.3.7f1 - 6000.2.0a6)
    pub m_PS4HardwareTarget: Option<i32>,
    /// bool: (2019.4.23f1 - 2019.4.40f1)
    pub m_PS5KeepPackageFiles: Option<bool>,
    /// String: (2019.4.23f1 - 2019.4.40f1)
    pub m_PS5WorkspaceName: Option<String>,
    /// String: (2021.2.14f1 - 6000.2.0a6)
    pub m_PathOnRemoteDevice: Option<String>,
    /// Vec<(String, PlatformSettingsData)>: (5.4.0f3 - 6000.2.0a6)
    pub m_PlatformSettings: Option<Vec<(String, PlatformSettingsData)>>,
    /// bool: (5.6.2f1 - 2023.3.0a18)
    pub m_RedirectWritesToHostMountForSwitch: Option<bool>,
    /// String: (2021.2.14f1 - 6000.2.0a6)
    pub m_RemoteDeviceAddress: Option<String>,
    /// String: (2021.2.14f1 - 6000.2.0a6)
    pub m_RemoteDeviceExports: Option<String>,
    /// bool: (2021.2.14f1 - 6000.2.0a6)
    pub m_RemoteDeviceInfo: Option<bool>,
    /// String: (2021.2.14f1 - 6000.2.0a6)
    pub m_RemoteDeviceUsername: Option<String>,
    /// String: (2019.4.38f1 - 6000.2.0a6)
    pub m_RomCompressionConfigForSwitch: Option<String>,
    /// i32: (2019.4.38f1 - 6000.2.0a6)
    pub m_RomCompressionLevelForSwitch: Option<i32>,
    /// i32: (2019.4.38f1 - 6000.2.0a6)
    pub m_RomCompressionTypeForSwitch: Option<i32>,
    /// bool: (2019.4.38f1 - 6000.2.0a6)
    pub m_SaveADFForSwitch: Option<bool>,
    /// i32: (2017.3.0b1 - 2023.2.0a17)
    pub m_SelectedAndroidETC2Fallback: Option<i32>,
    /// i32: (4.2.0 - 5.3.8f2)
    pub m_SelectedBlackBerryBuildType: Option<i32>,
    /// i32: (4.2.0 - 5.3.8f2)
    pub m_SelectedBlackBerrySubtarget: Option<i32>,
    /// String: (2021.2.0b1 - 6000.2.0a6)
    pub m_SelectedBuildPlatformGroupName: Option<String>,
    /// i32: (2023.3.0b1 - 6000.2.0a6)
    pub m_SelectedBuildTarget: Option<i32>,
    /// Vec<(String, i32)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_SelectedCompressionType: Option<Vec<(String, i32)>>,
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_SelectedEmbeddedLinuxArchitecture: Option<i32>,
    /// i32: (5.6.0b1 - 2019.3.0a10)
    pub m_SelectedFacebookTarget: Option<i32>,
    /// i32: (5.1.0f1 - 6000.2.0a6)
    pub m_SelectedIOSBuildType: Option<i32>,
    /// i32: (4.5.3 - 4.7.2)
    pub m_SelectedMetroBuildAndRunDeployTarget: Option<i32>,
    /// i32: (4.0.0 - 4.7.2)
    pub m_SelectedMetroBuildType: Option<i32>,
    /// i32: (4.3.0 - 4.7.2)
    pub m_SelectedMetroSDK: Option<i32>,
    /// i32: (4.2.0 - 4.7.2)
    pub m_SelectedMetroTarget: Option<i32>,
    /// i32: (4.0.0 - 5.4.1f1)
    pub m_SelectedPS3Subtarget: Option<i32>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SelectedPS4Subtarget: Option<i32>,
    /// i32: (2019.4.23f1 - 2019.4.40f1)
    pub m_SelectedPS5CompressionLevel: Option<i32>,
    /// i32: (2019.4.23f1 - 2019.4.40f1)
    pub m_SelectedPS5CompressionType: Option<i32>,
    /// i32: (2019.4.23f1 - 2019.4.40f1)
    pub m_SelectedPS5Subtarget: Option<i32>,
    /// i32: (5.0.0f4 - 2017.4.40f1)
    pub m_SelectedPSMSubtarget: Option<i32>,
    /// i32: (4.5.0 - 2018.2.21f1)
    pub m_SelectedPSP2Subtarget: Option<i32>,
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_SelectedQNXArchitecture: Option<i32>,
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_SelectedQNXOsVersion: Option<i32>,
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_SelectedStandaloneBuildSubtarget: Option<i32>,
    /// i32: (4.5.0 - 2018.1.9f2)
    pub m_SelectedTizenSubtarget: Option<i32>,
    /// String: (2018.3.0b1 - 6000.2.0a6)
    pub m_SelectedWSAArchitecture: Option<String>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SelectedWSABuildAndRunDeployTarget: Option<i32>,
    /// String: (2018.3.0b1 - 6000.2.0a6)
    pub m_SelectedWSAMinUWPSDK: Option<String>,
    /// i32: (5.0.0f4 - 2017.1.0b1)
    pub m_SelectedWSASDK: Option<i32>,
    /// i32: (5.5.0f3 - 2021.1.28f1)
    pub m_SelectedWSASubtarget: Option<i32>,
    /// i32: (5.2.0f2 - 6000.2.0a6)
    pub m_SelectedWSAUWPBuildType: Option<i32>,
    /// String: (5.5.2f1 - 6000.2.0a6)
    pub m_SelectedWSAUWPSDK: Option<String>,
    /// String: (2017.3.0b1 - 6000.2.0a6)
    pub m_SelectedWSAUWPVSVersion: Option<String>,
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_SelectedWebGLSubtarget: Option<i32>,
    /// i32: (3.5.0 - 4.3.4)
    pub m_SelectedWiiDebugLevel: Option<i32>,
    /// i32: (3.4.0 - 4.3.4)
    pub m_SelectedWiiSubtarget: Option<i32>,
    /// i32: (5.2.0f2 - 2017.4.40f1)
    pub m_SelectedWiiUBootMode: Option<i32>,
    /// i32: (5.2.0f2 - 2017.4.40f1)
    pub m_SelectedWiiUBuildOutput: Option<i32>,
    /// i32: (5.2.0f2 - 2017.4.40f1)
    pub m_SelectedWiiUDebugLevel: Option<i32>,
    /// i32: (2023.1.10f1 - 6000.2.0a6)
    pub m_SelectedWindowsBuildAndRunDeployTarget: Option<i32>,
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub m_SelectedXboxOneDeployDrive: Option<i32>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SelectedXboxOneDeployMethod: Option<i32>,
    /// i32: (3.5.0 - 5.4.6f3)
    pub m_SelectedXboxRunMethod: Option<i32>,
    /// i32: (3.5.0 - 6000.2.0a6)
    pub m_SelectedXboxSubtarget: Option<i32>,
    /// bool: (3.4.0 - 2021.1.28f1)
    pub m_SymlinkLibraries: Option<bool>,
    /// bool: (2021.2.0b1 - 6000.2.0a6)
    pub m_SymlinkSources: Option<bool>,
    /// bool: (5.1.0f1 - 6000.2.0a6)
    pub m_SymlinkTrampoline: Option<bool>,
    /// bool: (2019.4.24f1 - 6000.2.0a6)
    pub m_UseLegacyNvnPoolAllocatorForSwitch: Option<bool>,
    /// Vec<bool>: (5.3.0f1 - 2018.4.36f1)
    pub m_WSADotNetNativeEnabled: Option<Vec<bool>>,
    /// bool: (2019.1.0b1 - 6000.2.0a6)
    pub m_WaitForPlayerConnection: Option<bool>,
    /// bool: (2021.3.29f1 - 6000.2.0a6)
    pub m_WaitForSwitchMemoryTrackerOnStartup: Option<bool>,
    /// String: (2023.2.0b1 - 6000.2.0a6)
    pub m_WebGLClientBrowserPath: Option<String>,
    /// i32: (2023.2.0b1 - 6000.2.0a6)
    pub m_WebGLClientBrowserType: Option<i32>,
    /// i32: (2023.3.0b1 - 6000.2.0a6)
    pub m_WebGLClientPlatform: Option<i32>,
    /// i32: (5.0.0f4 - 5.3.8f2)
    pub m_WebGLOptimizationLevel: Option<i32>,
    /// bool: (5.4.0f3 - 2018.4.36f1)
    pub m_WebGLUsePreBuiltUnityEngine: Option<bool>,
    /// bool: (3.5.0 - 5.1.5f1)
    pub m_WebPlayerDeployOnline: Option<bool>,
    /// bool: (3.4.0 - 3.5.7)
    pub m_WebPlayerNaClSupport: Option<bool>,
    /// bool: (3.4.0 - 2017.1.5f1)
    pub m_WebPlayerOfflineDeployment: Option<bool>,
    /// bool: (3.4.0 - 2017.1.5f1)
    pub m_WebPlayerStreamed: Option<bool>,
    /// bool: (5.2.0f2 - 2017.4.40f1)
    pub m_WiiUEnableNetAPI: Option<bool>,
    /// String: (2019.2.0b1 - 6000.2.0a6)
    pub m_WindowsDevicePortalAddress: Option<String>,
    /// String: (2019.2.0b1 - 6000.2.0a6)
    pub m_WindowsDevicePortalUsername: Option<String>,
    /// bool: (2018.2.0b1 - 2019.1.0a12)
    pub m_WsaHolographicRemoting: Option<bool>,
    /// bool: (3.5.0 - 3.5.7)
    pub m_XboxCompressedXex: Option<bool>,
    /// String: (5.3.0f1 - 2019.1.14f1)
    pub m_XboxOneNetworkSharePath: Option<String>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_XboxOneStreamingInstallLaunchChunkRange: Option<i32>,
    /// String: (5.3.0f1 - 2019.1.14f1)
    pub m_XboxOneUsername: Option<String>,
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_macosXcodeBuildConfig: Option<i32>,
}

/// EditorUserSettings is a  class of the Unity engine since version 4.1.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorUserSettings {
    pub m_VCAutomaticAdd: bool,
    pub m_VCDebugCmd: bool,
    pub m_VCDebugCom: bool,
    pub m_VCDebugOut: bool,
    /// bool: (2021.2.0b1 - 6000.2.0a6)
    pub m_ArtifactGarbageCollection: Option<bool>,
    /// i32: (2019.2.0b1 - 2019.2.21f1)
    pub m_AssetPipelineMode: Option<i32>,
    /// i32: (2019.3.0b1 - 2020.1.0a8)
    pub m_AssetPipelineMode2: Option<i32>,
    /// i32: (2019.2.0b1 - 2020.1.0a8)
    pub m_CacheServerMode: Option<i32>,
    /// Vec<String>: (2019.2.0b1 - 2020.1.0a8)
    pub m_CacheServers: Option<Vec<String>>,
    /// bool: (2023.2.0b1 - 6000.2.0a6)
    pub m_CompressAssetsOnImport: Option<bool>,
    /// Vec<(String, ConfigSetting)>: (5.4.2f2 - 6000.2.0a6)
    pub m_ConfigSettings: Option<Vec<(String, ConfigSetting)>>,
    /// Vec<(String, String)>: (4.1.0 - 5.4.1f1)
    pub m_ConfigValues: Option<Vec<(String, String)>>,
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_DesiredImportWorkerCount: Option<i32>,
    /// i32: (2021.2.0f1 - 6000.2.0a6)
    pub m_IdleImportWorkerShutdownDelay: Option<i32>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SemanticMergeMode: Option<i32>,
    /// i32: (2021.2.0f1 - 6000.2.0a6)
    pub m_StandbyImportWorkerCount: Option<i32>,
    /// bool: (5.6.6f1 - 6000.2.0a6)
    pub m_VCAllowAsyncUpdate: Option<bool>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_VCHierarchyOverlayIcons: Option<bool>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_VCOtherOverlayIcons: Option<bool>,
    /// bool: (2019.3.0b1 - 2020.1.17f1)
    pub m_VCOverlayIcons: Option<bool>,
    /// bool: (2018.4.5f1 - 6000.2.0a6)
    pub m_VCOverwriteFailedCheckoutAssets: Option<bool>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_VCProjectOverlayIcons: Option<bool>,
    /// bool: (2021.3.40f1 - 6000.2.0a6)
    pub m_VCScanLocalPackagesOnConnect: Option<bool>,
    /// bool: (5.1.0f1 - 6000.2.0a6)
    pub m_VCShowFailedCheckout: Option<bool>,
}

/// EffectConstant is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct EffectConstant {
    pub bypass: bool,
    pub groupConstantIndex: u32,
    pub parameterIndices: Vec<u32>,
    pub prevEffectIndex: u32,
    pub sendTargetEffectIndex: u32,
    pub wetMixLevelIndex: u32,
    /// i32: (5.0.0f4 - 6000.2.0a6)
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 2018.2.21f1)
    pub m_GameObject: PPtr,
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
    /// bool: (3.4.0 - 2018.2.21f1)
    #[serde(alias = "Simulate in Worldspace?")]
    pub Simulate_in_Worldspace_: Option<bool>,
}

/// EmbeddedNativeType is a sub class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddedNativeType {
    pub m_FloatArray: Vec<f32>,
    pub m_String: String,
}

/// EmissionModule is a sub class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.EmissionModule.html):
/**
Script interface for the EmissionModule of a Particle System.
Additional resources: ParticleSystem, ParticleSystem.emission.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct EmissionModule {
    /**Specifies whether the EmissionModule is enabled or disabled.*/
    pub enabled: bool,
    /**The current number of bursts.*/
    pub m_BurstCount: i32,
    /// u16: (3.5.0 - 5.3.8f2); i32: (5.4.0f3 - 5.6.0b4)
    pub cnt0: Option<i32>,
    /// u16: (3.5.0 - 5.3.8f2); i32: (5.4.0f3 - 5.6.0b4)
    pub cnt1: Option<i32>,
    /// u16: (3.5.0 - 5.3.8f2); i32: (5.4.0f3 - 5.6.0b4)
    pub cnt2: Option<i32>,
    /// u16: (3.5.0 - 5.3.8f2); i32: (5.4.0f3 - 5.6.0b4)
    pub cnt3: Option<i32>,
    /// u16: (5.3.0f1 - 5.3.8f2); i32: (5.4.0f3 - 5.6.0b4)
    pub cntmax0: Option<i32>,
    /// u16: (5.3.0f1 - 5.3.8f2); i32: (5.4.0f3 - 5.6.0b4)
    pub cntmax1: Option<i32>,
    /// u16: (5.3.0f1 - 5.3.8f2); i32: (5.4.0f3 - 5.6.0b4)
    pub cntmax2: Option<i32>,
    /// u16: (5.3.0f1 - 5.3.8f2); i32: (5.4.0f3 - 5.6.0b4)
    pub cntmax3: Option<i32>,
    /// Vec<ParticleSystemEmissionBurst>: (5.6.0f1 - 6000.2.0a6)
    pub m_Bursts: Option<Vec<ParticleSystemEmissionBurst>>,
    /// i32: (3.5.0 - 5.4.6f3)
    pub m_Type: Option<i32>,
    /// MinMaxCurve: (3.5.0 - 5.4.6f3)
    pub rate: Option<MinMaxCurve>,
    /**The rate at which the emitter spawns new particles over distance.*/
    /// MinMaxCurve: (5.5.0f3 - 6000.2.0a6)
    pub rateOverDistance: Option<MinMaxCurve>,
    /**The rate at which the emitter spawns new particles over time.*/
    /// MinMaxCurve: (5.5.0f3 - 6000.2.0a6)
    pub rateOverTime: Option<MinMaxCurve>,
    /// f32: (3.5.0 - 5.6.0b4)
    pub time0: Option<f32>,
    /// f32: (3.5.0 - 5.6.0b4)
    pub time1: Option<f32>,
    /// f32: (3.5.0 - 5.6.0b4)
    pub time2: Option<f32>,
    /// f32: (3.5.0 - 5.6.0b4)
    pub time3: Option<f32>,
}

/// EmptyObject is a  class of the Unity engine since version 2019.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyObject {}

/// EnlightenRendererInformation is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnlightenRendererInformation {
    pub dynamicLightmapSTInSystem: Vector4f,
    pub instanceHash: Hash128,
    /// PPtr<[`Object`]>: (5.0.0f4 - 6000.2.0a6)
    pub renderer: PPtr,
    pub systemId: i32,
}

/// EnlightenSceneMapping is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnlightenSceneMapping {
    pub m_Renderers: Vec<EnlightenRendererInformation>,
    pub m_SystemAtlases: Vec<EnlightenSystemAtlasInformation>,
    pub m_Systems: Vec<EnlightenSystemInformation>,
    pub m_TerrainChunks: Vec<EnlightenTerrainChunksInformation>,
    /// Vec<Hash128>: (5.3.0f1 - 6000.2.0a6)
    pub m_Probesets: Option<Vec<Hash128>>,
}

/// EnlightenSystemAtlasInformation is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnlightenSystemAtlasInformation {
    pub atlasHash: Hash128,
    pub atlasSize: i32,
    pub firstSystemId: i32,
}

/// EnlightenSystemInformation is a sub class of the Unity engine since version 5.0.0f4.
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

/// EnlightenTerrainChunksInformation is a sub class of the Unity engine since version 5.0.0f4.
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

/// ExposedReferenceTable is a sub class of the Unity engine since version 2017.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExposedReferenceTable {
    /// Vec<(String, PPtr<[`Object`]>)>: (2017.1.0b1 - 6000.2.0a6)
    pub m_References: Vec<(String, PPtr)>,
}

/// Expression is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct Expression {
    pub op: i32,
    pub valueIndex: i32,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    #[serde(alias = "data[0]")]
    pub data_0_: Option<i32>,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    #[serde(alias = "data[1]")]
    pub data_1_: Option<i32>,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    #[serde(alias = "data[2]")]
    pub data_2_: Option<i32>,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    #[serde(alias = "data[3]")]
    pub data_3_: Option<i32>,
}

/// ExtensionPropertyValue is a sub class of the Unity engine since version 2017.2.0b2.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionPropertyValue {
    pub extensionName: String,
    pub pluginName: String,
    pub propertyName: String,
    pub propertyValue: f32,
}

/// ExternalForcesModule is a sub class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.ExternalForcesModule.html):
/**
Script interface for the ExternalForcesModule of a Particle System.
Additional resources: ParticleSystem, ParticleSystem.externalForces.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalForcesModule {
    /**Specifies whether the ExternalForcesModule is enabled or disabled.*/
    pub enabled: bool,
    /**Apply all Force Fields belonging to a matching Layer to this Particle System.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub influenceFilter: Option<i32>,
    /// Vec<PPtr<[`ParticleSystemForceField`]>>: (2018.3.0b1 - 6000.2.0a6)
    pub influenceList: Option<Vec<PPtr>>,
    /**Particle System Force Field Components with a matching Layer affect this Particle System.*/
    /// BitField: (2018.3.0b1 - 6000.2.0a6)
    pub influenceMask: Option<BitField>,
    /**Multiplies the magnitude of external forces affecting the particles.*/
    /// f32: (4.0.0 - 2019.2.0a8)
    pub multiplier: Option<f32>,
    /**Multiplies the magnitude of applied external forces.*/
    /// MinMaxCurve: (2019.1.0f2 - 6000.2.0a6)
    pub multiplierCurve: Option<MinMaxCurve>,
}

/// FalloffTable is a sub class of the Unity engine since version 2017.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct FalloffTable {
    /// f32: (2017.1.0b1 - 2017.1.0b10)
    #[serde(alias = "m_Table[0]")]
    pub m_Table_0_: Option<f32>,
    /// f32: (2017.1.0b1 - 2017.1.0b10)
    #[serde(alias = "m_Table[10]")]
    pub m_Table_10_: Option<f32>,
    /// f32: (2017.1.0b1 - 2017.1.0b10)
    #[serde(alias = "m_Table[11]")]
    pub m_Table_11_: Option<f32>,
    /// f32: (2017.1.0b1 - 2017.1.0b10)
    #[serde(alias = "m_Table[12]")]
    pub m_Table_12_: Option<f32>,
    /// f32: (2017.1.0b1 - 2017.1.0b10)
    #[serde(alias = "m_Table[1]")]
    pub m_Table_1_: Option<f32>,
    /// f32: (2017.1.0b1 - 2017.1.0b10)
    #[serde(alias = "m_Table[2]")]
    pub m_Table_2_: Option<f32>,
    /// f32: (2017.1.0b1 - 2017.1.0b10)
    #[serde(alias = "m_Table[3]")]
    pub m_Table_3_: Option<f32>,
    /// f32: (2017.1.0b1 - 2017.1.0b10)
    #[serde(alias = "m_Table[4]")]
    pub m_Table_4_: Option<f32>,
    /// f32: (2017.1.0b1 - 2017.1.0b10)
    #[serde(alias = "m_Table[5]")]
    pub m_Table_5_: Option<f32>,
    /// f32: (2017.1.0b1 - 2017.1.0b10)
    #[serde(alias = "m_Table[6]")]
    pub m_Table_6_: Option<f32>,
    /// f32: (2017.1.0b1 - 2017.1.0b10)
    #[serde(alias = "m_Table[7]")]
    pub m_Table_7_: Option<f32>,
    /// f32: (2017.1.0b1 - 2017.1.0b10)
    #[serde(alias = "m_Table[8]")]
    pub m_Table_8_: Option<f32>,
    /// f32: (2017.1.0b1 - 2017.1.0b10)
    #[serde(alias = "m_Table[9]")]
    pub m_Table_9_: Option<f32>,
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
Additional resources: CharacterJoint, HingeJoint, SpringJoint, ConfigurableJoint.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct FixedJoint {
    /**The force that needs to be applied for this joint to break.*/
    pub m_BreakForce: f32,
    /**The torque that needs to be applied for this joint to break. To be able to break, a joint must be _Locked_ or _Limited_ on the axis of rotation where the torque is being applied. This means that some joints cannot break, such as an unconstrained Configurable Joint.*/
    pub m_BreakTorque: f32,
    /**A reference to another rigidbody this joint connects to.*/
    /// PPtr<[`Rigidbody`]>: (3.4.0 - 6000.2.0a6)
    pub m_ConnectedBody: PPtr,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**A reference to an articulation body this joint connects to.*/
    /// PPtr<[`ArticulationBody`]>: (2020.2.0b1 - 6000.2.0a6)
    pub m_ConnectedArticulationBody: Option<PPtr>,
    /**The scale to apply to the inverse mass and inertia tensor of the connected body prior to solving the constraints.*/
    /// f32: (2017.1.0f1 - 6000.2.0a6)
    pub m_ConnectedMassScale: Option<f32>,
    /**Enable collision between bodies connected with the joint.*/
    /// bool: (4.5.0 - 6000.2.0a6)
    pub m_EnableCollision: Option<bool>,
    /**Toggle preprocessing for this joint.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_EnablePreprocessing: Option<bool>,
    /// bool: (2017.1.0b2 - 2017.1.0b5)
    pub m_Enabled: Option<bool>,
    /**The scale to apply to the inverse mass and inertia tensor of the body prior to solving the constraints.*/
    /// f32: (2017.1.0f1 - 6000.2.0a6)
    pub m_MassScale: Option<f32>,
}

/// FixedJoint2D is a  class of the Unity engine since version 5.3.0f1.
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
    /// PPtr<[`Rigidbody2D`]>: (5.3.0f1 - 6000.2.0a6)
    pub m_ConnectedRigidBody: PPtr,
    /**The amount by which the spring force is reduced in proportion to the movement speed.*/
    pub m_DampingRatio: f32,
    /**Should the two Rigidbody2D connected with this joint collide with each other?*/
    pub m_EnableCollision: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The frequency at which the spring oscillates around the distance between the objects.*/
    pub m_Frequency: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (5.3.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_BreakAction: Option<i32>,
}

/// Flare is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Flare.html):
/**
A flare asset. Read more about flares in the components reference.
The flare class has no properties. It needs to be setup up in the inspector.
You can reference flares and assign them to a Light at runtime.Additional resources: Flare assets, LensFlare class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Flare {
    pub m_Elements: Vec<FlareElement>,
    /// PPtr<[`Texture`]>: (3.4.0 - 6000.2.0a6)
    pub m_FlareTexture: PPtr,
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
    /// PPtr<[`MonoScript`]>: (3.4.0 - 6000.2.0a6)
    pub script: PPtr,
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub flags: Option<i32>,
}

/// Font is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Font.html):
/**
Script interface for font assets.
You can use this class to dynamically switch fonts on Text Meshes.Additional resources: TextMesh.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Font {
    /**The ascent of the font.*/
    pub m_Ascent: f32,
    pub m_AsciiStartOffset: i32,
    pub m_CharacterRects: Vec<CharacterInfo>,
    pub m_ConvertCase: i32,
    /// PPtr<[`Material`]>: (3.4.0 - 6000.2.0a6)
    pub m_DefaultMaterial: PPtr,
    pub m_DefaultStyle: u32,
    pub m_FontData: Vec<char>,
    pub m_FontNames: Vec<String>,
    /**The default size of the font.*/
    pub m_FontSize: f32,
    pub m_KerningValues: Vec<((u16, u16), f32)>,
    pub m_LineSpacing: f32,
    /**The name of the object.*/
    pub m_Name: String,
    /// PPtr<[`Texture`]>: (3.4.0 - 6000.2.0a6)
    pub m_Texture: PPtr,
    /// i32: (4.0.0 - 6000.2.0a6)
    pub m_CharacterPadding: Option<i32>,
    /// i32: (4.0.0 - 6000.2.0a6)
    pub m_CharacterSpacing: Option<i32>,
    /// f32: (5.4.0f3 - 6000.2.0a6)
    pub m_Descent: Option<f32>,
    /// Vec<PPtr<[`Font`]>>: (4.0.0 - 6000.2.0a6)
    pub m_FallbackFonts: Option<Vec<PPtr>>,
    /// i32: (3.4.0 - 3.5.7)
    pub m_FontCountX: Option<i32>,
    /// i32: (3.4.0 - 3.5.7)
    pub m_FontCountY: Option<i32>,
    /// i32: (4.0.0 - 6000.2.0a6)
    pub m_FontRenderingMode: Option<i32>,
    /// bool: (3.4.0 - 3.5.7)
    pub m_GridFont: Option<bool>,
    /// f32: (3.4.0 - 5.2.5f1)
    pub m_Kerning: Option<f32>,
    /// Vec<(i32, f32)>: (3.4.0 - 3.5.7)
    pub m_PerCharacterKerning: Option<Vec<(i32, f32)>>,
    /// f32: (4.0.0 - 6000.2.0a6)
    pub m_PixelScale: Option<f32>,
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub m_ShouldRoundAdvanceValue: Option<bool>,
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_Tracking: Option<f32>,
    /// bool: (5.6.5f1 - 6000.2.0a6)
    pub m_UseLegacyBoundsCalculation: Option<bool>,
}

/// ForceModule is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ForceModule {
    pub enabled: bool,
    pub inWorldSpace: bool,
    pub randomizePerFrame: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
}

/// FrictionJoint2D is a  class of the Unity engine since version 5.3.0f1.
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
    /// PPtr<[`Rigidbody2D`]>: (5.3.0f1 - 6000.2.0a6)
    pub m_ConnectedRigidBody: PPtr,
    /**Should the two Rigidbody2D connected with this joint collide with each other?*/
    pub m_EnableCollision: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (5.3.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The maximum force that can be generated when trying to maintain the friction joint constraint.*/
    pub m_MaxForce: f32,
    /**The maximum torque that can be generated when trying to maintain the friction joint constraint.*/
    pub m_MaxTorque: f32,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_BreakAction: Option<i32>,
}

/// GISettings is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct GISettings {
    pub m_AlbedoBoost: f32,
    pub m_BounceScale: f32,
    pub m_EnableBakedLightmaps: bool,
    pub m_EnableRealtimeLightmaps: bool,
    pub m_EnvironmentLightingMode: u32,
    pub m_IndirectOutputScale: f32,
    /// f32: (5.0.0f4 - 2018.2.21f1)
    pub m_TemporalCoherenceThreshold: Option<f32>,
}

/// GLTextureSettings is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct GLTextureSettings {
    pub m_Aniso: i32,
    pub m_FilterMode: i32,
    pub m_MipBias: f32,
    /// i32: (3.4.0 - 5.6.7f1)
    pub m_WrapMode: Option<i32>,
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub m_WrapU: Option<i32>,
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub m_WrapV: Option<i32>,
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub m_WrapW: Option<i32>,
}

/// GUID is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct GUID {
    /// u32: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "data[0]")]
    pub data_0_: Option<u32>,
    /// u32: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "data[1]")]
    pub data_1_: Option<u32>,
    /// u32: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "data[2]")]
    pub data_2_: Option<u32>,
    /// u32: (3.4.0 - 6000.2.0a6)
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
    /// PPtr<[`Font`]>: (3.4.0 - 2019.3.0a3)
    pub m_Font: PPtr,
    pub m_FontSize: i32,
    pub m_FontStyle: i32,
    /// PPtr<[`GameObject`]>: (3.4.0 - 2019.3.0a3)
    pub m_GameObject: PPtr,
    pub m_LineSpacing: f32,
    /// PPtr<[`Material`]>: (3.4.0 - 2019.3.0a3)
    pub m_Material: PPtr,
    pub m_PixelCorrect: bool,
    pub m_PixelOffset: Vector2f,
    pub m_TabSize: f32,
    pub m_Text: String,
    /// ColorRGBA: (4.2.0 - 2019.3.0a3)
    pub m_Color: Option<ColorRGBA>,
    /// bool: (4.0.0 - 2019.3.0a3)
    pub m_RichText: Option<bool>,
}

/// GUITexture is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct GUITexture {
    pub m_BottomBorder: i32,
    pub m_Color: ColorRGBA,
    pub m_Enabled: u8,
    /// PPtr<[`GameObject`]>: (3.4.0 - 2019.3.0a3)
    pub m_GameObject: PPtr,
    pub m_LeftBorder: i32,
    pub m_PixelInset: Rectf,
    pub m_RightBorder: i32,
    /// PPtr<[`Texture`]>: (3.4.0 - 2019.3.0a3)
    pub m_Texture: PPtr,
    pub m_TopBorder: i32,
}

/// GameManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameManager {}

/// GameObject is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/GameObject.html):
/**
Base class for all objects that can exist in a scene. Add components to a GameObject to control its appearance and behavior.
The GameObject is the fundamental object type in Unity. Use GameObject to represent everything in your project, including characters, props, and scenery. A GameObject acts as a container for functional components that determine how the GameObject looks and behaves.Any script that derives from MonoBehaviour can be added to a GameObject as a component. Use the Component.gameObject property from your MonoBehaviour code to access the GameObject the component is attached to.MonoBehaviour event functions such as the regular per-frame MonoBehaviour.Update allow you to make the object responsive to events. To receive these event callbacks the GameObject must be active in the scene, which means both the activeSelf and activeInHierarchy properties must be true.You can create an empty GameObject from code by invoking one of the constructors. However, a more common method is to instantiate a GameObject in Prefab form, with preconfigured components, property values, and child objects. For more information, refer to Instantiating Prefabs at runtime in the Manual.You can modify many of the properties of this class in the Editor using the Inspector window. For a more comprehensive guide to using the GameObject class, refer to GameObject in the Manual.The following example creates a GameObject named "myExampleGO" and adds a component of type AudioSource:
Additional resources: Component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct GameObject {
    /// Vec<(i32, PPtr<[`Component`]>)>: (3.4.0 - 5.4.6f3); Vec<ComponentPair>: (5.5.0f3 - 6000.2.0a6)
    pub m_Component: Vec<Enum_ComponentPair___i32__PPtr>,
    pub m_IsActive: Enum_u8__bool,
    /**Integer identifying the layer the GameObject is assigned to.*/
    pub m_Layer: u32,
    /**The name of the object.*/
    pub m_Name: String,
    /**The tag assigned to the GameObject.*/
    pub m_Tag: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_ComponentPair___i32__PPtr {
    ComponentPair(ComponentPair),
    i32__PPtr((i32, PPtr)),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_u8__bool {
    u8(u8),
    bool(bool),
}

/// GameObjectRecorder is a  class of the Unity engine since version 2017.1.0b1.
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

/// GenericBinding is a sub class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Animations.GenericBinding.html):
/**
Defines an animatable property on a Unity Component.
GenericBinding and BoundProperty are classes for animating properties on objects in a completely generic way.See also GenericBindingUtility.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericBinding {
    pub attribute: u32,
    pub customType: u8,
    pub isPPtrCurve: u8,
    pub path: u32,
    /// PPtr<[`Object`]>: (4.3.0 - 6000.2.0a6)
    pub script: PPtr,
    /// u16: (4.3.0 - 5.5.6f1)
    pub classID: Option<u16>,
    /// u8: (2022.1.0b1 - 6000.2.0a6)
    pub isIntCurve: Option<u8>,
    /// u8: (2022.2.0b1 - 6000.2.0a6)
    pub isSerializeReferenceCurve: Option<u8>,
    /**The Unity component type ID.*/
    /// i32: (5.6.0b1 - 6000.2.0a6)
    pub typeID: Option<i32>,
}

/// GfxBlendState is a sub class of the Unity engine since version 6000.0.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct GfxBlendState {
    pub alphaToMask: u8,
    pub rt: Vec<GfxRenderTargetBlendState>,
    pub separateMRTBlend: u8,
}

/// GfxDepthState is a sub class of the Unity engine since version 6000.0.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct GfxDepthState {
    pub depthFunc: i8,
    pub depthWrite: u8,
}

/// GfxRasterState is a sub class of the Unity engine since version 6000.0.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct GfxRasterState {
    pub conservative: u8,
    pub cullMode: i32,
    pub depthBias: i32,
    pub depthClip: u8,
    pub slopeScaledDepthBias: f32,
}

/// GfxRenderTargetBlendState is a sub class of the Unity engine since version 6000.0.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct GfxRenderTargetBlendState {
    pub blendOp: u8,
    pub blendOpAlpha: u8,
    pub dstBlend: u8,
    pub dstBlendAlpha: u8,
    pub srcBlend: u8,
    pub srcBlendAlpha: u8,
    pub writeMask: u8,
}

/// GfxStencilState is a sub class of the Unity engine since version 6000.0.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct GfxStencilState {
    pub padding: u8,
    pub readMask: u8,
    pub stencilEnable: u8,
    pub stencilFailOpBack: u8,
    pub stencilFailOpFront: u8,
    pub stencilFuncBack: u8,
    pub stencilFuncFront: u8,
    pub stencilPassOpBack: u8,
    pub stencilPassOpFront: u8,
    pub stencilZFailOpBack: u8,
    pub stencilZFailOpFront: u8,
    pub writeMask: u8,
}

/// GlobalGameManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalGameManager {}

/// Google is a sub class of the Unity engine since version 5.6.0b1.
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
Additional resources: GradientColorKey, GradientAlphaKey, SerializedProperty.gradientValue.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Gradient {
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub atime0: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub atime1: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub atime2: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub atime3: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub atime4: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub atime5: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub atime6: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub atime7: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub ctime0: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub ctime1: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub ctime2: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub ctime3: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub ctime4: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub ctime5: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub ctime6: Option<u16>,
    /// u16: (5.5.0f3 - 6000.2.0a6)
    pub ctime7: Option<u16>,
    /// ColorRGBA: (5.5.0f3 - 6000.2.0a6)
    pub key0: Option<ColorRGBA>,
    /// ColorRGBA: (5.5.0f3 - 6000.2.0a6)
    pub key1: Option<ColorRGBA>,
    /// ColorRGBA: (5.5.0f3 - 6000.2.0a6)
    pub key2: Option<ColorRGBA>,
    /// ColorRGBA: (5.5.0f3 - 6000.2.0a6)
    pub key3: Option<ColorRGBA>,
    /// ColorRGBA: (5.5.0f3 - 6000.2.0a6)
    pub key4: Option<ColorRGBA>,
    /// ColorRGBA: (5.5.0f3 - 6000.2.0a6)
    pub key5: Option<ColorRGBA>,
    /// ColorRGBA: (5.5.0f3 - 6000.2.0a6)
    pub key6: Option<ColorRGBA>,
    /// ColorRGBA: (5.5.0f3 - 6000.2.0a6)
    pub key7: Option<ColorRGBA>,
    /**Indicates the color space that the gradient color keys are using.*/
    /// i8: (2022.2.0b1 - 6000.2.0a6)
    pub m_ColorSpace: Option<i8>,
    /// ColorRGBA: (3.4.0 - 5.4.6f3)
    #[serde(alias = "m_Color[0]")]
    pub m_Color_0_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 5.4.6f3)
    #[serde(alias = "m_Color[1]")]
    pub m_Color_1_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 5.4.6f3)
    #[serde(alias = "m_Color[2]")]
    pub m_Color_2_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 5.4.6f3)
    #[serde(alias = "m_Color[3]")]
    pub m_Color_3_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 5.4.6f3)
    #[serde(alias = "m_Color[4]")]
    pub m_Color_4_: Option<ColorRGBA>,
    /**Controls how the gradient colors are interpolated.*/
    /// i32: (5.5.0f3 - 2022.1.24f1); u8: (2022.2.0b1 - 6000.2.0a6)
    pub m_Mode: Option<i32>,
    /// u8: (5.5.0f3 - 6000.2.0a6)
    pub m_NumAlphaKeys: Option<u8>,
    /// u8: (5.5.0f3 - 6000.2.0a6)
    pub m_NumColorKeys: Option<u8>,
}

/// GradientNEW is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct GradientNEW {
    pub atime0: u16,
    pub atime1: u16,
    pub atime2: u16,
    pub atime3: u16,
    pub atime4: u16,
    pub atime5: u16,
    pub atime6: u16,
    pub atime7: u16,
    pub ctime0: u16,
    pub ctime1: u16,
    pub ctime2: u16,
    pub ctime3: u16,
    pub ctime4: u16,
    pub ctime5: u16,
    pub ctime6: u16,
    pub ctime7: u16,
    pub key0: ColorRGBA,
    pub key1: ColorRGBA,
    pub key2: ColorRGBA,
    pub key3: ColorRGBA,
    pub key4: ColorRGBA,
    pub key5: ColorRGBA,
    pub key6: ColorRGBA,
    pub key7: ColorRGBA,
    pub m_NumAlphaKeys: u8,
    pub m_NumColorKeys: u8,
}

/// GraphicsSettings is a  class of the Unity engine since version 4.2.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rendering.GraphicsSettings.html):
/**
Script interface for Graphics Settings.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphicsSettings {
    /// Vec<PPtr<[`Shader`]>>: (4.2.0 - 6000.2.0a6)
    pub m_AlwaysIncludedShaders: Vec<PPtr>,
    /// bool: (2019.3.0b1 - 2020.1.0a7)
    pub m_AllowEnlightenSupportForUpgradedProject: Option<bool>,
    /// bool: (2020.3.45f1 - 6000.2.0a6)
    pub m_CameraRelativeLightCulling: Option<bool>,
    /// bool: (2020.3.45f1 - 6000.2.0a6)
    pub m_CameraRelativeShadowCulling: Option<bool>,
    /// PPtr<[`Object`]>: (2023.2.0b1 - 6000.2.0a6)
    pub m_CurrentRenderPipelineGlobalSettings: Option<PPtr>,
    /// PPtr<[`MonoBehaviour`]>: (5.6.0f1 - 6000.2.0a6)
    pub m_CustomRenderPipeline: Option<PPtr>,
    /// u32: (2020.2.0b1 - 2023.3.0a17)
    pub m_DefaultRenderingLayerMask: Option<u32>,
    /// BuiltinShaderSettings: (5.0.0f4 - 6000.2.0a6)
    pub m_Deferred: Option<BuiltinShaderSettings>,
    /// BuiltinShaderSettings: (5.2.0f2 - 6000.2.0a6)
    pub m_DeferredReflections: Option<BuiltinShaderSettings>,
    /// BuiltinShaderSettings: (5.4.0f3 - 6000.2.0a6)
    pub m_DepthNormals: Option<BuiltinShaderSettings>,
    /// BuiltinShaderSettings: (5.0.0f4 - 2022.1.24f1)
    pub m_LegacyDeferred: Option<BuiltinShaderSettings>,
    /// BuiltinShaderSettings: (5.4.0f3 - 6000.2.0a6)
    pub m_LensFlare: Option<BuiltinShaderSettings>,
    /// BuiltinShaderSettings: (5.4.0f3 - 6000.2.0a6)
    pub m_LightHalo: Option<BuiltinShaderSettings>,
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LightProbeOutsideHullStrategy: Option<i32>,
    /// bool: (5.6.0b1 - 5.6.0b9)
    pub m_LightsUseCCT: Option<bool>,
    /// bool: (5.6.0f1 - 6000.2.0a6)
    pub m_LightsUseColorTemperature: Option<bool>,
    /// bool: (5.6.0b1 - 6000.2.0a6)
    pub m_LightsUseLinearIntensity: Option<bool>,
    /// bool: (2018.4.6f1 - 6000.2.0a6)
    pub m_LogWhenShaderIsCompiled: Option<bool>,
    /// BuiltinShaderSettings: (5.4.0f3 - 6000.2.0a6)
    pub m_MotionVectors: Option<BuiltinShaderSettings>,
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_PreloadShadersBatchTimeLimit: Option<i32>,
    /// Vec<PPtr<[`ShaderVariantCollection`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_PreloadedShaders: Option<Vec<PPtr>>,
    /// Vec<(String, PPtr<[`Object`]>)>: (2021.2.0b1 - 2023.2.0a14)
    pub m_SRPDefaultSettings: Option<Vec<(String, PPtr)>>,
    /// BuiltinShaderSettings: (5.4.0f3 - 6000.2.0a6)
    pub m_ScreenSpaceShadows: Option<BuiltinShaderSettings>,
    /// Vec<PlatformShaderDefines>: (2017.1.0f1 - 6000.2.0a6)
    pub m_ShaderDefinesPerShaderCompiler: Option<Vec<PlatformShaderDefines>>,
    /// PlatformShaderSettings: (5.3.0f1 - 5.3.8f2)
    pub m_ShaderSettings: Option<PlatformShaderSettings>,
    /// PlatformShaderSettings: (5.4.0f3 - 5.4.6f3)
    pub m_ShaderSettings_Tier1: Option<PlatformShaderSettings>,
    /// PlatformShaderSettings: (5.4.0f3 - 5.4.6f3)
    pub m_ShaderSettings_Tier2: Option<PlatformShaderSettings>,
    /// PlatformShaderSettings: (5.4.0f3 - 5.4.6f3)
    pub m_ShaderSettings_Tier3: Option<PlatformShaderSettings>,
    /// PPtr<[`Material`]>: (5.4.0f3 - 6000.2.0a6)
    pub m_SpritesDefaultMaterial: Option<PPtr>,
    /// TierGraphicsSettings: (5.5.0f3 - 6000.2.0a6)
    pub m_TierSettings_Tier1: Option<TierGraphicsSettings>,
    /// TierGraphicsSettings: (5.5.0f3 - 6000.2.0a6)
    pub m_TierSettings_Tier2: Option<TierGraphicsSettings>,
    /// TierGraphicsSettings: (5.5.0f3 - 6000.2.0a6)
    pub m_TierSettings_Tier3: Option<TierGraphicsSettings>,
    /// Vector3f: (5.6.0b1 - 6000.2.0a6)
    pub m_TransparencySortAxis: Option<Vector3f>,
    /// i32: (5.6.0b1 - 6000.2.0a6)
    pub m_TransparencySortMode: Option<i32>,
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub m_VideoShadersIncludeMode: Option<i32>,
}

/// GraphicsStateCollection is a  class of the Unity engine since version 6000.0.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Experimental.Rendering.GraphicsStateCollection.html):
/**
Collection of shader variants and associated graphics states.
Use graphics state collections to record shader variants and graphics states encountered at runtime and prewarm shader variants.Each shader variant in the collection may have one or more associated graphics states. Each permutation of a shader variant and a graphics state will result in a single GPU representation of the shader being created by Unity.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphicsStateCollection {
    pub m_DeviceRenderer: i32,
    /**The name of the object.*/
    pub m_Name: String,
    /**The quality level the collection is intended to be used with.*/
    pub m_QualityLevelName: String,
    pub m_RenderPassInfoMap: Vec<(u64, RenderPassInfo)>,
    pub m_RenderStateMap: Vec<(u64, RenderStateInfo)>,
    /**The platform that the collection is intended to be used with.*/
    pub m_RuntimePlatform: i32,
    pub m_VariantInfoMap: Vec<(Hash128, VariantInfo)>,
    /**The current version of the collection.*/
    pub m_Version: i32,
    pub m_VertexLayoutInfoMap: Vec<(u64, VertexLayoutInfo)>,
}

/// Grid is a  class of the Unity engine since version 2017.2.0f1.
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
    /// PPtr<[`GameObject`]>: (2017.2.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
}

/// GroupConnection is a sub class of the Unity engine since version 2021.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupConnection {
    pub sendEffectIndex: u32,
    pub sourceGroupIndex: u32,
    pub targetGroupIndex: u32,
}

/// GroupConstant is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupConstant {
    pub bypassEffects: bool,
    pub mute: bool,
    pub parentConstantIndex: i32,
    pub pitchIndex: u32,
    pub solo: bool,
    pub volumeIndex: u32,
    /// u32: (2019.1.0b1 - 2022.1.0a9)
    pub sendIndex: Option<u32>,
}

/// Halo is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Halo {
    pub m_Color: ColorRGBA,
    pub m_Enabled: u8,
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub m_Size: f32,
}

/// HandPose is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct HandPose {
    pub m_CloseOpen: f32,
    pub m_DoFArray: Vec<f32>,
    pub m_Grab: f32,
    pub m_GrabX: xform,
    pub m_InOut: f32,
    pub m_Override: f32,
}

/// Hash128 is a sub class of the Unity engine since version 4.1.0.
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
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[0]")]
    pub bytes_0_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[10]")]
    pub bytes_10_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[11]")]
    pub bytes_11_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[12]")]
    pub bytes_12_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[13]")]
    pub bytes_13_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[14]")]
    pub bytes_14_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[15]")]
    pub bytes_15_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[1]")]
    pub bytes_1_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[2]")]
    pub bytes_2_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[3]")]
    pub bytes_3_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[4]")]
    pub bytes_4_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[5]")]
    pub bytes_5_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[6]")]
    pub bytes_6_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[7]")]
    pub bytes_7_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[8]")]
    pub bytes_8_: Option<u8>,
    /// u8: (4.1.0 - 6000.2.0a6)
    #[serde(alias = "bytes[9]")]
    pub bytes_9_: Option<u8>,
}

/// HeightMeshBVNode is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct HeightMeshBVNode {
    pub i: i32,
    pub max: Vector3f,
    pub min: Vector3f,
    pub n: i32,
}

/// HeightMeshData is a sub class of the Unity engine since version 5.0.0f4.
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
    /// PPtr<[`PhysicMaterial`]>: (3.4.0 - 4.7.2)
    pub m_DefaultPhysicMaterial: Option<PPtr>,
    /// bool: (2019.3.0b1 - 6000.2.0a6)
    pub m_EnableHolesTextureCompression: Option<bool>,
    /// bool: (2019.3.0a5 - 2019.3.0a7)
    pub m_EnableSurfaceMaskTextureCompression: Option<bool>,
    /// i32: (3.4.0 - 2019.3.0a4)
    pub m_Height: Option<i32>,
    /// Vec<u8>: (2019.3.0b1 - 6000.2.0a6)
    pub m_Holes: Option<Vec<u8>>,
    /// Vec<u8>: (2019.3.0b1 - 6000.2.0a6)
    pub m_HolesLOD: Option<Vec<u8>>,
    /// i32: (2019.3.0b1 - 6000.2.0a6)
    pub m_Resolution: Option<i32>,
    /// Vec<u8>: (2019.3.0a5 - 2019.3.0a7)
    pub m_SurfaceMask: Option<Vec<u8>>,
    /// Vec<u8>: (2019.3.0a5 - 2019.3.0a7)
    pub m_SurfaceMaskLOD: Option<Vec<u8>>,
    /// f32: (5.0.0f4 - 2019.3.0a8)
    pub m_Thickness: Option<f32>,
    /// i32: (3.4.0 - 2019.3.0a4)
    pub m_Width: Option<i32>,
}

/// HeightmapData is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct HeightmapData {
    /// PPtr<[`Object`]>: (3.5.0 - 6000.2.0a6)
    pub terrainData: PPtr,
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub isRotated: Option<bool>,
    /// Vector3f: (3.5.0 - 2022.1.24f1)
    pub position: Option<Vector3f>,
    /// Matrix4x4f: (2022.2.0b1 - 6000.2.0a6)
    pub surfaceToTerrain: Option<Matrix4x4f>,
}

/// HierarchicalSceneData is a sub class of the Unity engine since version 2019.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct HierarchicalSceneData {
    pub m_SceneGUID: GUID,
}

/// HierarchyState is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct HierarchyState {
    /// Vec<PPtr<[`Object`]>>: (3.4.0 - 6000.2.0a6)
    pub expanded: Vec<PPtr>,
    /// Vec<PPtr<[`Object`]>>: (3.4.0 - 6000.2.0a6)
    pub selection: Vec<PPtr>,
    /// f32: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "scrollposition.x")]
    pub scrollposition_x: Option<f32>,
    /// f32: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "scrollposition.y")]
    pub scrollposition_y: Option<f32>,
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
    /// PPtr<[`Rigidbody`]>: (3.4.0 - 6000.2.0a6)
    pub m_ConnectedBody: PPtr,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /// bool: (4.3.0 - 6000.2.0a6)
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    /**Position of the anchor relative to the connected Rigidbody.*/
    /// Vector3f: (4.3.0 - 6000.2.0a6)
    pub m_ConnectedAnchor: Option<Vector3f>,
    /**A reference to an articulation body this joint connects to.*/
    /// PPtr<[`ArticulationBody`]>: (2020.2.0b1 - 6000.2.0a6)
    pub m_ConnectedArticulationBody: Option<PPtr>,
    /**The scale to apply to the inverse mass and inertia tensor of the connected body prior to solving the constraints.*/
    /// f32: (2017.1.0f1 - 6000.2.0a6)
    pub m_ConnectedMassScale: Option<f32>,
    /**Enable collision between bodies connected with the joint.*/
    /// bool: (4.5.0 - 6000.2.0a6)
    pub m_EnableCollision: Option<bool>,
    /**Toggle preprocessing for this joint.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_EnablePreprocessing: Option<bool>,
    /// bool: (2017.1.0b2 - 2017.1.0b5)
    pub m_Enabled: Option<bool>,
    /**If enabled, the angle of the hinge is extended to [-360, 360] degrees.*/
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExtendedLimits: Option<bool>,
    /**The scale to apply to the inverse mass and inertia tensor of the body prior to solving the constraints.*/
    /// f32: (2017.1.0f1 - 6000.2.0a6)
    pub m_MassScale: Option<f32>,
    /**Defines whether the HingeJoint.spring outputs accelerations instead of forces.*/
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_UseAcceleration: Option<bool>,
}

/// HingeJoint2D is a  class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/HingeJoint2D.html):
/**
Joint that allows a Rigidbody2D object to rotate around a point in space or a point on another object.
Additional resources: DistanceJoint2D, SliderJoint2D, SpringJoint2D, JointAngleLimits2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct HingeJoint2D {
    /**The joint's anchor point on the object that has the joint component.*/
    pub m_Anchor: Vector2f,
    pub m_AngleLimits: Enum_JointAngleLimit2D__JointAngleLimits2D,
    /**The joint's anchor point on the second object (ie, the one which doesn't have the joint component).*/
    pub m_ConnectedAnchor: Vector2f,
    /// PPtr<[`Rigidbody2D`]>: (4.3.0 - 6000.2.0a6)
    pub m_ConnectedRigidBody: PPtr,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.3.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Parameters for the motor force applied to the joint.*/
    pub m_Motor: JointMotor2D,
    /**Should limits be placed on the range of rotation?*/
    pub m_UseLimits: bool,
    /**Should the joint be rotated automatically by a motor torque?*/
    pub m_UseMotor: bool,
    /**Should the connectedAnchor be calculated automatically?*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_BreakAction: Option<i32>,
    /**The force that needs to be applied for this joint to break.*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_BreakForce: Option<f32>,
    /**The torque that needs to be applied for this joint to break.*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_BreakTorque: Option<f32>,
    /// bool: (4.3.0 - 5.0.0f4)
    pub m_CollideConnected: Option<bool>,
    /**Should the two Rigidbody2D connected with this joint collide with each other?*/
    /// bool: (5.0.1f1 - 6000.2.0a6)
    pub m_EnableCollision: Option<bool>,
    /**Controls whether the connected anchor is used or not.*/
    /// bool: (2023.1.0b1 - 6000.2.0a6)
    pub m_UseConnectedAnchor: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_JointAngleLimit2D__JointAngleLimits2D {
    JointAngleLimit2D(JointAngleLimit2D),
    JointAngleLimits2D(JointAngleLimits2D),
}

/// HoloLens is a sub class of the Unity engine since version 5.6.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct HoloLens {
    pub depthFormat: i32,
    /// bool: (2017.3.0f1 - 2020.2.0a15)
    pub depthBufferSharingEnabled: Option<bool>,
}

/// HumanBone is a sub class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/HumanBone.html):
/**
The mapping between a bone in the model and the conceptual bone in the Mecanim human anatomy.
The names of the Mecanim human bone and the bone in the model are stored along with the limiting muscle values that constrain the bone's rotation during animation.
Additional resources: HumanDescription, AvatarBuilder.
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

/// HumanDescription is a sub class of the Unity engine since version 4.0.0.
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
    /**Mapping between Mecanim bone names and bone names in the rig.*/
    pub m_Human: Vec<HumanBone>,
    /**Amount by which the leg's length is allowed to stretch when using IK.*/
    pub m_LegStretch: f32,
    pub m_LegTwist: f32,
    pub m_RootMotionBoneName: String,
    /**List of bone Transforms to include in the model.*/
    pub m_Skeleton: Vec<SkeletonBone>,
    /**Defines how the upper leg's roll/twisting is distributed between the thigh and knee joints.*/
    pub m_UpperLegTwist: f32,
    /// f32: (2019.1.0b1 - 6000.2.0a6)
    pub m_GlobalScale: Option<f32>,
    /// Vec<HumanHandle>: (4.0.0 - 4.2.2)
    pub m_Handles: Option<Vec<HumanHandle>>,
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub m_HasExtraRoot: Option<bool>,
    /**True for any human that has a translation Degree of Freedom (DoF). It is set to false by default.*/
    /// bool: (5.2.0f2 - 6000.2.0a6)
    pub m_HasTranslationDoF: Option<bool>,
    /// Quaternionf: (5.5.0f3 - 2018.2.12f1)
    pub m_RootMotionBoneRotation: Option<Quaternionf>,
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub m_SkeletonHasParents: Option<bool>,
}

/// HumanGoal is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct HumanGoal {
    pub m_WeightR: f32,
    pub m_WeightT: f32,
    pub m_X: xform,
    /// float4: (5.0.0f4 - 5.3.8f2); float3: (5.4.0f3 - 6000.2.0a6)
    pub m_HintT: Option<Enum_float4__float3>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_HintWeightT: Option<f32>,
}

/// HumanHandle is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct HumanHandle {
    pub m_BoneName: String,
    pub m_LookAt: bool,
    pub m_Name: String,
    pub m_Position: Vector3f,
    pub m_Rotation: Quaternionf,
    pub m_Scale: Vector3f,
}

/// HumanPose is a sub class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/HumanPose.html):
/**
Retargetable humanoid pose.
Represents a humanoid pose that is completely abstracted from any skeleton rig.
Additional resources: HumanPoseHandler.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct HumanPose {
    pub m_DoFArray: Vec<f32>,
    pub m_GoalArray: Vec<HumanGoal>,
    pub m_LeftHandPose: HandPose,
    pub m_LookAtPosition: Enum_float4__float3,
    pub m_LookAtWeight: float4,
    pub m_RightHandPose: HandPose,
    pub m_RootX: xform,
    /// Vec<float4>: (5.2.0f2 - 5.3.8f2); Vec<float3>: (5.4.0f3 - 6000.2.0a6)
    pub m_TDoFArray: Option<Vec<Enum_float3__float4>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_float3__float4 {
    float3(float3),
    float4(float4),
}

/// HumanTemplate is a  class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct HumanTemplate {
    pub m_BoneTemplate: Vec<(String, String)>,
    pub m_Name: String,
}

/// IHVImageFormatImporter is a  class of the Unity engine since version 5.6.0b1.
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
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /**Enable if the texture should ignore any texture mipmap limit settings set in the Project Settings.*/
    /// bool: (2022.2.0f1 - 6000.2.0a6)
    pub m_IgnoreMipmapLimit: Option<bool>,
    /**Name of the texture mipmap limit group to which this texture belongs.*/
    /// String: (2022.2.0f1 - 6000.2.0a6)
    pub m_MipmapLimitGroupName: Option<String>,
    /**Enable mipmap streaming for this texture.*/
    /// bool: (2018.2.0b1 - 6000.2.0a6)
    pub m_StreamingMipmaps: Option<bool>,
    /**Relative priority for this texture when reducing memory size in order to hit the memory budget.*/
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub m_StreamingMipmapsPriority: Option<i32>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /// bool: (2017.3.0b1 - 6000.2.0a6)
    pub m_sRGBTexture: Option<bool>,
}

/// Image is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/UIElements.Image.html):
/**
A VisualElement representing a source texture.
Additional resources: UXML element Image.

Note: This is the Image control for the UI Toolkit framework. This is not related to the
 UnityEngine.UI.Image
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub m_Format: i64,
    pub m_Height: i32,
    pub m_RowBytes: i32,
    pub m_Width: i32,
    /// Vec<u8>: (3.4.0 - 2020.1.17f1)
    #[serde(alias = "image data")]
    pub image_data: Option<Vec<u8>>,
}

/// ImportLog is a  class of the Unity engine since version 2022.2.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AssetImporters.ImportLog.html):
/**
Container class that holds the collection of logs generated by an importer during the import process.
Additional resources: AssetImportContext.LogImportError, AssetImportContext.LogImportWarning.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportLog {
    pub m_Logs: Vec<ImportLog_ImportLogEntry>,
    /**The name of the object.*/
    pub m_Name: String,
}

/// ImportLog_ImportLogEntry is a sub class of the Unity engine since version 2022.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportLog_ImportLogEntry {
    pub file: String,
    pub line: i32,
    pub message: String,
    pub mode: i32,
    /// PPtr<[`Object`]>: (2022.2.0b1 - 6000.2.0a6)
    pub object: PPtr,
}

/// InheritVelocityModule is a sub class of the Unity engine since version 5.3.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.InheritVelocityModule.html):
/**
The Inherit Velocity Module controls how the velocity of the emitter is transferred to the particles as they are emitted.
NOTE: The inherit velocity module only has an effect if the Particle System is set to simulate in world space. If the system is simulating in local space, this module is ignored.
Additional resources: ParticleSystem, ParticleSystem.inheritVelocity.
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

/// InitialModule is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct InitialModule {
    pub enabled: bool,
    pub gravityModifier: Enum_f32__MinMaxCurve,
    pub maxNumParticles: i32,
    pub startColor: MinMaxGradient,
    pub startLifetime: MinMaxCurve,
    pub startRotation: MinMaxCurve,
    pub startSize: MinMaxCurve,
    pub startSpeed: MinMaxCurve,
    /// Vector3f: (2021.1.0b1 - 6000.2.0a6)
    pub customEmitterVelocity: Option<Vector3f>,
    /// i32: (2022.1.0b1 - 6000.2.0a6)
    pub gravitySource: Option<i32>,
    /// f32: (3.5.0 - 5.2.5f1)
    pub inheritVelocity: Option<f32>,
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub randomizeRotationDirection: Option<f32>,
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub rotation3D: Option<bool>,
    /// bool: (5.4.0f3 - 6000.2.0a6)
    pub size3D: Option<bool>,
    /// MinMaxCurve: (5.3.0f1 - 6000.2.0a6)
    pub startRotationX: Option<MinMaxCurve>,
    /// MinMaxCurve: (5.3.0f1 - 6000.2.0a6)
    pub startRotationY: Option<MinMaxCurve>,
    /// MinMaxCurve: (5.4.0f3 - 6000.2.0a6)
    pub startSizeY: Option<MinMaxCurve>,
    /// MinMaxCurve: (5.4.0f3 - 6000.2.0a6)
    pub startSizeZ: Option<MinMaxCurve>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_f32__MinMaxCurve {
    f32(f32),
    MinMaxCurve(MinMaxCurve),
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
    /// i32: (3.4.0 - 6000.2.0a6)
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
    /// bool: (2021.2.0b1 - 6000.2.0a6)
    pub m_UsePhysicalKeys: Option<bool>,
}

/// InspectorExpandedState is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct InspectorExpandedState {
    pub m_ExpandedData: Vec<ExpandedData>,
}

/// IntPoint is a sub class of the Unity engine since version 5.6.0b1.
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 4.7.2)
    pub m_GameObject: PPtr,
    /// PPtr<[`Mesh`]>: (3.4.0 - 4.7.2)
    pub m_Mesh: PPtr,
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
    /// i32: (3.4.0 - 5.6.7f1)
    #[serde(alias = "(int&)downloadResolution")]
    pub downloadResolution: Option<i32>,
    /// i32: (3.4.0 - 5.6.7f1)
    #[serde(alias = "(int&)nameConflictResolution")]
    pub nameConflictResolution: Option<i32>,
}

/// JointAngleLimit2D is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct JointAngleLimit2D {
    pub m_LowerAngle: f32,
    pub m_UpperAngle: f32,
}

/// JointAngleLimits2D is a sub class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointAngleLimits2D.html):
/**
Angular limits on the rotation of a Rigidbody2D object around a HingeJoint2D.
Additional resources: Rigidbody2D class, HingeJoint2D class.
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
    /// i32: (3.4.0 - 5.2.5f1)
    pub mode: Option<i32>,
    /**Defines whether the drive is an acceleration drive or a force drive.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub useAcceleration: Option<i32>,
}

/// JointLimits is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointLimits.html):
/**
JointLimits is used by the HingeJoint to limit the joints angle.
Additional resources: HingeJoint.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct JointLimits {
    /**The upper angular limit (in degrees) of the joint.*/
    pub max: f32,
    /**The lower angular limit (in degrees) of the joint.*/
    pub min: f32,
    /**The minimum impact velocity which will cause the joint to bounce.*/
    /// f32: (5.1.0f1 - 6000.2.0a6)
    pub bounceMinVelocity: Option<f32>,
    /**Determines the size of the bounce when the joint hits it's limit. Also known as restitution.*/
    /// f32: (5.1.0f1 - 6000.2.0a6)
    pub bounciness: Option<f32>,
    /**Distance inside the limit value at which the limit will be considered to be active by the solver.*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub contactDistance: Option<f32>,
    /// f32: (3.4.0 - 5.0.4f1)
    pub maxBounce: Option<f32>,
    /// f32: (3.4.0 - 5.0.4f1)
    pub minBounce: Option<f32>,
}

/// JointMotor is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointMotor.html):
/**
The JointMotor is used to motorize a joint.
For example the HingeJoint can be told to rotate at a given speed and force.
The joint will then attempt to reach the velocity with the given maximum force.
Additional resources: HingeJoint.
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

/// JointMotor2D is a sub class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointMotor2D.html):
/**
Parameters for the optional motor force applied to a Joint2D.
Additional resources: HingeJoint2D class, SliderJoint2D class.
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
JointSpring is used add a spring force to HingeJoint and PhysicsMaterial.
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

/// JointSuspension2D is a sub class of the Unity engine since version 4.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointSuspension2D.html):
/**
Joint suspension is used to define how suspension works on a WheelJoint2D.
Additional resources: WheelJoint2D class.
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

/// JointTranslationLimits2D is a sub class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/JointTranslationLimits2D.html):
/**
Motion limits of a Rigidbody2D object along a SliderJoint2D.
Additional resources: Rigidbody2D class, SliderJoint2D class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct JointTranslationLimits2D {
    pub m_LowerTranslation: f32,
    pub m_UpperTranslation: f32,
}

/// KTXImporter is a  class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct KTXImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_Name: String,
    pub m_UserData: String,
}

/// Keyframe is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Keyframe.html):
/**
A keyframe that can be injected into an AnimationCurve.
Keyframes are used to control how the value of an AnimationCurve changes over time. Each Keyframe is determined by its position in the AnimationCurve through the Keyframe.time (horizontal axis) and Keyframe.value (vertical axis) properties. You can control how interpolation is handled between different Keyframes and the AnimationCurve slopes through the Keyframe.inTangent, Keyframe.outTangent, Keyframe.weightedMode, Keyframe.inWeight and Keyframe.outWeight properties. Note that unlike what the Keyframe.time property name might imply, it is not actually tied to time and can simply be thought of as a position on the horizontal axis of a two-dimensional space. Hermite Segment            When both keyframes weightedMode are set to
            WeightedMode.None, the curve segment is interpolated using a Hermite curve algorithm.
            Hermite evaluates faster than Bezier and is the default weightedMode for a
            Keyframe. Bezier Segment            Setting weightedMode will change how the neighbouring segments are
            interpolated. Weighted segments are interpolated using a Bezier curve algorithm. For a given weighted
            keyframe, the interpolating function uses the Keyframe.outWeight value and the next keyframe’s
            Keyframe.inWeight alongside both keyframes values to calculate the curve. If no
            weight is set (when either keyframe Keyframe.weightedMode is set to WeightedMode.None),
            then a weight value of 1/3 is used.            Additional resources: AnimationCurve.AddKey, AnimationCurve.keys, AnimationCurve.Evaluate.
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
    /// Quaternionf: (2018.1.0f1 - 6000.2.0a6)
    pub inWeight: Option<Quaternionf>,
    /**Sets the outgoing weight for this key. The outgoing weight affects the slope of the curve from this key to the next key.*/
    /// Quaternionf: (2018.1.0f1 - 6000.2.0a6)
    pub outWeight: Option<Quaternionf>,
    /**Weighted mode for the keyframe.*/
    /// i32: (2018.1.0f1 - 6000.2.0a6)
    pub weightedMode: Option<i32>,
}

/// LOD is a sub class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LOD.html):
/**
A structure that you can pass to LODGroup.SetLODs to configure a particular LOD level on a LODGroup.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LOD {
    /**List of renderers for this LOD level.*/
    pub renderers: Vec<LODRenderer>,
    pub screenRelativeHeight: f32,
    /// i32: (5.0.0f4 - 5.0.4f1)
    pub fadeMode: Option<i32>,
    /**Width of the cross-fade transition zone (proportion to the current LOD's whole length) [0-1]. Only used if it's not animated.*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub fadeTransitionWidth: Option<f32>,
}

/// LODGroup is a  class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LODGroup.html):
/**
LODGroup lets you group multiple Renderers into LOD levels.
This can be used to switch between different LOD levels at runtime based on size on screen.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LODGroup {
    /**Allows you to enable or disable the LODGroup.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.5.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub m_LODs: Vec<LOD>,
    /**The local reference point against which the LOD distance is calculated.*/
    pub m_LocalReferencePoint: Vector3f,
    /**The size of the LOD object in local space.*/
    pub m_Size: f32,
    /**Specify if the cross-fading should be animated by time. The animation duration is specified globally as crossFadeAnimationDuration.*/
    /// bool: (5.1.0f1 - 6000.2.0a6)
    pub m_AnimateCrossFading: Option<bool>,
    /**The LOD fade mode used.*/
    /// i32: (5.1.0f1 - 6000.2.0a6)
    pub m_FadeMode: Option<i32>,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_LastLODIsBillboard: Option<bool>,
    /// f32: (3.5.0 - 4.7.2)
    pub m_ScreenRelativeTransitionHeight: Option<f32>,
}

/// LODRenderer is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct LODRenderer {
    /// PPtr<[`Renderer`]>: (3.5.0 - 6000.2.0a6)
    pub renderer: PPtr,
}

/// LayoutDataOne is a sub class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutDataOne {
    pub m_FloatArray: Vec<f32>,
}

/// LayoutDataThree is a sub class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutDataThree {
    pub m_AnotherFloatArray: Vec<f32>,
}

/// LayoutDataTwo is a sub class of the Unity engine since version 2019.1.0b1.
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
    /// PPtr<[`Flare`]>: (3.4.0 - 6000.2.0a6)
    pub m_Flare: PPtr,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub m_IgnoreLayers: BitField,
    /**The fade speed of the flare.*/
    /// f32: (4.3.0 - 6000.2.0a6)
    pub m_FadeSpeed: Option<f32>,
}

/// LevelGameManager is a  class of the Unity engine since version 4.1.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct LevelGameManager {}

/// LibraryAssetImporter is a  class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryAssetImporter {
    pub m_Name: String,
    pub m_UserData: String,
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleName: Option<String>,
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleVariant: Option<String>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// LibraryRepresentation is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryRepresentation {
    pub name: String,
    pub scriptClassName: String,
    pub thumbnail: Image,
    pub thumbnailClassID: i32,
    /// u16: (4.0.0 - 2020.1.17f1)
    pub flags: Option<u16>,
    /// GUID: (5.1.0f1 - 2020.1.17f1)
    pub guid: Option<GUID>,
    /// i64: (5.1.0f1 - 2020.1.17f1)
    pub localIdentifier: Option<i64>,
    /// PPtr<[`EditorExtension`]>: (3.4.0 - 3.4.2); PPtr<[`Object`]>: (3.5.0 - 5.0.4f1)
    pub object: Option<PPtr>,
    /// String: (5.1.0f1 - 2020.1.17f1)
    pub path: Option<String>,
}

/// LifetimeByEmitterSpeedModule is a sub class of the Unity engine since version 2020.1.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.LifetimeByEmitterSpeedModule.html):
/**
The Lifetime By Emitter Speed Module controls the initial lifetime of each particle based on the speed of the emitter when the particle was spawned.
This module multiplies the start lifetime of particles with a value that depends on the speed of the object that spawned them. For most Particle Systems, this is the GameObject velocity, but for sub-emitters, the velocity comes from the parent particle that the sub-emitter particle originated from.Additional resources: ParticleSystem, ParticleSystem.MainModule.startLifetime.
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
                values shown in the Inspector.                For more information about shadow maps, refer to Shadow Mapping.                Usually lights are just created in the editor, but sometimes you want to create a light from a script:
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    /**Specifies the color emitted by the light.*/
    pub m_Color: ColorRGBA,
    /**The cookie texture projected by the light.*/
    /// PPtr<[`Texture`]>: (3.4.0 - 6000.2.0a6)
    pub m_Cookie: PPtr,
    /**The size of a directional light's cookie.*/
    pub m_CookieSize: f32,
    /**This is used to light certain objects in the Scene selectively.*/
    pub m_CullingMask: BitField,
    pub m_DrawHalo: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The flare asset to use for this light.*/
    /// PPtr<[`Flare`]>: (3.4.0 - 6000.2.0a6)
    pub m_Flare: PPtr,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The Intensity of a light is multiplied with the Light color.*/
    pub m_Intensity: f32,
    pub m_Lightmapping: i32,
    /**The maximum distance that light travels from a point of emission. This property does not apply to directional lights.*/
    pub m_Range: f32,
    /**Controls how often the light's contribution is calculated during rendering.*/
    pub m_RenderMode: i32,
    /**Determines if this light will cast soft or hard shadows, or not cast shadows at all.*/
    pub m_Shadows: ShadowSettings,
    /**The angle of the spot light's cone in degrees.*/
    pub m_SpotAngle: f32,
    /**The type of the light.*/
    pub m_Type: i32,
    /// bool: (3.4.0 - 5.3.8f2)
    pub m_ActuallyLightmapped: Option<bool>,
    /**The size of the area light.*/
    /// Vector2f: (5.4.0f3 - 6000.2.0a6)
    pub m_AreaSize: Option<Vector2f>,
    /// i32: (5.4.0f3 - 5.6.0b1)
    pub m_BakedIndex: Option<i32>,
    /**This property describes the output of the last Global Illumination bake.*/
    /// LightBakingOutput: (5.6.0f1 - 6000.2.0a6)
    pub m_BakingOutput: Option<LightBakingOutput>,
    /**The multiplier that defines the strength of the bounce lighting.*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_BounceIntensity: Option<f32>,
    /**Bounding sphere used to override the regular light bounding sphere during culling.*/
    /// Vector4f: (2019.1.0f2 - 6000.2.0a6)
    pub m_BoundingSphereOverride: Option<Vector4f>,
    /// f32: (5.6.0b1 - 5.6.0b9)
    pub m_CCT: Option<f32>,
    /**The color temperature of the light.          Correlated Color Temperature (abbreviated as CCT) is multiplied with the color filter when calculating the final color of a light source. The color temperature of the electromagnetic radiation emitted from an ideal black body is defined as its surface temperature in Kelvin. White is 6500K according to the D65 standard. A candle light is 1800K and a soft warm light bulb is 2700K.          If you want to use colorTemperature, GraphicsSettings.lightsUseLinearIntensity and Light.useColorTemperature has to be enabled.          Additional resources: GraphicsSettings.lightsUseLinearIntensity, GraphicsSettings.useColorTemperature.*/
    /// f32: (5.6.0f1 - 6000.2.0a6)
    pub m_ColorTemperature: Option<f32>,
    /**Wether a Spot Light should simulate having a reflector.*/
    /// bool: (2023.3.0b5 - 6000.2.0a6)
    pub m_EnableSpotReflector: Option<bool>,
    /// FalloffTable: (2017.1.0b1 - 2017.1.0b10)
    pub m_FalloffTable: Option<FalloffTable>,
    /**Force a light to be visible even if outside the view frustum.*/
    /// bool: (2023.3.0b1 - 6000.2.0a6)
    pub m_ForceVisible: Option<bool>,
    /**The angle of the spot light's inner cone in degrees.*/
    /// f32: (2019.1.0b1 - 6000.2.0a6)
    pub m_InnerSpotAngle: Option<f32>,
    /**Allows you to override the global Shadowmask Mode per light. Only use this with render pipelines that can handle per light Shadowmask modes. Incompatible with the legacy renderers.*/
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub m_LightShadowCasterMode: Option<i32>,
    /**The unit Light.intensity should be displayed in.*/
    /// i32: (2023.3.0b5 - 6000.2.0a6)
    pub m_LightUnit: Option<i32>,
    /**How far away to measure LightUnit.Lux from.*/
    /// f32: (2023.3.0b5 - 6000.2.0a6)
    pub m_LuxAtDistance: Option<f32>,
    /**Determines which rendering LayerMask this Light affects.*/
    /// u32: (2019.1.0f2 - 6000.2.0a6)
    pub m_RenderingLayerMask: Option<u32>,
    /// i32: (2019.3.0b1 - 2023.1.20f1)
    pub m_Shape: Option<i32>,
    /**Set to true to override light bounding sphere for culling.*/
    /// bool: (2019.1.0f2 - 6000.2.0a6)
    pub m_UseBoundingSphereOverride: Option<bool>,
    /**Set to true to use the color temperature.*/
    /// bool: (5.6.0f1 - 6000.2.0a6)
    pub m_UseColorTemperature: Option<bool>,
    /**Whether to cull shadows for this Light when the Light is outside of the view frustum.*/
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_UseViewFrustumForShadowCasterCull: Option<bool>,
}

/// LightBakingOutput is a sub class of the Unity engine since version 5.6.0f1.
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
    /// bool: (2017.3.0b1 - 6000.2.0a6)
    pub isBaked: Option<bool>,
    /// LightmapBakeMode: (2017.3.0b1 - 6000.2.0a6)
    pub lightmapBakeMode: Option<LightmapBakeMode>,
    /// i32: (5.6.0f1 - 2017.2.5f1)
    pub lightmappingMask: Option<i32>,
    /**In case of a LightmapBakeType.Mixed light, contains the index of the occlusion mask channel to use if any, otherwise -1.*/
    /// i32: (5.6.0f1 - 6000.2.0a6)
    pub occlusionMaskChannel: Option<i32>,
    /// i32: (5.6.0b2 - 5.6.0b4)
    pub shadowMaskChannel: Option<i32>,
}

/// LightProbeData is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct LightProbeData {
    pub m_NonTetrahedralizedProbeSetIndexMap: Vec<(Hash128, i32)>,
    pub m_Positions: Vec<Vector3f>,
    pub m_ProbeSets: Vec<ProbeSetIndex>,
    pub m_Tetrahedralization: ProbeSetTetrahedralization,
}

/// LightProbeGroup is a  class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightProbeGroup.html):
/**
Specifies where and how to bake a list of light probes.
Light Probe Groups act only as input for Unity’s light baking system and have no direct impact on rendering.Light Probe Groups do not affect probes used by other systems such as Reflection Probes or the Adaptive Probe Volumes.Additional resources: LightProbes.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightProbeGroup {
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.5.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    /// u8: (4.0.0 - 6000.2.0a6)
    pub m_Enabled: Option<u8>,
}

/// LightProbeOcclusion is a sub class of the Unity engine since version 5.4.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct LightProbeOcclusion {
    pub m_Occlusion: Vec<f32>,
    /// Vec<i32>: (5.4.0f3 - 5.6.0b1)
    pub m_BakedLightIndex: Option<Vec<i32>>,
    /// Vec<i8>: (5.6.0f1 - 6000.2.0a6)
    pub m_OcclusionMaskChannel: Option<Vec<i8>>,
    /// Vec<i32>: (5.6.0f1 - 6000.2.0a6)
    pub m_ProbeOcclusionLightIndex: Option<Vec<i32>>,
    /// Vec<i8>: (5.6.0b2 - 5.6.0b4)
    pub m_ShadowMaskChannel: Option<Vec<i8>>,
}

/// LightProbeProxyVolume is a  class of the Unity engine since version 5.4.0f3.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightProbeProxyVolume.html):
/**
The Light Probe Proxy Volume component offers the possibility to use higher resolution lighting for large non-static GameObjects.
By default, a probe-lit Renderer receives lighting from a single Light Probe that is interpolated from the surrounding Light Probes in the Scene. Because of this, GameObjects have constant ambient lighting regardless of their position on the surface. The light has have a rotational gradient because it's using spherical harmonics, but it lacks a spatial gradient. This is more noticeable on larger GameObjects and Particle Systems. The lighting across the GameObject matches the lighting at the anchor point, and if the GameObject straddles a lighting gradient, parts of the GameObject will look incorrect.This component will generate a 3D grid of interpolated Light Probes inside a bounding volume. The resolution of the grid can be user-specified. The spherical harmonics coefficients of the interpolated Light Probes are updated into 3D textures, which are sampled at render time to compute the contribution to the diffuse ambient lighting. This adds a spatial gradient to probe-lit GameObjects.Additional resources: Light Probes.
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
    /// PPtr<[`GameObject`]>: (5.4.0f3 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /// i32: (2020.2.0b1 - 6000.2.0a6)
    pub m_DataFormat: Option<i32>,
    /**Determines how many Spherical Harmonics bands will be evaluated to compute the ambient color.*/
    /// i32: (2018.2.0f1 - 6000.2.0a6)
    pub m_QualityMode: Option<i32>,
}

/// LightProbes is a  class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightProbes.html):
/**
Stores light probe data for all currently loaded Scenes.
The data includes: probe positions, Spherical Harmonics (SH) coefficients and the tetrahedral tessellation.You can modify the probe positions and coefficients, and update the tetrahedral tessellation at runtime. You can also swap the entire LightProbes object for a different pre-baked one using LightmapSettings.lightProbes.To retrieve the LightProbes objects for a specific scene, use LightProbes.GetInstantiatedLightProbesForScene or LightProbes.GetSharedLightProbesForScene.Additional resources: Light Probes in the Unity Manual, LightmapSettings, ReceiveGI.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightProbes {
    /**The name of the object.*/
    pub m_Name: String,
    /// Vec<LightmapData>: (3.5.0 - 4.7.2)
    pub bakedCoefficients: Option<Vec<LightmapData>>,
    /// Vec<Vector3f>: (3.5.0 - 4.7.2)
    pub bakedPositions: Option<Vec<Vector3f>>,
    /// Vec<Vector3f>: (3.5.0 - 4.7.2)
    pub hullRays: Option<Vec<Vector3f>>,
    /// Vec<SphericalHarmonicsL2>: (5.0.0f4 - 6000.2.0a6)
    pub m_BakedCoefficients: Option<Vec<SphericalHarmonicsL2>>,
    /// Vec<LightProbeOcclusion>: (5.4.0f3 - 6000.2.0a6)
    pub m_BakedLightOcclusion: Option<Vec<LightProbeOcclusion>>,
    /// LightProbeData: (5.0.0f4 - 6000.2.0a6)
    pub m_Data: Option<LightProbeData>,
    /// bool: (2023.2.0b1 - 6000.2.0a6)
    pub m_HasBeenEdited: Option<bool>,
    /// Vec<Tetrahedron>: (3.5.0 - 4.7.2)
    pub tetrahedra: Option<Vec<Tetrahedron>>,
}

/// LightingDataAsset is a  class of the Unity engine since version 5.3.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightingDataAsset.html):
/**
The lighting data asset used by the active Scene.
Please note that modifying this value currently does not affect the Scene immediately, the lighting data is only patched into the active Scene when loading the Scene.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightingDataAsset {
    pub m_BakedAmbientProbeInLinear: SphericalHarmonicsL2,
    /// Vec<PPtr<[`Texture`]>>: (5.3.0f1 - 6000.2.0a6)
    pub m_BakedReflectionProbeCubemaps: Vec<PPtr>,
    pub m_BakedReflectionProbes: Vec<SceneObjectIdentifier>,
    pub m_EnlightenData: Vec<u8>,
    pub m_EnlightenSceneMapping: EnlightenSceneMapping,
    pub m_EnlightenSceneMappingRendererIDs: Vec<SceneObjectIdentifier>,
    /// PPtr<[`LightProbes`]>: (5.3.0f1 - 6000.2.0a6)
    pub m_LightProbes: PPtr,
    pub m_LightmappedRendererData: Vec<RendererData>,
    pub m_LightmappedRendererDataIDs: Vec<SceneObjectIdentifier>,
    pub m_Lightmaps: Vec<LightmapData>,
    pub m_Lights: Vec<SceneObjectIdentifier>,
    /**The name of the object.*/
    pub m_Name: String,
    /// Vec<PPtr<[`Texture2D`]>>: (2019.1.0b1 - 6000.2.0a6)
    pub m_AOTextures: Option<Vec<PPtr>>,
    /// Vec<i32>: (5.4.0f3 - 5.6.0b1)
    pub m_BakedLightIndices: Option<Vec<i32>>,
    /// Vec<String>: (2018.2.0b1 - 2023.2.0a18)
    pub m_BakedReflectionProbeCubemapCacheFiles: Option<Vec<String>>,
    /// i32: (5.3.6f1 - 6000.2.0a6)
    pub m_EnlightenDataVersion: Option<i32>,
    /// Vec<LightBakingOutput>: (5.6.0f1 - 6000.2.0a6)
    pub m_LightBakingOutputs: Option<Vec<LightBakingOutput>>,
    /// Vec<String>: (2018.2.0b1 - 2023.2.0a18)
    pub m_LightmapsCacheFiles: Option<Vec<String>>,
    /// i32: (5.4.0f3 - 6000.2.0a6)
    pub m_LightmapsMode: Option<i32>,
    /// PPtr<[`SceneAsset`]>: (2017.1.0f1 - 6000.2.0a6)
    pub m_Scene: Option<PPtr>,
    /// GUID: (5.3.0f1 - 5.3.0f2)
    pub m_SceneGUID: Option<GUID>,
}

/// LightingSettings is a  class of the Unity engine since version 2020.1.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightingSettings.html):
/**
An object containing settings for precomputing lighting data, that Unity can serialize as a Lighting Settings Asset.
When the Unity Editor precomputes lighting data for a Scene that uses the Baked Global Illumination system or the Enlighten Realtime Global Illumination system, it uses settings from a LightingSettings object. The same LightingSettings object can be assigned to more than one Scene, which makes it possible to share settings across multiple Scenes.The following example shows how to create a LightingSettings object and assign it to the active Scene using the Lightmapping.lightingSettings API:
The following example shows how to create a LightingSettings object, and save it to disk as a Lighting Settings Asset using the AssetDatabase.CreateAsset API.
Additional resources: Lighting Settings Asset.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightingSettings {
    /**The intensity of surface albedo throughout the Scene when considered in lighting calculations. This value influences the energy of light at each bounce. (Editor only).*/
    pub m_AlbedoBoost: f32,
    pub m_BounceScale: f32,
    pub m_EnableBakedLightmaps: bool,
    pub m_EnableRealtimeLightmaps: bool,
    /**The name of the object.*/
    pub m_Name: String,
    /**Determines the lightmap that Unity stores environment lighting in.*/
    pub m_RealtimeEnvironmentLighting: bool,
    pub m_UsingShadowmask: bool,
    /**Whether to apply ambient occlusion to lightmaps. (Editor only).*/
    /// bool: (2020.1.0a3 - 2020.1.0a13)
    pub m_AO: Option<bool>,
    /**The distance that a ray travels before Unity considers it to be unoccluded when calculating ambient occlusion in lightmaps. (Editor only).*/
    /// f32: (2020.1.0a3 - 2020.1.0a13)
    pub m_AOMaxDistance: Option<f32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_BakeBackend: Option<i32>,
    /// f32: (2020.1.0a3 - 2020.1.0a13)
    pub m_BakeResolution: Option<f32>,
    /// f32: (2020.1.0a3 - 2020.1.0a13)
    pub m_CompAOExponent: Option<f32>,
    /// f32: (2020.1.0a3 - 2020.1.0a13)
    pub m_CompAOExponentDirect: Option<f32>,
    /// bool: (2020.1.0a3 - 2020.1.0a13)
    pub m_ExportTrainingData: Option<bool>,
    /**Whether the Progressive Lightmapper extracts Ambient Occlusion to a separate lightmap. (Editor only).*/
    /// bool: (2020.1.0a3 - 2020.1.0a13)
    pub m_ExtractAO: Option<bool>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_FilterMode: Option<i32>,
    /// bool: (2020.1.0a3 - 2020.1.0a13)
    pub m_FinalGather: Option<bool>,
    /// bool: (2020.1.0a3 - 2020.1.0a13)
    pub m_FinalGatherFiltering: Option<bool>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_FinalGatherRayCount: Option<i32>,
    /// bool: (2020.1.0a3 - 2020.1.0a13)
    pub m_ForceUpdates: Option<bool>,
    /// bool: (2020.1.0a3 - 2020.1.0a13)
    pub m_ForceWhiteAlbedo: Option<bool>,
    /// i32: (2020.1.0b1 - 2023.2.0a18)
    pub m_GIWorkflowMode: Option<i32>,
    /// f32: (2020.1.0b1 - 6000.2.0a6)
    pub m_IndirectOutputScale: Option<f32>,
    /**The maximum size in pixels of an individual lightmap texture. (Editor only).*/
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_LightmapMaxSize: Option<i32>,
    /// PPtr<[`LightmapParameters`]>: (2020.1.0a3 - 2020.1.0a13)
    pub m_LightmapParameters: Option<PPtr>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_LightmapsBakeMode: Option<i32>,
    /**Sets the MixedLightingMode that Unity uses for all Mixed Lights in the Scene. (Editor only).*/
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_MixedBakeMode: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRBounces: Option<i32>,
    /// bool: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRCulling: Option<bool>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRDenoiserTypeAO: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRDenoiserTypeDirect: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRDenoiserTypeIndirect: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRDirectSampleCount: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVREnvironmentMIS: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVREnvironmentReferencePointCount: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVREnvironmentSampleCount: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRFilterTypeAO: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRFilterTypeDirect: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRFilterTypeIndirect: Option<i32>,
    /// f32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRFilteringAtrousPositionSigmaAO: Option<f32>,
    /// f32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRFilteringAtrousPositionSigmaDirect: Option<f32>,
    /// f32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRFilteringAtrousPositionSigmaIndirect: Option<f32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRFilteringGaussRadiusAO: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRFilteringGaussRadiusDirect: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRFilteringGaussRadiusIndirect: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRFilteringMode: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRSampleCount: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_PVRSampling: Option<i32>,
    /// i32: (2020.1.0a3 - 2020.1.0a13)
    pub m_Padding: Option<i32>,
    /// f32: (2020.1.0a3 - 2020.1.0a13)
    pub m_RealtimeResolution: Option<f32>,
    /// bool: (2020.1.0a3 - 2020.1.0a13)
    pub m_TextureCompression: Option<bool>,
    /// String: (2020.1.0a3 - 2020.1.0a13)
    pub m_TrainingDataDestination: Option<String>,
}

/// LightmapBakeMode is a sub class of the Unity engine since version 2017.3.0b1.
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
- lightmapDir if you use LightmapsMode.CombinedDirectional.
- shadowMask if you use MixedLightingMode.Shadowmask.
Additional resources: LightmapSettings class, Renderer.lightmapIndex
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightmapData {
    /// PPtr<[`Texture2D`]>: (3.4.0 - 6000.2.0a6)
    pub m_Lightmap: PPtr,
    /// PPtr<[`Texture2D`]>: (5.6.0f1 - 6000.2.0a6)
    pub m_DirLightmap: Option<PPtr>,
    /// PPtr<[`Texture2D`]>: (3.4.0 - 5.6.0b1)
    pub m_IndirectLightmap: Option<PPtr>,
    /**Texture storing occlusion mask per light (ShadowMask, up to four lights).*/
    /// PPtr<[`Texture2D`]>: (5.6.0f1 - 6000.2.0a6)
    pub m_ShadowMask: Option<PPtr>,
}

/// LightmapParameters is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightmapParameters.html):
/**
Configures how Unity bakes lighting and can be assigned to a LightingSettings instance or asset.
Note that Unity's built-in Lightmap Parameters Assets are read-only.

Additional resources: LightmapParameters.SetLightmapParametersForLightingSettings.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightmapParameters {
    /**The maximum number of times to supersample a texel to reduce aliasing in AO.*/
    pub AOAntiAliasingSamples: i32,
    /**The number of rays to cast for computing ambient occlusion.*/
    pub AOQuality: i32,
    /**The kernel width the lightmapper uses when sampling a lightmap texel.*/
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
    /**The texel resolution per meter used for real-time lightmaps. This value is multiplied by LightingSettings.indirectResolution.*/
    pub resolution: f32,
    /**System tag is an integer identifier. It lets you force an object into a different Enlighten Realtime Global Illumination system even though all the other parameters are the same.*/
    pub systemTag: i32,
    /**If enabled, objects sharing the same lightmap parameters will be packed into LightmapParameters.maxLightmapCount lightmaps.*/
    /// bool: (2019.1.0b1 - 6000.2.0a6)
    pub limitLightmapCount: Option<bool>,
    /**The maximum number of lightmaps created for objects sharing the same lightmap parameters. This property is ignored if LightmapParameters.limitLightmapCount is false.*/
    /// i32: (2019.1.0b1 - 6000.2.0a6)
    pub maxLightmapCount: Option<i32>,
    /**The distance to offset the ray origin from the geometry when performing ray tracing, in modelling units. Unity applies the offset to all baked lighting: direct lighting, indirect lighting, environment lighting and ambient occlusion.*/
    /// f32: (5.0.1f1 - 6000.2.0a6)
    pub pushoff: Option<f32>,
}

/// LightmapSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LightmapSettings.html):
/**
Stores lightmaps of the Scene.
A Scene can have several lightmaps stored in it, and Renderer components can use those
lightmaps. This makes it possible to use the same material on multiple objects, while
each object can refer to a different lightmap or different portion of the same lightmap.Additional resources: LightmapData class, Renderer.lightmapIndex
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LightmapSettings {
    pub m_Lightmaps: Vec<LightmapData>,
    pub m_LightmapsMode: i32,
    /// i32: (6000.0.22f1 - 6000.2.0a6)
    pub m_BakeOnSceneLoad: Option<i32>,
    /// i32: (3.5.0 - 4.7.2)
    pub m_BakedColorSpace: Option<i32>,
    /// EnlightenSceneMapping: (5.0.0f4 - 6000.2.0a6)
    pub m_EnlightenSceneMapping: Option<EnlightenSceneMapping>,
    /// GISettings: (5.0.0f4 - 6000.2.0a6)
    pub m_GISettings: Option<GISettings>,
    /// PPtr<[`LightProbes`]>: (3.5.0 - 6000.2.0a6)
    pub m_LightProbes: Option<PPtr>,
    /// PPtr<[`LightingSettings`]>: (2020.1.0b1 - 6000.2.0a6)
    pub m_LightingSettings: Option<PPtr>,
    /// i32: (5.0.0f4 - 5.6.0b6)
    pub m_RuntimeCPUUsage: Option<i32>,
    /// i32: (5.6.0f1 - 5.6.7f1)
    pub m_ShadowMaskMode: Option<i32>,
    /// bool: (3.4.0 - 4.7.2)
    pub m_UseDualLightmapsInForward: Option<bool>,
    /// bool: (2017.1.0b1 - 2020.1.0a13)
    pub m_UseShadowmask: Option<bool>,
}

/// LightmapSnapshot is a  class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct LightmapSnapshot {
    /// Vec<PPtr<[`Texture`]>>: (5.0.0f4 - 5.2.5f1)
    pub m_BakedReflectionProbeCubemaps: Vec<PPtr>,
    pub m_BakedReflectionProbes: Vec<SceneObjectIdentifier>,
    pub m_EnlightenData: Vec<u8>,
    pub m_EnlightenSceneMapping: EnlightenSceneMapping,
    pub m_EnlightenSceneMappingRendererIDs: Vec<SceneObjectIdentifier>,
    /// PPtr<[`LightProbes`]>: (5.0.0f4 - 5.2.5f1)
    pub m_LightProbes: PPtr,
    pub m_LightmappedRendererData: Vec<RendererData>,
    pub m_LightmappedRendererDataIDs: Vec<SceneObjectIdentifier>,
    pub m_Lightmaps: Vec<LightmapData>,
    pub m_Lights: Vec<SceneObjectIdentifier>,
    pub m_Name: String,
    /// SphericalHarmonicsL2: (5.2.0f2 - 5.2.5f1)
    pub m_BakedAmbientProbeInGamma: Option<SphericalHarmonicsL2>,
    /// SphericalHarmonicsL2: (5.2.0f2 - 5.2.5f1)
    pub m_BakedAmbientProbeInLinear: Option<SphericalHarmonicsL2>,
    /// Vec<SphericalHarmonicsL2>: (5.0.0f4 - 5.1.5f1)
    pub m_BakedAmbientProbesInGamma: Option<Vec<SphericalHarmonicsL2>>,
    /// Vec<SphericalHarmonicsL2>: (5.0.0f4 - 5.1.5f1)
    pub m_BakedAmbientProbesInLinear: Option<Vec<SphericalHarmonicsL2>>,
    /// Vec<PPtr<[`Texture`]>>: (5.0.0f4 - 5.1.5f1)
    pub m_BakedSkyboxProbeCubemaps: Option<Vec<PPtr>>,
    /// GUID: (5.2.0f2 - 5.2.5f1)
    pub m_SceneGUID: Option<GUID>,
}

/// LightsModule is a sub class of the Unity engine since version 5.5.0f3.
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
    /// PPtr<[`Light`]>: (5.5.0f3 - 6000.2.0a6)
    pub light: PPtr,
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
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub alignment: Option<i32>,
    /// Gradient: (5.5.0f3 - 6000.2.0a6)
    pub colorGradient: Option<Gradient>,
    /// f32: (3.4.0 - 5.4.6f3)
    pub endWidth: Option<f32>,
    /// bool: (2017.1.0f1 - 6000.2.0a6)
    pub generateLightingData: Option<bool>,
    /// ColorRGBA: (3.4.0 - 5.4.6f3)
    pub m_EndColor: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 5.4.6f3)
    pub m_StartColor: Option<ColorRGBA>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub numCapVertices: Option<i32>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub numCornerVertices: Option<i32>,
    /// f32: (2018.3.0b1 - 6000.2.0a6)
    pub shadowBias: Option<f32>,
    /// f32: (3.4.0 - 5.4.6f3)
    pub startWidth: Option<f32>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub textureMode: Option<i32>,
    /// Vector2f: (2022.1.0b1 - 6000.2.0a6)
    pub textureScale: Option<Vector2f>,
    /// AnimationCurve: (5.5.0f3 - 6000.2.0a6)
    pub widthCurve: Option<AnimationCurve>,
    /// f32: (5.5.0f3 - 6000.2.0a6)
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapTilingOffset: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    /// Vec<PPtr<[`Material`]>>: (3.4.0 - 6000.2.0a6)
    pub m_Materials: Vec<PPtr>,
    pub m_Parameters: LineParameters,
    pub m_Positions: Vec<Vector3f>,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: Enum_bool__u8,
    /// PPtr<[`Transform`]>: (3.4.0 - 6000.2.0a6)
    pub m_StaticBatchRoot: PPtr,
    /**If enabled, the lines are defined in world space.*/
    pub m_UseWorldSpace: bool,
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_ApplyActiveColorSpace: Option<bool>,
    /// u8: (2017.2.0f1 - 6000.2.0a6)
    pub m_DynamicOccludee: Option<u8>,
    /// PPtr<[`Transform`]>: (3.5.0 - 4.7.2)
    pub m_LightProbeAnchor: Option<PPtr>,
    /**The light probe interpolation type.*/
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeUsage: Option<u8>,
    /// PPtr<[`GameObject`]>: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    /// u16: (5.0.0f4 - 6000.2.0a6)
    pub m_LightmapIndexDynamic: Option<u16>,
    /// Vector4f: (5.0.0f4 - 6000.2.0a6)
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    /**Connect the start and end positions of the line together to form a continuous loop.*/
    /// bool: (5.6.0f1 - 6000.2.0a6)
    pub m_Loop: Option<bool>,
    /**Specifies how the LineRenderer interacts with SpriteMask.*/
    /// i32: (2022.1.0b1 - 6000.2.0a6)
    pub m_MaskInteraction: Option<i32>,
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_MotionVectors: Option<u8>,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr<[`Transform`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_ProbeAnchor: Option<PPtr>,
    /// u8: (2020.1.0b1 - 6000.2.0a6)
    pub m_RayTraceProcedural: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlags: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlagsOverride: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0b1 - 6000.2.0a6)
    pub m_RayTracingMode: Option<u8>,
    /**Should reflection probes be used for this Renderer?*/
    /// i32: (5.0.0f4 - 5.3.8f2); u8: (5.4.0f3 - 6000.2.0a6)
    pub m_ReflectionProbeUsage: Option<i32>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on, if you use a scriptable render pipeline.*/
    /// u32: (2018.1.0f1 - 6000.2.0a6)
    pub m_RenderingLayerMask: Option<u32>,
    /// u8: (2023.3.0b1 - 6000.2.0a6)
    pub m_SmallMeshCulling: Option<u8>,
    /// i16: (4.3.0 - 6000.2.0a6)
    pub m_SortingLayer: Option<i16>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// u32: (4.5.0 - 4.7.2); i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SortingLayerID: Option<i64>,
    /**Renderer's order within a sorting layer.*/
    /// i16: (4.3.0 - 6000.2.0a6)
    pub m_SortingOrder: Option<i16>,
    /// StaticBatchInfo: (5.5.0f3 - 6000.2.0a6)
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.1.0b1 - 6000.2.0a6)
    pub m_StaticShadowCaster: Option<u8>,
    /// Vec<u32>: (3.4.0 - 5.4.6f3)
    pub m_SubsetIndices: Option<Vec<u32>>,
    /// bool: (3.5.0 - 5.3.8f2)
    pub m_UseLightProbes: Option<bool>,
}

/// LocalizationAsset is a  class of the Unity engine since version 2018.2.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/LocalizationAsset.html):
/**
An asset to represent a table of localized strings for one specific locale.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizationAsset {
    /**The name of the object.*/
    pub m_Name: String,
    /// bool: (2018.2.0b1 - 6000.2.0a6)
    #[serde(alias = "Editor Asset")]
    pub Editor_Asset: Option<bool>,
    /**ISO Code used to identify the locale. ex: en-uk, zh-hans, ja*/
    /// String: (2018.2.0b1 - 6000.2.0a6)
    #[serde(alias = "Locale ISO Code")]
    pub Locale_ISO_Code: Option<String>,
    /// Vec<(String, String)>: (2018.2.0b1 - 6000.2.0a6)
    #[serde(alias = "String Table")]
    pub String_Table: Option<Vec<(String, String)>>,
}

/// LocalizationImporter is a  class of the Unity engine since version 2018.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizationImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2018.2.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UserData: String,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// LookAtConstraint is a  class of the Unity engine since version 2018.2.0b1.
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
    /// PPtr<[`GameObject`]>: (2018.2.0b1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /// PPtr<[`Transform`]>: (2018.2.0b1 - 6000.2.0a6)
    pub m_WorldUpObject: PPtr,
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_Active: Option<bool>,
    /// bool: (2018.2.0b1 - 2022.1.0a9)
    pub m_IsContraintActive: Option<bool>,
}

/// Lumin is a sub class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct Lumin {
    pub depthFormat: i32,
    pub enableGLCache: bool,
    pub frameTiming: i32,
    pub glCacheMaxBlobSize: u32,
    pub glCacheMaxFileSize: u32,
}

/// MarshallingTestObject is a  class of the Unity engine since version 2023.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct MarshallingTestObject {
    pub m_Prop: i32,
}

/// Material is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Material.html):
/**
The material class.
This class exposes all properties from a material, allowing you to animate them.
You can also use it to set custom shader properties that can't be accessed through
the inspector (e.g. matrices).In order to get the material used by an object, use the Renderer.material property.Additional resources: Materials, Shaders.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    /**The name of the object.*/
    pub m_Name: String,
    pub m_SavedProperties: UnityPropertySheet,
    /**The shader used by the material.*/
    /// PPtr<[`Shader`]>: (3.4.0 - 6000.2.0a6)
    pub m_Shader: PPtr,
    /// Vec<String>: (5.6.0f1 - 6000.2.0a6)
    pub disabledShaderPasses: Option<Vec<String>>,
    /// Vec<BuildTextureStackReference>: (2020.1.0b1 - 6000.2.0a6)
    pub m_BuildTextureStacks: Option<Vec<BuildTextureStackReference>>,
    /// i32: (4.3.0 - 6000.2.0a6)
    pub m_CustomRenderQueue: Option<i32>,
    /**Gets and sets whether the Double Sided Global Illumination setting is enabled for this material.*/
    /// bool: (5.6.2f1 - 6000.2.0a6)
    pub m_DoubleSidedGI: Option<bool>,
    /// bool: (5.6.0f1 - 6000.2.0a6)
    pub m_EnableInstancingVariants: Option<bool>,
    /// Vec<String>: (2021.2.18f1 - 6000.2.0a6)
    pub m_InvalidKeywords: Option<Vec<String>>,
    /// u32: (5.0.0f4 - 6000.2.0a6)
    pub m_LightmapFlags: Option<u32>,
    /**An array containing names of the local shader keywords that are currently enabled for this material.*/
    /// Vec<String>: (4.1.0 - 4.7.2); String: (5.0.0f4 - 2022.1.0a16)
    pub m_ShaderKeywords: Option<Enum_Vec_String___String>,
    /// Vec<String>: (2021.2.18f1 - 6000.2.0a6)
    pub m_ValidKeywords: Option<Vec<String>>,
    /// Vec<(String, String)>: (5.1.0f1 - 6000.2.0a6)
    pub stringTagMap: Option<Vec<(String, String)>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_Vec_String___String {
    Vec(Vec<String>),
    String(String),
}

/// MaterialImportOutput is a sub class of the Unity engine since version 4.0.0.
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
    pub textureParameters: Vec<InputImportSettings>,
    /// u32: (5.0.2f1 - 2017.4.40f1)
    pub lightmapFlags: Option<u32>,
    /// i32: (5.0.0f4 - 2017.4.40f1)
    pub renderQueue: Option<i32>,
    /// PPtr<[`Shader`]>: (4.5.0 - 2017.4.40f1)
    pub shader: Option<PPtr>,
    /// String: (5.0.0f4 - 2017.4.40f1)
    pub shaderKeywords: Option<String>,
    /// String: (3.4.0 - 2017.4.40f1)
    pub shaderName: Option<String>,
    /// Vec<ProceduralTextureAssignment>: (4.5.0 - 2017.4.40f1)
    pub textureAssignments: Option<Vec<ProceduralTextureAssignment>>,
}

/// Matrix3x4f is a sub class of the Unity engine since version 3.5.0.
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

/// MatrixParameter is a sub class of the Unity engine since version 5.5.0f3.
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
    /// Vec<u8>: (3.4.0 - 4.7.2)
    #[serde(alias = "md4 hash")]
    pub md4_hash: Option<Vec<u8>>,
}

/// MemorySettings is a  class of the Unity engine since version 2021.2.0b1.
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
    /// Vec<u8>: (5.0.0f4 - 6000.2.0a6)
    pub m_BakedConvexCollisionMesh: Option<Vec<u8>>,
    /// Vec<u8>: (5.0.0f4 - 6000.2.0a6)
    pub m_BakedTriangleCollisionMesh: Option<Vec<u8>>,
    /// Vec<u32>: (4.3.0 - 6000.2.0a6)
    pub m_BoneNameHashes: Option<Vec<u32>>,
    /// Vec<MinMaxAABB>: (2019.1.0b1 - 6000.2.0a6)
    pub m_BonesAABB: Option<Vec<MinMaxAABB>>,
    /// Vec<u32>: (3.4.0 - 3.4.2)
    pub m_CollisionTriangles: Option<Vec<u32>>,
    /// i32: (3.4.0 - 3.4.2)
    pub m_CollisionVertexCount: Option<i32>,
    /**Vertex colors of the Mesh.*/
    /// Vec<ColorRGBA>: (3.4.0 - 3.4.2)
    pub m_Colors: Option<Vec<ColorRGBA>>,
    /// i32: (2022.1.0b1 - 6000.2.0a6)
    pub m_CookingOptions: Option<i32>,
    /**Format of the mesh index buffer data.*/
    /// i32: (2017.3.0b1 - 6000.2.0a6)
    pub m_IndexFormat: Option<i32>,
    /**Returns true if the Mesh is read/write enabled, or false if it is not.*/
    /// bool: (4.0.0 - 6000.2.0a6)
    pub m_IsReadable: Option<bool>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub m_KeepIndices: Option<bool>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub m_KeepVertices: Option<bool>,
    /// f32: (6000.2.0a6 - 6000.2.0a6)
    #[serde(alias = "m_MeshMetrics[0]")]
    pub m_MeshMetrics_0_: Option<f32>,
    /// f32: (6000.2.0a6 - 6000.2.0a6)
    #[serde(alias = "m_MeshMetrics[1]")]
    pub m_MeshMetrics_1_: Option<f32>,
    /**The normals of the Mesh.*/
    /// Vec<Vector3f>: (3.4.0 - 3.4.2)
    pub m_Normals: Option<Vec<Vector3f>>,
    /// u32: (4.3.0 - 6000.2.0a6)
    pub m_RootBoneNameHash: Option<u32>,
    /// Vec<MeshBlendShapeVertex>: (4.1.0 - 4.2.2)
    pub m_ShapeVertices: Option<Vec<MeshBlendShapeVertex>>,
    /// Vec<MeshBlendShape>: (4.1.0 - 4.2.2); BlendShapeData: (4.3.0 - 6000.2.0a6)
    pub m_Shapes: Option<Enum_Vec_MeshBlendShape___BlendShapeData>,
    /// Vec<BoneInfluence>: (3.4.0 - 5.6.7f1); Vec<BoneWeights4>: (2017.1.0b1 - 2018.1.9f2)
    pub m_Skin: Option<Vec<Enum_BoneWeights4__BoneInfluence>>,
    /// u8: (4.0.0 - 4.7.2)
    pub m_StreamCompression: Option<u8>,
    /// StreamingInfo: (2018.3.0b1 - 6000.2.0a6)
    pub m_StreamData: Option<StreamingInfo>,
    /**The tangents of the Mesh.*/
    /// Vec<Vector4f>: (3.4.0 - 3.4.2)
    pub m_Tangents: Option<Vec<Vector4f>>,
    /**The texture coordinates (UVs) in the first channel.*/
    /// Vec<Vector2f>: (3.4.0 - 3.4.2)
    pub m_UV: Option<Vec<Vector2f>>,
    /// Vec<Vector2f>: (3.4.0 - 3.4.2)
    pub m_UV1: Option<Vec<Vector2f>>,
    /// i32: (3.4.0 - 3.4.2)
    pub m_Use16BitIndices: Option<i32>,
    /// VariableBoneCountWeights: (2019.1.0b1 - 6000.2.0a6)
    pub m_VariableBoneCountWeights: Option<VariableBoneCountWeights>,
    /// VertexData: (3.5.0 - 6000.2.0a6)
    pub m_VertexData: Option<VertexData>,
    /**Returns a copy of the vertex positions or assigns a new vertex positions array.*/
    /// Vec<Vector3f>: (3.4.0 - 3.4.2)
    pub m_Vertices: Option<Vec<Vector3f>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_Vec_MeshBlendShape___BlendShapeData {
    Vec(Vec<MeshBlendShape>),
    BlendShapeData(BlendShapeData),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_BoneWeights4__BoneInfluence {
    BoneWeights4(BoneWeights4),
    BoneInfluence(BoneInfluence),
}

/// MeshBlendShape is a sub class of the Unity engine since version 4.1.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshBlendShape {
    pub firstVertex: u32,
    pub hasNormals: bool,
    pub hasTangents: bool,
    pub vertexCount: u32,
    /// Vector3f: (4.1.0 - 4.2.2)
    pub aabbMaxDelta: Option<Vector3f>,
    /// Vector3f: (4.1.0 - 4.2.2)
    pub aabbMinDelta: Option<Vector3f>,
    /// String: (4.1.0 - 4.2.2)
    pub name: Option<String>,
}

/// MeshBlendShapeChannel is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshBlendShapeChannel {
    pub frameCount: i32,
    pub frameIndex: i32,
    pub name: String,
    pub nameHash: u32,
}

/// MeshBlendShapeVertex is a sub class of the Unity engine since version 4.1.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshBlendShapeVertex {
    pub index: i32,
    pub normal: Vector3f,
    pub tangent: Vector3f,
    pub vertex: Vector3f,
}

/// MeshCollider is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/MeshCollider.html):
/**
A mesh collider allows you to do collision detection between meshes and primitives.
Additional resources: BoxCollider, CapsuleCollider, PhysicsMaterial, Rigidbody.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshCollider {
    /**Use a convex collider from the mesh.*/
    pub m_Convex: bool,
    /**Enabled Colliders will collide with other Colliders, disabled Colliders won't.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Specify if this collider is configured as a trigger.*/
    pub m_IsTrigger: bool,
    /**The material used by the collider.*/
    /// PPtr<[`PhysicMaterial`]>: (3.4.0 - 2023.3.0a8); PPtr<[`PhysicsMaterial`]>: (2023.3.0b1 - 6000.2.0a6)
    pub m_Material: PPtr,
    /// PPtr<[`Mesh`]>: (3.4.0 - 6000.2.0a6)
    pub m_Mesh: PPtr,
    /**Options used to enable or disable certain features in mesh cooking.*/
    /// i32: (2017.3.0b1 - 6000.2.0a6)
    pub m_CookingOptions: Option<i32>,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /// bool: (5.5.0f3 - 2017.2.5f1)
    pub m_InflateMesh: Option<bool>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_ProvidesContacts: Option<bool>,
    /// f32: (5.5.0f3 - 2018.2.21f1)
    pub m_SkinWidth: Option<f32>,
    /// bool: (3.4.0 - 4.7.2)
    pub m_SmoothSphereCollisions: Option<bool>,
}

/// MeshFilter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/MeshFilter.html):
/**
A class to access the Mesh of the mesh filter.
Use this with a procedural mesh interface. Additional resources: Mesh class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshFilter {
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Returns either a new mesh or a duplicate of the existing mesh, and assigns it to the mesh filter.*/
    /// PPtr<[`Mesh`]>: (3.4.0 - 6000.2.0a6)
    pub m_Mesh: PPtr,
}

/// MeshParticleEmitter is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshParticleEmitter {
    pub angularVelocity: f32,
    pub emitterVelocityScale: f32,
    pub localVelocity: Vector3f,
    pub m_Emit: bool,
    pub m_Enabled: bool,
    /// PPtr<[`GameObject`]>: (3.4.0 - 2018.2.21f1)
    pub m_GameObject: PPtr,
    pub m_InterpolateTriangles: bool,
    pub m_MaxNormalVelocity: f32,
    /// PPtr<[`Mesh`]>: (3.4.0 - 2018.2.21f1)
    pub m_Mesh: PPtr,
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
    /// bool: (3.4.0 - 2018.2.21f1)
    #[serde(alias = "Simulate in Worldspace?")]
    pub Simulate_in_Worldspace_: Option<bool>,
}

/// MeshRenderer is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/MeshRenderer.html):
/**
Renders the mesh from the MeshFilter component on the same GameObject.
This component requires a MeshFilter to be attached to the same GameObject. When the component is active, it renders the Mesh specified in the MeshFilter, including any submeshes.

You can disable the MeshRenderer to make the GameObject invisible.To render deformable meshes, use the SkinnedMeshRenderer component instead.Additional resources: MeshFilter, Mesh.The following code sample creates 10 randomly-colored renderers at random positions in the scene. To use it, attach it to a GameObject and run the project.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshRenderer {
    /**Vertex attributes in this mesh will override or add attributes of the primary mesh in the MeshRenderer.*/
    /// PPtr<[`Mesh`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_AdditionalVertexStreams: PPtr,
    pub m_CastShadows: u8,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    /// Vec<PPtr<[`Material`]>>: (5.0.0f4 - 6000.2.0a6)
    pub m_Materials: Vec<PPtr>,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr<[`Transform`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_ProbeAnchor: PPtr,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: Enum_bool__u8,
    /**Should reflection probes be used for this Renderer?*/
    pub m_ReflectionProbeUsage: i32,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    /// PPtr<[`Transform`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_StaticBatchRoot: PPtr,
    /// u8: (2017.2.0f1 - 6000.2.0a6)
    pub m_DynamicOccludee: Option<u8>,
    /**Vertex attributes that override the primary mesh when the MeshRenderer uses lightmaps in the Realtime Global Illumination system.*/
    /// PPtr<[`Mesh`]>: (2020.1.0b1 - 6000.2.0a6)
    pub m_EnlightenVertexStream: Option<PPtr>,
    /**The light probe interpolation type.*/
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeUsage: Option<u8>,
    /// PPtr<[`GameObject`]>: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_MotionVectors: Option<u8>,
    /// u8: (2020.1.0b1 - 6000.2.0a6)
    pub m_RayTraceProcedural: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlags: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlagsOverride: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0b1 - 6000.2.0a6)
    pub m_RayTracingMode: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on, if you use a scriptable render pipeline.*/
    /// u32: (2018.1.0f1 - 6000.2.0a6)
    pub m_RenderingLayerMask: Option<u32>,
    /// u8: (2023.3.0b1 - 6000.2.0a6)
    pub m_SmallMeshCulling: Option<u8>,
    /// i16: (5.6.0b1 - 6000.2.0a6)
    pub m_SortingLayer: Option<i16>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SortingLayerID: Option<i32>,
    /// StaticBatchInfo: (5.5.0f3 - 6000.2.0a6)
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.1.0b1 - 6000.2.0a6)
    pub m_StaticShadowCaster: Option<u8>,
    /// Vec<u32>: (5.0.0f4 - 5.4.6f3)
    pub m_SubsetIndices: Option<Vec<u32>>,
    /// bool: (5.0.0f4 - 5.3.8f2)
    pub m_UseLightProbes: Option<bool>,
}

/// MinMaxAABB is a sub class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct MinMaxAABB {
    pub m_Max: Vector3f,
    pub m_Min: Vector3f,
}

/// MinMaxCurve is a sub class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.MinMaxCurve.html):
/**
Script interface for a Min-Max Curve.
Min-Max Curve. describes functions which take a value between a minimum and maximum limit and return a value based on ParticleSystem.MinMaxCurve.mode. Depending on the mode, this may return randomized values.
For modes that require curves, the value returned is dependent on one or two curves designed in the ParticleSystem Inspector, that can be evaluated to a single value between -n and n, where n is a constant also set in the Inspector. See ParticleSystemCurveMode for more information.Additional resources: ParticleSystem.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MinMaxCurve {
    pub maxCurve: AnimationCurve,
    pub minCurve: AnimationCurve,
    pub minMaxState: i32,
    pub scalar: f32,
    /// f32: (5.6.1f1 - 6000.2.0a6)
    pub minScalar: Option<f32>,
}

/// MinMaxGradient is a sub class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.MinMaxGradient.html):
/**
Script interface for a Min-Max Gradient.
This contains two Gradients, and returns a Color based on ParticleSystem.MinMaxGradient.mode. Depending on the mode, this may return the value randomized.
Gradients are edited via the ParticleSystem Inspector once a ParticleSystemGradientMode requiring them has been selected. Some modes do not require gradients, only colors.
Additional resources: ParticleSystem.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MinMaxGradient {
    pub maxColor: ColorRGBA,
    pub maxGradient: Enum_GradientNEW__Gradient,
    pub minColor: ColorRGBA,
    pub minGradient: Enum_GradientNEW__Gradient,
    pub minMaxState: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_GradientNEW__Gradient {
    GradientNEW(GradientNEW),
    Gradient(Gradient),
}

/// MipmapLimitSettings is a sub class of the Unity engine since version 2022.2.0f1.
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
    /**Animation clips to split animation into. Additional resources: ModelImporterClipAnimation.*/
    pub m_ClipAnimations: Vec<ClipAnimationInfo>,
    /**Global scale factor for importing.*/
    pub m_GlobalScale: f32,
    pub m_HasExtraRoot: bool,
    /// Vec<PPtr<[`GameObject`]>>: (3.4.0 - 6000.2.0a6)
    pub m_ImportedRoots: Vec<PPtr>,
    /**Mesh compression setting.*/
    pub m_MeshCompression: i32,
    /**The name of the object.*/
    pub m_Name: String,
    /**Detect file units and import as 1FileUnit=1UnityUnit, otherwise it will import as 1cm=1UnityUnit.*/
    pub m_UseFileUnits: bool,
    pub normalSmoothAngle: f32,
    /**Computes the axis conversion on geometry and animation for Models defined in an axis system that differs from Unity's (left handed, Z forward, Y-up).                     When enabled, Unity transforms the geometry and animation data in order to convert the axis.                     When disabled, Unity transforms the root GameObject of the hierarchy in order to convert the axis.*/
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub bakeAxisConversion: Option<bool>,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub blendShapeNormalImportMode: Option<i32>,
    /**Generate secondary UV set for lightmapping.*/
    /// bool: (3.5.0 - 6000.2.0a6)
    pub generateSecondaryUV: Option<bool>,
    /**Format of the imported mesh index buffer data.*/
    /// i32: (2017.3.0b1 - 6000.2.0a6)
    pub indexFormat: Option<i32>,
    /**If this is true, any quad faces that exist in the mesh data before it is imported are kept as quads instead of being split into two triangles, for the purposes of tessellation. Set this to false to disable this behavior.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub keepQuads: Option<bool>,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub legacyComputeAllNormalsFromSmoothingGroupsWhenMeshHasBlendShapes: Option<bool>,
    /// bool: (2020.2.0f1 - 6000.2.0a6)
    pub m_AddHumanoidExtraRootOnlyWhenUsingAvatar: Option<bool>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub m_AdditionalBone: Option<bool>,
    /// bool: (5.1.0f1 - 6000.2.0a6)
    pub m_AnimationDoRetargetingWarnings: Option<bool>,
    /// String: (5.1.0f1 - 6000.2.0a6)
    pub m_AnimationImportErrors: Option<String>,
    /// String: (5.1.0f1 - 6000.2.0a6)
    pub m_AnimationImportWarnings: Option<String>,
    /// String: (5.1.0f1 - 6000.2.0a6)
    pub m_AnimationRetargetingWarnings: Option<String>,
    /**Animator generation mode.*/
    /// i32: (4.0.0 - 6000.2.0a6)
    pub m_AnimationType: Option<i32>,
    /**Get or set the AssetBundle name.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleName: Option<String>,
    /**Get or set the AssetBundle variant.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleVariant: Option<String>,
    /**Generate auto mapping if no avatarSetup is provided when importing humanoid animation.*/
    /// bool: (2019.3.0f1 - 6000.2.0a6)
    pub m_AutoGenerateAvatarMappingIfUnspecified: Option<bool>,
    /// bool: (2017.2.0b2 - 2017.2.0b6)
    pub m_AutoMapExternalMaterials: Option<bool>,
    /**The Avatar generation of the imported model.*/
    /// i32: (2019.3.0b1 - 6000.2.0a6)
    pub m_AvatarSetup: Option<i32>,
    /// bool: (2023.1.0b1 - 6000.2.0a6)
    pub m_ContainsAnimation: Option<bool>,
    /// bool: (4.0.0 - 2019.3.0a2)
    pub m_CopyAvatar: Option<bool>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /**Animation optimization setting.*/
    /// Vec<String>: (4.3.0 - 6000.2.0a6)
    pub m_ExtraExposedTransformPaths: Option<Vec<String>>,
    /**A list of default FBX properties to treat as user properties during OnPostprocessGameObjectWithUserProperties.*/
    /// Vec<String>: (2017.1.0f1 - 6000.2.0a6)
    pub m_ExtraUserProperties: Option<Vec<String>>,
    /// Vec<(i32, String)>: (3.4.0 - 4.7.2); Vec<(i64, String)>: (5.0.0f4 - 2018.4.36f1)
    pub m_FileIDToRecycleName: Option<Vec<(i64, String)>>,
    /// i32: (2019.4.0f1 - 6000.2.0a6)
    pub m_FileIdsGeneration: Option<i32>,
    /**Scaling factor used when useFileScale is set to true (Read-only).*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_FileScale: Option<f32>,
    /// f32: (2018.3.0b1 - 6000.2.0a6)
    pub m_FileScaleFactor: Option<f32>,
    /// String: (2018.3.0b1 - 6000.2.0a6)
    pub m_FileScaleUnit: Option<String>,
    /// i32: (3.4.0 - 3.4.2)
    pub m_FirstImportVersion: Option<i32>,
    /**Animation generation options.*/
    /// i32: (3.4.0 - 3.5.7)
    pub m_GenerateAnimations: Option<i32>,
    /// i32: (3.4.0 - 3.4.2)
    pub m_GenerateMaterials: Option<i32>,
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub m_HasEmbeddedTextures: Option<bool>,
    /// bool: (2018.2.0b1 - 2019.1.0a13)
    pub m_HasPreviousCalculatedGlobalScale: Option<bool>,
    /**The human description that is used to generate an Avatar during the import process.*/
    /// HumanDescription: (4.0.0 - 6000.2.0a6)
    pub m_HumanDescription: Option<HumanDescription>,
    /**Controls how much oversampling is used when importing humanoid animations for retargeting.*/
    /// i32: (5.2.0f2 - 6000.2.0a6)
    pub m_HumanoidOversampling: Option<i32>,
    /**Import animated custom properties from file.*/
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub m_ImportAnimatedCustomProperties: Option<bool>,
    /**Import animation from file.*/
    /// bool: (4.0.0 - 6000.2.0a6)
    pub m_ImportAnimation: Option<bool>,
    /**Import BlendShapes deform percent.*/
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_ImportBlendShapeDeformPercent: Option<bool>,
    /**Controls import of BlendShapes.*/
    /// bool: (4.1.0 - 6000.2.0a6)
    pub m_ImportBlendShapes: Option<bool>,
    /**Controls import of cameras. Basic properties like field of view, near plane distance and far plane distance can be animated.*/
    /// bool: (2017.1.0b1 - 6000.2.0a6)
    pub m_ImportCameras: Option<bool>,
    /**Import animation constraints.*/
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub m_ImportConstraints: Option<bool>,
    /**Controls import of lights. Note that because light are defined differently in DCC tools, some light types or properties may not be exported. Basic properties like color and intensity can be animated.*/
    /// bool: (2017.1.0b1 - 6000.2.0a6)
    pub m_ImportLights: Option<bool>,
    /// bool: (3.5.0 - 2019.3.0a6)
    pub m_ImportMaterials: Option<bool>,
    /// bool: (2022.2.19f1 - 6000.2.0a6)
    pub m_ImportPhysicalCameras: Option<bool>,
    /**Use visibility properties to enable or disable MeshRenderer components.*/
    /// bool: (2017.1.0b1 - 6000.2.0a6)
    pub m_ImportVisibility: Option<bool>,
    /**Generates the list of all imported take.*/
    /// Vec<TakeInfo>: (4.0.0 - 6000.2.0a6)
    pub m_ImportedTakeInfos: Option<Vec<TakeInfo>>,
    /// Vec<((i32, i64), String)>: (2019.1.0b1 - 6000.2.0a6)
    pub m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
    /**Are mesh vertices and indices accessible from script?*/
    /// bool: (4.0.0 - 6000.2.0a6)
    pub m_IsReadable: Option<bool>,
    /// Vec<f32>: (3.5.0 - 6000.2.0a6)
    pub m_LODScreenPercentages: Option<Vec<f32>>,
    /// PPtr<[`Avatar`]>: (4.0.0 - 6000.2.0a6)
    pub m_LastHumanDescriptionAvatarSource: Option<PPtr>,
    /// i32: (4.0.0 - 6000.2.0a6)
    pub m_LegacyGenerateAnimations: Option<i32>,
    /**Material creation options.*/
    /// i32: (2019.3.0b1 - 6000.2.0a6)
    pub m_MaterialImportMode: Option<i32>,
    /**Material import location options.*/
    /// i32: (2017.2.0f1 - 6000.2.0a6)
    pub m_MaterialLocation: Option<i32>,
    /**Material naming setting.*/
    /// i32: (3.5.0 - 6000.2.0a6)
    pub m_MaterialName: Option<i32>,
    /**Existing material search setting.*/
    /// i32: (3.5.0 - 6000.2.0a6)
    pub m_MaterialSearch: Option<i32>,
    /// Vec<SourceAssetIdentifier>: (2017.2.0f1 - 6000.2.0a6)
    pub m_Materials: Option<Vec<SourceAssetIdentifier>>,
    /// bool: (3.4.0 - 3.4.2)
    #[serde(alias = "m_MeshSettings.generateSecondaryUV")]
    pub m_MeshSettings_generateSecondaryUV: Option<bool>,
    /// i32: (3.4.0 - 3.4.2)
    #[serde(alias = "m_MeshSettings.normalImportMode")]
    pub m_MeshSettings_normalImportMode: Option<i32>,
    /// f32: (3.4.0 - 3.4.2)
    #[serde(alias = "m_MeshSettings.secondaryUVAngleDistortion")]
    pub m_MeshSettings_secondaryUVAngleDistortion: Option<f32>,
    /// f32: (3.4.0 - 3.4.2)
    #[serde(alias = "m_MeshSettings.secondaryUVAreaDistortion")]
    pub m_MeshSettings_secondaryUVAreaDistortion: Option<f32>,
    /// f32: (3.4.0 - 3.4.2)
    #[serde(alias = "m_MeshSettings.secondaryUVHardAngle")]
    pub m_MeshSettings_secondaryUVHardAngle: Option<f32>,
    /// f32: (3.4.0 - 3.4.2)
    #[serde(alias = "m_MeshSettings.secondaryUVPackMargin")]
    pub m_MeshSettings_secondaryUVPackMargin: Option<f32>,
    /// bool: (3.4.0 - 3.4.2)
    #[serde(alias = "m_MeshSettings.swapUVChannels")]
    pub m_MeshSettings_swapUVChannels: Option<bool>,
    /// i32: (3.4.0 - 3.4.2)
    #[serde(alias = "m_MeshSettings.tangentImportMode")]
    pub m_MeshSettings_tangentImportMode: Option<i32>,
    /**The path of the transform used to generation the motion of the animation.*/
    /// String: (4.5.0 - 6000.2.0a6)
    pub m_MotionNodeName: Option<String>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_NewHashIdentity: Option<MdFour>,
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_NodeNameCollisionStrategy: Option<i32>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_OldHashIdentity: Option<MdFour>,
    /**Animation optimization setting.*/
    /// bool: (4.3.0 - 6000.2.0a6)
    pub m_OptimizeGameObjects: Option<bool>,
    /**If true, always create an explicit Prefab root. Otherwise, if the model has a single root, it is reused as the Prefab root.*/
    /// bool: (2017.3.0b1 - 6000.2.0a6)
    pub m_PreserveHierarchy: Option<bool>,
    /// f32: (2018.2.0b1 - 2019.1.0a13)
    pub m_PreviousCalculatedGlobalScale: Option<f32>,
    /**Returns the matching referenced clip assets for this model.*/
    /// Vec<GUID>: (4.0.0 - 6000.2.0a6)
    pub m_ReferencedClips: Option<Vec<GUID>>,
    /// bool: (2020.3.37f1 - 6000.2.0a6)
    pub m_RemapMaterialsIfMaterialImportModeIsNone: Option<bool>,
    /**Removes constant animation curves with values identical to the object initial scale value.*/
    /// bool: (2021.2.0b1 - 6000.2.0a6)
    pub m_RemoveConstantScaleCurves: Option<bool>,
    /**If set to false, the importer will not resample curves when possible.Read more about animation curve resampling.Notes:- Some unsupported FBX features (such as PreRotation or PostRotation on transforms) will override this setting. In these situations, animation curves will still be resampled even if the setting is disabled. For best results, avoid using PreRotation, PostRotation and GetRotationPivot.- This option was introduced in Version 5.3. Prior to this version, Unity's import behaviour was as if this option was always enabled. Therefore enabling the option gives the same behaviour as pre-5.3 animation import.*/
    /// bool: (5.4.0f3 - 6000.2.0a6)
    pub m_ResampleCurves: Option<bool>,
    /// bool: (5.3.0f1 - 5.3.8f2)
    pub m_ResampleRotations: Option<bool>,
    /// String: (5.6.0b1 - 2023.1.0a15)
    pub m_RigImportErrors: Option<String>,
    /// String: (5.6.0b1 - 2023.1.0a15)
    pub m_RigImportWarnings: Option<String>,
    /**Sorts the gameObject hierarchy by name.*/
    /// bool: (2019.2.0b1 - 6000.2.0a6)
    pub m_SortHierarchyByName: Option<bool>,
    /// bool: (3.4.0 - 3.5.7)
    pub m_SplitAnimations: Option<bool>,
    /**Enables strict checks on imported vertex data.*/
    /// bool: (2021.3.36f1 - 6000.2.0a6)
    pub m_StrictVertexDataChecks: Option<bool>,
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub m_SupportsEmbeddedMaterials: Option<bool>,
    /**Use FileScale when importing.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_UseFileScale: Option<bool>,
    /**When disabled, imported material albedo colors are converted to gamma space. This property should be disabled when using linear color space in Player rendering settings.The default value is true.*/
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_UseSRGBMaterialColor: Option<bool>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /**Get or set any user data.*/
    /// String: (4.0.0 - 6000.2.0a6)
    pub m_UserData: Option<String>,
    /**The maximum number of bones per vertex stored in this mesh data.*/
    /// i32: (2019.1.0b1 - 6000.2.0a6)
    pub maxBonesPerVertex: Option<i32>,
    /**Options to control the optimization of mesh data during asset import.*/
    /// i32: (2019.1.0b1 - 6000.2.0a6)
    pub meshOptimizationFlags: Option<i32>,
    /**Minimum bone weight to keep.*/
    /// f32: (2019.1.0b1 - 6000.2.0a6)
    pub minBoneWeight: Option<f32>,
    /**Normal generation options for ModelImporter.*/
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub normalCalculationMode: Option<i32>,
    /// i32: (3.5.0 - 6000.2.0a6)
    pub normalImportMode: Option<i32>,
    /**Source of smoothing information for calculation of normals.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub normalSmoothingSource: Option<i32>,
    /**Only import bones where they are connected to vertices.*/
    /// bool: (2021.2.0b1 - 6000.2.0a6)
    pub optimizeBones: Option<bool>,
    /// bool: (3.5.0 - 3.5.7)
    pub optimizeMesh: Option<bool>,
    /// bool: (4.0.0 - 2018.4.36f1)
    pub optimizeMeshForGPU: Option<bool>,
    /**Threshold for angle distortion (in degrees) when generating secondary UV.*/
    /// f32: (3.5.0 - 6000.2.0a6)
    pub secondaryUVAngleDistortion: Option<f32>,
    /**Threshold for area distortion when generating secondary UV.*/
    /// f32: (3.5.0 - 6000.2.0a6)
    pub secondaryUVAreaDistortion: Option<f32>,
    /**Hard angle (in degrees) for generating secondary UV.*/
    /// f32: (3.5.0 - 6000.2.0a6)
    pub secondaryUVHardAngle: Option<f32>,
    /**Method to use for handling margins when generating secondary UV.*/
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub secondaryUVMarginMethod: Option<i32>,
    /**The minimum lightmap resolution in texels per unit that the associated model is expected to have.*/
    /// f32: (2020.1.0b1 - 6000.2.0a6)
    pub secondaryUVMinLightmapResolution: Option<f32>,
    /**The minimum object scale that the associated model is expected to have.*/
    /// f32: (2020.1.0b1 - 6000.2.0a6)
    pub secondaryUVMinObjectScale: Option<f32>,
    /**Margin to be left between charts when packing secondary UV.*/
    /// f32: (3.5.0 - 6000.2.0a6)
    pub secondaryUVPackMargin: Option<f32>,
    /// i32: (2019.1.0b1 - 6000.2.0a6)
    pub skinWeightsMode: Option<i32>,
    /// bool: (3.4.0 - 5.2.5f1)
    pub splitTangentsAcrossUV: Option<bool>,
    /**Swap primary and secondary UV channels when importing.*/
    /// bool: (3.5.0 - 6000.2.0a6)
    pub swapUVChannels: Option<bool>,
    /// i32: (3.5.0 - 6000.2.0a6)
    pub tangentImportMode: Option<i32>,
    /**Combine vertices that share the same position in space.*/
    /// bool: (4.1.0 - 6000.2.0a6)
    pub weldVertices: Option<bool>,
}

/// Module is a sub class of the Unity engine since version 2018.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub dependencies: Vec<String>,
    pub name: String,
    pub strippable: bool,
    /// bool: (2018.2.0f1 - 6000.2.0a6)
    pub controlledByBuiltinPackage: Option<bool>,
}

/// MonoAssemblyImporter is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct MonoAssemblyImporter {
    pub m_ExecutionOrder: Vec<(String, i32)>,
    /// Vec<(String, PPtr<[`Texture2D`]>)>: (3.4.0 - 4.7.2)
    pub m_IconMap: Vec<(String, PPtr)>,
    pub m_Name: String,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.2)
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_NewHashIdentity: Option<MdFour>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_OldHashIdentity: Option<MdFour>,
    /// String: (4.0.0 - 4.7.2)
    pub m_UserData: Option<String>,
}

/// MonoBehaviour is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/MonoBehaviour.html):
/**
MonoBehaviour is a base class that many Unity scripts derive from.
MonoBehaviour offers life cycle functions that make it easier to develop with Unity.MonoBehaviours always exist as a Component of a GameObject, and can be instantiated with GameObject.AddComponent.  Objects that need to exist independently of a GameObject should derive from ScriptableObject instead.A MonoBehaviour can be deleted with Object.Destroy or Object.DestroyImmediate.  When the parent GameObject is destroyed all components are automatically deleted, including MonoBehaviours.After the underlying component is destroyed, the C# object for the MonoBehaviour remains in memory until garbage is collected. A MonoBehaviour in this state acts as if it is null. For example, it returns true for a "obj == null" check.
However, this class doesn't support the null-conditional operator  (?.) and the null-coalescing operator (??).When a MonoBehaviour is serialized, the value of C# fields are included according to Unity's Serialization rules. See Script Serialization for details.
The serialized data also includes internal properties, such as the reference to the MonoScript that tracks the implementation class for the object.For code samples, see the individual MonoBehaviour methods.Note: In the Editor, when the MonoBehaviour component is deactivated through the checkbox in the Inspector, it stops receiving calls to its Event functions.Additional resources: The Deactivating GameObjects page in the manual.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MonoBehaviour {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The name of the object.*/
    pub m_Name: String,
    /// PPtr<[`MonoScript`]>: (3.4.0 - 6000.2.0a6)
    pub m_Script: PPtr,
}

/// MonoImporter is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/MonoImporter.html):
/**
Represents a C# script in the project.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MonoImporter {
    pub executionOrder: i16,
    /// PPtr<[`Texture2D`]>: (3.4.0 - 6000.2.0a6)
    pub icon: PPtr,
    /// Vec<(String, PPtr<[`Object`]>)>: (3.4.0 - 6000.2.0a6)
    pub m_DefaultReferences: Vec<(String, PPtr)>,
    /**The name of the object.*/
    pub m_Name: String,
    /**Get or set the AssetBundle name.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleName: Option<String>,
    /**Get or set the AssetBundle variant.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleVariant: Option<String>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.2)
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_NewHashIdentity: Option<MdFour>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_OldHashIdentity: Option<MdFour>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /**Get or set any user data.*/
    /// String: (4.0.0 - 6000.2.0a6)
    pub m_UserData: Option<String>,
}

/// MonoManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct MonoManager {
    /// Vec<PPtr<[`MonoScript`]>>: (3.4.0 - 6000.2.0a6)
    pub m_Scripts: Vec<PPtr>,
    /// Vec<String>: (3.4.0 - 2020.2.0a19)
    pub m_AssemblyNames: Option<Vec<String>>,
    /// Vec<i32>: (2017.1.0b1 - 2020.2.0a19)
    pub m_AssemblyTypes: Option<Vec<i32>>,
    /// Vec<(i32, Hash128)>: (2020.2.0b1 - 6000.2.0a6)
    pub m_RuntimeClassHashes: Option<Vec<(i32, Hash128)>>,
    /// Vec<(Hash128, Hash128)>: (2020.2.0b1 - 6000.2.0a6)
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
    /// bool: (3.4.0 - 2018.1.9f2)
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
    /// String: (5.0.0f4 - 2019.2.21f1)
    pub m_AssetBundleName: Option<String>,
    /// String: (5.0.0f4 - 2019.2.21f1)
    pub m_AssetBundleVariant: Option<String>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 2019.2.21f1)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.2)
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    /// bool: (3.5.0 - 2019.2.21f1)
    pub m_LinearTexture: Option<bool>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_NewHashIdentity: Option<MdFour>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_OldHashIdentity: Option<MdFour>,
    /// Vec<i64>: (2019.1.0b1 - 2019.2.21f1)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /// String: (4.0.0 - 2019.2.21f1)
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
    /// PPtr<[`AudioClip`]>: (3.4.0 - 2019.2.21f1)
    pub m_AudioClip: Option<PPtr>,
    /// i32: (3.5.0 - 2019.2.21f1)
    pub m_ColorSpace: Option<i32>,
    /// bool: (2017.3.0b1 - 2023.2.0a17)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.3.0b1 - 2023.2.0a17)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// bool: (3.4.0 - 2019.2.21f1)
    pub m_Loop: Option<bool>,
    /// Vec<u8>: (3.4.0 - 2019.2.21f1)
    pub m_MovieData: Option<Vec<u8>>,
}

/// MultiArtifactTestImporter is a  class of the Unity engine since version 2019.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiArtifactTestImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2019.2.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// MultiModeParameter is a sub class of the Unity engine since version 5.6.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiModeParameter {
    pub mode: i32,
    pub speed: MinMaxCurve,
    pub spread: f32,
    /// f32: (5.6.0f1 - 2018.2.21f1)
    pub value: Option<f32>,
}

/// MultiplayerManager is a  class of the Unity engine since version 2023.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiplayerManager {
    /// i32: (2023.2.0b1 - 2023.2.20f1)
    pub m_ActiveMultiplayerRole: Option<i32>,
    /// i32: (2023.3.0b1 - 6000.2.0a6)
    pub m_ActiveMultiplayerRoles: Option<i32>,
}

/// MultiplayerRolesData is a  class of the Unity engine since version 2023.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiplayerRolesData {
    pub m_ComponentsRolesMasks: Vec<ObjectRolePair>,
    /// PPtr<[`GameObject`]>: (2023.2.0b1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub m_GameObjectRolesMask: i32,
}

/// NScreenBridge is a  class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct NScreenBridge {}

/// NameToObjectMap is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct NameToObjectMap {
    /// Vec<(PPtr<[`Shader`]>, String)>: (3.4.0 - 6000.2.0a6)
    pub m_ObjectToName: Vec<(PPtr, String)>,
}

/// NamedObject is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct NamedObject {
    pub m_Name: String,
}

/// NativeFormatImporter is a  class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct NativeFormatImporter {
    pub m_Name: String,
    pub m_UserData: String,
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleName: Option<String>,
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleVariant: Option<String>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// i64: (5.6.0f1 - 6000.2.0a6)
    pub m_MainObjectFileID: Option<i64>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// NativeObjectType is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct NativeObjectType {
    pub m_Inner: NativeType,
}

/// NativeType is a sub class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct NativeType {
    pub a: i32,
    pub b: f32,
    pub embedded: EmbeddedNativeType,
}

/// NavMesh is a  class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AI.NavMesh.html):
/**
Singleton class to access the baked NavMesh.
Use the NavMesh class to perform spatial queries such as pathfinding and walkability tests. This class also lets you set the pathfinding cost for specific area types, and tweak the global behavior of pathfinding and avoidance.Before you can use spatial queries, you must first bake the NavMesh to your scene.See also:
• Create a NavMesh – for more information on how to setup and bake NavMesh
• Areas and Costs – to learn how to use different Area types.
• NavMeshAgent – to learn how to control and move NavMesh Agents.
• NavMeshObstacle – to learn how to control NavMesh Obstacles using scripting.
• NavMeshLink – to learn how to control Off-Mesh Links using scripting.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMesh {
    pub m_Heightmaps: Vec<HeightmapData>,
    pub m_MeshData: Vec<u8>,
    pub m_Name: String,
}

/// NavMeshAgent is a  class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AI.NavMeshAgent.html):
/**
Navigation mesh agent.
Attach this component to a mobile character in the game to allow the character to use the NavMesh to navigate the scene. For more details refer to  AI Navigation.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshAgent {
    /**The maximum acceleration of an agent as it follows a path, given in units / sec^2.*/
    pub m_Acceleration: f32,
    /**Maximum turning speed in (deg/s) while following a path.*/
    pub m_AngularSpeed: f32,
    /**Should the agent attempt to acquire a new path if the existing path becomes invalid?*/
    pub m_AutoRepath: bool,
    /**Should the agent move across OffMeshLinks automatically?*/
    pub m_AutoTraverseOffMeshLink: bool,
    /**The relative vertical displacement of the owning GameObject.*/
    pub m_BaseOffset: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.5.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /**The avoidance priority level.*/
    /// i32: (4.0.0 - 6000.2.0a6)
    pub avoidancePriority: Option<i32>,
    /**The type ID for the agent.*/
    /// i32: (5.6.0b1 - 6000.2.0a6)
    pub m_AgentTypeID: Option<i32>,
    /**Should the agent brake automatically to avoid overshooting the destination point?*/
    /// bool: (4.1.0 - 6000.2.0a6)
    pub m_AutoBraking: Option<bool>,
}

/// NavMeshAreaData is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshAreaData {
    pub cost: f32,
    pub name: String,
}

/// NavMeshAreas is a  class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshAreas {
    pub areas: Vec<NavMeshAreaData>,
}

/// NavMeshBuildDebugSettings is a sub class of the Unity engine since version 2017.2.0f1.
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
TODO ol
Additional resources: NavMeshBuildSettings, NavMeshBuilder.BuildNavMeshData, NavMeshEditorHelpers.DrawBuildDebug.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshBuildDebugSettings {
    /**Specify which types of debug data to collect when building the NavMesh.*/
    pub m_Flags: u8,
}

/// NavMeshBuildSettings is a sub class of the Unity engine since version 5.6.0b1.
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
    /// bool: (5.6.0b1 - 5.6.0b11); i32: (5.6.0f2 - 2022.1.24f1)
    pub accuratePlacement: Option<Enum_bool__i32>,
    /**Enables the creation of additional data needed to determine the height at any position on the NavMesh more accurately.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub buildHeightMesh: Option<i32>,
    /**Options for collecting debug data during the build process.*/
    /// NavMeshBuildDebugSettings: (2017.2.0f1 - 6000.2.0a6)
    pub debug: Option<NavMeshBuildDebugSettings>,
    /// i32: (2020.1.0a23 - 2020.1.0a23)
    pub keepTiles: Option<i32>,
    /**The maximum number of worker threads that the build process can utilize when building a NavMesh with these settings.*/
    /// u32: (2020.1.0b1 - 6000.2.0a6)
    pub maxJobWorkers: Option<u32>,
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub preserveTilesOutsideBounds: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_bool__i32 {
    bool(bool),
    i32(i32),
}

/// NavMeshData is a  class of the Unity engine since version 5.0.0f4.
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
    pub m_NavMeshTiles: Vec<NavMeshTileData>,
    pub m_OffMeshLinks: Vec<AutoOffMeshLinkData>,
    /// i32: (5.6.1f1 - 6000.2.0a6)
    pub m_AgentTypeID: Option<i32>,
    /// NavMeshBuildSettings: (5.6.0b1 - 6000.2.0a6)
    pub m_NavMeshBuildSettings: Option<NavMeshBuildSettings>,
    /// NavMeshParams: (5.0.0f4 - 5.5.6f1)
    pub m_NavMeshParams: Option<NavMeshParams>,
    /**Gets or sets the world space position of the NavMesh data.*/
    /// Vector3f: (5.6.1f1 - 6000.2.0a6)
    pub m_Position: Option<Vector3f>,
    /**Gets or sets the orientation of the NavMesh data.*/
    /// Quaternionf: (5.6.1f1 - 6000.2.0a6)
    pub m_Rotation: Option<Quaternionf>,
    /**Returns the bounding volume of the input geometry used to build this NavMesh (Read Only).*/
    /// AABB: (5.6.1f1 - 6000.2.0a6)
    pub m_SourceBounds: Option<AABB>,
}

/// NavMeshLayerData is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshLayerData {
    pub cost: f32,
    pub editType: i32,
    pub name: String,
}

/// NavMeshLayers is a  class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshLayers {
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "Built-in Layer 0")]
    pub Built_in_Layer_0: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "Built-in Layer 1")]
    pub Built_in_Layer_1: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "Built-in Layer 2")]
    pub Built_in_Layer_2: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 0")]
    pub User_Layer_0: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 1")]
    pub User_Layer_1: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 10")]
    pub User_Layer_10: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 11")]
    pub User_Layer_11: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 12")]
    pub User_Layer_12: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 13")]
    pub User_Layer_13: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 14")]
    pub User_Layer_14: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 15")]
    pub User_Layer_15: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 16")]
    pub User_Layer_16: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 17")]
    pub User_Layer_17: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 18")]
    pub User_Layer_18: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 19")]
    pub User_Layer_19: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 2")]
    pub User_Layer_2: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 20")]
    pub User_Layer_20: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 21")]
    pub User_Layer_21: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 22")]
    pub User_Layer_22: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 23")]
    pub User_Layer_23: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 24")]
    pub User_Layer_24: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 25")]
    pub User_Layer_25: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 26")]
    pub User_Layer_26: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 27")]
    pub User_Layer_27: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 28")]
    pub User_Layer_28: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 3")]
    pub User_Layer_3: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 4")]
    pub User_Layer_4: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 5")]
    pub User_Layer_5: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 6")]
    pub User_Layer_6: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 7")]
    pub User_Layer_7: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 8")]
    pub User_Layer_8: Option<NavMeshLayerData>,
    /// NavMeshLayerData: (3.5.0 - 4.7.2)
    #[serde(alias = "User Layer 9")]
    pub User_Layer_9: Option<NavMeshLayerData>,
}

/// NavMeshObsolete is a  class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshObsolete {
    pub m_Name: String,
}

/// NavMeshObstacle is a  class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AI.NavMeshObstacle.html):
/**
An obstacle for NavMeshAgents to avoid.
A NavMeshObstacle is cylindrical in shape and can move around the surface of the NavMesh with a specified velocity. By default, the obstacle will only affect the agent's avoidance behaviour rather than the pathfinding. This means that the agent will ignore the obstacle when plotting a path but will sidestep around it while moving along the path. If carving is enabled, the obstacle will create a temporary "hole" in the NavMesh. The hole will be recognised by the pathfinding, so paths will be plotted to avoid the obstacle. This means that if, say, an obstacle blocks a narrow gap, the pathfinding will seek an alternative route to the target. Without carving, the agent will head for the gap but won't be able to pass until the obstacle is clear.Additional resources: NavMeshAgent.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshObstacle {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.0.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /// bool: (4.3.0 - 6000.2.0a6)
    pub m_Carve: Option<bool>,
    /**Should this obstacle be carved when it is constantly moving?*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_CarveOnlyStationary: Option<bool>,
    /**The center of the obstacle, measured in the object's local space.*/
    /// Vector3f: (5.0.0f4 - 6000.2.0a6)
    pub m_Center: Option<Vector3f>,
    /// Vector3f: (5.0.0f4 - 6000.2.0a6)
    pub m_Extents: Option<Vector3f>,
    /**Height of the obstacle's cylinder shape.*/
    /// f32: (4.0.0 - 4.7.2)
    pub m_Height: Option<f32>,
    /// f32: (4.3.0 - 6000.2.0a6)
    pub m_MoveThreshold: Option<f32>,
    /**Radius of the obstacle's capsule shape.*/
    /// f32: (4.0.0 - 4.7.2)
    pub m_Radius: Option<f32>,
    /**The shape of the obstacle.*/
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_Shape: Option<i32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_TimeToStationary: Option<f32>,
}

/// NavMeshParams is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshParams {
    pub cellSize: f32,
    pub tileSize: f32,
    pub walkableClimb: f32,
    pub walkableHeight: f32,
    pub walkableRadius: f32,
}

/// NavMeshProjectSettings is a  class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshProjectSettings {
    pub areas: Vec<NavMeshAreaData>,
    /// i32: (5.6.0b1 - 6000.2.0a6)
    pub m_LastAgentTypeID: Option<i32>,
    /// Vec<String>: (5.6.0b1 - 6000.2.0a6)
    pub m_SettingNames: Option<Vec<String>>,
    /// Vec<NavMeshBuildSettings>: (5.6.0b1 - 6000.2.0a6)
    pub m_Settings: Option<Vec<NavMeshBuildSettings>>,
}

/// NavMeshSettings is a  class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshSettings {
    /// PPtr<[`NavMesh`]>: (3.5.0 - 4.7.2)
    pub m_NavMesh: Option<PPtr>,
    /// PPtr<[`NavMeshData`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_NavMeshData: Option<PPtr>,
}

/// NavMeshTileData is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct NavMeshTileData {
    pub m_MeshData: Vec<u8>,
    /// Hash128: (5.6.0b1 - 6000.2.0a6)
    pub m_Hash: Option<Hash128>,
}

/// NetworkManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkManager {
    /// Vec<(GUID, PPtr<[`GameObject`]>)>: (3.4.0 - 2018.1.9f2)
    pub m_AssetToPrefab: Vec<(GUID, PPtr)>,
    pub m_DebugLevel: i32,
    pub m_Sendrate: f32,
}

/// NetworkView is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkView {
    pub m_Enabled: u8,
    /// PPtr<[`GameObject`]>: (3.4.0 - 2018.1.9f2)
    pub m_GameObject: PPtr,
    /// PPtr<[`Component`]>: (3.4.0 - 2018.1.9f2)
    pub m_Observed: PPtr,
    pub m_StateSynchronization: i32,
    pub m_ViewID: NetworkViewID,
}

/// NetworkViewID is a sub class of the Unity engine since version 3.4.0.
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

/// NoiseModule is a sub class of the Unity engine since version 5.5.0f3.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.NoiseModule.html):
/**
Script interface for the NoiseModule.
The Noise Module allows you to apply turbulence to the movement of your particles. Use the low quality settings to create computationally efficient Noise, or simulate smoother, richer Noise with the higher quality settings. You can also choose to define the behavior of the Noise individually for each axis.Additional resources: ParticleSystem, ParticleSystem.noise.
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
    /// MinMaxCurve: (2017.1.0f1 - 6000.2.0a6)
    pub positionAmount: Option<MinMaxCurve>,
    /**How much the noise affects the particle rotation, in degrees per second.*/
    /// MinMaxCurve: (2017.1.0f1 - 6000.2.0a6)
    pub rotationAmount: Option<MinMaxCurve>,
    /**How much the noise affects the particle sizes, applied as a multiplier on the size of each particle.*/
    /// MinMaxCurve: (2017.1.0f1 - 6000.2.0a6)
    pub sizeAmount: Option<MinMaxCurve>,
}

/// NonAlignedStruct is a sub class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct NonAlignedStruct {
    pub m_Bool: bool,
}

/// ObjectRolePair is a sub class of the Unity engine since version 2023.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRolePair {
    /// PPtr<[`Object`]>: (2023.2.0b1 - 6000.2.0a6)
    pub m_Object: PPtr,
    pub m_RolesMask: i32,
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub m_IsViewVolume: bool,
    /**Size that the occlusion area will have.*/
    pub m_Size: Vector3f,
    /// bool: (3.4.0 - 4.2.2)
    pub m_IsTargetVolume: Option<bool>,
    /// i32: (3.4.0 - 4.2.2)
    pub m_TargetResolution: Option<i32>,
}

/// OcclusionCullingData is a  class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct OcclusionCullingData {
    pub m_Name: String,
    pub m_PVSData: Vec<u8>,
    pub m_Scenes: Vec<OcclusionScene>,
}

/// OcclusionCullingSettings is a  class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct OcclusionCullingSettings {
    /// PPtr<[`OcclusionCullingData`]>: (5.5.0f3 - 6000.2.0a6)
    pub m_OcclusionCullingData: PPtr,
    /// Vec<PPtr<[`OcclusionPortal`]>>: (5.5.0f3 - 6000.2.0a6)
    pub m_Portals: Vec<PPtr>,
    pub m_SceneGUID: GUID,
    /// Vec<PPtr<[`Renderer`]>>: (5.5.0f3 - 6000.2.0a6)
    pub m_StaticRenderers: Vec<PPtr>,
}

/// OcclusionPortal is a  class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/OcclusionPortal.html):
/**
The portal for dynamically changing occlusion at runtime.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct OcclusionPortal {
    pub m_Center: Vector3f,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.5.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Gets / sets the portal's open state.*/
    pub m_Open: bool,
    pub m_Size: Vector3f,
}

/// OcclusionScene is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct OcclusionScene {
    pub indexPortals: i32,
    pub indexRenderers: i32,
    pub scene: GUID,
    pub sizePortals: i32,
    pub sizeRenderers: i32,
}

/// Oculus is a sub class of the Unity engine since version 2017.3.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct Oculus {
    pub dashSupport: bool,
    pub sharedDepthBuffer: bool,
    /// bool: (2018.4.9f1 - 2020.2.0a15)
    pub lowOverheadMode: Option<bool>,
    /// bool: (2018.4.9f1 - 2020.2.0a15)
    pub protectedContext: Option<bool>,
    /// bool: (2018.4.9f1 - 2020.2.0a15)
    pub v2Signing: Option<bool>,
}

/// OffMeshLink is a  class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/AI.OffMeshLink.html):
/**
Link allowing movement outside the planar navigation mesh.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct OffMeshLink {
    pub m_Activated: bool,
    pub m_BiDirectional: bool,
    pub m_CostOverride: f32,
    /// PPtr<[`Transform`]>: (3.5.0 - 6000.2.0a6)
    pub m_End: PPtr,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.5.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /// PPtr<[`Transform`]>: (3.5.0 - 6000.2.0a6)
    pub m_Start: PPtr,
    /// i32: (5.6.0b1 - 6000.2.0a6)
    pub m_AgentTypeID: Option<i32>,
    /// u32: (5.0.0f4 - 6000.2.0a6)
    pub m_AreaIndex: Option<u32>,
    /// bool: (4.3.0 - 6000.2.0a6)
    pub m_AutoUpdatePositions: Option<bool>,
    /// u32: (3.5.0 - 4.7.2)
    pub m_DtPolyRef: Option<u32>,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    /// u8: (4.3.0 - 6000.2.0a6)
    pub m_Enabled: Option<u8>,
    /// u32: (4.0.0 - 4.7.2)
    pub m_NavMeshLayer: Option<u32>,
}

/// OffsetPtr is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct OffsetPtr {
    pub data: Clip,
}

/// Output is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub hasEmptyFontData: Option<bool>,
    /// i32: (4.0.0 - 4.7.2)
    pub importedType: Option<i32>,
    /// Vec<f32>: (4.0.0 - 4.7.2)
    pub previewData: Option<Vec<f32>>,
}

/// PPtrCurve is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct PPtrCurve {
    pub attribute: String,
    pub classID: i32,
    /// Vec<PPtrKeyframe>: (4.3.0 - 6000.2.0a6)
    pub curve: Vec<PPtrKeyframe>,
    pub path: String,
    /// PPtr<[`MonoScript`]>: (4.3.0 - 6000.2.0a6)
    pub script: PPtr,
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub flags: Option<i32>,
}

/// PPtrKeyframe is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct PPtrKeyframe {
    pub time: f32,
    /// PPtr<[`Object`]>: (4.3.0 - 6000.2.0a6)
    pub value: PPtr,
}

/// PVRImporter is a  class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct PVRImporter {
    pub m_Name: String,
    pub m_UserData: String,
    /// String: (5.0.0f4 - 5.5.6f1)
    pub m_AssetBundleName: Option<String>,
    /// String: (5.0.0f4 - 5.5.6f1)
    pub m_AssetBundleVariant: Option<String>,
}

/// PackageManifest is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManifest {
    pub m_Name: String,
    pub m_Script: String,
}

/// PackageManifestImporter is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManifestImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2019.1.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// PackedAssets is a  class of the Unity engine since version 5.4.0f3.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Build.Reporting.PackedAssets.html):
/**
An extension to the BuildReport class that tracks how Assets contribute to the size of the build.
The build process generates a PackedAssets object for each Serialized File, resS and resource file generated by a build.  Serialized Files contain serialized Unity Objects from scenes and assets.The PackedAsset information can be used to analyze how specific assets or types contribute to the size of the build, for example to calculate the total size of all textures.Example file names for Serialized Files in the output of a Player build are resources.assets, sharedassets0.assets and globalgamemanagers.assets.
Note: The content of scenes in the output of a Player build are written to Serialized Files with the names level0, level1, level2 etc.
However there are no PackedAsset objects generated for level files, nor for the globalgamemanager file.AssetBundles containing assets will have a single Serialized File, with a name like "CAB-b8befc517982290c55526f35cbb7f03d".  AssetBundles containing scenes will contain multiple Serialized Files.The PackedAssets for a Serialized File records the size of the header in the overhead property, and then information about each object in the contents property.Files with the .resource contain audio or video data which is referenced by an AudioClip or VideoClip object inside the associated Serialized File.
For example level1.resource contains audio and video data from objects inside the Serialized File level1.
The PackedAsset for a .resource file records information about the originating asset for each blob of audio or video data in the .resource file.Similarly, the PackedAsset object for a .resS file records information about the size and origin of Texture and Mesh data inside the file.Note: For large builds the PackedAsset objects can grow very large, and consume a significant amount of memory.  When using this data it is recommended to do a single pass through the data to populate smaller data structures or to export it to another format, as required by external tools.Additional resources: PackedAssetInfo
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PackedAssets {
    /**An array of PackedAssetInfo objects that holds information about the content portion of the referenced file.*/
    pub m_Contents: Vec<BuildReportPackedAssetInfo>,
    /**The size of the header section of the referenced file.*/
    pub m_Overhead: u64,
    /**The file name*/
    pub m_ShortPath: String,
    /// u32: (5.4.0f3 - 2021.2.0a11)
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

/// PackingSettings is a sub class of the Unity engine since version 2020.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackingSettings {
    pub allowAlphaSplitting: bool,
    pub blockOffset: i32,
    pub enableRotation: bool,
    pub enableTightPacking: bool,
    pub padding: i32,
    /// bool: (2021.1.0b1 - 6000.2.0a6)
    pub enableAlphaDilation: Option<bool>,
}

/// Parameter is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub m_GUID: GUID,
    pub m_ParameterName: String,
}

/// ParentConstraint is a  class of the Unity engine since version 2018.1.0f1.
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
    /// PPtr<[`GameObject`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_Active: Option<bool>,
    /// bool: (2018.1.0f1 - 2022.1.0a9)
    pub m_IsContraintActive: Option<bool>,
}

/// ParserBindChannels is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParserBindChannels {
    pub m_Channels: Vec<ShaderBindChannel>,
    pub m_SourceMap: i64,
}

/// ParticleAnimator is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticleAnimator {
    pub autodestruct: bool,
    pub damping: f32,
    pub force: Vector3f,
    pub localRotationAxis: Vector3f,
    /// PPtr<[`GameObject`]>: (3.4.0 - 2018.2.21f1)
    pub m_GameObject: PPtr,
    pub rndForce: Vector3f,
    pub sizeGrow: f32,
    pub stopSimulation: bool,
    pub worldRotationAxis: Vector3f,
    /// bool: (3.4.0 - 2018.2.21f1)
    #[serde(alias = "Does Animate Color?")]
    pub Does_Animate_Color_: Option<bool>,
    /// ColorRGBA: (3.4.0 - 2018.2.21f1)
    #[serde(alias = "colorAnimation[0]")]
    pub colorAnimation_0_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 2018.2.21f1)
    #[serde(alias = "colorAnimation[1]")]
    pub colorAnimation_1_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 2018.2.21f1)
    #[serde(alias = "colorAnimation[2]")]
    pub colorAnimation_2_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 2018.2.21f1)
    #[serde(alias = "colorAnimation[3]")]
    pub colorAnimation_3_: Option<ColorRGBA>,
    /// ColorRGBA: (3.4.0 - 2018.2.21f1)
    #[serde(alias = "colorAnimation[4]")]
    pub colorAnimation_4_: Option<ColorRGBA>,
}

/// ParticleRenderer is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticleRenderer {
    pub m_CameraVelocityScale: f32,
    pub m_CastShadows: Enum_bool__u8,
    pub m_Enabled: bool,
    /// PPtr<[`GameObject`]>: (3.4.0 - 2018.2.21f1)
    pub m_GameObject: PPtr,
    pub m_LengthScale: f32,
    pub m_LightmapIndex: u16,
    pub m_LightmapTilingOffset: Vector4f,
    /// Vec<PPtr<[`Material`]>>: (3.4.0 - 2018.2.21f1)
    pub m_Materials: Vec<PPtr>,
    pub m_MaxParticleSize: f32,
    pub m_ReceiveShadows: Enum_bool__u8,
    /// PPtr<[`Transform`]>: (3.4.0 - 2018.2.21f1)
    pub m_StaticBatchRoot: PPtr,
    pub m_StretchParticles: i32,
    pub m_VelocityScale: f32,
    /// UVAnimation: (3.4.0 - 2018.2.21f1)
    #[serde(alias = "UV Animation")]
    pub UV_Animation: Option<UVAnimation>,
    /// u8: (2017.2.0f1 - 2018.2.21f1)
    pub m_DynamicOccludee: Option<u8>,
    /// PPtr<[`Transform`]>: (3.5.0 - 4.7.2)
    pub m_LightProbeAnchor: Option<PPtr>,
    /// u8: (5.4.0f3 - 2018.2.21f1)
    pub m_LightProbeUsage: Option<u8>,
    /// PPtr<[`GameObject`]>: (5.4.0f3 - 2018.2.21f1)
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    /// u16: (5.0.0f4 - 2018.2.21f1)
    pub m_LightmapIndexDynamic: Option<u16>,
    /// Vector4f: (5.0.0f4 - 2018.2.21f1)
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    /// u8: (5.4.0f3 - 2018.2.21f1)
    pub m_MotionVectors: Option<u8>,
    /// PPtr<[`Transform`]>: (5.0.0f4 - 2018.2.21f1)
    pub m_ProbeAnchor: Option<PPtr>,
    /// i32: (5.0.0f4 - 5.3.8f2); u8: (5.4.0f3 - 2018.2.21f1)
    pub m_ReflectionProbeUsage: Option<i32>,
    /// u32: (2018.1.0f1 - 2018.2.21f1)
    pub m_RenderingLayerMask: Option<u32>,
    /// i16: (4.3.0 - 2018.2.21f1)
    pub m_SortingLayer: Option<i16>,
    /// u32: (4.5.0 - 4.7.2); i32: (5.0.0f4 - 2018.2.21f1)
    pub m_SortingLayerID: Option<i64>,
    /// i16: (4.3.0 - 2018.2.21f1)
    pub m_SortingOrder: Option<i16>,
    /// StaticBatchInfo: (5.5.0f3 - 2018.2.21f1)
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    /// Vec<u32>: (3.4.0 - 5.4.6f3)
    pub m_SubsetIndices: Option<Vec<u32>>,
    /// bool: (3.5.0 - 5.3.8f2)
    pub m_UseLightProbes: Option<bool>,
}

/// ParticleSystem is a  class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.html):
/**
Script interface for the Built-in Particle System. Unity's powerful and versatile particle system implementation.
General parameters

The Particle System's general parameters are kept inside a special Main module. These parameters are visible in the Inspector above all the other modules.In script, these parameters are accessible through ParticleSystem.main.Accessing module propertiesParticle System properties are grouped by the module they belong to, such as ParticleSystem.noise and ParticleSystem.emission. These properties are structs, but do not behave like normal C# structs. They are simply interfaces directly into the native code, so it is important to know how to use them, compared to a normal C# struct.The key difference is that it is not necessary to assign the struct back to the Particle System component. When you set any property on a module struct, Unity immediately assigns that value to the Particle System.Also, because each module is a struct, you must cache it in a local variable before you can assign any new values to the module. For example, instead of:
ParticleSystem.emission.enabled = true;    // Doesn't compile
write:
var emission = ParticleSystem.emission;    // Stores the module in a local variable
emission.enabled = true;    // Applies the new value directly to the Particle SystemModule effect multipliersEvery module has special multiplier properties that allow you to change the overall effect of a curve without having to edit the curve itself. These multiplier properties are all named after the curve they affect - for instance ParticleSystem.emission.rateMultiplier controls the overall effect of ParticleSystem.emission.rate in a given system.Constant value shorthandParameters support a shorthand notation for simple constant values. To set a constant value for a parameter, all you need to do is assign a number to it. It is not necessary to create a MinMaxCurve or MinMaxGradient object in the ParticleSystemCurveMode.Constant mode.For example, instead of:
var emission = ParticleSystem.emission;
emission.rate = new ParticleSystem.MinMaxCurve(5.0f);
write:
var emission = ParticleSystem.emission;
emission.rate = 5.0f;Performance note: When setting properties on particle modules, the settings are passed immediately into native code. This gives the best performance. This means that setting properties on a module struct doesn't set something in script that requires setting back to the Particle System; it all happens automatically.Additional resources: Particle.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticleSystem {
    pub ClampVelocityModule: ClampVelocityModule,
    pub CollisionModule: CollisionModule,
    pub ColorBySpeedModule: ColorBySpeedModule,
    pub ColorModule: ColorModule,
    pub EmissionModule: EmissionModule,
    pub ForceModule: ForceModule,
    pub InitialModule: InitialModule,
    pub RotationBySpeedModule: RotationBySpeedModule,
    pub RotationModule: RotationModule,
    pub ShapeModule: ShapeModule,
    pub SizeBySpeedModule: SizeBySpeedModule,
    pub SizeModule: SizeModule,
    pub SubModule: SubModule,
    pub UVModule: UVModule,
    pub VelocityModule: VelocityModule,
    pub lengthInSec: f32,
    pub looping: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.5.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub moveWithTransform: Enum_bool__i32,
    pub playOnAwake: bool,
    pub prewarm: bool,
    /**Override the random seed used for the Particle System emission.*/
    pub randomSeed: i64,
    pub startDelay: Enum_f32__MinMaxCurve,
    /// CustomDataModule: (5.6.0b1 - 6000.2.0a6)
    pub CustomDataModule: Option<CustomDataModule>,
    /// ExternalForcesModule: (4.0.0 - 6000.2.0a6)
    pub ExternalForcesModule: Option<ExternalForcesModule>,
    /// InheritVelocityModule: (5.3.0f1 - 6000.2.0a6)
    pub InheritVelocityModule: Option<InheritVelocityModule>,
    /// LifetimeByEmitterSpeedModule: (2020.1.0b1 - 6000.2.0a6)
    pub LifetimeByEmitterSpeedModule: Option<LifetimeByEmitterSpeedModule>,
    /// LightsModule: (5.5.0f3 - 6000.2.0a6)
    pub LightsModule: Option<LightsModule>,
    /// NoiseModule: (5.5.0f3 - 6000.2.0a6)
    pub NoiseModule: Option<NoiseModule>,
    /// TrailModule: (5.5.0f3 - 6000.2.0a6)
    pub TrailModule: Option<TrailModule>,
    /// TriggerModule: (5.4.0f3 - 6000.2.0a6)
    pub TriggerModule: Option<TriggerModule>,
    /// bool: (5.4.1f1 - 6000.2.0a6)
    pub autoRandomSeed: Option<bool>,
    /// i32: (2018.3.0f1 - 6000.2.0a6)
    pub cullingMode: Option<i32>,
    /// i32: (2021.1.0b1 - 6000.2.0a6)
    pub emitterVelocityMode: Option<i32>,
    /// PPtr<[`Transform`]>: (5.5.0f3 - 6000.2.0a6)
    pub moveWithCustomTransform: Option<PPtr>,
    /// Vector2f: (2018.3.0b1 - 6000.2.0a6)
    pub ringBufferLoopRange: Option<Vector2f>,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub ringBufferMode: Option<i32>,
    /// i32: (5.3.0f1 - 6000.2.0a6)
    pub scalingMode: Option<i32>,
    /// f32: (5.5.1f1 - 6000.2.0a6)
    pub simulationSpeed: Option<f32>,
    /// f32: (3.5.0 - 5.5.0f3)
    pub speed: Option<f32>,
    /// i32: (2017.2.0f1 - 6000.2.0a6)
    pub stopAction: Option<i32>,
    /// bool: (2017.1.0f1 - 2021.1.0a6)
    pub useRigidbodyForVelocity: Option<bool>,
    /// bool: (2017.1.0f1 - 6000.2.0a6)
    pub useUnscaledTime: Option<bool>,
}

/// ParticleSystemEmissionBurst is a sub class of the Unity engine since version 5.6.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticleSystemEmissionBurst {
    pub cycleCount: i64,
    pub repeatInterval: f32,
    pub time: f32,
    /// MinMaxCurve: (2017.2.0f1 - 6000.2.0a6)
    pub countCurve: Option<MinMaxCurve>,
    /// u32: (5.6.0f1 - 2017.1.5f1)
    pub maxCount: Option<u32>,
    /// u32: (5.6.0f1 - 2017.1.5f1)
    pub minCount: Option<u32>,
    /// f32: (2018.3.0b1 - 6000.2.0a6)
    pub probability: Option<f32>,
}

/// ParticleSystemForceField is a  class of the Unity engine since version 2018.3.0b1.
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
    /// PPtr<[`GameObject`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub m_Parameters: ParticleSystemForceFieldParameters,
}

/// ParticleSystemForceFieldParameters is a sub class of the Unity engine since version 2018.3.0b1.
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
    /// PPtr<[`Texture3D`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_VectorField: PPtr,
    pub m_VectorFieldAttractionCurve: MinMaxCurve,
    pub m_VectorFieldSpeedCurve: MinMaxCurve,
}

/// ParticleSystemRenderer is a  class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystemRenderer.html):
/**
Use this class to render particles on to the screen.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticleSystemRenderer {
    /**How much do the particles stretch depending on the Camera's speed.*/
    pub m_CameraVelocityScale: f32,
    pub m_CastShadows: Enum_bool__u8,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.5.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**How much are the particles stretched in their direction of motion, defined as the length of the particle compared to its width.*/
    pub m_LengthScale: f32,
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapTilingOffset: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    /// Vec<PPtr<[`Material`]>>: (3.5.0 - 6000.2.0a6)
    pub m_Materials: Vec<PPtr>,
    /**Clamp the maximum particle size.*/
    pub m_MaxParticleSize: f32,
    /**The Mesh that the particle uses instead of a billboarded Texture.*/
    /// PPtr<[`Mesh`]>: (3.5.0 - 6000.2.0a6)
    pub m_Mesh: PPtr,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: Enum_bool__u8,
    /**Specifies how the system draws particles.*/
    pub m_RenderMode: i32,
    /**Specifies how to sort particles within a system.*/
    pub m_SortMode: i32,
    /**Biases Particle System sorting amongst other transparencies.*/
    pub m_SortingFudge: f32,
    /// PPtr<[`Transform`]>: (3.5.0 - 6000.2.0a6)
    pub m_StaticBatchRoot: PPtr,
    /**Specifies how much particles stretch depending on their velocity.*/
    pub m_VelocityScale: f32,
    /**Allow billboard particles to roll around their z-axis.*/
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_AllowRoll: Option<bool>,
    /// bool: (2018.2.0b1 - 6000.2.0a6)
    pub m_ApplyActiveColorSpace: Option<bool>,
    /// u8: (2017.2.0f1 - 6000.2.0a6)
    pub m_DynamicOccludee: Option<u8>,
    /**Enables GPU Instancing on platforms that support it.*/
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub m_EnableGPUInstancing: Option<bool>,
    /**Flip a percentage of the particles, along each axis.*/
    /// Vector3f: (2018.3.0b1 - 6000.2.0a6)
    pub m_Flip: Option<Vector3f>,
    /**Enables freeform stretching behavior.*/
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub m_FreeformStretching: Option<bool>,
    /// PPtr<[`Transform`]>: (3.5.0 - 4.7.2)
    pub m_LightProbeAnchor: Option<PPtr>,
    /**The light probe interpolation type.*/
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeUsage: Option<u8>,
    /// PPtr<[`GameObject`]>: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    /// u16: (5.0.0f4 - 6000.2.0a6)
    pub m_LightmapIndexDynamic: Option<u16>,
    /// Vector4f: (5.0.0f4 - 6000.2.0a6)
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    /**Specifies how the Particle System Renderer interacts with SpriteMask.*/
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub m_MaskInteraction: Option<i32>,
    /// PPtr<[`Mesh`]>: (4.0.0 - 6000.2.0a6)
    pub m_Mesh1: Option<PPtr>,
    /// PPtr<[`Mesh`]>: (4.0.0 - 6000.2.0a6)
    pub m_Mesh2: Option<PPtr>,
    /// PPtr<[`Mesh`]>: (4.0.0 - 6000.2.0a6)
    pub m_Mesh3: Option<PPtr>,
    /**Specifies how the system randomly assigns meshes to particles.*/
    /// u8: (2021.2.0b1 - 6000.2.0a6)
    pub m_MeshDistribution: Option<u8>,
    /// f32: (2021.2.0b1 - 6000.2.0a6)
    pub m_MeshWeighting: Option<f32>,
    /// f32: (2021.2.0b1 - 6000.2.0a6)
    pub m_MeshWeighting1: Option<f32>,
    /// f32: (2021.2.0b1 - 6000.2.0a6)
    pub m_MeshWeighting2: Option<f32>,
    /// f32: (2021.2.0b1 - 6000.2.0a6)
    pub m_MeshWeighting3: Option<f32>,
    /**Clamp the minimum particle size.*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_MinParticleSize: Option<f32>,
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_MotionVectors: Option<u8>,
    /**Specifies how to calculate lighting for the billboard.*/
    /// f32: (4.0.0 - 6000.2.0a6)
    pub m_NormalDirection: Option<f32>,
    /**Modify the pivot point used for rotating particles.*/
    /// Vector3f: (5.3.0f1 - 6000.2.0a6)
    pub m_Pivot: Option<Vector3f>,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr<[`Transform`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_ProbeAnchor: Option<PPtr>,
    /// u8: (2020.1.0b1 - 6000.2.0a6)
    pub m_RayTraceProcedural: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlags: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlagsOverride: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0b1 - 6000.2.0a6)
    pub m_RayTracingMode: Option<u8>,
    /**Should reflection probes be used for this Renderer?*/
    /// i32: (5.0.0f4 - 5.3.8f2); u8: (5.4.0f3 - 6000.2.0a6)
    pub m_ReflectionProbeUsage: Option<i32>,
    /// i32: (5.3.0f1 - 6000.2.0a6)
    pub m_RenderAlignment: Option<i32>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on, if you use a scriptable render pipeline.*/
    /// u32: (2018.1.0f1 - 6000.2.0a6)
    pub m_RenderingLayerMask: Option<u32>,
    /**Rotate the particles based on the direction they are stretched in. This is added on top of other particle rotation.*/
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub m_RotateWithStretchDirection: Option<bool>,
    /**Apply a shadow bias to prevent self-shadowing artifacts. The specified value is the proportion of the particle size.*/
    /// f32: (2018.3.0b1 - 6000.2.0a6)
    pub m_ShadowBias: Option<f32>,
    /// u8: (2023.3.0b1 - 6000.2.0a6)
    pub m_SmallMeshCulling: Option<u8>,
    /// i16: (4.3.0 - 6000.2.0a6)
    pub m_SortingLayer: Option<i16>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// u32: (4.5.0 - 4.7.2); i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SortingLayerID: Option<i64>,
    /**Renderer's order within a sorting layer.*/
    /// i16: (4.3.0 - 6000.2.0a6)
    pub m_SortingOrder: Option<i16>,
    /// StaticBatchInfo: (5.5.0f3 - 6000.2.0a6)
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.1.0b1 - 6000.2.0a6)
    pub m_StaticShadowCaster: Option<u8>,
    /// Vec<u32>: (3.5.0 - 5.4.6f3)
    pub m_SubsetIndices: Option<Vec<u32>>,
    /// Vec<u8>: (2022.3.11f1 - 6000.2.0a6)
    pub m_TrailVertexStreams: Option<Vec<u8>>,
    /// bool: (2022.3.11f1 - 6000.2.0a6)
    pub m_UseCustomTrailVertexStreams: Option<bool>,
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub m_UseCustomVertexStreams: Option<bool>,
    /// bool: (3.5.0 - 5.3.8f2)
    pub m_UseLightProbes: Option<bool>,
    /// i32: (5.5.0f3 - 5.5.6f1)
    pub m_VertexStreamMask: Option<i32>,
    /// Vec<u8>: (5.6.0b1 - 6000.2.0a6)
    pub m_VertexStreams: Option<Vec<u8>>,
}

/// PerLODSettings is a sub class of the Unity engine since version 5.0.0f4.
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
    /// bool: (2022.3.12f1 - 6000.2.0a6)
    pub enableSettingOverride: Option<bool>,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub enableSubsurface: Option<bool>,
}

/// PerformanceReportingManager is a  class of the Unity engine since version 5.6.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceReportingManager {}

/// PerformanceReportingSettings is a sub class of the Unity engine since version 5.6.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceReportingSettings {
    pub m_Enabled: bool,
}

/// PhysicMaterial is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicMaterial {
    pub bounceCombine: i32,
    pub bounciness: f32,
    pub dynamicFriction: f32,
    pub frictionCombine: i32,
    pub m_Name: String,
    pub staticFriction: f32,
    /// f32: (3.4.0 - 5.1.5f1)
    pub dynamicFriction2: Option<f32>,
    /// Vector3f: (3.4.0 - 5.1.5f1)
    pub frictionDirection2: Option<Vector3f>,
    /// f32: (3.4.0 - 5.1.5f1)
    pub staticFriction2: Option<f32>,
}

/// Physics2DSettings is a  class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Physics2DSettings {
    /// PPtr<[`PhysicsMaterial2D`]>: (4.3.0 - 6000.2.0a6)
    pub m_DefaultMaterial: PPtr,
    pub m_Gravity: Vector2f,
    pub m_LayerCollisionMatrix: Vec<u32>,
    pub m_PositionIterations: i32,
    pub m_VelocityIterations: i32,
    /// f32: (4.5.0 - 6000.2.0a6)
    pub m_AngularSleepTolerance: Option<f32>,
    /// bool: (2017.1.0f1 - 2019.4.40f1)
    pub m_AutoSimulation: Option<bool>,
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub m_AutoSyncTransforms: Option<bool>,
    /// f32: (4.5.0 - 6000.2.0a6)
    pub m_BaumgarteScale: Option<f32>,
    /// f32: (4.5.0 - 6000.2.0a6)
    pub m_BaumgarteTimeOfImpactScale: Option<f32>,
    /// f32: (2023.1.0b1 - 6000.2.0a6)
    pub m_BounceThreshold: Option<f32>,
    /// bool: (5.6.1f1 - 6000.2.0a6)
    pub m_CallbacksOnDisable: Option<bool>,
    /// bool: (4.6.1 - 2017.4.40f1)
    pub m_ChangeStopsCallbacks: Option<bool>,
    /// f32: (6000.0.1f1 - 6000.2.0a6)
    pub m_ContactThreshold: Option<f32>,
    /// f32: (5.6.0f1 - 6000.2.0a6)
    pub m_DefaultContactOffset: Option<f32>,
    /// bool: (4.5.3 - 4.6.0)
    pub m_DeleteStopsCallbacks: Option<bool>,
    /// PhysicsJobOptions2D: (2018.1.0f1 - 6000.2.0a6)
    pub m_JobOptions: Option<PhysicsJobOptions2D>,
    /// f32: (4.5.0 - 6000.2.0a6)
    pub m_LinearSleepTolerance: Option<f32>,
    /// f32: (4.5.0 - 6000.2.0a6)
    pub m_MaxAngularCorrection: Option<f32>,
    /// f32: (4.5.0 - 6000.2.0a6)
    pub m_MaxLinearCorrection: Option<f32>,
    /// f32: (4.5.0 - 6000.2.0a6)
    pub m_MaxRotationSpeed: Option<f32>,
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_MaxSubStepCount: Option<i32>,
    /// f32: (4.5.0 - 6000.2.0a6)
    pub m_MaxTranslationSpeed: Option<f32>,
    /// f32: (4.6.1 - 5.6.0b6)
    pub m_MinPenetrationForPenalty: Option<f32>,
    /// f32: (2023.1.0b1 - 6000.2.0a6)
    pub m_MinSubStepFPS: Option<f32>,
    /// bool: (5.2.0f2 - 6000.2.0a6)
    pub m_QueriesHitTriggers: Option<bool>,
    /// bool: (5.2.0f2 - 6000.2.0a6)
    pub m_QueriesStartInColliders: Option<bool>,
    /// bool: (4.3.0 - 5.1.5f1)
    pub m_RaycastsHitTriggers: Option<bool>,
    /// bool: (4.6.1 - 5.1.5f1)
    pub m_RaycastsStartInColliders: Option<bool>,
    /// bool: (2018.3.0f1 - 6000.2.0a6)
    pub m_ReuseCollisionCallbacks: Option<bool>,
    /// BitField: (2023.1.0b1 - 6000.2.0a6)
    pub m_SimulationLayers: Option<BitField>,
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub m_SimulationMode: Option<i32>,
    /// f32: (4.5.0 - 6000.2.0a6)
    pub m_TimeToSleep: Option<f32>,
    /// bool: (2023.1.0b1 - 6000.2.0a6)
    pub m_UseSubStepContacts: Option<bool>,
    /// bool: (2023.1.0b1 - 6000.2.0a6)
    pub m_UseSubStepping: Option<bool>,
    /// f32: (4.5.0 - 2023.1.0a18)
    pub m_VelocityThreshold: Option<f32>,
}

/// PhysicsJobOptions2D is a sub class of the Unity engine since version 2018.1.0f1.
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
    /// bool: (2018.1.0b2 - 2018.1.0b10)
    pub m_UseConsistencySorting: Option<bool>,
    /**Should physics simulation use multithreading?*/
    /// bool: (2018.1.0b2 - 2018.1.0b10)
    pub m_UseMultithreading: Option<bool>,
    /**Should physics simulation sort multi-threaded results to maintain processing order consistency?*/
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub useConsistencySorting: Option<bool>,
    /**Should physics simulation use multithreading?*/
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub useMultithreading: Option<bool>,
}

/// PhysicsManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsManager {
    pub m_BounceThreshold: f32,
    /// PPtr<[`PhysicMaterial`]>: (3.4.0 - 2023.3.0a8); PPtr<[`PhysicsMaterial`]>: (2023.3.0b1 - 6000.2.0a6)
    pub m_DefaultMaterial: PPtr,
    pub m_Gravity: Vector3f,
    pub m_LayerCollisionMatrix: Vec<u32>,
    /// bool: (2017.1.0f1 - 2022.1.24f1)
    pub m_AutoSimulation: Option<bool>,
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub m_AutoSyncTransforms: Option<bool>,
    /// i32: (2017.3.0b1 - 6000.2.0a6)
    pub m_BroadphaseType: Option<i32>,
    /// Vector3f: (2019.1.0b1 - 6000.2.0a6)
    pub m_ClothGravity: Option<Vector3f>,
    /// f32: (2017.3.0b1 - 6000.2.0a6)
    pub m_ClothInterCollisionDistance: Option<f32>,
    /// bool: (2017.3.0b1 - 6000.2.0a6)
    pub m_ClothInterCollisionSettingsToggle: Option<bool>,
    /// f32: (2017.3.0b1 - 6000.2.0a6)
    pub m_ClothInterCollisionStiffness: Option<f32>,
    /// i32: (2017.3.0b1 - 6000.2.0a6)
    pub m_ContactPairsMode: Option<i32>,
    /// i32: (2017.3.0b1 - 2023.1.0a21)
    pub m_ContactsGeneration: Option<i32>,
    /// u32: (2023.3.0b8 - 6000.2.0a6)
    pub m_CurrentBackendId: Option<u32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_DefaultContactOffset: Option<f32>,
    /// f32: (2019.1.0b1 - 2019.1.0a14)
    pub m_DefaultMaxAngluarSpeed: Option<f32>,
    /// f32: (2019.1.0f2 - 6000.2.0a6)
    pub m_DefaultMaxAngularSpeed: Option<f32>,
    /// f32: (2020.1.0b1 - 6000.2.0a6)
    pub m_DefaultMaxDepenetrationVelocity: Option<f32>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_DefaultSolverIterations: Option<i32>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_DefaultSolverVelocityIterations: Option<i32>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_EnableAdaptiveForce: Option<bool>,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_EnableEnhancedDeterminism: Option<bool>,
    /// bool: (5.5.0f3 - 2017.2.5f1)
    pub m_EnablePCM: Option<bool>,
    /// bool: (2018.3.0b1 - 6000.1.0a2)
    pub m_EnableUnifiedHeightmaps: Option<bool>,
    /// f32: (2022.3.21f1 - 6000.2.0a6)
    pub m_FastMotionThreshold: Option<f32>,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_FrictionType: Option<i32>,
    /// bool: (6000.0.50f1 - 6000.1.5f1)
    pub m_GenerateOnTriggerStayEvents: Option<bool>,
    /// bool: (2021.2.0b1 - 6000.2.0a6)
    pub m_ImprovedPatchFriction: Option<bool>,
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_InvokeCollisionCallbacks: Option<bool>,
    /// f32: (3.4.0 - 4.7.2)
    pub m_MaxAngularVelocity: Option<f32>,
    /// f32: (3.4.0 - 4.7.2)
    pub m_MinPenetrationForPenalty: Option<f32>,
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub m_QueriesHitBackfaces: Option<bool>,
    /// bool: (5.2.0f2 - 6000.2.0a6)
    pub m_QueriesHitTriggers: Option<bool>,
    /// bool: (3.4.0 - 5.1.5f1)
    pub m_RaycastsHitTriggers: Option<bool>,
    /// bool: (2018.3.0f1 - 6000.2.0a6)
    pub m_ReuseCollisionCallbacks: Option<bool>,
    /// u32: (2023.1.0b1 - 6000.2.0a6)
    pub m_ScratchBufferChunkCount: Option<u32>,
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_SimulationMode: Option<i32>,
    /// f32: (3.4.0 - 4.7.2)
    pub m_SleepAngularVelocity: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_SleepThreshold: Option<f32>,
    /// f32: (3.4.0 - 4.7.2)
    pub m_SleepVelocity: Option<f32>,
    /// i32: (3.4.0 - 5.4.6f3)
    pub m_SolverIterationCount: Option<i32>,
    /// i32: (2019.3.0b1 - 6000.2.0a6)
    pub m_SolverType: Option<i32>,
    /// i32: (5.4.0f3 - 5.4.6f3)
    pub m_SolverVelocityIterations: Option<i32>,
    /// AABB: (2017.3.0b1 - 2023.1.0a21)
    pub m_WorldBounds: Option<AABB>,
    /// i32: (2017.3.0b1 - 2023.1.0a21)
    pub m_WorldSubdivisions: Option<i32>,
}

/// PhysicsMaterial is a  class of the Unity engine since version 2023.3.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PhysicsMaterial.html):
/**
Physics material describes how to handle colliding objects (friction, bounciness).
Additional resources: Collider.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsMaterial {
    /**The name of the object.*/
    pub m_Name: String,
    /**Determines how the bounciness is combined.*/
    /// i32: (2023.3.0a10 - 2023.3.0a14)
    pub bounceCombine: Option<i32>,
    /**How bouncy is the surface? A value of 0 will not bounce. A value of 1 will bounce without any loss of energy.*/
    /// f32: (2023.3.0a10 - 2023.3.0a14)
    pub bounciness: Option<f32>,
    /**The friction used when already moving.  This value is usually between 0 and 1.*/
    /// f32: (2023.3.0a10 - 2023.3.0a14)
    pub dynamicFriction: Option<f32>,
    /**Determines how the friction is combined.*/
    /// i32: (2023.3.0a10 - 2023.3.0a14)
    pub frictionCombine: Option<i32>,
    /**Determines how the bounciness is combined.*/
    /// i32: (2023.3.0b1 - 6000.2.0a6)
    pub m_BounceCombine: Option<i32>,
    /**How bouncy is the surface? A value of 0 will not bounce. A value of 1 will bounce without any loss of energy.*/
    /// f32: (2023.3.0b1 - 6000.2.0a6)
    pub m_Bounciness: Option<f32>,
    /**The friction used when already moving.  This value is usually between 0 and 1.*/
    /// f32: (2023.3.0b1 - 6000.2.0a6)
    pub m_DynamicFriction: Option<f32>,
    /**Determines how the friction is combined.*/
    /// i32: (2023.3.0b1 - 6000.2.0a6)
    pub m_FrictionCombine: Option<i32>,
    /**The friction coefficient used when an object is lying on a surface.*/
    /// f32: (2023.3.0b1 - 6000.2.0a6)
    pub m_StaticFriction: Option<f32>,
    /**The friction coefficient used when an object is lying on a surface.*/
    /// f32: (2023.3.0a10 - 2023.3.0a14)
    pub staticFriction: Option<f32>,
}

/// PhysicsMaterial2D is a  class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PhysicsMaterial2D.html):
/**
Asset type that defines the surface properties of a Collider2D.
When two Colllider2D come into contact, the physics system uses both friction and bounciness if it needs to calculate a collision response.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsMaterial2D {
    /**Coefficient of restitution.*/
    pub bounciness: f32,
    /**Coefficient of friction.*/
    pub friction: f32,
    /**The name of the object.*/
    pub m_Name: String,
    /**Determines how the effective bounciness is calculated when two Collider2D come into contact.*/
    /// i32: (6000.0.14f1 - 6000.2.0a6)
    pub m_BounceCombine: Option<i32>,
    /**Determines how the effective friction is calculated when two Collider2D come into contact.*/
    /// i32: (6000.0.14f1 - 6000.2.0a6)
    pub m_FrictionCombine: Option<i32>,
}

/// PhysicsShape is a sub class of the Unity engine since version 2021.2.0b1.
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

/// PhysicsShapeGroup2D is a sub class of the Unity engine since version 2021.2.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PhysicsShapeGroup2D.html):
/**
Represents a group of PhysicsShape2D and their geometry.
A shape group represents multiple PhysicsShape2D of the same or mixed PhysicsShapeType2D along with their geometry. It is comprised of a single list of vertices (GetShapeVertices) along with a list of PhysicsShape2D which refer to specific ranges of those vertices i.e. they index into the list of vertices. Some shape types (PhysicsShapeType2D) use a fixed number of vertices and some use a variable number of vertices therefore this single vertices list is a compact and efficient representation for multiple PhysicsShape2D in a group.The shape group can be created by using the following methods:
- Calling Collider2D.GetShapes where it would then represent all the shapes produced by that Collider2D
- Calling Rigidbody2D.GetShapes where it would then represent all the shapes produced by all theCollider2D attached to that Rigidbody2D
- Manually populating with custom shapes by calling AddCircle, AddCapsule, AddPolygon, AddBox or AddEdges.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsShapeGroup2D {
    pub m_Shapes: Vec<PhysicsShape>,
    pub m_Vertices: Vec<Vector2f>,
}

/// Pipeline is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline {
    /// PPtr<[`GameObject`]>: (3.4.0 - 4.7.2)
    pub m_GameObject: PPtr,
}

/// PlatformEffector2D is a  class of the Unity engine since version 5.0.0f4.
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
    /// PPtr<[`GameObject`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /// bool: (5.0.0f4 - 5.0.0f4)
    pub m_OneWay: Option<bool>,
    /**The rotational offset angle from the local 'up'.*/
    /// f32: (5.5.0f3 - 6000.2.0a6)
    pub m_RotationalOffset: Option<f32>,
    /// f32: (5.0.0f4 - 5.1.1f1)
    pub m_SideAngleVariance: Option<f32>,
    /**The angle of an arc that defines the sides of the platform centered on the local 'left' and 'right' of the effector. Any collision normals within this arc are considered for the 'side' behaviours.*/
    /// f32: (5.2.2f1 - 6000.2.0a6)
    pub m_SideArc: Option<f32>,
    /// bool: (5.0.0f4 - 5.0.0f4)
    pub m_SideBounce: Option<bool>,
    /// bool: (5.0.0f4 - 5.0.0f4)
    pub m_SideFriction: Option<bool>,
    /**The angle of an arc that defines the surface of the platform centered of the local 'up' of the effector.*/
    /// f32: (5.1.2f1 - 6000.2.0a6)
    pub m_SurfaceArc: Option<f32>,
    /**Should the collider-mask be used or the global collision matrix?*/
    /// bool: (5.0.2f1 - 6000.2.0a6)
    pub m_UseColliderMask: Option<bool>,
    /**Should the one-way collision behaviour be used?*/
    /// bool: (5.0.1f1 - 6000.2.0a6)
    pub m_UseOneWay: Option<bool>,
    /**Ensures that all contacts controlled by the one-way behaviour act the same.*/
    /// bool: (5.2.4f1 - 6000.2.0a6)
    pub m_UseOneWayGrouping: Option<bool>,
    /**Should bounce be used on the platform sides?*/
    /// bool: (5.0.1f1 - 6000.2.0a6)
    pub m_UseSideBounce: Option<bool>,
    /**Should friction be used on the platform sides?*/
    /// bool: (5.0.1f1 - 6000.2.0a6)
    pub m_UseSideFriction: Option<bool>,
}

/// PlatformModuleSetup is a  class of the Unity engine since version 2018.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformModuleSetup {
    pub modules: Vec<Module>,
}

/// PlatformSettings is a sub class of the Unity engine since version 5.5.0f3.
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
    /// i32: (2017.2.0f1 - 2017.2.5f1)
    pub m_ResizeAlgorithm: Option<i32>,
}

/// PlatformSettingsData is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformSettingsData {
    pub enabled: bool,
    pub settings: Vec<(String, String)>,
}

/// PlatformShaderDefines is a sub class of the Unity engine since version 2017.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformShaderDefines {
    pub defines_Tier1: Vec<u32>,
    pub defines_Tier2: Vec<u32>,
    pub defines_Tier3: Vec<u32>,
    pub shaderPlatform: i32,
}

/// PlatformShaderSettings is a sub class of the Unity engine since version 5.3.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rendering.PlatformShaderSettings.html):
/**
Used to set up shader settings, per-platform and per-shader-hardware-tier.
This struct is deprecated. Please use TierSettings.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformShaderSettings {
    /// bool: (5.4.0f3 - 5.4.6f3)
    pub useCascadedShadowMaps: Option<bool>,
    /// bool: (5.3.0f1 - 5.3.8f2)
    pub useScreenSpaceShadows: Option<bool>,
}

/// PlayableDirector is a  class of the Unity engine since version 2017.1.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Playables.PlayableDirector.html):
/**
Controls playback of Playable objects.
The PlayableDirector is primarily used by the Timeline Package to handle bindings to scene objects and control playback of the PlayableGraph.
The PlayableDirector object builds a PlayableGraph from a PlayableAsset. Once the graph is built, use the PlayableDirector to manage playback of the graph's Playable. To control playback of the PlayableGraph, use:
- PlayableDirector.Play
- PlayableDirector.Stop
- PlayableDirector.Pause
- PlayableDirector.Resume
To be notified of playback state changes, subscribe to:
- PlayableDirector.played
- PlayableDirector.paused
- PlayableDirector.stopped
To handle references between assets and scene objects, PlayableDirector implements IExposedPropertyTable. To set or get bindings, use:
- PlayableDirector.SetGenericBinding
- PlayableDirector.GetGenericBinding
Multiple PlayableDirectors can reference the same PlayableAsset. When this occurs, each PlayableDirector creates its own independent PlayableGraph with its own scene bindings.
The following example demonstrates how to use the Playable Director to bind scene objects to assets and how to control the Playable Graph.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableDirector {
    pub m_DirectorUpdateMode: i32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    pub m_ExposedReferences: ExposedReferenceTable,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (2017.1.0b1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub m_InitialState: i32,
    /**The time at which the Playable should start when first played.*/
    pub m_InitialTime: f64,
    /**The PlayableAsset that is used to instantiate a playable for playback.*/
    /// PPtr<[`Object`]>: (2017.1.0b1 - 6000.2.0a6)
    pub m_PlayableAsset: PPtr,
    pub m_SceneBindings: Vec<DirectorGenericBinding>,
    pub m_WrapMode: i32,
}

/// PlayerSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PlayerSettings.html):
/**
Use Player settings to define how Unity builds and displays your final application.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerSettings {
    pub AndroidProfiler: bool,
    pub allowedAutorotateToLandscapeLeft: bool,
    pub allowedAutorotateToLandscapeRight: bool,
    pub allowedAutorotateToPortrait: bool,
    pub allowedAutorotateToPortraitUpsideDown: bool,
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
    /// Hash128: (2020.2.0b1 - 6000.2.0a6)
    pub AID: Option<Hash128>,
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub AndroidEnableSustainedPerformanceMode: Option<bool>,
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub AndroidFilterTouchesWhenObscured: Option<bool>,
    /// String: (3.4.0 - 3.5.7)
    pub AndroidLicensePublicKey: Option<String>,
    /// i32: (2019.3.0b1 - 2023.1.0a21)
    pub D3DHDRBitDepth: Option<i32>,
    /// bool: (6000.2.0a6 - 6000.2.0a6)
    #[serde(alias = "Force IOS Speakers When Recording")]
    pub Force_IOS_Speakers_When_Recording: Option<bool>,
    /// bool: (3.4.0 - 5.4.6f3)
    #[serde(alias = "Override IPod Music")]
    pub Override_IPod_Music: Option<bool>,
    /// bool: (6000.2.0a6 - 6000.2.0a6)
    #[serde(alias = "Prepare IOS For Recording")]
    pub Prepare_IOS_For_Recording: Option<bool>,
    /// i32: (3.5.0 - 6000.2.0a6)
    pub accelerometerFrequency: Option<i32>,
    /// i32: (2020.2.0f1 - 6000.2.0a6)
    pub activeInputHandler: Option<i32>,
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub allowFullscreenSwitch: Option<bool>,
    /// bool: (2022.3.11f1 - 6000.2.0a6)
    pub allowHDRDisplaySupport: Option<bool>,
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub androidApplicationEntry: Option<i32>,
    /// i32: (2021.3.35f1 - 6000.2.0a6)
    pub androidAutoRotationBehavior: Option<i32>,
    /// i32: (2017.2.0f1 - 6000.2.0a6)
    pub androidBlitType: Option<i32>,
    /// i32: (2019.4.31f1 - 6000.2.0a6)
    pub androidDefaultWindowHeight: Option<i32>,
    /// i32: (2019.4.31f1 - 6000.2.0a6)
    pub androidDefaultWindowWidth: Option<i32>,
    /// i32: (2019.4.31f1 - 6000.2.0a6)
    pub androidFullscreenMode: Option<i32>,
    /// f32: (2017.2.0f1 - 6000.2.0a6)
    pub androidMaxAspectRatio: Option<f32>,
    /// f32: (2023.2.0b1 - 6000.2.0a6)
    pub androidMinAspectRatio: Option<f32>,
    /// i32: (2019.4.31f1 - 6000.2.0a6)
    pub androidMinimumWindowHeight: Option<i32>,
    /// i32: (2019.4.31f1 - 6000.2.0a6)
    pub androidMinimumWindowWidth: Option<i32>,
    /// bool: (2022.3.61f1 - 6000.2.0a6)
    pub androidPredictiveBackSupport: Option<bool>,
    /// bool: (2018.3.0f1 - 6000.2.0a6)
    pub androidRenderOutsideSafeArea: Option<bool>,
    /// bool: (2019.4.31f1 - 6000.0.3f1)
    pub androidResizableWindow: Option<bool>,
    /// bool: (6000.0.4f1 - 6000.2.0a6)
    pub androidResizeableActivity: Option<bool>,
    /// i32: (3.5.0 - 6000.2.0a6)
    pub androidShowActivityIndicatorOnLoading: Option<i32>,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub androidStartInFullscreen: Option<bool>,
    /// i32: (2017.2.0f1 - 6000.2.0a6)
    pub androidSupportedAspectRatio: Option<i32>,
    /// bool: (2019.2.0b1 - 6000.2.0a6)
    pub androidUseSwappy: Option<bool>,
    /// Vec<AndroidDeviceFilterData>: (6000.0.5f1 - 6000.2.0a6)
    pub androidVulkanAllowFilterList: Option<Vec<AndroidDeviceFilterData>>,
    /// Vec<AndroidDeviceFilterData>: (6000.0.5f1 - 6000.2.0a6)
    pub androidVulkanDenyFilterList: Option<Vec<AndroidDeviceFilterData>>,
    /// PPtr<[`VulkanDeviceFilterLists`]>: (6000.1.0b1 - 6000.2.0a6)
    pub androidVulkanDeviceFilterListAsset: Option<PPtr>,
    /// i32: (2022.3.49f1 - 2022.3.62f1)
    pub audioSpatialExperience: Option<i32>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub bakeCollisionMeshes: Option<bool>,
    /// String: (5.0.0f4 - 5.6.0b5)
    pub bundleIdentifier: Option<String>,
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub bundleVersion: Option<String>,
    /// bool: (3.4.0 - 6000.0.14f1)
    pub captureSingleScreen: Option<bool>,
    /// bool: (5.1.0f1 - 6000.2.0a6)
    pub cloudEnabled: Option<bool>,
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub cloudProjectId: Option<String>,
    /// Vec<i32>: (2022.2.0b3 - 2023.1.0a8)
    pub cpuConfiguration: Option<Vec<i32>>,
    /// Vector2f: (4.0.0 - 6000.2.0a6)
    pub cursorHotspot: Option<Vector2f>,
    /// bool: (4.5.5 - 4.7.2)
    pub d3d11ForceExclusiveMode: Option<bool>,
    /// i32: (5.0.0f4 - 2017.4.40f1)
    pub d3d11FullscreenMode: Option<i32>,
    /// i32: (4.5.3 - 2017.2.5f1)
    pub d3d9FullscreenMode: Option<i32>,
    /// i32: (3.5.5 - 3.5.7)
    pub debugUnloadMode: Option<i32>,
    /// bool: (2022.3.8f1 - 6000.2.0a6)
    pub dedicatedServerOptimizations: Option<bool>,
    /// PPtr<[`Texture2D`]>: (4.0.0 - 6000.2.0a6)
    pub defaultCursor: Option<PPtr>,
    /// bool: (3.4.0 - 2017.4.40f1)
    pub defaultIsFullScreen: Option<bool>,
    /// bool: (4.2.0 - 6000.2.0a6)
    pub defaultIsNativeResolution: Option<bool>,
    /// i32: (5.6.5f1 - 6000.2.0a6)
    pub deferSystemGesturesMode: Option<i32>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub disableDepthAndStencilBuffers: Option<bool>,
    /// bool: (2017.1.0f1 - 2020.2.0a21)
    pub disableOldInputManagerSupport: Option<bool>,
    /// i32: (3.4.0 - 2019.3.0a10)
    pub displayResolutionDialog: Option<i32>,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub enableFrameTimingStats: Option<bool>,
    /// bool: (2022.2.0b3 - 2023.1.0a8)
    pub enableGamepadInput: Option<bool>,
    /// bool: (4.1.0 - 4.7.2)
    pub enableHWStatistics: Option<bool>,
    /// bool: (2017.1.0f1 - 2020.2.0a21)
    pub enableNativePlatformBackendsForNewInputSystem: Option<bool>,
    /// bool: (5.6.0b1 - 2017.1.0b1)
    pub enableNewInputSystem: Option<bool>,
    /// bool: (2021.3.2f1 - 6000.2.0a6)
    pub enableOpenGLProfilerGPURecorders: Option<bool>,
    /// bool: (2021.2.0b1 - 2023.1.0a8)
    pub forceSRGBBlit: Option<bool>,
    /// bool: (4.2.0 - 6000.2.0a6)
    pub forceSingleInstance: Option<bool>,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub framebufferDepthMemorylessMode: Option<i32>,
    /// i32: (2018.1.0f1 - 6000.2.0a6)
    pub fullscreenMode: Option<i32>,
    /// bool: (4.2.0 - 6000.2.0a6)
    pub gpuSkinning: Option<bool>,
    /// i32: (5.5.2f1 - 2019.3.0a12)
    pub graphicsJobMode: Option<i32>,
    /// bool: (5.4.0f3 - 2019.3.0a11)
    pub graphicsJobs: Option<bool>,
    /// i32: (2022.2.15f1 - 6000.2.0a6)
    pub hdrBitDepth: Option<i32>,
    /// bool: (5.6.5f1 - 6000.2.0a6)
    pub hideHomeButton: Option<bool>,
    /// PPtr<[`Texture2D`]>: (2022.2.0f1 - 6000.2.0a6)
    pub hmiLoadingImage: Option<PPtr>,
    /// String: (4.1.2 - 4.7.2)
    pub iPhoneBundleIdentifier: Option<String>,
    /// bool: (5.3.1f1 - 2017.2.5f1)
    pub ignoreAlphaClear: Option<bool>,
    /// i32: (2022.1.0b1 - 6000.2.0a6)
    pub insecureHttpOption: Option<i32>,
    /// PPtr<[`Texture2D`]>: (2022.2.3f1 - 6000.2.0a6)
    pub invalidatedPatternTexture: Option<PPtr>,
    /// bool: (5.2.1f1 - 2021.3.45f1)
    pub iosAllowHTTPDownload: Option<bool>,
    /// i32: (5.0.0f4 - 2019.3.0a11)
    pub iosAppInBackgroundBehavior: Option<i32>,
    /// i32: (3.5.0 - 6000.2.0a6)
    pub iosShowActivityIndicatorOnLoading: Option<i32>,
    /// bool: (2017.4.40f1 - 6000.2.0a6)
    pub iosUseCustomAppBackgroundBehavior: Option<bool>,
    /// bool: (2018.2.9f1 - 6000.2.0a6)
    pub isWsaHolographicRemotingEnabled: Option<bool>,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub legacyClampBlendShapeWeights: Option<bool>,
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub loadStoreDebugModeEnabled: Option<bool>,
    /// i32: (3.5.0 - 6000.2.0a6)
    pub m_ActiveColorSpace: Option<i32>,
    /// Vec<i32>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ColorGamuts: Option<Vec<i32>>,
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub m_HolographicPauseOnTrackingLoss: Option<bool>,
    /// PPtr<[`Texture2D`]>: (5.5.0f3 - 6000.2.0a6)
    pub m_HolographicTrackingLossScreen: Option<PPtr>,
    /// bool: (3.5.0 - 6000.2.0a6)
    pub m_MTRendering: Option<bool>,
    /// bool: (4.3.0 - 2017.1.5f1)
    pub m_MobileMTRendering: Option<bool>,
    /// i32: (4.3.0 - 5.4.6f3)
    pub m_MobileRenderingPath: Option<i32>,
    /// i32: (3.4.0 - 5.4.6f3)
    pub m_RenderingPath: Option<i32>,
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub m_ShowUnitySplashLogo: Option<bool>,
    /// bool: (5.0.1f1 - 6000.2.0a6)
    pub m_ShowUnitySplashScreen: Option<bool>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenAnimation: Option<i32>,
    /// f32: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenBackgroundAnimationZoom: Option<f32>,
    /// ColorRGBA: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenBackgroundColor: Option<ColorRGBA>,
    /// PPtr<[`Texture2D`]>: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenBackgroundLandscape: Option<PPtr>,
    /// f32: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenBackgroundLandscapeAspect: Option<f32>,
    /// Rectf: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenBackgroundLandscapeUvs: Option<Rectf>,
    /// PPtr<[`Texture2D`]>: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenBackgroundPortrait: Option<PPtr>,
    /// f32: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenBackgroundPortraitAspect: Option<f32>,
    /// Rectf: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenBackgroundPortraitUvs: Option<Rectf>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenDrawMode: Option<i32>,
    /// f32: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenLogoAnimationZoom: Option<f32>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenLogoStyle: Option<i32>,
    /// Vec<SplashScreenLogo>: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenLogos: Option<Vec<SplashScreenLogo>>,
    /// f32: (5.5.0f3 - 6000.2.0a6)
    pub m_SplashScreenOverlayOpacity: Option<f32>,
    /// i32: (5.4.0f3 - 5.4.6f3)
    pub m_SplashScreenStyle: Option<i32>,
    /// i32: (6000.0.20f1 - 6000.2.0a6)
    pub m_SpriteBatchMaxVertexCount: Option<i32>,
    /// i32: (2022.1.0b1 - 6000.2.0a6)
    pub m_SpriteBatchVertexThreshold: Option<i32>,
    /// Vec<i32>: (5.4.0f3 - 6000.2.0a6)
    pub m_StackTraceTypes: Option<Vec<i32>>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_StereoRenderingPath: Option<i32>,
    /// bool: (4.5.0 - 5.3.8f2)
    pub m_Stereoscopic3D: Option<bool>,
    /// AspectRatios: (3.4.0 - 2022.1.24f1)
    pub m_SupportedAspectRatios: Option<AspectRatios>,
    /// PPtr<[`Sprite`]>: (2022.2.0b1 - 6000.2.0a6)
    pub m_UnitySplashLogo: Option<PPtr>,
    /// bool: (4.0.0 - 5.0.4f1)
    pub m_UseDX11: Option<bool>,
    /// PPtr<[`Texture2D`]>: (5.3.0f1 - 6000.2.0a6)
    pub m_VirtualRealitySplashScreen: Option<PPtr>,
    /// String: (5.6.1f1 - 6000.2.0a6)
    pub macAppStoreCategory: Option<String>,
    /// i32: (4.0.0 - 2017.4.40f1)
    pub macFullscreenMode: Option<i32>,
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub macRetinaSupport: Option<bool>,
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub meshDeformation: Option<i32>,
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub metalFramebufferOnly: Option<bool>,
    /// bool: (4.3.3 - 5.4.6f3)
    pub metroEnableIndependentInputSource: Option<bool>,
    /// bool: (4.3.3 - 5.3.8f2)
    pub metroEnableLowLatencyPresentationAPI: Option<bool>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub metroInputSource: Option<i32>,
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub mipStripping: Option<bool>,
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub mobileMTRenderingBaked: Option<bool>,
    /// bool: (5.3.8f1 - 6000.2.0a6)
    pub muteOtherAudioSources: Option<bool>,
    /// bool: (5.2.0f2 - 2018.2.21f1)
    pub n3dsDisableStereoscopicView: Option<bool>,
    /// bool: (5.2.0f2 - 2018.2.21f1)
    pub n3dsEnableSharedListOpt: Option<bool>,
    /// bool: (5.2.0f2 - 2018.2.21f1)
    pub n3dsEnableVSync: Option<bool>,
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub numberOfMipsStripped: Option<i32>,
    /// Vec<(String, i32)>: (2022.2.3f1 - 6000.2.0a6)
    pub numberOfMipsStrippedPerMipmapLimitGroup: Option<Vec<(String, i32)>>,
    /// String: (5.1.0f1 - 6000.2.0a6)
    pub organizationId: Option<String>,
    /// bool: (2022.2.19f1 - 6000.2.0a6)
    pub platformRequiresReadableAssets: Option<bool>,
    /// String: (2021.2.0b1 - 2023.1.0a8)
    pub playerDataPath: Option<String>,
    /// i32: (2020.2.0f1 - 6000.2.0a6)
    pub playerMinOpenGLESVersion: Option<i32>,
    /// Vec<PPtr<[`Object`]>>: (5.0.0f4 - 6000.2.0a6)
    pub preloadedAssets: Option<Vec<PPtr>>,
    /// bool: (2017.3.0f1 - 6000.2.0a6)
    pub preserveFramebufferAlpha: Option<bool>,
    /// GUID: (5.4.0f3 - 6000.2.0a6)
    pub productGUID: Option<GUID>,
    /// String: (5.1.0f1 - 5.1.5f1)
    pub projectId: Option<String>,
    /// String: (5.1.0f1 - 6000.2.0a6)
    pub projectName: Option<String>,
    /// bool: (5.4.0f3 - 2019.3.0a12)
    pub protectGraphicsMemory: Option<bool>,
    /// PPtr<[`Texture2D`]>: (5.0.0f4 - 5.4.6f3)
    pub ps3SplashScreen: Option<PPtr>,
    /// bool: (5.0.0f4 - 2018.2.21f1)
    pub psp2AcquireBGM: Option<bool>,
    /// i32: (5.0.0f4 - 2018.2.21f1)
    pub psp2PowerMode: Option<i32>,
    /// Vec<String>: (2020.2.0b1 - 6000.2.0a6)
    pub qualitySettingsNames: Option<Vec<String>>,
    /// bool: (2020.3.35f1 - 6000.2.0a6)
    pub resetResolutionOnWindowResize: Option<bool>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub resizableWindow: Option<bool>,
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub resolutionScalingMode: Option<i32>,
    /// bool: (5.4.0f3 - 5.4.6f3)
    pub singlePassStereoRendering: Option<bool>,
    /// i32: (2019.3.15f1 - 2023.1.0a26)
    pub stadiaPresentMode: Option<i32>,
    /// i32: (2019.3.15f1 - 2023.1.0a26)
    pub stadiaTargetFramerate: Option<i32>,
    /// bool: (4.0.0 - 4.7.2)
    pub stripPhysics: Option<bool>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub submitAnalytics: Option<bool>,
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub switchAllowGpuScratchShrinking: Option<bool>,
    /// i32: (2022.1.0b1 - 6000.2.0a6)
    pub switchGpuScratchPoolGranularity: Option<i32>,
    /// i32: (2021.3.30f1 - 6000.2.0a6)
    pub switchMaxWorkerMultiple: Option<i32>,
    /// i32: (2018.3.5f1 - 6000.2.0a6)
    pub switchNVNDefaultPoolsGranularity: Option<i32>,
    /// i32: (2022.1.0b1 - 6000.2.0a6)
    pub switchNVNGraphicsFirmwareMemory: Option<i32>,
    /// i32: (2018.4.26f1 - 6000.2.0a6)
    pub switchNVNMaxPublicSamplerIDCount: Option<i32>,
    /// i32: (2018.4.26f1 - 6000.2.0a6)
    pub switchNVNMaxPublicTextureIDCount: Option<i32>,
    /// i32: (2018.3.5f1 - 6000.2.0a6)
    pub switchNVNOtherPoolsGranularity: Option<i32>,
    /// i32: (2018.3.5f1 - 6000.2.0a6)
    pub switchNVNShaderPoolsGranularity: Option<i32>,
    /// i32: (2018.1.0f1 - 6000.2.0a6)
    pub switchQueueCommandMemory: Option<i32>,
    /// i32: (2018.3.5f1 - 6000.2.0a6)
    pub switchQueueComputeMemory: Option<i32>,
    /// i32: (2018.3.5f1 - 6000.2.0a6)
    pub switchQueueControlMemory: Option<i32>,
    /// i32: (4.0.0 - 5.0.4f1)
    pub targetGlesGraphics: Option<i32>,
    /// i32: (4.6.3 - 5.0.4f1)
    pub targetIOSGraphics: Option<i32>,
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub targetPixelDensity: Option<i32>,
    /// i32: (3.4.0 - 3.5.7)
    pub targetPlatform: Option<i32>,
    /// i32: (3.4.0 - 5.2.5f1)
    pub targetResolution: Option<i32>,
    /// i32: (5.4.3f1 - 2018.1.9f2)
    pub tizenShowActivityIndicatorOnLoading: Option<i32>,
    /// String: (2022.3.18f1 - 6000.2.0a6)
    pub tvOSBundleVersion: Option<String>,
    /// bool: (5.3.0f1 - 5.6.0b2)
    pub uiUse16BitDepthBuffer: Option<bool>,
    /// i32: (2021.3.32f1 - 6000.2.0a6)
    pub unsupportedMSAAFallback: Option<i32>,
    /// bool: (2021.2.0b1 - 2022.1.0a15)
    pub uploadClearedTextureDataAfterCreationFromScript: Option<bool>,
    /// bool: (3.5.0 - 4.7.2)
    pub use24BitDepthBuffer: Option<bool>,
    /// bool: (3.4.0 - 3.4.2)
    pub useAlphaInDashboard: Option<bool>,
    /// bool: (2019.1.7f1 - 6000.2.0a6)
    pub useFlipModelSwapchain: Option<bool>,
    /// bool: (5.5.4f1 - 6000.2.0a6)
    pub useHDRDisplay: Option<bool>,
    /// bool: (5.2.1f1 - 6000.2.0a6)
    pub useOnDemandResources: Option<bool>,
    /// i32: (4.5.0 - 2018.2.21f1)
    pub videoMemoryForVertexBuffers: Option<i32>,
    /// bool: (5.1.1f1 - 5.3.8f2)
    pub virtualRealitySupported: Option<bool>,
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub virtualTexturingSupportEnabled: Option<bool>,
    /// bool: (4.5.4 - 6000.2.0a6)
    pub visibleInBackground: Option<bool>,
    /// String: (2022.3.18f1 - 6000.2.0a6)
    pub visionOSBundleVersion: Option<String>,
    /// VRSettings: (5.6.0b1 - 6000.2.0a6)
    pub vrSettings: Option<VRSettings>,
    /// bool: (2020.3.18f1 - 6000.2.0a6)
    pub vulkanEnableCommandBufferRecycling: Option<bool>,
    /// bool: (2019.4.6f1 - 6000.2.0a6)
    pub vulkanEnableLateAcquireNextImage: Option<bool>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub vulkanEnablePreTransform: Option<bool>,
    /// bool: (2018.2.0b1 - 6000.2.0a6)
    pub vulkanEnableSetSRGBWrite: Option<bool>,
    /// u32: (2019.3.0f1 - 6000.2.0a6)
    pub vulkanNumSwapchainBuffers: Option<u32>,
    /// bool: (2018.2.0b1 - 2018.2.21f1)
    pub vulkanUseSWCommandBuffers: Option<bool>,
    /// i32: (3.5.0 - 4.3.4)
    pub wiiHio2Usage: Option<i32>,
    /// ColorRGBA: (3.5.0 - 4.3.4)
    pub wiiLoadingScreenBackground: Option<ColorRGBA>,
    /// String: (3.5.0 - 4.3.4)
    pub wiiLoadingScreenFileName: Option<String>,
    /// i32: (3.5.0 - 4.3.4)
    pub wiiLoadingScreenPeriod: Option<i32>,
    /// Rectf: (3.5.0 - 4.3.4)
    pub wiiLoadingScreenRect: Option<Rectf>,
    /// i32: (3.5.0 - 4.3.4)
    pub wiiLoadingScreenRectPlacement: Option<i32>,
    /// bool: (5.2.0f2 - 2017.4.40f1)
    pub wiiUAllowScreenCapture: Option<bool>,
    /// i32: (5.2.0f2 - 2017.4.40f1)
    pub wiiUControllerCount: Option<i32>,
    /// i32: (5.2.0f2 - 2017.4.40f1)
    pub wiiUGamePadMSAA: Option<i32>,
    /// bool: (5.2.0f2 - 2017.4.40f1)
    pub wiiUSupportsBalanceBoard: Option<bool>,
    /// bool: (5.2.0f2 - 2017.4.40f1)
    pub wiiUSupportsClassicController: Option<bool>,
    /// bool: (5.2.0f2 - 2017.4.40f1)
    pub wiiUSupportsMotionPlus: Option<bool>,
    /// bool: (5.2.0f2 - 2017.4.40f1)
    pub wiiUSupportsNunchuk: Option<bool>,
    /// bool: (5.2.0f2 - 2017.4.40f1)
    pub wiiUSupportsProController: Option<bool>,
    /// i32: (5.2.0f2 - 2017.4.40f1)
    pub wiiUTVResolution: Option<i32>,
    /// i32: (2020.3.42f1 - 6000.2.0a6)
    pub windowsGamepadBackendHint: Option<i32>,
    /// bool: (2017.3.0b1 - 6000.2.0a6)
    pub wsaTransparentSwapchain: Option<bool>,
    /// bool: (3.5.0 - 6000.2.0a6)
    pub xboxEnableAvatar: Option<bool>,
    /// bool: (5.3.6f1 - 5.3.8f2)
    pub xboxEnableEnableRenderThreadRunsJobs: Option<bool>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub xboxEnableFitness: Option<bool>,
    /// bool: (4.3.0 - 6000.2.0a6)
    pub xboxEnableGuest: Option<bool>,
    /// bool: (4.1.0 - 6000.2.0a6)
    pub xboxEnableHeadOrientation: Option<bool>,
    /// bool: (3.5.0 - 6000.2.0a6)
    pub xboxEnableKinect: Option<bool>,
    /// bool: (3.5.0 - 6000.2.0a6)
    pub xboxEnableKinectAutoTracking: Option<bool>,
    /// bool: (5.3.0f2 - 6000.2.0a6)
    pub xboxEnablePIXSampling: Option<bool>,
    /// bool: (3.5.0 - 3.5.7)
    pub xboxEnableSpeech: Option<bool>,
    /// bool: (2017.1.0f1 - 6000.2.0a6)
    pub xboxOneDisableEsram: Option<bool>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub xboxOneDisableKinectGpuReservation: Option<bool>,
    /// bool: (5.6.0b1 - 6000.2.0a6)
    pub xboxOneEnable7thCore: Option<bool>,
    /// bool: (2018.4.17f1 - 6000.2.0a6)
    pub xboxOneEnableTypeOptimization: Option<bool>,
    /// i32: (5.3.7f1 - 6000.2.0a6)
    pub xboxOneLoggingLevel: Option<i32>,
    /// i32: (5.3.6f1 - 6000.2.0a6)
    pub xboxOneMonoLoggingLevel: Option<i32>,
    /// u32: (2017.2.0f1 - 6000.2.0a6)
    pub xboxOnePresentImmediateThreshold: Option<u32>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub xboxOneResolution: Option<i32>,
    /// i32: (5.5.6f1 - 6000.2.0a6)
    pub xboxOneSResolution: Option<i32>,
    /// i32: (5.5.6f1 - 6000.2.0a6)
    pub xboxOneXResolution: Option<i32>,
    /// bool: (4.3.0 - 6000.2.0a6)
    pub xboxPIXTextureCapture: Option<bool>,
    /// bool: (3.4.0 - 4.1.5)
    pub xboxSkinOnGPU: Option<bool>,
    /// u32: (4.0.0 - 6000.2.0a6)
    pub xboxSpeechDB: Option<u32>,
}

/// PluginBuildInfo is a  class of the Unity engine since version 2018.4.13f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginBuildInfo {
    pub m_EditorPlugins: Vec<String>,
    pub m_RuntimePlugins: Vec<String>,
}

/// PluginImportOutput is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginImportOutput {
    /// i32: (5.4.0f3 - 6000.2.0a6)
    pub dllType: Option<i32>,
    /// i32: (5.0.0f4 - 5.3.8f2)
    pub pluginType: Option<i32>,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub scriptingRuntimeVersion: Option<i32>,
}

/// PluginImporter is a  class of the Unity engine since version 5.0.0f4.
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
    /// Vec<(String, PPtr<[`Texture2D`]>)>: (5.0.0f4 - 6000.2.0a6)
    pub m_IconMap: Vec<(String, PPtr)>,
    /**Is a native plugin loaded during startup or on demand?*/
    pub m_IsPreloaded: bool,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Output: PluginImportOutput,
    pub m_PlatformData: Vec<(Enum_String__String___String, PlatformSettingsData)>,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /**Allows you to specify a list of #define directives which controls whether your plug-in should be included.*/
    /// Vec<String>: (2018.3.0b1 - 6000.2.0a6)
    pub m_DefineConstraints: Option<Vec<String>>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_IsExplicitlyReferenced: Option<bool>,
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub m_IsOverridable: Option<bool>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /// bool: (2018.3.10f1 - 6000.2.0a6)
    pub m_ValidateReferences: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_String__String___String {
    String__String((String, String)),
    String(String),
}

/// PointEffector2D is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PointEffector2D.html):
/**
Applies forces to attract/repulse against a point.
When the source Collider2D is a trigger, the effector will apply forces whenever the target Collider2D overlaps the source.  When the source Collider isn't a trigger, the effector will apply forces whenever the target Collider2D is in contact with the source only.This effector is designed primarily to work with source Collider2D that are set as triggers so that target Collider2D can overlap the defined area.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PointEffector2D {
    /**The mask used to select specific layers allowed to interact with the effector.*/
    pub m_ColliderMask: BitField,
    /**The scale applied to the calculated distance between source and target.*/
    pub m_DistanceScale: f32,
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
    /// PPtr<[`GameObject`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The angular damping to apply to rigid-bodies.*/
    /// f32: (6000.1.0b1 - 6000.2.0a6)
    pub m_AngularDamping: Option<f32>,
    /// f32: (5.0.0f4 - 6000.1.0a5)
    pub m_AngularDrag: Option<f32>,
    /// f32: (5.0.0f4 - 6000.1.0a5)
    pub m_Drag: Option<f32>,
    /**The linear damping to apply to rigid-bodies.*/
    /// f32: (6000.1.0b1 - 6000.2.0a6)
    pub m_LinearDamping: Option<f32>,
    /**Should the collider-mask be used or the global collision matrix?*/
    /// bool: (5.0.2f1 - 6000.2.0a6)
    pub m_UseColliderMask: Option<bool>,
}

/// Polygon2D is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Polygon2D {
    pub m_Paths: Vec<Vec<Vector2f>>,
}

/// PolygonCollider2D is a  class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PolygonCollider2D.html):
/**
Collider for 2D physics representing an arbitrary polygon defined by its vertices.
Additional resources: BoxCollider2D, CircleCollider2D, EdgeCollider2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonCollider2D {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.3.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    /// PPtr<[`PhysicsMaterial2D`]>: (4.3.0 - 6000.2.0a6)
    pub m_Material: PPtr,
    /**Determines whether the PolygonCollider2D's shape is automatically updated based on a SpriteRenderer's tiling properties.*/
    /// bool: (5.6.0f1 - 6000.2.0a6)
    pub m_AutoTiling: Option<bool>,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_CallbackLayers: Option<BitField>,
    /**The composite operation to be used by a CompositeCollider2D.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOperation: Option<i32>,
    /**The composite operation order to be used when a CompositeCollider2D is used.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOrder: Option<i32>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**The density of the collider used to calculate its mass (when auto mass is enabled).*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_Density: Option<f32>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /**The local offset of the collider geometry.*/
    /// Vector2f: (5.0.0f4 - 6000.2.0a6)
    pub m_Offset: Option<Vector2f>,
    /**Corner points that define the collider's shape in local space.*/
    /// Polygon2D: (5.0.0f4 - 6000.2.0a6)
    pub m_Points: Option<Polygon2D>,
    /// Polygon2D: (4.3.0 - 4.7.2)
    pub m_Poly: Option<Polygon2D>,
    /// SpriteTilingProperty: (5.6.0f1 - 6000.2.0a6)
    pub m_SpriteTilingProperty: Option<SpriteTilingProperty>,
    /**When the value is true, the Collider uses an additional Delaunay triangulation step to produce the Collider mesh. When the value is false, this additional step does not occur.*/
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_UseDelaunayMesh: Option<bool>,
    /// bool: (5.6.0b1 - 2023.1.0a21)
    pub m_UsedByComposite: Option<bool>,
    /**Whether the collider is used by an attached effector or not.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_UsedByEffector: Option<bool>,
}

/// PositionConstraint is a  class of the Unity engine since version 2018.1.0f1.
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
    /// PPtr<[`GameObject`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub m_Sources: Vec<ConstraintSource>,
    /**The translation used when the sources have a total weight of 0.*/
    pub m_TranslationAtRest: Vector3f,
    /**The offset from the constrained position.*/
    pub m_TranslationOffset: Vector3f,
    /**The weight of the constraint component.*/
    pub m_Weight: f32,
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_Active: Option<bool>,
    /// bool: (2018.1.0f1 - 2022.1.0a9)
    pub m_IsContraintActive: Option<bool>,
}

/// Prefab is a  class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Prefab {
    /// PPtr<[`GameObject`]>: (3.5.0 - 6000.2.0a6)
    pub m_RootGameObject: PPtr,
    /// bool: (2021.2.0b1 - 6000.2.0a6)
    pub m_ContainsMissingSerializeReferenceTypes: Option<bool>,
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub m_HideFlagsBehaviour: Option<i32>,
    /// bool: (3.5.0 - 4.7.2)
    pub m_IsExploded: Option<bool>,
    /// bool: (2018.2.0b1 - 2018.2.21f1)
    pub m_IsPrefabAsset: Option<bool>,
    /// bool: (3.5.0 - 2018.1.9f2)
    pub m_IsPrefabParent: Option<bool>,
    /// PrefabModification: (3.5.0 - 2018.2.21f1)
    pub m_Modification: Option<PrefabModification>,
    /// PPtr<[`Prefab`]>: (3.5.0 - 2018.1.9f2)
    pub m_ParentPrefab: Option<PPtr>,
    /// PPtr<[`Prefab`]>: (2018.2.0b1 - 2018.2.21f1)
    pub m_SourcePrefab: Option<PPtr>,
}

/// PrefabImporter is a  class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PrefabImporter {
    pub m_AddedObjectFileIDs: Vec<i64>,
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2018.3.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_IsPrefabVariant: bool,
    pub m_Name: String,
    pub m_UserData: String,
    /// bool: (2018.3.0f1 - 2020.1.0a25)
    pub m_UnableToImportOnPreviousDomainReload: Option<bool>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /// GUID: (2022.2.0b1 - 6000.2.0a6)
    pub m_VariantParentGUID: Option<GUID>,
}

/// PrefabInstance is a  class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PrefabInstance {
    pub m_Modification: PrefabModification,
    /// PPtr<[`GameObject`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_RootGameObject: PPtr,
    /// PPtr<[`Prefab`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_SourcePrefab: PPtr,
}

/// PrefabModification is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct PrefabModification {
    pub m_Modifications: Vec<PropertyModification>,
    /// Vec<PPtr<[`Object`]>>: (3.5.0 - 2018.2.21f1); Vec<PPtr<[`Component`]>>: (2018.3.0b1 - 6000.2.0a6)
    pub m_RemovedComponents: Vec<PPtr>,
    /// PPtr<[`Transform`]>: (3.5.0 - 6000.2.0a6)
    pub m_TransformParent: PPtr,
    /// Vec<AddedComponent>: (2022.2.0b1 - 6000.2.0a6)
    pub m_AddedComponents: Option<Vec<AddedComponent>>,
    /// Vec<AddedGameObject>: (2022.1.0b1 - 6000.2.0a6)
    pub m_AddedGameObjects: Option<Vec<AddedGameObject>>,
    /// Vec<PPtr<[`GameObject`]>>: (2022.2.0b1 - 6000.2.0a6)
    pub m_RemovedGameObjects: Option<Vec<PPtr>>,
}

/// PreloadData is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct PreloadData {
    /// Vec<PPtr<[`Object`]>>: (3.4.0 - 6000.2.0a6)
    pub m_Assets: Vec<PPtr>,
    pub m_Name: String,
    /// Vec<String>: (5.0.0f4 - 6000.2.0a6)
    pub m_Dependencies: Option<Vec<String>>,
    /// bool: (2018.2.0b1 - 6000.2.0a6)
    pub m_ExplicitDataLayout: Option<bool>,
}

/// Preset is a  class of the Unity engine since version 2018.1.0f1.
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
    /// Vec<PropertyModification>: (2023.1.0b1 - 6000.2.0a6)
    pub m_CoupledProperties: Option<Vec<PropertyModification>>,
    /// PresetType: (2023.1.0b1 - 6000.2.0a6)
    pub m_CoupledType: Option<PresetType>,
    /**List of properties to ignore when applying the Preset to an object.*/
    /// Vec<String>: (2020.1.0b1 - 6000.2.0a6)
    pub m_ExcludedProperties: Option<Vec<String>>,
}

/// PresetManager is a  class of the Unity engine since version 2018.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PresetManager {
    /// Vec<DefaultPresetList>: (2018.1.0f1 - 2019.3.0a8)
    pub m_DefaultList: Option<Vec<DefaultPresetList>>,
    /// Vec<(PresetType, Vec<DefaultPreset>)>: (2019.3.0b1 - 6000.2.0a6)
    pub m_DefaultPresets: Option<Vec<(PresetType, Vec<DefaultPreset>)>>,
}

/// PresetType is a sub class of the Unity engine since version 2018.1.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Presets.PresetType.html):
/**
Stores a type to which a Preset can be applied.
Only classes that inherit from UnityEngine.Object are represented by a PresetType.This structure is used instead of Type because some native C++ types in Unity are not exposed to managed C# for optimization reasons.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PresetType {
    pub m_ManagedTypeFallback: String,
    /// PPtr<[`MonoScript`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_ManagedTypePPtr: PPtr,
    pub m_NativeTypeID: i32,
}

/// PreviewAnimationClip is a  class of the Unity engine since version 5.6.0b1.
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
    /// Vec<PPtrCurve>: (5.6.0b1 - 6000.2.0a6)
    pub m_PPtrCurves: Vec<PPtrCurve>,
    pub m_PositionCurves: Vec<Vector3Curve>,
    pub m_RotationCurves: Vec<QuaternionCurve>,
    pub m_SampleRate: f32,
    pub m_ScaleCurves: Vec<Vector3Curve>,
    pub m_UseHighQualityCurve: bool,
    pub m_WrapMode: i32,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_HasGenericRootTransform: Option<bool>,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_HasMotionFloatCurves: Option<bool>,
}

/// PreviewData is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewData {
    pub m_CompSize: i32,
    pub m_OrigSize: i32,
    pub m_PreviewData: Vec<f32>,
}

/// PreviewImporter is a  class of the Unity engine since version 2020.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2020.2.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// ProbeSetIndex is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProbeSetIndex {
    pub m_Hash: Hash128,
    pub m_Offset: i32,
    pub m_Size: i32,
}

/// ProbeSetTetrahedralization is a sub class of the Unity engine since version 5.0.0f4.
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
    /// PPtr<[`Shader`]>: (3.4.0 - 6000.2.0a6)
    pub m_Shader: PPtr,
    /// Vec<String>: (5.6.0f1 - 6000.2.0a6)
    pub disabledShaderPasses: Option<Vec<String>>,
    /// i32: (3.5.0 - 2017.4.40f1)
    pub m_AnimationUpdateRate: Option<i32>,
    /// Vec<BuildTextureStackReference>: (2020.1.0b1 - 6000.2.0a6)
    pub m_BuildTextureStacks: Option<Vec<BuildTextureStackReference>>,
    /// i32: (3.4.0 - 3.4.2)
    pub m_CacheSize: Option<i32>,
    /// i32: (4.3.0 - 6000.2.0a6)
    pub m_CustomRenderQueue: Option<i32>,
    /**Gets and sets whether the Double Sided Global Illumination setting is enabled for this material.*/
    /// bool: (5.6.2f1 - 6000.2.0a6)
    pub m_DoubleSidedGI: Option<bool>,
    /// bool: (5.6.0f1 - 6000.2.0a6)
    pub m_EnableInstancingVariants: Option<bool>,
    /// u32: (3.4.0 - 2017.4.40f1)
    pub m_Flags: Option<u32>,
    /// bool: (4.5.0 - 2017.4.40f1)
    pub m_GenerateMipmaps: Option<bool>,
    /// Hash128: (4.1.0 - 2017.4.40f1)
    pub m_Hash: Option<Hash128>,
    /// i32: (3.5.0 - 2017.4.40f1)
    pub m_Height: Option<i32>,
    /// Vec<SubstanceInput>: (3.4.0 - 2017.4.40f1)
    pub m_Inputs: Option<Vec<SubstanceInput>>,
    /// Vec<String>: (2021.2.18f1 - 6000.2.0a6)
    pub m_InvalidKeywords: Option<Vec<String>>,
    /// u32: (5.0.0f4 - 6000.2.0a6)
    pub m_LightmapFlags: Option<u32>,
    /// i32: (4.0.0 - 2017.4.40f1)
    pub m_LoadingBehavior: Option<i32>,
    /// i32: (3.4.0 - 3.4.2)
    pub m_MaximumSize: Option<i32>,
    /// String: (4.2.0 - 2017.4.40f1)
    pub m_PrototypeName: Option<String>,
    /**An array containing names of the local shader keywords that are currently enabled for this material.*/
    /// Vec<String>: (4.1.0 - 4.7.2); String: (5.0.0f4 - 2022.1.0a16)
    pub m_ShaderKeywords: Option<Enum_Vec_String___String>,
    /// PPtr<[`SubstanceArchive`]>: (3.4.0 - 2017.4.40f1)
    pub m_SubstancePackage: Option<PPtr>,
    /// Vec<PPtr<[`ProceduralTexture`]>>: (3.4.0 - 2017.4.40f1)
    pub m_Textures: Option<Vec<PPtr>>,
    /// Vec<String>: (2021.2.18f1 - 6000.2.0a6)
    pub m_ValidKeywords: Option<Vec<String>>,
    /// i32: (3.5.0 - 2017.4.40f1)
    pub m_Width: Option<i32>,
    /// Vec<(String, String)>: (5.1.0f1 - 6000.2.0a6)
    pub stringTagMap: Option<Vec<(String, String)>>,
}

/// ProceduralMaterialInformation is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProceduralMaterialInformation {
    pub m_Offset: Vector2f,
    pub m_Scale: Vector2f,
    /// i32: (3.5.0 - 2017.4.40f1)
    pub m_AnimationUpdateRate: Option<i32>,
    /// i32: (3.5.0 - 2017.4.40f1)
    pub m_GenerateAllOutputs: Option<i32>,
    /// bool: (4.5.0 - 2017.4.40f1)
    pub m_GenerateMipmaps: Option<bool>,
    /// i32: (3.4.0 - 3.5.7)
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
    /// i32: (3.4.0 - 2017.4.40f1)
    pub AlphaSource: Option<i32>,
    /// bool: (4.5.0 - 2017.4.40f1)
    pub AlphaSourceIsGrayscale: Option<bool>,
    /// i32: (3.4.0 - 2017.4.40f1)
    pub Format: Option<i32>,
    /// i32: (3.4.0 - 2017.4.40f1)
    pub Type: Option<i32>,
    /// bool: (5.0.0f4 - 2017.4.40f1)
    pub m_AlphaSourceIsInverted: Option<bool>,
    /// u64: (4.5.0 - 2017.4.40f1)
    pub m_AlphaSourceUID: Option<u64>,
    /// Vec<u8>: (3.4.0 - 2017.4.40f1)
    pub m_BakedData: Option<Vec<u8>>,
    /// TextureParameters: (3.4.0 - 2017.4.40f1)
    pub m_BakedParameters: Option<TextureParameters>,
    /// i32: (4.0.0 - 2017.4.40f1)
    pub m_ColorSpace: Option<i32>,
    /// bool: (2017.3.0b1 - 2023.2.0a17)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.3.0b1 - 2023.2.0a17)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// i32: (3.4.0 - 2017.4.40f1)
    pub m_LightmapFormat: Option<i32>,
    /// i32: (4.5.0 - 2017.4.40f1)
    pub m_Mipmaps: Option<i32>,
    /// PPtr<[`ProceduralMaterial`]>: (3.4.0 - 2017.4.40f1)
    pub m_SubstanceMaterial: Option<PPtr>,
    /// u64: (3.4.0 - 2017.4.40f1)
    pub m_SubstanceTextureUID: Option<u64>,
    /// TextureParameters: (3.4.0 - 2017.4.40f1)
    pub m_TextureParameters: Option<TextureParameters>,
    /// GLTextureSettings: (3.4.0 - 2017.4.40f1)
    pub m_TextureSettings: Option<GLTextureSettings>,
}

/// ProceduralTextureAssignment is a sub class of the Unity engine since version 4.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProceduralTextureAssignment {
    pub baseUID: u32,
    /// PPtr<[`ProceduralMaterial`]>: (4.5.0 - 2017.4.40f1)
    pub material: PPtr,
    pub shaderProp: Enum_FastPropertyName__String,
}

/// Projector is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Projector.html):
/**
A script interface for a projector component.
The Projector can be used to project any material onto the Scene - just like a real world projector.
The properties exposed by this class are an exact match for the values in the Projector's inspector.It can be used to implement blob or projected shadows. You could also project an animated texture or
a render texture that films another part of the Scene. The projector will render all objects in
its view frustum with the provided material.There is no shortcut property in GameObject or Component to access the Projector, so you must
use Component.GetComponent to do it:
Additional resources: projector component.
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Which object layers are ignored by the projector.*/
    pub m_IgnoreLayers: BitField,
    /**The material that will be projected onto every object.*/
    /// PPtr<[`Material`]>: (3.4.0 - 6000.2.0a6)
    pub m_Material: PPtr,
    /**The near clipping plane distance.*/
    pub m_NearClipPlane: f32,
    /**Is the projection orthographic (true) or perspective (false)?*/
    pub m_Orthographic: bool,
    /**Projection's half-size when in orthographic mode.*/
    pub m_OrthographicSize: f32,
}

/// PropertyModification is a sub class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PropertyModification.html):
/**
Defines a single modified property.
Used by the Prefab system to track any changes applied to an instance.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyModification {
    /**The value being applied when it is an object reference (which can not be represented as a string).*/
    /// PPtr<[`Object`]>: (3.5.0 - 6000.2.0a6)
    pub objectReference: PPtr,
    /**Property path of the property being modified (Matches as SerializedProperty.propertyPath).*/
    pub propertyPath: String,
    /**Object that will be modified.*/
    /// PPtr<[`Object`]>: (3.5.0 - 6000.2.0a6)
    pub target: PPtr,
    /**The value being applied.*/
    pub value: String,
}

/// PropertyModificationsTargetTestNativeObject is a sub class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyModificationsTargetTestNativeObject {
    pub m_FloatValue: f32,
    pub m_IntegerValue: i32,
}

/// PropertyModificationsTargetTestObject is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyModificationsTargetTestObject {
    pub m_Array: Vec<PropertyModificationsTargetTestNativeObject>,
    pub m_Data: PropertyModificationsTargetTestNativeObject,
    pub m_FloatTestValue: f32,
    /// Vec<u8>: (2020.3.24f1 - 2020.3.24f1)
    #[serde(alias = "byte data")]
    pub byte_data: Option<Vec<u8>>,
    /// Vec<u8>: (2019.3.0b1 - 2020.3.24f1)
    pub m_Bytes: Option<Vec<u8>>,
    /// u32: (2019.3.0b1 - 2020.3.24f1)
    pub m_BytesSize: Option<u32>,
    /// Vec<f32>: (2019.3.0b1 - 2020.3.24f1)
    pub m_Floats: Option<Vec<f32>>,
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
    /// bool: (2023.2.0b1 - 6000.2.0a6)
    pub adaptiveVsync: Option<bool>,
    /// i32: (2023.2.0b1 - 6000.2.0a6)
    pub adaptiveVsyncExtraA: Option<i32>,
    /// i32: (2023.2.0b1 - 6000.2.0a6)
    pub adaptiveVsyncExtraB: Option<i32>,
    /// i32: (5.3.0f1 - 6000.2.0a6)
    pub asyncUploadBufferSize: Option<i32>,
    /// bool: (2018.3.0f1 - 6000.2.0a6)
    pub asyncUploadPersistentBuffer: Option<bool>,
    /// i32: (5.3.0f1 - 6000.2.0a6)
    pub asyncUploadTimeSlice: Option<i32>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub billboardsFaceCameraPosition: Option<bool>,
    /// i32: (3.4.0 - 2018.4.36f1)
    pub blendWeights: Option<i32>,
    /// PPtr<[`MonoBehaviour`]>: (2019.3.0b1 - 6000.2.0a6)
    pub customRenderPipeline: Option<PPtr>,
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub enableLODCrossFade: Option<bool>,
    /// i32: (2022.2.0f1 - 6000.2.0a6)
    pub globalTextureMipmapLimit: Option<i32>,
    /// f32: (3.5.0 - 6000.2.0a6)
    pub lodBias: Option<f32>,
    /// i32: (3.5.0 - 6000.2.0a6)
    pub maximumLODLevel: Option<i32>,
    /// String: (3.5.0 - 6000.2.0a6)
    pub name: Option<String>,
    /// i32: (4.0.0 - 6000.2.0a6)
    pub particleRaycastBudget: Option<i32>,
    /// i32: (2021.3.29f1 - 6000.2.0a6)
    pub realtimeGICPUUsage: Option<i32>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub realtimeReflectionProbes: Option<bool>,
    /// f32: (2017.1.0b1 - 6000.2.0a6)
    pub resolutionScalingFixedDPIFactor: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub shadowCascade2Split: Option<f32>,
    /// Vector3f: (5.0.0f4 - 6000.2.0a6)
    pub shadowCascade4Split: Option<Vector3f>,
    /// f32: (5.2.0f2 - 6000.2.0a6)
    pub shadowNearPlaneOffset: Option<f32>,
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub shadowmaskMode: Option<i32>,
    /// i32: (2019.1.0b1 - 6000.2.0a6)
    pub skinWeights: Option<i32>,
    /// bool: (2018.2.0b1 - 6000.2.0a6)
    pub streamingMipmapsActive: Option<bool>,
    /// bool: (2018.2.0b1 - 6000.2.0a6)
    pub streamingMipmapsAddAllCameras: Option<bool>,
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub streamingMipmapsMaxFileIORequests: Option<i32>,
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub streamingMipmapsMaxLevelReduction: Option<i32>,
    /// f32: (2018.2.0b1 - 6000.2.0a6)
    pub streamingMipmapsMemoryBudget: Option<f32>,
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub streamingMipmapsRenderersPerFrame: Option<i32>,
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub terrainBasemapDistance: Option<f32>,
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub terrainBillboardStart: Option<f32>,
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub terrainDetailDensityScale: Option<f32>,
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub terrainDetailDistance: Option<f32>,
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub terrainFadeLength: Option<f32>,
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub terrainMaxTrees: Option<i32>,
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub terrainPixelError: Option<f32>,
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub terrainQualityOverrides: Option<i32>,
    /// f32: (2022.2.0b1 - 6000.2.0a6)
    pub terrainTreeDistance: Option<f32>,
    /// Vec<MipmapLimitSettings>: (2022.2.0f1 - 6000.2.0a6)
    pub textureMipmapLimitSettings: Option<Vec<MipmapLimitSettings>>,
    /// i32: (3.4.0 - 2023.1.0a6)
    pub textureQuality: Option<i32>,
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub useLegacyDetailDistribution: Option<bool>,
}

/// QualitySettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/QualitySettings.html):
/**
This represents the script interface for Quality Settings.
Use the QualitySettings class to change the current quality level at runtime. You can check the details of quality settings in your project's Quality Settings.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySettings {
    /// QualitySetting: (3.4.0 - 3.4.2)
    pub Beautiful: Option<QualitySetting>,
    /// QualitySetting: (3.4.0 - 3.4.2)
    pub Fantastic: Option<QualitySetting>,
    /// QualitySetting: (3.4.0 - 3.4.2)
    pub Fast: Option<QualitySetting>,
    /// QualitySetting: (3.4.0 - 3.4.2)
    pub Fastest: Option<QualitySetting>,
    /// QualitySetting: (3.4.0 - 3.4.2)
    pub Good: Option<QualitySetting>,
    /// QualitySetting: (3.4.0 - 3.4.2)
    pub Simple: Option<QualitySetting>,
    /// i32: (3.5.0 - 6000.2.0a6)
    pub m_CurrentQuality: Option<i32>,
    /// i32: (3.4.0 - 3.4.2)
    pub m_DefaultMobileQuality: Option<i32>,
    /// i32: (3.4.0 - 3.4.2)
    pub m_DefaultStandaloneQuality: Option<i32>,
    /// i32: (3.4.0 - 3.4.2)
    pub m_DefaultWebPlayerQuality: Option<i32>,
    /// i32: (3.4.0 - 3.4.2)
    pub m_EditorQuality: Option<i32>,
    /// Vec<QualitySetting>: (3.5.0 - 6000.2.0a6)
    pub m_QualitySettings: Option<Vec<QualitySetting>>,
    /// i32: (3.5.0 - 6000.2.0a6)
    pub m_StrippedMaximumLODLevel: Option<i32>,
    /// Vec<String>: (2022.2.0f1 - 6000.2.0a6)
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

/// RationalTime is a sub class of the Unity engine since version 2023.1.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Unity.IntegerTime.RationalTime.html):
/**
Data type that represents time as an integer count of a rational number.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RationalTime {
    /**Returns the number of ticks.*/
    pub m_Count: i64,
    pub m_Rate: TicksPerSecond,
}

/// RayTracingShader is a  class of the Unity engine since version 2019.3.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rendering.RayTracingShader.html):
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
    /// bool: (6000.0.17f1 - 6000.2.0a6)
    pub m_EnableRayPayloadSizeChecks: Option<bool>,
}

/// RayTracingShaderBuiltinSampler is a sub class of the Unity engine since version 2019.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderBuiltinSampler {
    pub bindPoint: i32,
    pub sampler: u32,
}

/// RayTracingShaderConstantBuffer is a sub class of the Unity engine since version 2019.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderConstantBuffer {
    pub byteSize: i32,
    pub name: String,
    pub params: Vec<RayTracingShaderParam>,
    /// u32: (2020.1.0f1 - 6000.2.0a6)
    pub hash: Option<u32>,
}

/// RayTracingShaderFunctionDesc is a sub class of the Unity engine since version 2019.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderFunctionDesc {
    pub attributeSizeInBytes: u32,
    pub identifier: RayTracingShaderID,
    pub payloadSizeInBytes: u32,
}

/// RayTracingShaderID is a sub class of the Unity engine since version 2019.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderID {
    pub name: String,
    /// i32: (2019.3.0b1 - 6000.2.0a6)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// RayTracingShaderImporter is a  class of the Unity engine since version 2019.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2019.3.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
    /// u32: (2019.3.0b1 - 2020.3.48f1)
    pub m_CurrentAPIMask: Option<u32>,
}

/// RayTracingShaderParam is a sub class of the Unity engine since version 2019.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderParam {
    pub arraySize: i64,
    pub colCount: i32,
    pub name: String,
    pub offset: i64,
    pub rowCount: i32,
    /// i32: (2019.3.0b1 - 2020.1.0a16)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// u32: (2020.1.0b1 - 6000.2.0a6)
    pub dataSize: Option<u32>,
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub dataType: Option<i32>,
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub propertySheetType: Option<i32>,
}

/// RayTracingShaderReflectionData is a sub class of the Unity engine since version 2019.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderReflectionData {
    pub code: Vec<u8>,
    pub functions: Vec<RayTracingShaderFunctionDesc>,
    pub globalResources: RayTracingShaderResources,
    pub hasErrors: bool,
    pub localResources: RayTracingShaderResources,
    /// Vec<u8>: (2022.2.0b1 - 6000.2.0a6)
    pub precompiled: Option<Vec<u8>>,
    /// i64: (2023.2.0b1 - 6000.2.0a6)
    pub requirements: Option<i64>,
}

/// RayTracingShaderResource is a sub class of the Unity engine since version 2019.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderResource {
    pub bindPoint: i32,
    pub name: String,
    pub rayGenMask: u64,
    pub samplerBindPoint: i32,
    pub texDimension: i32,
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub arraySize: Option<i32>,
    /// bool: (2020.1.0f1 - 6000.2.0a6)
    pub multisampled: Option<bool>,
}

/// RayTracingShaderResources is a sub class of the Unity engine since version 2019.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderResources {
    pub builtinSamplers: Vec<RayTracingShaderBuiltinSampler>,
    pub constantBuffers: Vec<RayTracingShaderResource>,
    pub constantBuffersDesc: Vec<RayTracingShaderConstantBuffer>,
    pub inputBuffers: Vec<RayTracingShaderResource>,
    pub outputBuffers: Vec<RayTracingShaderResource>,
    pub textures: Vec<RayTracingShaderResource>,
}

/// RayTracingShaderVariant is a sub class of the Unity engine since version 2019.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RayTracingShaderVariant {
    pub resourceReflectionData: RayTracingShaderReflectionData,
    pub targetRenderer: i32,
    /// bool: (6000.1.0f1 - 6000.2.0a6)
    pub editorOnlyVariant: Option<bool>,
}

/// RaycastCollider is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct RaycastCollider {
    pub m_Center: Vector3f,
    pub m_Enabled: bool,
    /// PPtr<[`GameObject`]>: (3.4.0 - 4.7.2)
    pub m_GameObject: PPtr,
    pub m_IsTrigger: bool,
    pub m_Length: f32,
    /// PPtr<[`PhysicMaterial`]>: (3.4.0 - 4.7.2)
    pub m_Material: PPtr,
}

/// RectTransform is a  class of the Unity engine since version 4.5.0.
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
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.5.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The normalized position in this RectTransform that it rotates around.*/
    pub m_Pivot: Vector2f,
    /**The size of this RectTransform relative to the distances between the anchors.*/
    pub m_SizeDelta: Vector2f,
    /**The position of the pivot of this RectTransform relative to the anchor reference point.*/
    /// Vector2f: (4.6.0 - 6000.2.0a6)
    pub m_AnchoredPosition: Option<Vector2f>,
    /// Vec<PPtr<[`Transform`]>>: (4.6.0 - 6000.2.0a6)
    pub m_Children: Option<Vec<PPtr>>,
    /// PPtr<[`Transform`]>: (4.6.0 - 6000.2.0a6)
    pub m_Father: Option<PPtr>,
    /**Position of the transform relative to the parent transform.*/
    /// Vector3f: (4.6.0 - 6000.2.0a6)
    pub m_LocalPosition: Option<Vector3f>,
    /**The rotation of the transform relative to the transform rotation of the parent.*/
    /// Quaternionf: (4.6.0 - 6000.2.0a6)
    pub m_LocalRotation: Option<Quaternionf>,
    /**The scale of the transform relative to the GameObjects parent.*/
    /// Vector3f: (4.6.0 - 6000.2.0a6)
    pub m_LocalScale: Option<Vector3f>,
    /**The world space position of the Transform.*/
    /// Vector2f: (4.5.0 - 4.5.5)
    pub m_Position: Option<Vector2f>,
}

/// Rectf is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Rectf {
    pub height: f32,
    pub width: f32,
    pub x: f32,
    pub y: f32,
}

/// ReferencesArtifactGenerator is a  class of the Unity engine since version 2019.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferencesArtifactGenerator {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2019.2.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// ReflectionProbe is a  class of the Unity engine since version 5.0.0f4.
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
    /// PPtr<[`Texture`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_BakedTexture: PPtr,
    pub m_BoxOffset: Vector3f,
    /**Should this reflection probe use box projection?*/
    pub m_BoxProjection: bool,
    pub m_BoxSize: Vector3f,
    /**How the reflection probe clears the background.*/
    pub m_ClearFlags: u32,
    /**This is used to render parts of the reflecion probe's surrounding selectively.*/
    pub m_CullingMask: BitField,
    /**Reference to the baked texture of the reflection probe's surrounding. Use this to assign custom reflection texture.*/
    /// PPtr<[`Texture`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_CustomBakedTexture: PPtr,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    pub m_FarClip: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Should this reflection probe use HDR rendering?*/
    pub m_HDR: bool,
    /**Reflection probe importance.*/
    pub m_Importance: i32,
    pub m_IntensityMultiplier: f32,
    /**Should reflection probe texture be generated in the Editor (ReflectionProbeMode.Baked) or should probe use custom specified texure (ReflectionProbeMode.Custom)?*/
    pub m_Mode: i32,
    pub m_NearClip: f32,
    /**Sets the way the probe will refresh.Additional resources: ReflectionProbeRefreshMode.*/
    pub m_RefreshMode: i32,
    /**Specifies whether Unity should render non-static GameObjects into the Reflection Probe. If you set this to true, Unity renders non-static GameObjects into the Reflection Probe. If you set this to false, Unity does not render non-static GameObjects into the Reflection Probe. Unity only takes this property into account if the Reflection Probe's Type is Custom.*/
    pub m_RenderDynamicObjects: bool,
    /**Resolution of the underlying reflection texture in pixels.*/
    pub m_Resolution: i32,
    /**Shadow drawing distance when rendering the probe.*/
    pub m_ShadowDistance: f32,
    /**Sets this probe time-slicing modeAdditional resources: ReflectionProbeTimeSlicingMode.*/
    pub m_TimeSlicingMode: i32,
    pub m_Type: i32,
    pub m_UpdateFrequency: i32,
    pub m_UseOcclusionCulling: bool,
    /**Distance around probe used for blending (used in deferred probes).*/
    /// f32: (5.2.0f2 - 6000.2.0a6)
    pub m_BlendDistance: Option<f32>,
}

/// RelativeJoint2D is a  class of the Unity engine since version 5.3.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/RelativeJoint2D.html):
/**
Keeps two Rigidbody2D at their relative orientations.
Two Rigidbody2D connected together with this joint will have forces applied to them to keep them both at their relative linear and angular offsets.  If the joint is not connected to another Rigidbody2D then the body with the joint will stay at its current linear and angular offset in world-space i.e. it will be anchored to the implicit static ground-body.You control the maximum linear force applied to maintain the linearOffset by using maxForce.You control the maximum torque applied to maintain the angularOffset by using maxTorqueAdditional resources: linearOffset, angularOffset, maxForce, maxTorque.
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
    /// PPtr<[`Rigidbody2D`]>: (5.3.0f1 - 6000.2.0a6)
    pub m_ConnectedRigidBody: PPtr,
    /**Scales both the linear and angular forces used to correct the required relative orientation.*/
    pub m_CorrectionScale: f32,
    /**Should the two Rigidbody2D connected with this joint collide with each other?*/
    pub m_EnableCollision: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (5.3.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The current linear offset between the Rigidbody2D that the joint connects.*/
    pub m_LinearOffset: Vector2f,
    /**The maximum force that can be generated when trying to maintain the relative joint constraint.*/
    pub m_MaxForce: f32,
    /**The maximum torque that can be generated when trying to maintain the relative joint constraint.*/
    pub m_MaxTorque: f32,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_BreakAction: Option<i32>,
}

/// RenderPassAttachment is a  class of the Unity engine since version 2017.2.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderPassAttachment {}

/// RenderPassInfo is a sub class of the Unity engine since version 6000.0.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderPassInfo {
    pub attachmentCount: i32,
    pub attachments: Vec<AttachmentInfo>,
    pub depthAttachmentIndex: i32,
    pub multiviewCount: i32,
    pub sampleCount: i32,
    pub shadingRateIndex: i32,
    pub subPassCount: i32,
    pub subPasses: Vec<SubPassDescriptor>,
    /// i32: (6000.1.0b1 - 6000.2.0a6)
    pub foveationImageIndex: Option<i32>,
}

/// RenderSettings is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Experimental.GlobalIllumination.RenderSettings.html):
/**
Experimental render settings features.
Additional resources: RenderSettings.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderSettings {
    pub m_FlareStrength: f32,
    pub m_Fog: bool,
    pub m_FogColor: ColorRGBA,
    pub m_FogDensity: f32,
    pub m_FogMode: i32,
    pub m_HaloStrength: f32,
    /// PPtr<[`Texture2D`]>: (3.4.0 - 6000.2.0a6)
    pub m_HaloTexture: PPtr,
    pub m_LinearFogEnd: f32,
    pub m_LinearFogStart: f32,
    /// PPtr<[`Material`]>: (3.4.0 - 6000.2.0a6)
    pub m_SkyboxMaterial: PPtr,
    /// PPtr<[`Texture2D`]>: (3.4.0 - 6000.2.0a6)
    pub m_SpotCookie: PPtr,
    /// ColorRGBA: (5.0.0f4 - 6000.2.0a6)
    pub m_AmbientEquatorColor: Option<ColorRGBA>,
    /// ColorRGBA: (5.0.0f4 - 6000.2.0a6)
    pub m_AmbientGroundColor: Option<ColorRGBA>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_AmbientIntensity: Option<f32>,
    /// ColorRGBA: (3.4.0 - 4.7.2)
    pub m_AmbientLight: Option<ColorRGBA>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_AmbientMode: Option<i32>,
    /// SphericalHarmonicsL2: (5.0.0f4 - 6000.2.0a6)
    pub m_AmbientProbe: Option<SphericalHarmonicsL2>,
    /// SphericalHarmonicsL2: (5.0.0f4 - 5.2.5f1)
    pub m_AmbientProbeInGamma: Option<SphericalHarmonicsL2>,
    /// ColorRGBA: (5.0.0f4 - 6000.2.0a6)
    pub m_AmbientSkyColor: Option<ColorRGBA>,
    /// PPtr<[`Cubemap`]>: (5.0.0f4 - 2021.1.28f1); PPtr<[`Texture`]>: (2021.2.0b1 - 6000.2.0a6)
    pub m_CustomReflection: Option<PPtr>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_DefaultReflectionMode: Option<i32>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_DefaultReflectionResolution: Option<i32>,
    /// f32: (4.3.0 - 6000.2.0a6)
    pub m_FlareFadeSpeed: Option<f32>,
    /// PPtr<[`Cubemap`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_GeneratedSkyboxReflection: Option<PPtr>,
    /// ColorRGBA: (5.4.0f3 - 6000.2.0a6)
    pub m_IndirectSpecularColor: Option<ColorRGBA>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_ReflectionBounces: Option<i32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_ReflectionIntensity: Option<f32>,
    /// ColorRGBA: (5.6.0f1 - 6000.2.0a6)
    pub m_SubtractiveShadowColor: Option<ColorRGBA>,
    /// PPtr<[`Light`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_Sun: Option<PPtr>,
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub m_UseRadianceAmbientProbe: Option<bool>,
}

/// RenderStateBlock is a sub class of the Unity engine since version 6000.0.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rendering.RenderStateBlock.html):
/**
A set of values that Unity uses to override the GPU's render state.
When you call ScriptableRenderContext.DrawRenderers, you can use this to override the render state for some or all of the geometry.Note: You must set mask to tell Unity which parts of the render state to override to apply. For example, to apply the values in blendState, mask must include RenderStateMask.Blend.
Additional resources: ScriptableRenderContext.DrawRenderers, RenderStateMask.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderStateBlock {
    /**Specifies the new blend state.*/
    pub blendState: GfxBlendState,
    /**Specifies the new depth state.*/
    pub depthState: GfxDepthState,
    /**Specifies which parts of the GPU's render state to override.*/
    pub mask: i32,
    /**Specifies the new raster state.*/
    pub rasterState: GfxRasterState,
    pub stencilRef: i32,
    /**Specifies the new stencil state.*/
    pub stencilState: GfxStencilState,
}

/// RenderStateInfo is a sub class of the Unity engine since version 6000.0.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderStateInfo {
    pub renderState: RenderStateBlock,
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
as they will not be garbage collected like normal managed types.A render texture only has a data representation on the GPU and you need to use Texture2D.ReadPixels to transfer its contents to CPU memory.The initial contents of a newly created render texture are undefined. On some platforms and APIs the contents will default to black, but you shouldn't depend on this. You can use LoadStoreActionDebugModeSettings to highlight undefined areas of the display, to help you debug rendering problems in your built application.Additional resources: Camera.targetTexture.
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
    /// i32: (4.2.0 - 6000.2.0a6)
    pub m_AntiAliasing: Option<i32>,
    /// bool: (2017.3.0b1 - 6000.2.0a6)
    pub m_BindMS: Option<bool>,
    /// i32: (3.4.0 - 2021.2.0a17)
    pub m_DepthFormat: Option<i32>,
    /**The format of the depth/stencil buffer.*/
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_DepthStencilFormat: Option<i32>,
    /**Dimensionality (type) of the Texture (Read Only).*/
    /// i32: (5.6.0b1 - 6000.2.0a6)
    pub m_Dimension: Option<i32>,
    /// bool: (2017.3.0b1 - 2023.2.0a17)
    pub m_DownscaleFallback: Option<bool>,
    /// bool: (2019.1.0b1 - 6000.2.0a6)
    pub m_EnableCompatibleFormat: Option<bool>,
    /**Enable random access write into this render texture on Shader Model 5.0 level shaders.*/
    /// bool: (2021.3.29f1 - 6000.2.0a6)
    pub m_EnableRandomWrite: Option<bool>,
    /// i32: (2017.3.0b1 - 2023.2.0a17)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (4.3.0 - 6000.2.0a6)
    pub m_GenerateMips: Option<bool>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// bool: (3.4.0 - 3.5.7)
    pub m_IsCubemap: Option<bool>,
    /// bool: (3.4.0 - 3.4.0)
    pub m_IsPowerOfTwo: Option<bool>,
    /// i32: (2019.2.0b1 - 6000.2.0a6)
    pub m_MipCount: Option<i32>,
    /**Does this render texture use sRGB read/write conversions? (Read Only).*/
    /// bool: (3.5.0 - 6000.2.0a6)
    pub m_SRGB: Option<bool>,
    /// i32: (2021.2.0f1 - 6000.2.0a6)
    pub m_ShadowSamplingMode: Option<i32>,
    /**When this flag is set to true, render texture is set to be used by the Dynamic Resolution system.*/
    /// bool: (2017.3.0b1 - 6000.2.0a6)
    pub m_UseDynamicScale: Option<bool>,
    /**When this flag is set to true, render texture is set to be used by the Dynamic Resolution system. Scale is applied with an explicit call to ApplyDynamicScale*/
    /// bool: (2023.2.0b1 - 6000.2.0a6)
    pub m_UseDynamicScaleExplicit: Option<bool>,
    /**Volume extent of a 3D render texture or number of slices of array texture.*/
    /// i32: (5.6.0b1 - 6000.2.0a6)
    pub m_VolumeDepth: Option<i32>,
}

/// Renderer is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Renderer.html):
/**
General functionality for all renderers.
A renderer is what makes an object appear on the screen. Use this class to access the renderer of any object, mesh or Particle System.
Renderers can be disabled to make objects invisible (see enabled), and the materials can be accessed
and modified through them (see material).Additional resources: Renderer components for meshes, particles,
lines and trails.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Renderer {
    pub m_CastShadows: bool,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 4.7.2)
    pub m_GameObject: PPtr,
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u8,
    pub m_LightmapTilingOffset: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    /// Vec<PPtr<[`Material`]>>: (3.4.0 - 4.7.2)
    pub m_Materials: Vec<PPtr>,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: bool,
    /// PPtr<[`Transform`]>: (3.4.0 - 4.7.2)
    pub m_StaticBatchRoot: PPtr,
    pub m_SubsetIndices: Vec<u32>,
    /// PPtr<[`Transform`]>: (3.5.0 - 4.7.2)
    pub m_LightProbeAnchor: Option<PPtr>,
    /// i16: (4.3.0 - 4.3.4)
    pub m_SortingLayer: Option<i16>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// u32: (4.5.0 - 4.7.2)
    pub m_SortingLayerID: Option<u32>,
    /**Renderer's order within a sorting layer.*/
    /// i16: (4.3.0 - 4.7.2)
    pub m_SortingOrder: Option<i16>,
    /// bool: (3.5.0 - 4.7.2)
    pub m_UseLightProbes: Option<bool>,
}

/// RendererData is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct RendererData {
    pub lightmapIndex: u16,
    pub lightmapIndexDynamic: u16,
    pub lightmapST: Vector4f,
    pub lightmapSTDynamic: Vector4f,
    pub terrainChunkDynamicUVST: Vector4f,
    pub terrainDynamicUVST: Vector4f,
    /// PPtr<[`Mesh`]>: (5.0.0f4 - 6000.2.0a6)
    pub uvMesh: PPtr,
    /// Hash128: (2018.2.0b1 - 6000.2.0a6)
    pub explicitProbeSetHash: Option<Hash128>,
}

/// ResourceManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceManager {
    /// Vec<(String, PPtr<[`Object`]>)>: (3.4.0 - 6000.2.0a6)
    pub m_Container: Vec<(String, PPtr)>,
    /// Vec<ResourceManager_Dependency>: (3.5.0 - 6000.2.0a6)
    pub m_DependentAssets: Option<Vec<ResourceManager_Dependency>>,
}

/// ResourceManager_Dependency is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceManager_Dependency {
    /// Vec<PPtr<[`Object`]>>: (3.5.0 - 6000.2.0a6)
    pub m_Dependencies: Vec<PPtr>,
    /// PPtr<[`Object`]>: (3.5.0 - 6000.2.0a6)
    pub m_Object: PPtr,
}

/// Rigidbody is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rigidbody.html):
/**
Control of an object's position through physics simulation.
Adding a Rigidbody component to an object will put its motion under the control of Unity's physics engine. Even without adding any code, a Rigidbody object will be pulled downward by gravity and will react to collisions with incoming objects if the right Collider component is also present.The Rigidbody also has a scripting API that lets you apply forces to the object and control it in a physically realistic way. For example, a car's behaviour can be specified in terms of the forces applied by the wheels. Given this information, the physics engine can handle most other aspects of the car's motion, so it will accelerate realistically and respond correctly to collisions.In a script, the FixedUpdate function is recommended as the place to apply forces and change Rigidbody settings (as opposed to Update, which is used for most other frame update tasks). The reason for this is that physics updates are carried out in measured time steps that don't coincide with the frame update. FixedUpdate is called immediately before each physics update and so any changes made there will be processed directly.A common problem when starting out with Rigidbodies is that the game physics appears to run in "slow motion". This is actually due to the scale used for your models. The default gravity settings assume that one world unit corresponds to one metre of distance. With non-physical games, it doesn't make much difference if your models are all 100 units long but when using physics, they will be treated as very large objects. If a large scale is used for objects that are supposed to be small, they will appear to fall very slowly - the physics engine thinks they are very large objects falling over very large distances. With this in mind, be sure to keep your objects more or less at their scale in real life (so a car should be about 4 units = 4 metres, for example).
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Rigidbody {
    pub m_CollisionDetection: i32,
    /**Controls which degrees of freedom are allowed for the simulation of this Rigidbody.*/
    pub m_Constraints: i32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub m_Interpolate: u8,
    /**Controls whether physics affects the rigidbody.*/
    pub m_IsKinematic: bool,
    /**The mass of the rigidbody.*/
    pub m_Mass: f32,
    /**Controls whether gravity affects this rigidbody.*/
    pub m_UseGravity: bool,
    /**The angular damping of the object.*/
    /// f32: (6000.1.0b1 - 6000.2.0a6)
    pub m_AngularDamping: Option<f32>,
    /// f32: (3.4.0 - 6000.1.0a3)
    pub m_AngularDrag: Option<f32>,
    /**The center of mass relative to the transform's origin.*/
    /// Vector3f: (2022.2.0b1 - 6000.2.0a6)
    pub m_CenterOfMass: Option<Vector3f>,
    /// f32: (3.4.0 - 6000.1.0a3)
    pub m_Drag: Option<f32>,
    /**The additional layers that all Colliders attached to this Rigidbody should exclude when deciding if the Collider can come into contact with another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_ImplicitCom: Option<bool>,
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_ImplicitTensor: Option<bool>,
    /**The additional layers that all Colliders attached to this Rigidbody should include when deciding if the Collider can come into contact with another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /// Quaternionf: (2022.2.0b1 - 6000.2.0a6)
    pub m_InertiaRotation: Option<Quaternionf>,
    /**The inertia tensor of this body, defined as a diagonal matrix in a reference frame positioned at this body's center of mass and rotated by Rigidbody.inertiaTensorRotation.*/
    /// Vector3f: (2022.2.0b1 - 6000.2.0a6)
    pub m_InertiaTensor: Option<Vector3f>,
    /**The linear damping of the object.*/
    /// f32: (6000.1.0b1 - 6000.2.0a6)
    pub m_LinearDamping: Option<f32>,
}

/// Rigidbody2D is a  class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rigidbody2D.html):
/**
Provides physics movement and other dynamics, and the ability to attach Collider2D to it.
The Rigidbody2D is a fundamental physics component that provides multiple simulation dynamics, such as Rigidbody2D.position and Rigidbody2D.rotation for pose control, and Rigidbody2D.linearVelocity and Rigidbody2D.angularVelocity for velocity control.You can attach multiple Collider2D to a Rigidbody2D to detect collisions and provide a collision response when you set Rigidbody2D.bodyType to RigidbodyType2D.Dynamic.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Rigidbody2D {
    pub m_CollisionDetection: i32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.3.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The degree to which this object is affected by gravity.*/
    pub m_GravityScale: f32,
    pub m_Interpolate: i32,
    /**Mass of the Rigidbody.*/
    pub m_Mass: f32,
    pub m_SleepingMode: i32,
    /**The angular damping of the Rigidbody2D angular velocity.*/
    /// f32: (6000.0.11f1 - 6000.2.0a6)
    pub m_AngularDamping: Option<f32>,
    /// f32: (4.3.0 - 6000.0.10f1)
    pub m_AngularDrag: Option<f32>,
    /**The physical behaviour type of the Rigidbody2D.*/
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_BodyType: Option<i32>,
    /**Controls which degrees of freedom are allowed for the simulation of this Rigidbody2D.*/
    /// i32: (5.1.0f1 - 6000.2.0a6)
    pub m_Constraints: Option<i32>,
    /**The additional Layers that all Collider2D attached to this Rigidbody2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /// bool: (4.3.0 - 5.0.4f1)
    pub m_FixedAngle: Option<bool>,
    /**The additional Layers that all Collider2D attached to this Rigidbody2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /// bool: (4.3.0 - 5.4.6f3)
    pub m_IsKinematic: Option<bool>,
    /**The linear damping of the Rigidbody2D linear velocity.*/
    /// f32: (6000.0.11f1 - 6000.2.0a6)
    pub m_LinearDamping: Option<f32>,
    /// f32: (4.3.0 - 6000.0.10f1)
    pub m_LinearDrag: Option<f32>,
    /// PPtr<[`PhysicsMaterial2D`]>: (5.5.0f3 - 6000.2.0a6)
    pub m_Material: Option<PPtr>,
    /**Indicates whether the rigid body should be simulated or not by the physics system.*/
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub m_Simulated: Option<bool>,
    /**Should the total rigid-body mass be automatically calculated from the Collider2D.density of attached colliders?*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_UseAutoMass: Option<bool>,
    /**Should kinematic/kinematic and kinematic/static collisions be allowed?*/
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub m_UseFullKinematicContacts: Option<bool>,
}

/// RippleGroup is a sub class of the Unity engine since version 2023.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RippleGroup {
    pub m_afDirectional_0: f32,
    pub m_afDirectional_1: f32,
    pub m_afDirectional_10: f32,
    pub m_afDirectional_11: f32,
    pub m_afDirectional_12: f32,
    pub m_afDirectional_13: f32,
    pub m_afDirectional_14: f32,
    pub m_afDirectional_15: f32,
    pub m_afDirectional_16: f32,
    pub m_afDirectional_17: f32,
    pub m_afDirectional_18: f32,
    pub m_afDirectional_19: f32,
    pub m_afDirectional_2: f32,
    pub m_afDirectional_3: f32,
    pub m_afDirectional_4: f32,
    pub m_afDirectional_5: f32,
    pub m_afDirectional_6: f32,
    pub m_afDirectional_7: f32,
    pub m_afDirectional_8: f32,
    pub m_afDirectional_9: f32,
    pub m_afFlexibility_0: f32,
    pub m_afFlexibility_1: f32,
    pub m_afFlexibility_10: f32,
    pub m_afFlexibility_11: f32,
    pub m_afFlexibility_12: f32,
    pub m_afFlexibility_13: f32,
    pub m_afFlexibility_14: f32,
    pub m_afFlexibility_15: f32,
    pub m_afFlexibility_16: f32,
    pub m_afFlexibility_17: f32,
    pub m_afFlexibility_18: f32,
    pub m_afFlexibility_19: f32,
    pub m_afFlexibility_2: f32,
    pub m_afFlexibility_3: f32,
    pub m_afFlexibility_4: f32,
    pub m_afFlexibility_5: f32,
    pub m_afFlexibility_6: f32,
    pub m_afFlexibility_7: f32,
    pub m_afFlexibility_8: f32,
    pub m_afFlexibility_9: f32,
    pub m_afPlanar_0: f32,
    pub m_afPlanar_1: f32,
    pub m_afPlanar_10: f32,
    pub m_afPlanar_11: f32,
    pub m_afPlanar_12: f32,
    pub m_afPlanar_13: f32,
    pub m_afPlanar_14: f32,
    pub m_afPlanar_15: f32,
    pub m_afPlanar_16: f32,
    pub m_afPlanar_17: f32,
    pub m_afPlanar_18: f32,
    pub m_afPlanar_19: f32,
    pub m_afPlanar_2: f32,
    pub m_afPlanar_3: f32,
    pub m_afPlanar_4: f32,
    pub m_afPlanar_5: f32,
    pub m_afPlanar_6: f32,
    pub m_afPlanar_7: f32,
    pub m_afPlanar_8: f32,
    pub m_afPlanar_9: f32,
    pub m_afSpeed_0: f32,
    pub m_afSpeed_1: f32,
    pub m_afSpeed_10: f32,
    pub m_afSpeed_11: f32,
    pub m_afSpeed_12: f32,
    pub m_afSpeed_13: f32,
    pub m_afSpeed_14: f32,
    pub m_afSpeed_15: f32,
    pub m_afSpeed_16: f32,
    pub m_afSpeed_17: f32,
    pub m_afSpeed_18: f32,
    pub m_afSpeed_19: f32,
    pub m_afSpeed_2: f32,
    pub m_afSpeed_3: f32,
    pub m_afSpeed_4: f32,
    pub m_afSpeed_5: f32,
    pub m_afSpeed_6: f32,
    pub m_afSpeed_7: f32,
    pub m_afSpeed_8: f32,
    pub m_afSpeed_9: f32,
    pub m_fIndependence: f32,
    pub m_fShimmer: f32,
}

/// RoslynAdditionalFileAsset is a  class of the Unity engine since version 2021.3.3f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoslynAdditionalFileAsset {
    pub m_Name: String,
}

/// RoslynAdditionalFileImporter is a  class of the Unity engine since version 2021.3.3f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoslynAdditionalFileImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2021.3.3f1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// RoslynAnalyzerConfigAsset is a  class of the Unity engine since version 2021.3.3f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoslynAnalyzerConfigAsset {
    pub m_Name: String,
}

/// RoslynAnalyzerConfigImporter is a  class of the Unity engine since version 2021.3.3f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoslynAnalyzerConfigImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2021.3.3f1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// RotationBySpeedModule is a sub class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.RotationBySpeedModule.html):
/**
Script interface for the RotationBySpeedModule.
Rotate particles based on their speed.Additional resources: ParticleSystem, ParticleSystem.rotationBySpeed.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RotationBySpeedModule {
    pub curve: MinMaxCurve,
    /**ESpecifies whether the RotationBySpeedModule is enabled or disabled.*/
    pub enabled: bool,
    /**Set the minimum and maximum speeds that this module applies the rotation curve between.*/
    pub range: Vector2f,
    /**Set the rotation by speed on each axis separately.*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub separateAxes: Option<bool>,
    /**Rotation by speed curve for the x-axis.*/
    /// MinMaxCurve: (5.3.0f1 - 6000.2.0a6)
    pub x: Option<MinMaxCurve>,
    /**Rotation by speed curve for the y-axis.*/
    /// MinMaxCurve: (5.3.0f1 - 6000.2.0a6)
    pub y: Option<MinMaxCurve>,
}

/// RotationConstraint is a  class of the Unity engine since version 2018.1.0f1.
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
    /// PPtr<[`GameObject`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The rotation used when the sources have a total weight of 0.*/
    pub m_RotationAtRest: Vector3f,
    /**The offset from the constrained rotation.*/
    pub m_RotationOffset: Vector3f,
    pub m_Sources: Vec<ConstraintSource>,
    /**The weight of the constraint component.*/
    pub m_Weight: f32,
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_Active: Option<bool>,
    /// bool: (2018.1.0f1 - 2022.1.0a9)
    pub m_IsContraintActive: Option<bool>,
}

/// RotationModule is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct RotationModule {
    pub curve: MinMaxCurve,
    pub enabled: bool,
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub separateAxes: Option<bool>,
    /// MinMaxCurve: (5.3.0f1 - 6000.2.0a6)
    pub x: Option<MinMaxCurve>,
    /// MinMaxCurve: (5.3.0f1 - 6000.2.0a6)
    pub y: Option<MinMaxCurve>,
}

/// RuleSetFileAsset is a  class of the Unity engine since version 2020.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleSetFileAsset {
    pub m_Name: String,
    pub m_Script: String,
}

/// RuleSetFileImporter is a  class of the Unity engine since version 2020.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleSetFileImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2020.2.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// RuntimeAnimatorController is a  class of the Unity engine since version 4.1.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/RuntimeAnimatorController.html):
/**
A representation of the Animator Controller, optimized for runtime.
At runtime, Unity replaces the AnimatorController class with this optimized runtime class. Access to Editor functions, such as modifying the structure of an Animator Controller, are restricted.This optimized class provides the following different ways to access and modify an Animator Controller at runtime:
- Store the reference of an Animator Controller so you can replace the Animator Controller of an Animator. This is useful for modifiying the structure of an Animator Controller at runtime. Use Animator.runtimeAnimatorController to access the controller to be replaced.
- Create an AnimatorOverrideController that you can use to override the Animation Clips associated with an Animator Controller. This is more efficient than replacing a controller because only the clips are updated. The Animator Override Controller is based on the Runtime Animator Controller that initializes it. Additional resources: AnimatorOverrideController.runtimeAnimatorController.
The following example demonstrates how to spawn GameObjects at runtime. Each GameObject is animated with different Animator Controllers.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeAnimatorController {
    /**Retrieves all AnimationClip used by the controller.*/
    /// Vec<PPtr<[`AnimationClip`]>>: (4.1.0 - 4.2.2)
    pub m_AnimationClips: Vec<PPtr>,
    pub m_Controller: ControllerConstant,
    pub m_ControllerSize: u32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_TOS: Vec<(u32, String)>,
}

/// RuntimeInitializeOnLoadManager is a  class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeInitializeOnLoadManager {
    /// Vec<i32>: (2019.1.0b1 - 2020.2.0a19)
    pub m_AfterAssembliesLoadedMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (2019.1.0b1 - 2020.2.0a19)
    pub m_AfterAssembliesLoadedUnityMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (5.2.0f2 - 2020.2.0a19)
    pub m_AfterMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (5.2.0f2 - 2020.2.0a19)
    pub m_AfterUnityMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<String>: (5.0.0f4 - 2020.2.0a19)
    pub m_AssemblyNames: Option<Vec<String>>,
    /// Vec<i32>: (5.2.0f2 - 2020.2.0a19)
    pub m_BeforeMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (2019.1.0b1 - 2020.2.0a19)
    pub m_BeforeSplashScreenMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (2019.1.0b1 - 2020.2.0a19)
    pub m_BeforeSplashScreenUnityMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (5.2.0f2 - 2020.2.0a19)
    pub m_BeforeUnityMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<ClassInfo>: (5.0.0f4 - 2020.2.0a19)
    pub m_ClassInfos: Option<Vec<ClassInfo>>,
    /// Vec<ClassMethodInfo>: (5.0.0f4 - 2020.2.0a19)
    pub m_ClassMethodInfos: Option<Vec<ClassMethodInfo>>,
    /// Vec<i32>: (5.0.0f4 - 5.1.5f1)
    pub m_MethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<String>: (5.0.0f4 - 2020.2.0a19)
    pub m_NamespaceNames: Option<Vec<String>>,
    /// Vec<i32>: (2019.2.0f1 - 2020.2.0a19)
    pub m_SubsystemRegistrationMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (2019.2.0f1 - 2020.2.0a19)
    pub m_SubsystemRegistrationUnityMethodExecutionOrders: Option<Vec<i32>>,
    /// Vec<i32>: (5.0.0f4 - 5.1.5f1)
    pub m_UnityMethodExecutionOrders: Option<Vec<i32>>,
}

/// SBranchWindLevel is a sub class of the Unity engine since version 5.0.0f4.
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

/// SParams is a sub class of the Unity engine since version 5.0.0f4.
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

/// SWindGroup is a sub class of the Unity engine since version 5.0.0f4.
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

/// SampleClip is a  class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct SampleClip {
    pub m_Name: String,
}

/// SampleSettings is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct SampleSettings {
    pub compressionFormat: i32,
    pub conversionMode: i32,
    pub loadType: i32,
    pub quality: f32,
    pub sampleRateOverride: u32,
    pub sampleRateSetting: i32,
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub preloadAudioData: Option<bool>,
}

/// SamplerParameter is a sub class of the Unity engine since version 2017.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SamplerParameter {
    pub bindPoint: i32,
    pub sampler: u32,
}

/// ScaleConstraint is a  class of the Unity engine since version 2018.1.0f1.
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
    /// PPtr<[`GameObject`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The scale used when the sources have a total weight of 0.*/
    pub m_ScaleAtRest: Vector3f,
    /**The offset from the constrained scale.*/
    pub m_ScaleOffset: Vector3f,
    pub m_Sources: Vec<ConstraintSource>,
    /**The weight of the constraint component.*/
    pub m_Weight: f32,
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_Active: Option<bool>,
    /// bool: (2018.1.0f1 - 2022.1.0a9)
    pub m_IsContraintActive: Option<bool>,
}

/// Scene is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SceneManagement.Scene.html):
/**
The runtime data structure for a scene.
This object can be used to query information about a scene, and as an input to various scene manipulation functions in SceneManager and EditorSceneManager.The following script prints some information about the currently loaded scene, creates and switches to a new scene, then prints the same information for the new scene. To use the script, attach it to a GameObject and run the project.
Additional resources: SceneAsset.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    /// bool: (4.0.0 - 6000.2.0a6)
    pub enabled: Option<bool>,
    /// GUID: (5.6.0f1 - 6000.2.0a6)
    pub guid: Option<GUID>,
    /// Vec<u8>: (3.4.0 - 3.5.7)
    pub m_PVSData: Option<Vec<u8>>,
    /// Vec<PPtr<[`Renderer`]>>: (3.4.0 - 3.5.7)
    pub m_PVSObjectsArray: Option<Vec<PPtr>>,
    /// Vec<PPtr<[`OcclusionPortal`]>>: (3.5.0 - 3.5.7)
    pub m_PVSPortalsArray: Option<Vec<PPtr>>,
    /// u32: (3.5.0 - 3.5.7)
    pub m_QueryMode: Option<u32>,
    /**Returns the relative path of the Scene. For example: "Assets/MyScenes/MyScene.unity".*/
    /// String: (4.0.0 - 6000.2.0a6)
    pub path: Option<String>,
}

/// SceneDataContainer is a sub class of the Unity engine since version 2019.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneDataContainer {
    pub m_SceneData: Vec<(SceneIdentifier, HierarchicalSceneData)>,
}

/// SceneIdentifier is a sub class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneIdentifier {
    pub guid: GUID,
    pub handle: i32,
}

/// SceneObjectIdentifier is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneObjectIdentifier {
    pub targetObject: i64,
    pub targetPrefab: i64,
}

/// SceneRoots is a  class of the Unity engine since version 2022.3.5f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneRoots {
    /// Vec<PPtr<[`Object`]>>: (2022.3.5f1 - 6000.2.0a6)
    pub m_Roots: Vec<PPtr>,
}

/// SceneSettings is a  class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneSettings {
    pub m_PVSData: Vec<u8>,
    /// Vec<PPtr<[`Renderer`]>>: (4.0.0 - 5.4.6f3)
    pub m_PVSObjectsArray: Vec<PPtr>,
    /// Vec<PPtr<[`OcclusionPortal`]>>: (4.0.0 - 5.4.6f3)
    pub m_PVSPortalsArray: Vec<PPtr>,
    /// u32: (4.0.0 - 4.2.2)
    pub m_QueryMode: Option<u32>,
}

/// SceneVisibilityData is a sub class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneVisibilityData {
    pub m_SceneGUID: GUID,
}

/// SceneVisibilityState is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneVisibilityState {
    /// bool: (2019.3.0b1 - 6000.2.0a6)
    pub m_IsolationMode: Option<bool>,
    /// bool: (2019.1.0b1 - 2019.3.0a8)
    pub m_MainStageIsolated: Option<bool>,
    /// bool: (2019.1.0b1 - 2019.3.0a8)
    pub m_PrefabStageIsolated: Option<bool>,
    /// Vec<(SceneIdentifier, SceneVisibilityData)>: (2019.1.0b1 - 2019.3.0a8)
    pub m_SceneData: Option<Vec<(SceneIdentifier, SceneVisibilityData)>>,
    /// SceneDataContainer: (2019.3.0b1 - 6000.2.0a6)
    pub m_ScenePickingData: Option<SceneDataContainer>,
    /// SceneDataContainer: (2019.3.0b1 - 6000.2.0a6)
    pub m_SceneVisibilityData: Option<SceneDataContainer>,
    /// SceneDataContainer: (2019.3.0b1 - 6000.2.0a6)
    pub m_SceneVisibilityDataIsolated: Option<SceneDataContainer>,
}

/// ScenesUsingAssets is a  class of the Unity engine since version 2020.1.0b1.
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
    /// bool: (5.0.0f4 - 2020.3.48f1)
    pub m_PreloadShaders: Option<bool>,
}

/// ScriptedImporter is a  class of the Unity engine since version 2017.1.0f1.
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
    /**The name of the object.*/
    pub m_Name: String,
    /// PPtr<[`MonoScript`]>: (2017.1.0f1 - 6000.2.0a6)
    pub m_Script: PPtr,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.3.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// Vec<(i64, String)>: (2017.3.0b1 - 2018.4.36f1)
    pub m_FileIDToRecycleName: Option<Vec<(i64, String)>>,
    /// Vec<((i32, i64), String)>: (2019.1.0b1 - 6000.2.0a6)
    pub m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// SecondarySpriteTexture is a sub class of the Unity engine since version 2019.1.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SecondarySpriteTexture.html):
/**
Encapsulates a Texture2D and its shader property name to give Sprite-based renderers access to a secondary texture, in addition to the main Sprite texture.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SecondarySpriteTexture {
    /**The shader property name of the secondary Sprite texture. Use this name to identify and sample the texture in the shader.*/
    pub name: String,
    /**The texture to be used as a secondary Sprite texture.*/
    /// PPtr<[`Texture2D`]>: (2019.1.0b1 - 6000.2.0a6)
    pub texture: PPtr,
}

/// SecondaryTextureSettings is a sub class of the Unity engine since version 2020.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecondaryTextureSettings {
    pub platformSettings: Vec<TextureImporterPlatformSettings>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub sRGB: Option<bool>,
}

/// SerializableManagedHost is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableManagedHost {
    /// PPtr<[`MonoScript`]>: (2019.1.0b1 - 2020.3.24f1)
    pub m_Script: PPtr,
}

/// SerializableManagedRefTestClass is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableManagedRefTestClass {
    /// PPtr<[`MonoScript`]>: (2019.1.0b1 - 2020.3.24f1)
    pub m_Script: PPtr,
}

/// SerializedCustomEditorForRenderPipeline is a sub class of the Unity engine since version 2021.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedCustomEditorForRenderPipeline {
    pub customEditorName: String,
    pub renderPipelineType: String,
}

/// SerializedPass is a sub class of the Unity engine since version 5.5.0f3.
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
    /// Vec<Hash128>: (2020.2.0b1 - 2023.1.0a8)
    pub m_EditorDataHash: Option<Vec<Hash128>>,
    /// Vec<u16>: (2020.2.0b1 - 2021.2.0a15)
    pub m_GlobalKeywordMask: Option<Vec<u16>>,
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub m_HasProceduralInstancingVariant: Option<bool>,
    /// Vec<u16>: (2020.2.0b1 - 2021.2.0a15)
    pub m_LocalKeywordMask: Option<Vec<u16>>,
    /// Vec<u8>: (2020.2.0b1 - 2023.1.0a8)
    pub m_Platforms: Option<Vec<u8>>,
    /// Vec<u16>: (2021.2.0b1 - 2022.1.0a16)
    pub m_SerializedKeywordStateMask: Option<Vec<u16>>,
    /// SerializedProgram: (2019.3.0b1 - 6000.2.0a6)
    pub progRayTracing: Option<SerializedProgram>,
}

/// SerializedPlayerSubProgram is a sub class of the Unity engine since version 2021.3.10f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedPlayerSubProgram {
    pub m_BlobIndex: u32,
    pub m_GpuProgramType: i8,
    pub m_KeywordIndices: Vec<u16>,
    pub m_ShaderRequirements: i64,
}

/// SerializedProgram is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedProgram {
    pub m_SubPrograms: Vec<SerializedSubProgram>,
    /// SerializedProgramParameters: (2020.3.2f1 - 6000.2.0a6)
    pub m_CommonParameters: Option<SerializedProgramParameters>,
    /// Vec<Vec<u32>>: (2021.3.10f1 - 6000.2.0a6)
    pub m_ParameterBlobIndices: Option<Vec<Vec<u32>>>,
    /// Vec<Vec<SerializedPlayerSubProgram>>: (2021.3.10f1 - 6000.2.0a6)
    pub m_PlayerSubPrograms: Option<Vec<Vec<SerializedPlayerSubProgram>>>,
    /// Vec<u16>: (2022.1.0f1 - 6000.2.0a6)
    pub m_SerializedKeywordStateMask: Option<Vec<u16>>,
}

/// SerializedProgramParameters is a sub class of the Unity engine since version 2020.3.2f1.
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

/// SerializedProperties is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedProperties {
    pub m_Props: Vec<SerializedProperty>,
}

/// SerializedProperty is a sub class of the Unity engine since version 5.5.0f3.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SerializedProperty.html):
/**
SerializedProperty and SerializedObject are classes for editing properties on objects in a completely generic way that automatically handles undo, multi-object editing and Prefab overrides.
SerializedProperty is primarily used to read or change the value of a property.  It can also iterate through the properties of an object using Next.
Additional resources: SerializedObject class, Editor class.
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
    /// f32: (5.5.0f3 - 6000.2.0a6)
    #[serde(alias = "m_DefValue[0]")]
    pub m_DefValue_0_: Option<f32>,
    /// f32: (5.5.0f3 - 6000.2.0a6)
    #[serde(alias = "m_DefValue[1]")]
    pub m_DefValue_1_: Option<f32>,
    /// f32: (5.5.0f3 - 6000.2.0a6)
    #[serde(alias = "m_DefValue[2]")]
    pub m_DefValue_2_: Option<f32>,
    /// f32: (5.5.0f3 - 6000.2.0a6)
    #[serde(alias = "m_DefValue[3]")]
    pub m_DefValue_3_: Option<f32>,
}

/// SerializedShader is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedShader {
    pub m_CustomEditorName: String,
    pub m_Dependencies: Vec<SerializedShaderDependency>,
    pub m_DisableNoSubshadersMessage: bool,
    pub m_FallbackName: String,
    pub m_Name: String,
    pub m_PropInfo: SerializedProperties,
    pub m_SubShaders: Vec<SerializedSubShader>,
    /// Vec<SerializedCustomEditorForRenderPipeline>: (2021.1.0b1 - 6000.2.0a6)
    pub m_CustomEditorForRenderPipelines: Option<Vec<SerializedCustomEditorForRenderPipeline>>,
    /// Vec<u8>: (2021.2.0b1 - 6000.2.0a6)
    pub m_KeywordFlags: Option<Vec<u8>>,
    /// Vec<String>: (2021.2.0b1 - 6000.2.0a6)
    pub m_KeywordNames: Option<Vec<String>>,
}

/// SerializedShaderDependency is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedShaderDependency {
    pub from: String,
    pub to: String,
}

/// SerializedShaderFloatValue is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedShaderFloatValue {
    pub name: Enum_FastPropertyName__String,
    pub val: f32,
}

/// SerializedShaderRTBlendState is a sub class of the Unity engine since version 5.5.0f3.
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

/// SerializedShaderState is a sub class of the Unity engine since version 5.5.0f3.
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
    /// SerializedShaderFloatValue: (2020.1.0b1 - 6000.2.0a6)
    pub conservative: Option<SerializedShaderFloatValue>,
    /// SerializedShaderFloatValue: (2017.2.0f1 - 6000.2.0a6)
    pub zClip: Option<SerializedShaderFloatValue>,
}

/// SerializedShaderVectorValue is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedShaderVectorValue {
    pub name: Enum_FastPropertyName__String,
    pub w: SerializedShaderFloatValue,
    pub x: SerializedShaderFloatValue,
    pub y: SerializedShaderFloatValue,
    pub z: SerializedShaderFloatValue,
}

/// SerializedStencilOp is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedStencilOp {
    pub comp: SerializedShaderFloatValue,
    pub fail: SerializedShaderFloatValue,
    pub pass: SerializedShaderFloatValue,
    pub zFail: SerializedShaderFloatValue,
}

/// SerializedSubProgram is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedSubProgram {
    pub m_BlobIndex: u32,
    pub m_Channels: ParserBindChannels,
    pub m_GpuProgramType: i8,
    pub m_ShaderHardwareTier: i8,
    /// Vec<BufferBinding>: (5.5.0f3 - 2021.1.0b12)
    pub m_BufferParams: Option<Vec<BufferBinding>>,
    /// Vec<BufferBinding>: (5.5.0f3 - 2021.1.0b12)
    pub m_ConstantBufferBindings: Option<Vec<BufferBinding>>,
    /// Vec<ConstantBuffer>: (5.5.0f3 - 2021.1.0b12)
    pub m_ConstantBuffers: Option<Vec<ConstantBuffer>>,
    /// Vec<u16>: (2019.1.0b1 - 2021.2.0a15)
    pub m_GlobalKeywordIndices: Option<Vec<u16>>,
    /// Vec<u16>: (5.5.0f3 - 6000.2.0a6)
    pub m_KeywordIndices: Option<Vec<u16>>,
    /// Vec<u16>: (2019.1.0b1 - 2021.2.0a15)
    pub m_LocalKeywordIndices: Option<Vec<u16>>,
    /// Vec<MatrixParameter>: (5.5.0f3 - 2021.1.0b12)
    pub m_MatrixParams: Option<Vec<MatrixParameter>>,
    /// SerializedProgramParameters: (2020.3.2f1 - 6000.2.0a6)
    pub m_Parameters: Option<SerializedProgramParameters>,
    /// Vec<SamplerParameter>: (2017.1.0b1 - 2021.1.0b12)
    pub m_Samplers: Option<Vec<SamplerParameter>>,
    /// i32: (2017.2.0f1 - 2020.3.48f1); i64: (2021.1.0b1 - 6000.2.0a6)
    pub m_ShaderRequirements: Option<i64>,
    /// Vec<TextureParameter>: (5.5.0f3 - 2021.1.0b12)
    pub m_TextureParams: Option<Vec<TextureParameter>>,
    /// Vec<UAVParameter>: (5.5.0f3 - 2021.1.0b12)
    pub m_UAVParams: Option<Vec<UAVParameter>>,
    /// Vec<VectorParameter>: (5.5.0f3 - 2021.1.0b12)
    pub m_VectorParams: Option<Vec<VectorParameter>>,
}

/// SerializedSubShader is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedSubShader {
    pub m_LOD: i32,
    pub m_Passes: Vec<SerializedPass>,
    pub m_Tags: SerializedTagMap,
}

/// SerializedTagMap is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedTagMap {
    pub tags: Vec<(String, String)>,
}

/// SerializedTextureProperty is a sub class of the Unity engine since version 5.5.0f3.
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
global shader properties and keywords, and finding shaders by name (Find method).Additional resources: Material class, Materials, ShaderLab documentation.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Shader {
    /**The name of the object.*/
    pub m_Name: String,
    /// Vec<u8>: (5.5.0f3 - 6000.2.0a6)
    pub compressedBlob: Option<Vec<u8>>,
    /// Vec<u32>: (5.5.0f3 - 2019.3.0a12); Vec<Vec<u32>>: (2019.3.0b1 - 6000.2.0a6)
    pub compressedLengths: Option<Vec<Enum_Vec_u32___u32>>,
    /// Vec<u32>: (5.5.0f3 - 2019.3.0a12); Vec<Vec<u32>>: (2019.3.0b1 - 6000.2.0a6)
    pub decompressedLengths: Option<Vec<Enum_Vec_u32___u32>>,
    /// u32: (5.3.0f1 - 5.4.6f3)
    pub decompressedSize: Option<u32>,
    /// GUID: (6000.0.0f1 - 6000.2.0a6)
    pub m_AssetGUID: Option<GUID>,
    /// Vec<PPtr<[`Shader`]>>: (4.0.0 - 6000.2.0a6)
    pub m_Dependencies: Option<Vec<PPtr>>,
    /// Vec<(String, PPtr<[`Texture`]>)>: (2018.1.0f1 - 6000.2.0a6)
    pub m_NonModifiableTextures: Option<Vec<(String, PPtr)>>,
    /// SerializedShader: (5.5.0f3 - 6000.2.0a6)
    pub m_ParsedForm: Option<SerializedShader>,
    /// String: (3.4.0 - 5.4.6f3)
    pub m_PathName: Option<String>,
    /// String: (3.4.0 - 5.4.6f3)
    pub m_Script: Option<String>,
    /// bool: (4.0.0 - 6000.2.0a6)
    pub m_ShaderIsBaked: Option<bool>,
    /// Vec<u8>: (5.3.0f1 - 5.4.6f3)
    pub m_SubProgramBlob: Option<Vec<u8>>,
    /// Vec<u32>: (5.5.0f3 - 2019.3.0a12); Vec<Vec<u32>>: (2019.3.0b1 - 6000.2.0a6)
    pub offsets: Option<Vec<Enum_Vec_u32___u32>>,
    /// Vec<u32>: (5.5.0f3 - 6000.2.0a6)
    pub platforms: Option<Vec<u32>>,
    /// Vec<u32>: (2021.3.12f1 - 6000.2.0a6)
    pub stageCounts: Option<Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_Vec_u32___u32 {
    Vec(Vec<u32>),
    u32(u32),
}

/// ShaderBindChannel is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderBindChannel {
    pub source: i8,
    pub target: i8,
}

/// ShaderContainer is a  class of the Unity engine since version 2022.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderContainer {}

/// ShaderImporter is a  class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ShaderImporter.html):
/**
Shader importer lets you modify shader import settings from Editor scripts.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderImporter {
    /**The name of the object.*/
    pub m_Name: String,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /**Get or set the AssetBundle name.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleName: Option<String>,
    /**Get or set the AssetBundle variant.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleVariant: Option<String>,
    /// Vec<(String, PPtr<[`Texture`]>)>: (4.2.0 - 6000.2.0a6)
    pub m_DefaultTextures: Option<Vec<(String, PPtr)>>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// Vec<(String, PPtr<[`Texture`]>)>: (2018.1.0f1 - 6000.2.0a6)
    pub m_NonModifiableTextures: Option<Vec<(String, PPtr)>>,
    /**This property has no effect.*/
    /// i32: (2020.2.0b1 - 2022.1.0a9)
    pub m_PreprocessorOverride: Option<i32>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// ShaderIncludeImporter is a  class of the Unity engine since version 2021.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderIncludeImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2021.2.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    pub m_UserData: String,
}

/// ShaderInfo is a sub class of the Unity engine since version 5.0.0f4.
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

/// ShaderNameRegistry is a  class of the Unity engine since version 2021.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderNameRegistry {
    pub m_PreloadShaders: bool,
    pub m_Shaders: NameToObjectMap,
}

/// ShaderVariantCollection is a  class of the Unity engine since version 5.0.0f4.
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
    /// Vec<(PPtr<[`Shader`]>, ShaderInfo)>: (5.0.0f4 - 6000.2.0a6)
    pub m_Shaders: Vec<(PPtr, ShaderInfo)>,
}

/// ShadowSettings is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShadowSettings {
    pub m_Bias: f32,
    pub m_Resolution: i32,
    pub m_Strength: f32,
    pub m_Type: i32,
    /// Matrix4x4f: (2019.1.0f2 - 6000.2.0a6)
    pub m_CullingMatrixOverride: Option<Matrix4x4f>,
    /// i32: (5.4.0f3 - 6000.2.0a6)
    pub m_CustomResolution: Option<i32>,
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_NearPlane: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_NormalBias: Option<f32>,
    /// f32: (3.4.0 - 4.7.2)
    pub m_Softness: Option<f32>,
    /// f32: (3.4.0 - 4.7.2)
    pub m_SoftnessFade: Option<f32>,
    /// bool: (2019.1.0f2 - 6000.2.0a6)
    pub m_UseCullingMatrixOverride: Option<bool>,
}

/// ShapeModule is a sub class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.ShapeModule.html):
/**
Script interface for the ShapeModule.
Configures the initial positions and directions of particles.Additional resources: ParticleSystem, ParticleSystem.shape.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeModule {
    /**Angle of the cone to emit particles from.*/
    pub angle: f32,
    /**Specifies whether the ShapeModule is enabled or disabled.*/
    pub enabled: bool,
    /**Mesh to emit particles from.*/
    /// PPtr<[`Mesh`]>: (3.5.0 - 6000.2.0a6)
    pub m_Mesh: PPtr,
    pub placementMode: i32,
    /**Radius of the shape to emit particles from.*/
    pub radius: Enum_f32__MultiModeParameter,
    /// i32: (3.5.0 - 6000.2.0a6)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /**Align particles based on their initial direction of travel.*/
    /// bool: (5.5.0f3 - 6000.2.0a6)
    pub alignToDirection: Option<bool>,
    /**Angle of the circle arc to emit particles from.*/
    /// f32: (5.0.0f4 - 5.6.0b4); MultiModeParameter: (5.6.0f1 - 6000.2.0a6)
    pub arc: Option<Enum_f32__MultiModeParameter>,
    /**Thickness of the box to emit particles from.*/
    /// Vector3f: (2017.1.0f1 - 6000.2.0a6)
    pub boxThickness: Option<Vector3f>,
    /// f32: (3.5.0 - 2017.1.0b1)
    pub boxX: Option<f32>,
    /// f32: (3.5.0 - 2017.1.0b1)
    pub boxY: Option<f32>,
    /// f32: (3.5.0 - 2017.1.0b1)
    pub boxZ: Option<f32>,
    /**The thickness of the Donut shape to emit particles from.*/
    /// f32: (2017.1.0f1 - 6000.2.0a6)
    pub donutRadius: Option<f32>,
    /**Length of the cone to emit particles from.*/
    /// f32: (4.0.0 - 6000.2.0a6)
    pub length: Option<f32>,
    /**Emit particles from a single Material of a Mesh.*/
    /// i32: (5.3.0f1 - 6000.2.0a6)
    pub m_MeshMaterialIndex: Option<i32>,
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_MeshNormalOffset: Option<f32>,
    /**MeshRenderer to emit particles from.*/
    /// PPtr<[`MeshRenderer`]>: (5.3.0f1 - 6000.2.0a6)
    pub m_MeshRenderer: Option<PPtr>,
    /// f32: (5.5.0f3 - 2017.1.0b1)
    pub m_MeshScale: Option<f32>,
    /// MultiModeParameter: (2018.3.0b1 - 6000.2.0a6)
    pub m_MeshSpawn: Option<MultiModeParameter>,
    /**Apply an offset to the position from which the system emits particles.*/
    /// Vector3f: (2017.1.0f1 - 6000.2.0a6)
    pub m_Position: Option<Vector3f>,
    /**Apply a rotation to the shape from which the system emits particles.*/
    /// Vector3f: (2017.1.0f1 - 6000.2.0a6)
    pub m_Rotation: Option<Vector3f>,
    /**Apply scale to the shape from which the system emits particles.*/
    /// Vector3f: (2017.1.0f1 - 6000.2.0a6)
    pub m_Scale: Option<Vector3f>,
    /**SkinnedMeshRenderer to emit particles from.*/
    /// PPtr<[`SkinnedMeshRenderer`]>: (5.3.0f1 - 6000.2.0a6)
    pub m_SkinnedMeshRenderer: Option<PPtr>,
    /**Sprite to emit particles from.*/
    /// PPtr<[`Sprite`]>: (2018.2.0b1 - 6000.2.0a6)
    pub m_Sprite: Option<PPtr>,
    /**SpriteRenderer to emit particles from.*/
    /// PPtr<[`SpriteRenderer`]>: (2018.2.0b1 - 6000.2.0a6)
    pub m_SpriteRenderer: Option<PPtr>,
    /**Specifies a Texture to tint the particle's start colors.*/
    /// PPtr<[`Texture2D`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Texture: Option<PPtr>,
    /**When enabled, the system applies the alpha channel of the Texture to the particle alpha when the particle spawns.*/
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub m_TextureAlphaAffectsParticles: Option<bool>,
    /**When enabled, the system takes four neighboring samples from the Texture then combines them to give the final particle value.*/
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub m_TextureBilinearFiltering: Option<bool>,
    /**Selects which channel of the Texture to use for discarding particles.*/
    /// i32: (2018.1.0f1 - 6000.2.0a6)
    pub m_TextureClipChannel: Option<i32>,
    /**Discards particles when they spawn on an area of the Texture with a value lower than this threshold.*/
    /// f32: (2018.1.0f1 - 6000.2.0a6)
    pub m_TextureClipThreshold: Option<f32>,
    /**When enabled, the system applies the RGB channels of the Texture to the particle color when the particle spawns.*/
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub m_TextureColorAffectsParticles: Option<bool>,
    /**When using a Mesh as a source shape type, this option controls which UV channel on the Mesh to use for reading the source Texture.*/
    /// i32: (2018.1.0f1 - 6000.2.0a6)
    pub m_TextureUVChannel: Option<i32>,
    /**Modulate the particle colors with the vertex colors, or the Material color if no vertex colors exist.*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_UseMeshColors: Option<bool>,
    /**Emit particles from a single Material, or the whole Mesh.*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_UseMeshMaterialIndex: Option<bool>,
    /**Radius thickness of the shape's edge from which to emit particles.*/
    /// f32: (2017.1.0f1 - 6000.2.0a6)
    pub radiusThickness: Option<f32>,
    /// bool: (3.5.0 - 5.4.6f3)
    pub randomDirection: Option<bool>,
    /**Randomizes the starting direction of particles.*/
    /// f32: (5.5.0f3 - 6000.2.0a6)
    pub randomDirectionAmount: Option<f32>,
    /**Randomizes the starting position of particles.*/
    /// f32: (2017.1.0f1 - 6000.2.0a6)
    pub randomPositionAmount: Option<f32>,
    /**Makes particles move in a spherical direction from their starting point.*/
    /// f32: (5.5.0f3 - 6000.2.0a6)
    pub sphericalDirectionAmount: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_f32__MultiModeParameter {
    f32(f32),
    MultiModeParameter(MultiModeParameter),
}

/// SiblingDerived is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SiblingDerived {}

/// SizeBySpeedModule is a sub class of the Unity engine since version 3.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.SizeBySpeedModule.html):
/**
Script interface for the SizeBySpeedModule.
This module controls the size of particles based on their speeds.Additional resources: ParticleSystem, ParticleSystem.sizeBySpeed.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SizeBySpeedModule {
    pub curve: MinMaxCurve,
    /**Specifies whether the SizeBySpeedModule is enabled or disabled.*/
    pub enabled: bool,
    /**Set the minimum and maximum speed that this modules applies the size curve between.*/
    pub range: Vector2f,
    /**Set the size by speed on each axis separately.*/
    /// bool: (5.4.0f3 - 6000.2.0a6)
    pub separateAxes: Option<bool>,
    /**Size by speed curve for the y-axis.*/
    /// MinMaxCurve: (5.4.0f3 - 6000.2.0a6)
    pub y: Option<MinMaxCurve>,
    /**Size by speed curve for the z-axis.*/
    /// MinMaxCurve: (5.4.0f3 - 6000.2.0a6)
    pub z: Option<MinMaxCurve>,
}

/// SizeModule is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SizeModule {
    pub curve: MinMaxCurve,
    pub enabled: bool,
    /// bool: (5.4.0f3 - 6000.2.0a6)
    pub separateAxes: Option<bool>,
    /// MinMaxCurve: (5.4.0f3 - 6000.2.0a6)
    pub y: Option<MinMaxCurve>,
    /// MinMaxCurve: (5.4.0f3 - 6000.2.0a6)
    pub z: Option<MinMaxCurve>,
}

/// SkeletonBone is a sub class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SkeletonBone.html):
/**
Details of the Transform name mapped to the skeleton bone of a model and its default position and rotation in the T-pose.
The skeleton models used in Unity have multiple bones.  The SkeletonBone struct has properties that are used to describe the position, rotation and scale of each bone.  The bones are not shown.  A MonoBehaviour.OnDrawGizmosSelected tool can be created to view the skeleton. An array of SkeletonBone positions can be used to make a line model using Gizmos.DrawLine.An array of SkeletonBones are used in HumanDescription.skeleton.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SkeletonBone {
    /**The name of the Transform mapped to the bone.*/
    pub m_Name: String,
    /**The T-pose position of the bone in local space.*/
    pub m_Position: Vector3f,
    /**The T-pose rotation of the bone in local space.*/
    pub m_Rotation: Quaternionf,
    /**The T-pose scaling of the bone in local space.*/
    pub m_Scale: Vector3f,
    /// String: (5.5.0f3 - 6000.2.0a6)
    pub m_ParentName: Option<String>,
    /// bool: (4.0.0 - 5.4.6f3)
    pub m_TransformModified: Option<bool>,
}

/// SkeletonBoneLimit is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SkeletonBoneLimit {
    pub m_Length: f32,
    pub m_Max: Vector3f,
    pub m_Min: Vector3f,
    pub m_Modified: bool,
    pub m_Value: Vector3f,
    /// Quaternionf: (4.0.0 - 4.2.2)
    pub m_PostQ: Option<Quaternionf>,
    /// Quaternionf: (4.0.0 - 4.2.2)
    pub m_PreQ: Option<Quaternionf>,
}

/// SketchUpImportCamera is a sub class of the Unity engine since version 5.1.0f1.
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
    /// f32: (2019.1.0b1 - 6000.2.0a6)
    pub farPlane: Option<f32>,
    /**The far clipping plane distance.*/
    /// f32: (2019.1.0b1 - 6000.2.0a6)
    pub nearPlane: Option<f32>,
}

/// SketchUpImportData is a sub class of the Unity engine since version 5.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SketchUpImportData {
    pub defaultCamera: SketchUpImportCamera,
    pub scenes: Vec<SketchUpImportScene>,
}

/// SketchUpImportScene is a sub class of the Unity engine since version 5.1.0f1.
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

/// SketchUpImporter is a  class of the Unity engine since version 5.1.0f1.
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
    /**Animation clips to split animation into. Additional resources: ModelImporterClipAnimation.*/
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
    /**Import animation from file.*/
    pub m_ImportAnimation: bool,
    /**Controls import of BlendShapes.*/
    pub m_ImportBlendShapes: bool,
    /// Vec<PPtr<[`GameObject`]>>: (5.1.0f1 - 6000.2.0a6)
    pub m_ImportedRoots: Vec<PPtr>,
    /**Generates the list of all imported take.*/
    pub m_ImportedTakeInfos: Vec<TakeInfo>,
    /**Are mesh vertices and indices accessible from script?*/
    pub m_IsReadable: bool,
    pub m_LODScreenPercentages: Vec<f32>,
    /// PPtr<[`Avatar`]>: (5.1.0f1 - 6000.2.0a6)
    pub m_LastHumanDescriptionAvatarSource: PPtr,
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
    /**Returns the matching referenced clip assets for this model.*/
    pub m_ReferencedClips: Vec<GUID>,
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
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub bakeAxisConversion: Option<bool>,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub blendShapeNormalImportMode: Option<i32>,
    /**Format of the imported mesh index buffer data.*/
    /// i32: (2017.3.0b1 - 6000.2.0a6)
    pub indexFormat: Option<i32>,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub legacyComputeAllNormalsFromSmoothingGroupsWhenMeshHasBlendShapes: Option<bool>,
    /// bool: (2020.2.0f1 - 6000.2.0a6)
    pub m_AddHumanoidExtraRootOnlyWhenUsingAvatar: Option<bool>,
    /**Generate auto mapping if no avatarSetup is provided when importing humanoid animation.*/
    /// bool: (2019.3.0f1 - 6000.2.0a6)
    pub m_AutoGenerateAvatarMappingIfUnspecified: Option<bool>,
    /// bool: (2017.2.0b2 - 2017.2.0b6)
    pub m_AutoMapExternalMaterials: Option<bool>,
    /**The Avatar generation of the imported model.*/
    /// i32: (2019.3.0b1 - 6000.2.0a6)
    pub m_AvatarSetup: Option<i32>,
    /// bool: (2023.1.0b1 - 6000.2.0a6)
    pub m_ContainsAnimation: Option<bool>,
    /// bool: (5.1.0f1 - 2019.3.0a2)
    pub m_CopyAvatar: Option<bool>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /**A list of default FBX properties to treat as user properties during OnPostprocessGameObjectWithUserProperties.*/
    /// Vec<String>: (2017.1.0f1 - 6000.2.0a6)
    pub m_ExtraUserProperties: Option<Vec<String>>,
    /// Vec<(i64, String)>: (5.1.0f1 - 2018.4.36f1)
    pub m_FileIDToRecycleName: Option<Vec<(i64, String)>>,
    /// i32: (2019.4.0f1 - 6000.2.0a6)
    pub m_FileIdsGeneration: Option<i32>,
    /// f32: (2018.3.0b1 - 6000.2.0a6)
    pub m_FileScaleFactor: Option<f32>,
    /// String: (2018.3.0b1 - 6000.2.0a6)
    pub m_FileScaleUnit: Option<String>,
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub m_HasEmbeddedTextures: Option<bool>,
    /// bool: (2018.2.0b1 - 2019.1.0a13)
    pub m_HasPreviousCalculatedGlobalScale: Option<bool>,
    /**Controls how much oversampling is used when importing humanoid animations for retargeting.*/
    /// i32: (5.2.0f2 - 6000.2.0a6)
    pub m_HumanoidOversampling: Option<i32>,
    /**Import animated custom properties from file.*/
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub m_ImportAnimatedCustomProperties: Option<bool>,
    /**Import BlendShapes deform percent.*/
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_ImportBlendShapeDeformPercent: Option<bool>,
    /**Controls import of cameras. Basic properties like field of view, near plane distance and far plane distance can be animated.*/
    /// bool: (2017.1.0b1 - 6000.2.0a6)
    pub m_ImportCameras: Option<bool>,
    /**Import animation constraints.*/
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub m_ImportConstraints: Option<bool>,
    /**Controls import of lights. Note that because light are defined differently in DCC tools, some light types or properties may not be exported. Basic properties like color and intensity can be animated.*/
    /// bool: (2017.1.0b1 - 6000.2.0a6)
    pub m_ImportLights: Option<bool>,
    /// bool: (5.1.0f1 - 2019.3.0a6)
    pub m_ImportMaterials: Option<bool>,
    /// bool: (2022.2.19f1 - 6000.2.0a6)
    pub m_ImportPhysicalCameras: Option<bool>,
    /**Use visibility properties to enable or disable MeshRenderer components.*/
    /// bool: (2017.1.0b1 - 6000.2.0a6)
    pub m_ImportVisibility: Option<bool>,
    /// Vec<((i32, i64), String)>: (2019.1.0b1 - 6000.2.0a6)
    pub m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
    /**Material creation options.*/
    /// i32: (2019.3.0b1 - 6000.2.0a6)
    pub m_MaterialImportMode: Option<i32>,
    /**Material import location options.*/
    /// i32: (2017.2.0f1 - 6000.2.0a6)
    pub m_MaterialLocation: Option<i32>,
    /// Vec<SourceAssetIdentifier>: (2017.2.0f1 - 6000.2.0a6)
    pub m_Materials: Option<Vec<SourceAssetIdentifier>>,
    /// i32: (2021.2.0b1 - 6000.2.0a6)
    pub m_NodeNameCollisionStrategy: Option<i32>,
    /**If true, always create an explicit Prefab root. Otherwise, if the model has a single root, it is reused as the Prefab root.*/
    /// bool: (2017.3.0b1 - 6000.2.0a6)
    pub m_PreserveHierarchy: Option<bool>,
    /// f32: (2018.2.0b1 - 2019.1.0a13)
    pub m_PreviousCalculatedGlobalScale: Option<f32>,
    /// bool: (2020.3.37f1 - 6000.2.0a6)
    pub m_RemapMaterialsIfMaterialImportModeIsNone: Option<bool>,
    /**Removes constant animation curves with values identical to the object initial scale value.*/
    /// bool: (2021.2.0b1 - 6000.2.0a6)
    pub m_RemoveConstantScaleCurves: Option<bool>,
    /**If set to false, the importer will not resample curves when possible.Read more about animation curve resampling.Notes:- Some unsupported FBX features (such as PreRotation or PostRotation on transforms) will override this setting. In these situations, animation curves will still be resampled even if the setting is disabled. For best results, avoid using PreRotation, PostRotation and GetRotationPivot.- This option was introduced in Version 5.3. Prior to this version, Unity's import behaviour was as if this option was always enabled. Therefore enabling the option gives the same behaviour as pre-5.3 animation import.*/
    /// bool: (5.4.0f3 - 6000.2.0a6)
    pub m_ResampleCurves: Option<bool>,
    /// bool: (5.3.0f1 - 5.3.8f2)
    pub m_ResampleRotations: Option<bool>,
    /// String: (5.6.0b1 - 2023.1.0a15)
    pub m_RigImportErrors: Option<String>,
    /// String: (5.6.0b1 - 2023.1.0a15)
    pub m_RigImportWarnings: Option<String>,
    /**Sorts the gameObject hierarchy by name.*/
    /// bool: (2019.2.0b1 - 6000.2.0a6)
    pub m_SortHierarchyByName: Option<bool>,
    /**Enables strict checks on imported vertex data.*/
    /// bool: (2021.3.36f1 - 6000.2.0a6)
    pub m_StrictVertexDataChecks: Option<bool>,
    /// bool: (2017.2.0f1 - 6000.2.0a6)
    pub m_SupportsEmbeddedMaterials: Option<bool>,
    /**When disabled, imported material albedo colors are converted to gamma space. This property should be disabled when using linear color space in Player rendering settings.The default value is true.*/
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_UseSRGBMaterialColor: Option<bool>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /**The maximum number of bones per vertex stored in this mesh data.*/
    /// i32: (2019.1.0b1 - 6000.2.0a6)
    pub maxBonesPerVertex: Option<i32>,
    /**Options to control the optimization of mesh data during asset import.*/
    /// i32: (2019.1.0b1 - 6000.2.0a6)
    pub meshOptimizationFlags: Option<i32>,
    /**Minimum bone weight to keep.*/
    /// f32: (2019.1.0b1 - 6000.2.0a6)
    pub minBoneWeight: Option<f32>,
    /**Normal generation options for ModelImporter.*/
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub normalCalculationMode: Option<i32>,
    /**Source of smoothing information for calculation of normals.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub normalSmoothingSource: Option<i32>,
    /**Only import bones where they are connected to vertices.*/
    /// bool: (2021.2.0b1 - 6000.2.0a6)
    pub optimizeBones: Option<bool>,
    /// bool: (5.1.0f1 - 2018.4.36f1)
    pub optimizeMeshForGPU: Option<bool>,
    /**Method to use for handling margins when generating secondary UV.*/
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub secondaryUVMarginMethod: Option<i32>,
    /**The minimum lightmap resolution in texels per unit that the associated model is expected to have.*/
    /// f32: (2020.1.0b1 - 6000.2.0a6)
    pub secondaryUVMinLightmapResolution: Option<f32>,
    /**The minimum object scale that the associated model is expected to have.*/
    /// f32: (2020.1.0b1 - 6000.2.0a6)
    pub secondaryUVMinObjectScale: Option<f32>,
    /// i32: (2019.1.0b1 - 6000.2.0a6)
    pub skinWeightsMode: Option<i32>,
    /// bool: (5.1.0f1 - 5.2.5f1)
    pub splitTangentsAcrossUV: Option<bool>,
}

/// SkinnedCloth is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SkinnedCloth {
    pub m_BendingStiffness: f32,
    pub m_Coefficients: Vec<ClothConstrainCoefficients>,
    pub m_Damping: f32,
    pub m_Enabled: u8,
    pub m_ExternalAcceleration: Vector3f,
    /// PPtr<[`GameObject`]>: (3.4.0 - 4.7.2)
    pub m_GameObject: PPtr,
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
    /// Vec<PPtr<[`Transform`]>>: (3.4.0 - 6000.2.0a6)
    pub m_Bones: Vec<PPtr>,
    pub m_CastShadows: Enum_bool__u8,
    pub m_DirtyAABB: bool,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapTilingOffset: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    /// Vec<PPtr<[`Material`]>>: (3.4.0 - 6000.2.0a6)
    pub m_Materials: Vec<PPtr>,
    /// PPtr<[`Mesh`]>: (3.4.0 - 6000.2.0a6)
    pub m_Mesh: PPtr,
    /**The maximum number of bones per vertex that are taken into account during skinning.*/
    pub m_Quality: i32,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: Enum_bool__u8,
    /// PPtr<[`Transform`]>: (3.4.0 - 6000.2.0a6)
    pub m_StaticBatchRoot: PPtr,
    /**If enabled, the Skinned Mesh will be updated when offscreen. If disabled, this also disables updating animations.*/
    pub m_UpdateWhenOffscreen: bool,
    /// Vec<f32>: (4.3.0 - 6000.2.0a6)
    pub m_BlendShapeWeights: Option<Vec<f32>>,
    /// u8: (2017.2.0f1 - 6000.2.0a6)
    pub m_DynamicOccludee: Option<u8>,
    /// PPtr<[`Transform`]>: (3.5.0 - 4.7.2)
    pub m_LightProbeAnchor: Option<PPtr>,
    /**The light probe interpolation type.*/
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeUsage: Option<u8>,
    /// PPtr<[`GameObject`]>: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    /// u16: (5.0.0f4 - 6000.2.0a6)
    pub m_LightmapIndexDynamic: Option<u16>,
    /// Vector4f: (5.0.0f4 - 6000.2.0a6)
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_MotionVectors: Option<u8>,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr<[`Transform`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_ProbeAnchor: Option<PPtr>,
    /// u8: (2020.1.0b1 - 6000.2.0a6)
    pub m_RayTraceProcedural: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlags: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlagsOverride: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0b1 - 6000.2.0a6)
    pub m_RayTracingMode: Option<u8>,
    /**Should reflection probes be used for this Renderer?*/
    /// i32: (5.0.0f4 - 5.3.8f2); u8: (5.4.0f3 - 6000.2.0a6)
    pub m_ReflectionProbeUsage: Option<i32>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on, if you use a scriptable render pipeline.*/
    /// u32: (2018.1.0f1 - 6000.2.0a6)
    pub m_RenderingLayerMask: Option<u32>,
    /// PPtr<[`Transform`]>: (3.5.0 - 6000.2.0a6)
    pub m_RootBone: Option<PPtr>,
    /**Specifies whether skinned motion vectors should be used for this renderer.*/
    /// bool: (5.4.0f3 - 6000.2.0a6)
    pub m_SkinnedMotionVectors: Option<bool>,
    /// u8: (2023.3.0b1 - 6000.2.0a6)
    pub m_SmallMeshCulling: Option<u8>,
    /// i16: (4.3.0 - 6000.2.0a6)
    pub m_SortingLayer: Option<i16>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// u32: (4.5.0 - 4.7.2); i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SortingLayerID: Option<i64>,
    /**Renderer's order within a sorting layer.*/
    /// i16: (4.3.0 - 6000.2.0a6)
    pub m_SortingOrder: Option<i16>,
    /// StaticBatchInfo: (5.5.0f3 - 6000.2.0a6)
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.1.0b1 - 6000.2.0a6)
    pub m_StaticShadowCaster: Option<u8>,
    /// Vec<u32>: (3.4.0 - 5.4.6f3)
    pub m_SubsetIndices: Option<Vec<u32>>,
    /// bool: (3.5.0 - 5.3.8f2)
    pub m_UseLightProbes: Option<bool>,
}

/// Skybox is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Skybox.html):
/**
A script interface for the skybox component.
The skybox class has only the material property.Additional resources: skybox component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Skybox {
    /// PPtr<[`Material`]>: (3.4.0 - 6000.2.0a6)
    pub m_CustomSkybox: PPtr,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
}

/// SliderJoint2D is a  class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SliderJoint2D.html):
/**
Joint that restricts the motion of a Rigidbody2D object to a single line.
Additional resources: Rigidbody2D, DistanceJoint2D, HingeJoint2D, SpringJoint2D, JointTranslationLimits2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SliderJoint2D {
    /**The joint's anchor point on the object that has the joint component.*/
    pub m_Anchor: Vector2f,
    /**The angle of the line in space (in degrees).*/
    pub m_Angle: f32,
    /**The joint's anchor point on the second object (ie, the one which doesn't have the joint component).*/
    pub m_ConnectedAnchor: Vector2f,
    /// PPtr<[`Rigidbody2D`]>: (4.3.0 - 6000.2.0a6)
    pub m_ConnectedRigidBody: PPtr,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.3.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Parameters for a motor force that is applied automatically to the Rigibody2D along the line.*/
    pub m_Motor: JointMotor2D,
    pub m_TranslationLimits: JointTranslationLimits2D,
    /**Should motion limits be used?*/
    pub m_UseLimits: bool,
    /**Should a motor force be applied automatically to the Rigidbody2D?*/
    pub m_UseMotor: bool,
    /**Should the angle be calculated automatically?*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_AutoConfigureAngle: Option<bool>,
    /**Should the connectedAnchor be calculated automatically?*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_BreakAction: Option<i32>,
    /**The force that needs to be applied for this joint to break.*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_BreakForce: Option<f32>,
    /**The torque that needs to be applied for this joint to break.*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_BreakTorque: Option<f32>,
    /// bool: (4.3.0 - 5.0.0f4)
    pub m_CollideConnected: Option<bool>,
    /**Should the two Rigidbody2D connected with this joint collide with each other?*/
    /// bool: (5.0.1f1 - 6000.2.0a6)
    pub m_EnableCollision: Option<bool>,
}

/// SnapshotConstant is a sub class of the Unity engine since version 5.0.0f4.
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
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub contactDistance: Option<f32>,
    /// f32: (3.4.0 - 4.7.2)
    pub damper: Option<f32>,
    /// f32: (3.4.0 - 4.7.2)
    pub spring: Option<f32>,
}

/// SoftJointLimitSpring is a sub class of the Unity engine since version 5.0.0f4.
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

/// SortingGroup is a  class of the Unity engine since version 5.6.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rendering.SortingGroup.html):
/**
Adding a SortingGroup component to a GameObject will ensure that all Renderers within the GameObject's descendants will be sorted and rendered together.
A common use case for having a SortingGroup is to create complex 2D characters that are made up of multiple SpriteRenderers. When several clones of such a character overlap, their individual body parts might not be sorted properly resulting in a visual glitch where the the body parts interleave. For example, the hands of two characters might be sorted in front of their bodies, where you would expect one entire character to be drawn in front of the other character. The SortingGroup component solves this by ensuring the entire branch of the character are sorted and rendered together.The descendants of the SortingGroup are sorted using the same SortingLayer and Renderer.sortingOrder. However, they are only sorted against other descendants of the SortingGroup and not with any renderers outside of it. This allows you to reuse the same SortingLayers (for example, "Hands", "Torso"...) to sort body parts while ensuring they never interleave with other clones of the character.The SortingGroups, together with other renderers, are sorted using the SortingLayer and Renderer.sortingOrder. Additionally, they can be nested within other SortingGroups. This is useful if you have branches of descendants that should not be mixed up i.e. the "Left Hand" vs the "Right Hand" hierarchy branches.The maximum number of sorting groups and renderers is 4096.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SortingGroup {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (5.6.0b1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub m_SortingLayer: i16,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    /**Ignores any parent SortingGroup and sorts this and its descendant Renderers against other Renderers at the root level.*/
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_SortAtRoot: Option<bool>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// i32: (5.6.0f3 - 6000.2.0a6)
    pub m_SortingLayerID: Option<i32>,
}

/// SortingLayerEntry is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SortingLayerEntry {
    pub name: String,
    pub uniqueID: u32,
    /// u32: (4.3.0 - 4.7.2)
    pub userID: Option<u32>,
}

/// SourceAssetIdentifier is a sub class of the Unity engine since version 2017.2.0f1.
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
    /// String: (2017.2.0f1 - 6000.2.0a6)
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
    /// bool: (3.5.0 - 4.7.2)
    pub doesTextureContainColor: Option<bool>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub sourceWasHDR: Option<bool>,
}

/// SparseTexture is a  class of the Unity engine since version 4.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SparseTexture.html):
/**
Class for handling Sparse Textures.
Sparse textures are textures where not the whole texture data can be present in memory at once. They are also commonly called "tiled textures" or "mega textures".Imagine a 16384x16384 texture at 32 bits per pixel - it would take 1GB of memory. The texture is broken down into rectangular "tiles", and each tile can either be present in memory or not. You can load & unload these tiles as needed based on distance from the camera, sectors of the world that the player has to see, etc.Otherwise, the sparse textures behave just like any other texture in shaders - they can have mipmaps, can use all texture filtering modes, etc. If you happen to read from a tile that's not present, you can get undefined result (on many GPUs the result is a black color, but that's not guaranteed).Not all hardware and platforms support sparse textures, so you should check SystemInfo.supportsSparseTextures before using them. For example, on DirectX systems they require DX11.2 (Windows 8.1) and a fairly recent GPU; and on OpenGL they require ARB_sparse_texture extension support. Sparse textures only support non-compressed texture formats.After creating the sparse texture, query the tile size with tileWidth & tileHeight. Tile sizes are platform and GPU dependent.Use UpdateTile or UpdateTileRaw to make a tile resident in memory and update its color data. Use UnloadTile to unload a tile.Additional resources:  Sparse Textures.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SparseTexture {
    pub m_ColorSpace: i32,
    pub m_Format: i64,
    /**Height of the Texture in pixels (Read Only).*/
    pub m_Height: i32,
    pub m_MipCount: i32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_TextureSettings: GLTextureSettings,
    /**Width of the Texture in pixels (Read Only).*/
    pub m_Width: i32,
    /// bool: (2017.3.0b1 - 2023.2.0a17)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.3.0b1 - 2023.2.0a17)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_IsAlphaChannelOptional: Option<bool>,
}

/// SpeedTreeImporter is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SpeedTreeImporter.html):
/**
AssetImporter for importing SpeedTree model assets.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpeedTreeImporter {
    /**Gets and sets a default alpha test reference values.*/
    pub m_AlphaTestRef: f32,
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
    /**Gets and sets a default hue variation color and amount (in alpha).*/
    pub m_HueVariation: ColorRGBA,
    pub m_LODSettings: Vec<PerLODSettings>,
    /**Gets and sets a default main color.*/
    pub m_MainColor: ColorRGBA,
    /**The name of the object.*/
    pub m_Name: String,
    /**How much to scale the tree model compared to what is in the imported SpeedTree model file.*/
    pub m_ScaleFactor: f32,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /**Indicates if the cross-fade LOD transition, applied to the last mesh LOD and the billboard, should be animated.*/
    /// bool: (5.1.0f1 - 6000.2.0a6)
    pub m_AnimateCrossFading: Option<bool>,
    /// bool: (2022.3.12f1 - 6000.2.0a6)
    pub m_EnableBumpMapping: Option<bool>,
    /// bool: (2022.3.12f1 - 6000.2.0a6)
    pub m_EnableHueVariation: Option<bool>,
    /// bool: (2022.3.12f1 - 6000.2.0a6)
    pub m_EnableLightProbes: Option<bool>,
    /// bool: (2022.3.12f1 - 6000.2.0a6)
    pub m_EnableShadowCasting: Option<bool>,
    /// bool: (2022.3.12f1 - 6000.2.0a6)
    pub m_EnableShadowReceiving: Option<bool>,
    /// bool: (2022.3.12f1 - 6000.2.0a6)
    pub m_EnableSubsurfaceScattering: Option<bool>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// i32: (2019.1.0b1 - 6000.2.0a6)
    pub m_FileIDType: Option<i32>,
    /**Gets and sets the boolean to toggle collider object generation during import.*/
    /// bool: (2023.3.0b10 - 6000.2.0a6)
    pub m_GenerateColliders: Option<bool>,
    /**Gets and sets the boolean to toggle Rigidbody generation during import.*/
    /// bool: (2023.3.0b10 - 6000.2.0a6)
    pub m_GenerateRigidbody: Option<bool>,
    /**Material import location options.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_MaterialLocation: Option<i32>,
    /// i32: (5.2.0f2 - 6000.2.0a6)
    pub m_MaterialVersion: Option<i32>,
    /// Vec<SourceAssetIdentifier>: (2018.3.0b1 - 6000.2.0a6)
    pub m_Materials: Option<Vec<SourceAssetIdentifier>>,
    /// i32: (2022.3.3f1 - 6000.2.0a6)
    pub m_MotionVectorModeEnumValue: Option<i32>,
    /// i32: (2022.3.12f1 - 6000.2.0a6)
    pub m_ReflectionProbeEnumValue: Option<i32>,
    /**Gets and sets an integer corresponding to the SpeedTreeWind enum values. The value is clamped by SpeedTreeImporter.bestWindQuality internally.*/
    /// i32: (2022.3.12f1 - 6000.2.0a6)
    pub m_SelectedWindQuality: Option<i32>,
    /// f32: (5.0.0f4 - 5.4.0b25)
    pub m_Shininess: Option<f32>,
    /// ColorRGBA: (5.0.0f4 - 5.4.0b25)
    pub m_SpecColor: Option<ColorRGBA>,
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_SupportsEmbeddedMaterials: Option<bool>,
    /// i32: (2022.3.12f1 - 6000.2.0a6)
    pub m_UnitConversionEnumValue: Option<i32>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// SpeedTreeWind is a sub class of the Unity engine since version 5.0.0f4.
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

/// SpeedTreeWindAsset is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SpeedTreeWindAsset.html):
/**
SpeedTreeWindAsset generated by the SpeedTreeImporter, contains wind version and configuration data for SpeedTree wind simulation.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpeedTreeWindAsset {
    /**The name of the object.*/
    pub m_Name: String,
    /// SpeedTreeWindConfig8: (2023.3.0b1 - 6000.2.0a6)
    pub m_Config8: Option<SpeedTreeWindConfig8>,
    /// SpeedTreeWindConfig9: (2023.3.0b1 - 6000.2.0a6)
    pub m_Config9: Option<SpeedTreeWindConfig9>,
    /// SpeedTreeWind: (5.0.0f4 - 2023.3.0a16)
    pub m_Wind: Option<SpeedTreeWind>,
    /// i32: (2023.3.0b1 - 6000.2.0a6)
    pub m_eVersion: Option<i32>,
}

/// SpeedTreeWindConfig8 is a sub class of the Unity engine since version 2023.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpeedTreeWindConfig8 {
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
    pub BranchLevel1: SBranchWindLevel,
    pub BranchLevel2: SBranchWindLevel,
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
    pub ROLLING: bool,
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
    pub m_fMaxBranchLevel1Length: f32,
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

/// SpeedTreeWindConfig9 is a sub class of the Unity engine since version 2023.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpeedTreeWindConfig9 {
    pub m_bDoBranch1: i32,
    pub m_bDoBranch2: i32,
    pub m_bDoRipple: i32,
    pub m_bDoShared: i32,
    pub m_bDoShimmer: i32,
    pub m_bLodFade: i32,
    pub m_fBranch1StretchLimit: f32,
    pub m_fBranch2StretchLimit: f32,
    pub m_fDirectionResponse: f32,
    pub m_fGustDurationMax: f32,
    pub m_fGustDurationMin: f32,
    pub m_fGustFallScalar: f32,
    pub m_fGustFrequency: f32,
    pub m_fGustRiseScalar: f32,
    pub m_fGustStrengthMax: f32,
    pub m_fGustStrengthMin: f32,
    pub m_fSharedHeightStart: f32,
    pub m_fStrengthResponse: f32,
    pub m_fWindIndependence: f32,
    pub m_sBranch1: BranchWindLevel,
    pub m_sBranch2: BranchWindLevel,
    pub m_sRipple: RippleGroup,
    pub m_sShared: BranchWindLevel,
    pub m_vTreeExtents: Vector3f,
    /// f32: (6000.0.8f1 - 6000.2.0a6)
    pub m_fImportScaling: Option<f32>,
    /// i32: (2023.3.0b1 - 6000.0.7f1)
    pub pad: Option<i32>,
}

/// SphereCollider is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SphereCollider.html):
/**
A sphere-shaped primitive collider.
Additional resources: BoxCollider, CapsuleCollider, PhysicsMaterial, Rigidbody.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SphereCollider {
    /**The center of the sphere in the object's local space.*/
    pub m_Center: Vector3f,
    /**Enabled Colliders will collide with other Colliders, disabled Colliders won't.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Specify if this collider is configured as a trigger.*/
    pub m_IsTrigger: bool,
    /**The material used by the collider.*/
    /// PPtr<[`PhysicMaterial`]>: (3.4.0 - 2023.3.0a8); PPtr<[`PhysicsMaterial`]>: (2023.3.0b1 - 6000.2.0a6)
    pub m_Material: PPtr,
    /**The radius of the sphere measured in the object's local space.*/
    pub m_Radius: f32,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_ProvidesContacts: Option<bool>,
}

/// SphericalHarmonicsL2 is a sub class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rendering.SphericalHarmonicsL2.html):
/**
Spherical harmonics up to the second order (3 bands, 9 coefficients).
Spherical harmonics (SH) represent a function or signal over directions, and are commonly used in computer graphics to efficiently evaluate smooth lighting. Unity uses them for LightProbes and environment lighting.L2 spherical harmonics is composed of 9 coefficients for each color channel.Additional resources: RenderSettings.ambientMode, RenderSettings.ambientProbe, LightProbes.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SphericalHarmonicsL2 {
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[10]")]
    pub sh_10_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[11]")]
    pub sh_11_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[12]")]
    pub sh_12_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[13]")]
    pub sh_13_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[14]")]
    pub sh_14_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[15]")]
    pub sh_15_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[16]")]
    pub sh_16_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[17]")]
    pub sh_17_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[18]")]
    pub sh_18_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[19]")]
    pub sh_19_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[20]")]
    pub sh_20_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[21]")]
    pub sh_21_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[22]")]
    pub sh_22_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[23]")]
    pub sh_23_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[24]")]
    pub sh_24_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[25]")]
    pub sh_25_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[26]")]
    pub sh_26_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[ 0]")]
    pub sh__0_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[ 1]")]
    pub sh__1_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[ 2]")]
    pub sh__2_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[ 3]")]
    pub sh__3_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[ 4]")]
    pub sh__4_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[ 5]")]
    pub sh__5_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[ 6]")]
    pub sh__6_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[ 7]")]
    pub sh__7_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[ 8]")]
    pub sh__8_: Option<f32>,
    /// f32: (5.0.0f4 - 6000.2.0a6)
    #[serde(alias = "sh[ 9]")]
    pub sh__9_: Option<f32>,
}

/// SplashScreenLogo is a sub class of the Unity engine since version 5.5.0f3.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/PlayerSettings.SplashScreenLogo.html):
/**
A single logo that is shown during the Splash Screen. Controls the Sprite that is displayed and its duration.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SplashScreenLogo {
    /**The total time in seconds for which the logo is shown. The minimum duration is 2 seconds.*/
    pub duration: f32,
    /**The Sprite that is shown during this logo. If this is null, then no logo will be displayed for the duration.*/
    /// PPtr<[`Sprite`]>: (5.5.0f3 - 6000.2.0a6)
    pub logo: PPtr,
}

/// SplatDatabase is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SplatDatabase {
    /// Vec<PPtr<[`Texture2D`]>>: (3.4.0 - 6000.2.0a6)
    pub m_AlphaTextures: Vec<PPtr>,
    pub m_AlphamapResolution: i32,
    pub m_BaseMapResolution: i32,
    /// i32: (5.0.1f1 - 2018.2.21f1)
    pub m_ColorSpace: Option<i32>,
    /// bool: (5.0.1f1 - 2018.2.21f1)
    pub m_MaterialRequiresMetallic: Option<bool>,
    /// bool: (5.0.1f1 - 2018.2.21f1)
    pub m_MaterialRequiresSmoothness: Option<bool>,
    /// Vec<SplatPrototype>: (3.4.0 - 2018.2.21f1)
    pub m_Splats: Option<Vec<SplatPrototype>>,
    /// Vec<PPtr<[`TerrainLayer`]>>: (2018.3.0b1 - 6000.2.0a6)
    pub m_TerrainLayers: Option<Vec<PPtr>>,
}

/// SplatPrototype is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SplatPrototype.html):
/**
Obsolete. Use TerrainLayer instead. A Splat prototype is just a texture that is used by the TerrainData.
A class on a Terrain GameObject.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SplatPrototype {
    /// PPtr<[`Texture2D`]>: (3.4.0 - 2018.2.21f1)
    pub texture: PPtr,
    pub tileOffset: Vector2f,
    pub tileSize: Vector2f,
    /// PPtr<[`Texture2D`]>: (4.0.0 - 2018.2.21f1)
    pub normalMap: Option<PPtr>,
    /// f32: (5.0.1f1 - 2018.2.21f1)
    pub smoothness: Option<f32>,
    /// Vector4f: (5.0.0f4 - 2018.2.21f1)
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
    /// PPtr<[`Rigidbody`]>: (3.4.0 - 6000.2.0a6)
    pub m_ConnectedBody: PPtr,
    /**The damper force used to dampen the spring force.*/
    pub m_Damper: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The maximum distance between the bodies relative to their initial distance.*/
    pub m_MaxDistance: f32,
    /**The minimum distance between the bodies relative to their initial distance.*/
    pub m_MinDistance: f32,
    /**The spring force used to keep the two objects together.*/
    pub m_Spring: f32,
    /**Should the connectedAnchor be calculated automatically?*/
    /// bool: (4.3.0 - 6000.2.0a6)
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    /**The Direction of the axis around which the body is constrained.*/
    /// Vector3f: (2023.2.0b1 - 6000.1.0a8)
    pub m_Axis: Option<Vector3f>,
    /**Position of the anchor relative to the connected Rigidbody.*/
    /// Vector3f: (4.3.0 - 6000.2.0a6)
    pub m_ConnectedAnchor: Option<Vector3f>,
    /**A reference to an articulation body this joint connects to.*/
    /// PPtr<[`ArticulationBody`]>: (2020.2.0b1 - 6000.2.0a6)
    pub m_ConnectedArticulationBody: Option<PPtr>,
    /**The scale to apply to the inverse mass and inertia tensor of the connected body prior to solving the constraints.*/
    /// f32: (2017.1.0f1 - 6000.2.0a6)
    pub m_ConnectedMassScale: Option<f32>,
    /**Enable collision between bodies connected with the joint.*/
    /// bool: (4.5.0 - 6000.2.0a6)
    pub m_EnableCollision: Option<bool>,
    /**Toggle preprocessing for this joint.*/
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_EnablePreprocessing: Option<bool>,
    /// bool: (2017.1.0b2 - 2017.1.0b5)
    pub m_Enabled: Option<bool>,
    /**The scale to apply to the inverse mass and inertia tensor of the body prior to solving the constraints.*/
    /// f32: (2017.1.0f1 - 6000.2.0a6)
    pub m_MassScale: Option<f32>,
    /**The maximum allowed error between the current spring length and the length defined by minDistance and maxDistance.*/
    /// f32: (5.2.3f1 - 6000.2.0a6)
    pub m_Tolerance: Option<f32>,
}

/// SpringJoint2D is a  class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SpringJoint2D.html):
/**
Joint that attempts to keep two Rigidbody2D objects a set distance apart by applying a force between them.
Note that unlike DistanceJoint2D, the length of the joint can stretch and oscillate.Additional resources: DistanceJoint2D, HingeJoint2D, SliderJoint2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpringJoint2D {
    /**The joint's anchor point on the object that has the joint component.*/
    pub m_Anchor: Vector2f,
    /**The joint's anchor point on the second object (ie, the one which doesn't have the joint component).*/
    pub m_ConnectedAnchor: Vector2f,
    /// PPtr<[`Rigidbody2D`]>: (4.3.0 - 6000.2.0a6)
    pub m_ConnectedRigidBody: PPtr,
    /**The amount by which the spring force is reduced in proportion to the movement speed.*/
    pub m_DampingRatio: f32,
    /**The distance the spring will try to keep between the two objects.*/
    pub m_Distance: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The frequency at which the spring oscillates around the distance distance between the objects.*/
    pub m_Frequency: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.3.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Should the connectedAnchor be calculated automatically?*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    /**Should the distance be calculated automatically?*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_AutoConfigureDistance: Option<bool>,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_BreakAction: Option<i32>,
    /**The force that needs to be applied for this joint to break.*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_BreakForce: Option<f32>,
    /**The torque that needs to be applied for this joint to break.*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_BreakTorque: Option<f32>,
    /// bool: (4.3.0 - 5.0.0f4)
    pub m_CollideConnected: Option<bool>,
    /**Should the two Rigidbody2D connected with this joint collide with each other?*/
    /// bool: (5.0.1f1 - 6000.2.0a6)
    pub m_EnableCollision: Option<bool>,
}

/// Sprite is a  class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Sprite.html):
/**
Represents a Sprite object for use in 2D gameplay.
Sprites are 2D graphic objects used for characters, props, projectiles and other elements of 2D gameplay. The graphics are obtained from bitmap images - Texture2D. The Sprite class primarily identifies the section of the image that should be used for a specific Sprite. This information can then be used by a SpriteRenderer component on a GameObject to actually display the graphic.Additional resources: SpriteRenderer class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Sprite {
    pub m_Extrude: u32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Offset: Vector2f,
    pub m_PixelsToUnits: f32,
    pub m_RD: SpriteRenderData,
    /**Location of the Sprite on the original Texture, specified in pixels.*/
    pub m_Rect: Rectf,
    /// Vec<String>: (2017.1.0b1 - 6000.2.0a6)
    pub m_AtlasTags: Option<Vec<String>>,
    /// Vec<SpriteBone>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Bones: Option<Vec<SpriteBone>>,
    /**Returns the border sizes of the Sprite.*/
    /// Vector4f: (4.5.0 - 6000.2.0a6)
    pub m_Border: Option<Vector4f>,
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_IsPolygon: Option<bool>,
    /// Vec<Vec<Vector2f>>: (2017.1.0b1 - 6000.2.0a6)
    pub m_PhysicsShape: Option<Vec<Vec<Vector2f>>>,
    /**Location of the Sprite's pivot point in the Rect on the original Texture, specified in pixels.*/
    /// Vector2f: (5.4.2f2 - 6000.2.0a6)
    pub m_Pivot: Option<Vector2f>,
    /// (GUID, i64): (2017.1.0b1 - 6000.2.0a6)
    pub m_RenderDataKey: Option<(GUID, i64)>,
    /// Vec<PPtr<[`MonoBehaviour`]>>: (2023.1.0b1 - 6000.2.0a6)
    pub m_ScriptableObjects: Option<Vec<PPtr>>,
    /// PPtr<[`SpriteAtlas`]>: (2017.1.0b1 - 6000.2.0a6)
    pub m_SpriteAtlas: Option<PPtr>,
}

/// SpriteAtlas is a  class of the Unity engine since version 2017.1.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/U2D.SpriteAtlas.html):
/**
Sprite Atlas is an asset created within Unity. It is part of the built-in sprite packing solution.
A Sprite Atlas stores a list of packable assets. A packable asset is either a Sprite, Texture2D of TextureImporterType.Sprite or Folder. Before the packing process begins, these packable assets will be grouped and traversed to gather all the sprites from them. These will be used in the packing process. At runtime, these sprites can be enumerated via the Sprite Atlas (Additional resources: SpriteAtlas.GetSprites).It also provides dedicated texture settings in the inspector for the packed texture. The original texture settings of the sprite will have no effect on the packed texture.By default, Sprite Atlas will be referenced by the sprite and be available at runtime. This means that the sprite will be able to acquire the packed texture from the Sprite Atlas when loaded. A Sprite can be loaded without referencing any Sprite Atlas. A Sprite loaded this way will render as invisible since there is no texture. A reference to a Sprite Atlas can be added later. Additional resources: SpriteAtlasManager.Sprite Atlas variants can be created by assigning another Sprite Atlas object as the master. Variants will not repack a new texture from the packable list. Instead of this, variants will duplicate the master's packed texture and downscale it according to a user-defined ratio and save this scaled texture.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlas {
    /**Return true if this SpriteAtlas is a variant.*/
    pub m_IsVariant: bool,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_PackedSpriteNamesToIndex: Vec<String>,
    /// Vec<PPtr<[`Sprite`]>>: (2017.1.0b1 - 6000.2.0a6)
    pub m_PackedSprites: Vec<PPtr>,
    pub m_RenderDataMap: Vec<((GUID, i64), SpriteAtlasData)>,
    /**Get the tag of this SpriteAtlas.*/
    pub m_Tag: String,
}

/// SpriteAtlasAsset is a  class of the Unity engine since version 2020.1.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/U2D.SpriteAtlasAsset.html):
/**
SpriteAtlasAsset stores inputs for generating SpriteAtlas and generates atlas textures on Import.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlasAsset {
    pub m_ImporterData: Enum_SpriteAtlasEditorData__SpriteAtlasAssetData,
    /**Checks whether the Sprite Atlas Importer set the Sprite Atlas as a Variant.*/
    pub m_IsVariant: bool,
    /// PPtr<[`SpriteAtlas`]>: (2020.1.0b1 - 6000.2.0a6)
    pub m_MasterAtlas: PPtr,
    /**The name of the object.*/
    pub m_Name: String,
    /// PPtr<[`Object`]>: (2023.1.0b1 - 6000.2.0a6)
    pub m_ScriptablePacker: Option<PPtr>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_SpriteAtlasEditorData__SpriteAtlasAssetData {
    SpriteAtlasEditorData(SpriteAtlasEditorData),
    SpriteAtlasAssetData(SpriteAtlasAssetData),
}

/// SpriteAtlasAssetData is a sub class of the Unity engine since version 2022.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlasAssetData {
    /// Vec<PPtr<[`Object`]>>: (2022.1.0b1 - 6000.2.0a6)
    pub packables: Vec<PPtr>,
}

/// SpriteAtlasData is a sub class of the Unity engine since version 2017.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlasData {
    /// PPtr<[`Texture2D`]>: (2017.1.0b1 - 6000.2.0a6)
    pub alphaTexture: PPtr,
    pub downscaleMultiplier: f32,
    pub settingsRaw: u32,
    /// PPtr<[`Texture2D`]>: (2017.1.0b1 - 6000.2.0a6)
    pub texture: PPtr,
    pub textureRect: Rectf,
    pub textureRectOffset: Vector2f,
    pub uvTransform: Vector4f,
    /// Vector2f: (2017.1.1p1 - 6000.2.0a6)
    pub atlasRectOffset: Option<Vector2f>,
    /// Vec<SecondarySpriteTexture>: (2020.2.0b1 - 6000.2.0a6)
    pub secondaryTextures: Option<Vec<SecondarySpriteTexture>>,
}

/// SpriteAtlasDatabase is a  class of the Unity engine since version 2017.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlasDatabase {}

/// SpriteAtlasEditorData is a sub class of the Unity engine since version 2020.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlasEditorData {
    pub bindAsDefault: bool,
    /// PPtr<[`CachedSpriteAtlasRuntimeData`]>: (2020.1.0b1 - 2022.1.0a13)
    pub cachedData: PPtr,
    pub isAtlasV2: bool,
    /// Vec<PPtr<[`Object`]>>: (2020.1.0b1 - 2022.1.0a13)
    pub packables: Vec<PPtr>,
    pub packingSettings: PackingSettings,
    pub platformSettings: Vec<TextureImporterPlatformSettings>,
    pub textureSettings: TextureSettings,
    pub variantMultiplier: f32,
    /// Vec<(String, SecondaryTextureSettings)>: (2020.2.0b1 - 2022.1.0a13)
    pub secondaryTextureSettings: Option<Vec<(String, SecondaryTextureSettings)>>,
    /// Hash128: (2020.1.0a15 - 2020.1.0a18)
    pub storedHash: Option<Hash128>,
    /// u32: (2020.1.0b1 - 2020.3.18f1)
    pub totalSpriteSurfaceArea: Option<u32>,
}

/// SpriteAtlasImporter is a  class of the Unity engine since version 2020.1.0b1.
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
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2020.1.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_UsedFileIDs: Vec<i64>,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_BindAsDefault: Option<bool>,
    /**SpriteAtlasPackingSettings to use when packing this SpriteAtlas.*/
    /// PackingSettings: (2022.1.0b1 - 6000.2.0a6)
    pub m_PackingSettings: Option<PackingSettings>,
    /// Vec<TextureImporterPlatformSettings>: (2022.1.0b1 - 6000.2.0a6)
    pub m_PlatformSettings: Option<Vec<TextureImporterPlatformSettings>>,
    /// Vec<(String, SecondaryTextureSettings)>: (2022.1.0b1 - 6000.2.0a6)
    pub m_SecondaryTextureSettings: Option<Vec<(String, SecondaryTextureSettings)>>,
    /**SpriteAtlasTextureSettings of the packed Texture generated by this SpriteAtlas.*/
    /// TextureSettings: (2022.1.0b1 - 6000.2.0a6)
    pub m_TextureSettings: Option<TextureSettings>,
    /// f32: (2022.1.0b1 - 6000.2.0a6)
    pub m_VariantMultiplier: Option<f32>,
}

/// SpriteBone is a sub class of the Unity engine since version 2018.1.0f1.
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
    /// ColorRGBA: (2021.1.0b1 - 6000.2.0a6)
    pub color: Option<ColorRGBA>,
    /**The Unique GUID of this bone.*/
    /// String: (2021.1.0b1 - 6000.2.0a6)
    pub guid: Option<String>,
}

/// SpriteCustomDataEntry is a sub class of the Unity engine since version 6000.0.7f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteCustomDataEntry {
    pub m_Key: String,
    pub m_Value: String,
}

/// SpriteCustomMetadata is a sub class of the Unity engine since version 6000.0.7f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteCustomMetadata {
    pub m_Entries: Vec<SpriteCustomDataEntry>,
}

/// SpriteData is a sub class of the Unity engine since version 2017.1.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/U2D.ScriptablePacker.SpriteData.html):
/**
A struct containing sprite specific data to pack sprites.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteData {
    /// PPtr<[`Object`]>: (2017.1.0b1 - 6000.2.0a6)
    pub sprite: PPtr,
}

/// SpriteMask is a  class of the Unity engine since version 2017.1.0b1.
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
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    pub m_FrontSortingLayer: i16,
    /**Order within the front sorting layer defining the start of the custom range.*/
    pub m_FrontSortingOrder: i16,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (2017.1.0b1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Mask sprites from front to back sorting values only.*/
    pub m_IsCustomRangeActive: bool,
    /**The light probe interpolation type.*/
    pub m_LightProbeUsage: u8,
    /// PPtr<[`GameObject`]>: (2017.1.0b1 - 6000.2.0a6)
    pub m_LightProbeVolumeOverride: PPtr,
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    pub m_MaskAlphaCutoff: f32,
    /**Returns all the instantiated materials of this object.*/
    /// Vec<PPtr<[`Material`]>>: (2017.1.0b1 - 6000.2.0a6)
    pub m_Materials: Vec<PPtr>,
    pub m_MotionVectors: u8,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr<[`Transform`]>: (2017.1.0b1 - 6000.2.0a6)
    pub m_ProbeAnchor: PPtr,
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
    /// PPtr<[`Sprite`]>: (2017.1.0b1 - 6000.2.0a6)
    pub m_Sprite: PPtr,
    pub m_StaticBatchInfo: StaticBatchInfo,
    /// PPtr<[`Transform`]>: (2017.1.0b1 - 6000.2.0a6)
    pub m_StaticBatchRoot: PPtr,
    /**Unique ID of the sorting layer defining the end of the custom range.*/
    /// i32: (2018.4.5f1 - 6000.2.0a6)
    pub m_BackSortingLayerID: Option<i32>,
    /// u8: (2017.2.0f1 - 6000.2.0a6)
    pub m_DynamicOccludee: Option<u8>,
    /**Unique ID of the sorting layer defining the start of the custom range.*/
    /// i32: (2018.4.5f1 - 6000.2.0a6)
    pub m_FrontSortingLayerID: Option<i32>,
    /**The source used for generating the mask for this SpriteMask.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_MaskSource: Option<i32>,
    /// u8: (2020.1.0b1 - 6000.2.0a6)
    pub m_RayTraceProcedural: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlags: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlagsOverride: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0b1 - 6000.2.0a6)
    pub m_RayTracingMode: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on, if you use a scriptable render pipeline.*/
    /// u32: (2018.1.0f1 - 6000.2.0a6)
    pub m_RenderingLayerMask: Option<u32>,
    /// u8: (2023.3.0b1 - 6000.2.0a6)
    pub m_SmallMeshCulling: Option<u8>,
    /**Determines the position of the Sprite used for sorting the SpriteMask.*/
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub m_SpriteSortPoint: Option<i32>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.1.0b1 - 6000.2.0a6)
    pub m_StaticShadowCaster: Option<u8>,
}

/// SpriteMetaData is a sub class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SpriteMetaData.html):
/**
Editor data used in producing a Sprite.
Additional resources: TextureImporter.spritesheet.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteMetaData {
    /**Edge-relative alignment of the sprite graphic.*/
    pub m_Alignment: i32,
    /**Name of the Sprite.*/
    pub m_Name: String,
    /**The pivot point of the Sprite, relative to its bounding rectangle.*/
    pub m_Pivot: Vector2f,
    /**Bounding rectangle of the sprite's graphic within the atlas image.*/
    pub m_Rect: Rectf,
    /// Vec<SpriteBone>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Bones: Option<Vec<SpriteBone>>,
    /**Edge border size for a sprite (in pixels).*/
    /// Vector4f: (4.5.0 - 6000.2.0a6)
    pub m_Border: Option<Vector4f>,
    /// String: (2023.3.0b1 - 6000.2.0a6)
    pub m_CustomData: Option<String>,
    /// Vec<int2_storage>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Edges: Option<Vec<int2_storage>>,
    /// Vec<i32>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Indices: Option<Vec<i32>>,
    /// i64: (2019.1.0b1 - 6000.2.0a6)
    pub m_InternalID: Option<i64>,
    /// Vec<Vec<Vector2f>>: (5.3.0f1 - 6000.2.0a6)
    pub m_Outline: Option<Vec<Vec<Vector2f>>>,
    /// Vec<Vec<Vector2f>>: (2017.1.0b1 - 6000.2.0a6)
    pub m_PhysicsShape: Option<Vec<Vec<Vector2f>>>,
    /// String: (2018.1.0f1 - 6000.2.0a6)
    pub m_SpriteID: Option<String>,
    /// f32: (5.4.0f3 - 6000.2.0a6)
    pub m_TessellationDetail: Option<f32>,
    /// Vec<Vector2f>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Vertices: Option<Vec<Vector2f>>,
    /// Vec<BoneWeights4>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Weights: Option<Vec<BoneWeights4>>,
}

/// SpriteRenderData is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteRenderData {
    pub settingsRaw: u32,
    /// PPtr<[`Texture2D`]>: (4.3.0 - 6000.2.0a6)
    pub texture: PPtr,
    pub textureRect: Rectf,
    pub textureRectOffset: Vector2f,
    /// PPtr<[`Texture2D`]>: (5.2.0f2 - 6000.2.0a6)
    pub alphaTexture: Option<PPtr>,
    /// Vector2f: (5.4.6f1 - 6000.2.0a6)
    pub atlasRectOffset: Option<Vector2f>,
    /// f32: (2017.1.0b1 - 6000.2.0a6)
    pub downscaleMultiplier: Option<f32>,
    /// Vec<u16>: (4.3.0 - 5.5.6f1)
    pub indices: Option<Vec<u16>>,
    /// Vec<Matrix4x4f>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Bindpose: Option<Vec<Matrix4x4f>>,
    /// Vec<u8>: (5.6.0b1 - 6000.2.0a6)
    pub m_IndexBuffer: Option<Vec<u8>>,
    /// Vec<BoneWeights4>: (2018.1.0f1 - 2018.1.9f2)
    pub m_SourceSkin: Option<Vec<BoneWeights4>>,
    /// Vec<SubMesh>: (5.6.0b1 - 6000.2.0a6)
    pub m_SubMeshes: Option<Vec<SubMesh>>,
    /// VertexData: (5.6.0b1 - 6000.2.0a6)
    pub m_VertexData: Option<VertexData>,
    /// Vec<SecondarySpriteTexture>: (2019.1.0b1 - 6000.2.0a6)
    pub secondaryTextures: Option<Vec<SecondarySpriteTexture>>,
    /// Vector4f: (4.5.0 - 6000.2.0a6)
    pub uvTransform: Option<Vector4f>,
    /// Vec<SpriteVertex>: (4.3.0 - 5.5.6f1)
    pub vertices: Option<Vec<SpriteVertex>>,
}

/// SpriteRenderer is a  class of the Unity engine since version 4.3.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/SpriteRenderer.html):
/**
A component that renders a Sprite.If a GameObject has a SpriteRenderer component, the component renders the assigned Sprite asset on the screen using the position, rotation and scale from the Transform component.You can use this to draw characters, items, backgrounds, and other visual elements in a 2D game.
Additional resources: Sprite, Transform, SpriteShapeRenderer, SpriteMask, SpriteAtlas.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteRenderer {
    pub m_CastShadows: Enum_bool__u8,
    /**Rendering color for the Sprite graphic.*/
    pub m_Color: ColorRGBA,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.3.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapTilingOffset: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    /// Vec<PPtr<[`Material`]>>: (4.3.0 - 6000.2.0a6)
    pub m_Materials: Vec<PPtr>,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: Enum_bool__u8,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    /**The Sprite to render.*/
    /// PPtr<[`Sprite`]>: (4.3.0 - 6000.2.0a6)
    pub m_Sprite: PPtr,
    /// PPtr<[`Transform`]>: (4.3.0 - 6000.2.0a6)
    pub m_StaticBatchRoot: PPtr,
    /**The current threshold for Sprite Renderer tiling.*/
    /// f32: (5.6.0f1 - 6000.2.0a6)
    pub m_AdaptiveModeThreshold: Option<f32>,
    /**The current draw mode of the Sprite Renderer.*/
    /// i32: (5.6.0f1 - 6000.2.0a6)
    pub m_DrawMode: Option<i32>,
    /// u8: (2017.2.0f1 - 6000.2.0a6)
    pub m_DynamicOccludee: Option<u8>,
    /**Flips the sprite on the X axis.*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_FlipX: Option<bool>,
    /**Flips the sprite on the Y axis.*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_FlipY: Option<bool>,
    /// PPtr<[`Transform`]>: (4.3.0 - 4.7.2)
    pub m_LightProbeAnchor: Option<PPtr>,
    /**The light probe interpolation type.*/
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeUsage: Option<u8>,
    /// PPtr<[`GameObject`]>: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    /// u16: (5.0.0f4 - 6000.2.0a6)
    pub m_LightmapIndexDynamic: Option<u16>,
    /// Vector4f: (5.0.0f4 - 6000.2.0a6)
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    /**Specifies how the sprite interacts with the masks.*/
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub m_MaskInteraction: Option<i32>,
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_MotionVectors: Option<u8>,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr<[`Transform`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_ProbeAnchor: Option<PPtr>,
    /// u8: (2020.1.0b1 - 6000.2.0a6)
    pub m_RayTraceProcedural: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlags: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlagsOverride: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0b1 - 6000.2.0a6)
    pub m_RayTracingMode: Option<u8>,
    /**Should reflection probes be used for this Renderer?*/
    /// i32: (5.0.0f4 - 5.3.8f2); u8: (5.4.0f3 - 6000.2.0a6)
    pub m_ReflectionProbeUsage: Option<i32>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on, if you use a scriptable render pipeline.*/
    /// u32: (2018.1.0f1 - 6000.2.0a6)
    pub m_RenderingLayerMask: Option<u32>,
    /**Property to set or get the size to render when the SpriteRenderer.drawMode is set to SpriteDrawMode.Sliced or SpriteDrawMode.Tiled.*/
    /// Vector2f: (5.6.0f1 - 6000.2.0a6)
    pub m_Size: Option<Vector2f>,
    /// u8: (2023.3.0b1 - 6000.2.0a6)
    pub m_SmallMeshCulling: Option<u8>,
    /// i16: (4.3.0 - 6000.2.0a6)
    pub m_SortingLayer: Option<i16>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// u32: (4.5.0 - 4.7.2); i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SortingLayerID: Option<i64>,
    /**Determines the position of the Sprite used for sorting the SpriteRenderer.*/
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub m_SpriteSortPoint: Option<i32>,
    /// i32: (5.6.0f1 - 6000.2.0a6)
    pub m_SpriteTileMode: Option<i32>,
    /// StaticBatchInfo: (5.5.0f3 - 6000.2.0a6)
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.1.0b1 - 6000.2.0a6)
    pub m_StaticShadowCaster: Option<u8>,
    /// Vec<u32>: (4.3.0 - 5.4.6f3)
    pub m_SubsetIndices: Option<Vec<u32>>,
    /// bool: (4.3.0 - 5.3.8f2)
    pub m_UseLightProbes: Option<bool>,
    /// bool: (5.6.2f1 - 6000.2.0a6)
    pub m_WasSpriteAssigned: Option<bool>,
}

/// SpriteShapeRenderer is a  class of the Unity engine since version 2018.1.0f1.
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
    /// PPtr<[`GameObject`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The light probe interpolation type.*/
    pub m_LightProbeUsage: u8,
    /// PPtr<[`GameObject`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_LightProbeVolumeOverride: PPtr,
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    pub m_LocalAABB: AABB,
    /**Specifies how the SpriteShape interacts with the masks.*/
    pub m_MaskInteraction: i32,
    /**Returns all the instantiated materials of this object.*/
    /// Vec<PPtr<[`Material`]>>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Materials: Vec<PPtr>,
    pub m_MotionVectors: u8,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr<[`Transform`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_ProbeAnchor: PPtr,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: u8,
    /**Should reflection probes be used for this Renderer?*/
    pub m_ReflectionProbeUsage: u8,
    /**Determines which rendering layer this renderer lives on, if you use a scriptable render pipeline.*/
    pub m_RenderingLayerMask: u32,
    /// PPtr<[`Texture2D`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_ShapeTexture: PPtr,
    pub m_SortingLayer: i16,
    /**Unique ID of the Renderer's sorting layer.*/
    pub m_SortingLayerID: i32,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    /// Vec<PPtr<[`Sprite`]>>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Sprites: Vec<PPtr>,
    pub m_StaticBatchInfo: StaticBatchInfo,
    /// PPtr<[`Transform`]>: (2018.1.0f1 - 6000.2.0a6)
    pub m_StaticBatchRoot: PPtr,
    /// u8: (2020.1.0b1 - 6000.2.0a6)
    pub m_RayTraceProcedural: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlags: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlagsOverride: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0b1 - 6000.2.0a6)
    pub m_RayTracingMode: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_RendererPriority: Option<i32>,
    /// u8: (2023.3.0b1 - 6000.2.0a6)
    pub m_SmallMeshCulling: Option<u8>,
    /// i32: (2022.1.0b1 - 6000.2.0a6)
    pub m_SpriteSortPoint: Option<i32>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.1.0b1 - 6000.2.0a6)
    pub m_StaticShadowCaster: Option<u8>,
}

/// SpriteSheetMetaData is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteSheetMetaData {
    pub m_Sprites: Vec<SpriteMetaData>,
    /// Vec<SpriteBone>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Bones: Option<Vec<SpriteBone>>,
    /// String: (2023.3.0b1 - 6000.2.0a6)
    pub m_CustomData: Option<String>,
    /// Vec<int2_storage>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Edges: Option<Vec<int2_storage>>,
    /// Vec<i32>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Indices: Option<Vec<i32>>,
    /// i64: (2019.1.0b1 - 6000.2.0a6)
    pub m_InternalID: Option<i64>,
    /// Vec<(String, i64)>: (2021.2.0b1 - 6000.2.0a6)
    pub m_NameFileIdTable: Option<Vec<(String, i64)>>,
    /// Vec<Vec<Vector2f>>: (5.3.0f1 - 6000.2.0a6)
    pub m_Outline: Option<Vec<Vec<Vector2f>>>,
    /// Vec<Vec<Vector2f>>: (2017.1.0b1 - 6000.2.0a6)
    pub m_PhysicsShape: Option<Vec<Vec<Vector2f>>>,
    /// Vec<SecondarySpriteTexture>: (2019.1.0b1 - 6000.2.0a6)
    pub m_SecondaryTextures: Option<Vec<SecondarySpriteTexture>>,
    /// SpriteCustomMetadata: (6000.0.7f1 - 6000.2.0a6)
    pub m_SpriteCustomMetadata: Option<SpriteCustomMetadata>,
    /// String: (2018.1.0f1 - 6000.2.0a6)
    pub m_SpriteID: Option<String>,
    /// Vec<Vector2f>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Vertices: Option<Vec<Vector2f>>,
    /// Vec<BoneWeights4>: (2018.1.0f1 - 6000.2.0a6)
    pub m_Weights: Option<Vec<BoneWeights4>>,
}

/// SpriteTilingProperty is a sub class of the Unity engine since version 5.6.0f1.
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

/// SpriteVertex is a sub class of the Unity engine since version 4.3.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteVertex {
    pub pos: Vector3f,
    /// Vector2f: (4.3.0 - 4.3.4)
    pub uv: Option<Vector2f>,
}

/// State is a  class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Random.State.html):
/**
Serializable structure used to hold the full internal state of the random number generator. Additional resources: Random.state.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub m_IKOnFeet: bool,
    /// Vec<PPtr<[`Motion`]>>: (4.0.0 - 4.7.2)
    pub m_Motions: Vec<PPtr>,
    pub m_Name: String,
    /// PPtr<[`StateMachine`]>: (4.0.0 - 4.7.2)
    pub m_ParentStateMachine: PPtr,
    pub m_Position: Vector3f,
    pub m_Speed: f32,
    pub m_Tag: String,
    /// f32: (4.1.0 - 4.7.2)
    pub m_CycleOffset: Option<f32>,
    /// bool: (4.1.0 - 4.7.2)
    pub m_Mirror: Option<bool>,
}

/// StateKey is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct StateKey {
    pub m_LayerIndex: i32,
    pub m_StateID: u32,
}

/// StateMachine is a  class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct StateMachine {
    pub m_AnyStatePosition: Vector3f,
    /// Vec<PPtr<[`StateMachine`]>>: (4.0.0 - 4.7.2)
    pub m_ChildStateMachine: Vec<PPtr>,
    pub m_ChildStateMachinePosition: Vec<Vector3f>,
    /// PPtr<[`State`]>: (4.0.0 - 4.7.2)
    pub m_DefaultState: PPtr,
    pub m_MotionSetCount: i32,
    pub m_Name: String,
    /// Vec<(PPtr<[`State`]>, Vec<PPtr<[`Transition`]>>)>: (4.0.0 - 4.7.2)
    pub m_OrderedTransitions: Vec<(PPtr, Vec<PPtr>)>,
    pub m_ParentStateMachinePosition: Vector3f,
    /// Vec<PPtr<[`State`]>>: (4.0.0 - 4.7.2)
    pub m_States: Vec<PPtr>,
    /// Vec<(PPtr<[`State`]>, Vec<PPtr<[`Transition`]>>)>: (4.0.0 - 4.1.5)
    pub m_LocalTransitions: Option<Vec<(PPtr, Vec<PPtr>)>>,
}

/// StateMachineBehaviourVectorDescription is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct StateMachineBehaviourVectorDescription {
    pub m_StateMachineBehaviourIndices: Vec<u32>,
    pub m_StateMachineBehaviourRanges: Vec<(StateKey, StateRange)>,
}

/// StateRange is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct StateRange {
    pub m_Count: u32,
    pub m_StartIndex: u32,
}

/// StaticBatchInfo is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct StaticBatchInfo {
    pub firstSubMesh: u16,
    pub subMeshCount: u16,
}

/// StreamInfo is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamInfo {
    pub channelMask: u32,
    pub offset: u32,
    pub stride: u32,
    /// u32: (3.5.0 - 3.5.7)
    pub align: Option<u32>,
    /// u8: (4.0.0 - 4.7.2)
    pub dividerOp: Option<u8>,
    /// u16: (4.0.0 - 4.7.2)
    pub frequency: Option<u16>,
}

/// StreamedClip is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamedClip {
    pub curveCount: u32,
    pub data: Vec<u32>,
    /// u16: (2022.3.19f1 - 6000.2.0a6)
    pub discreteCurveCount: Option<u16>,
}

/// StreamedResource is a sub class of the Unity engine since version 5.0.0f4.
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamedResource {
    pub m_Offset: i128,
    pub m_Size: u64,
    pub m_Source: String,
}

/// StreamingController is a  class of the Unity engine since version 2018.2.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/StreamingController.html):
/**
A StreamingController controls the streaming settings for an individual camera location.
The StreamingController component is used to control texture streaming settings for a camera location.
This component supports the preloading of textures in advance of a Camera becoming enabled. See SetPreloadingThe QualitySettings.streamingMipmapsFeature must be enabled and active for this feature to work.The Camera is not considered for texture streaming when this component is disabled.
When this component is enabled the Camera is considered for texture streaming if the Camera is enabled or the StreamingController is in the preloading state.A mipmap bias can be applied for texture streaming calculations. See streamingMipmapBias for details.Additional resources: camera component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamingController {
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (2018.2.0b1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Offset applied to the mipmap level chosen by the texture streaming system for any textures visible from this camera. This Offset can take either a positive or negative value.*/
    pub m_StreamingMipmapBias: f32,
}

/// StreamingInfo is a sub class of the Unity engine since version 5.3.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamingInfo {
    pub offset: u64,
    pub path: String,
    pub size: u32,
}

/// StreamingManager is a  class of the Unity engine since version 2018.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamingManager {}

/// StructParameter is a sub class of the Unity engine since version 2017.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct StructParameter {
    pub m_ArraySize: i32,
    pub m_Index: i32,
    pub m_MatrixMembers: Vec<MatrixParameter>,
    pub m_NameIndex: i32,
    pub m_StructSize: i32,
    pub m_VectorMembers: Vec<VectorParameter>,
}

/// StyleSheetImporter is a  class of the Unity engine since version 2017.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct StyleSheetImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    pub m_Name: String,
    pub m_UserData: String,
}

/// SubCollider is a sub class of the Unity engine since version 5.6.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubCollider {
    /// PPtr<[`Collider2D`]>: (5.6.0b1 - 6000.2.0a6)
    pub m_Collider: PPtr,
    pub m_ColliderPaths: Vec<Vec<IntPoint>>,
}

/// SubDerived is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubDerived {}

/// SubEmitterData is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubEmitterData {
    /// PPtr<[`ParticleSystem`]>: (5.5.0f3 - 6000.2.0a6)
    pub emitter: PPtr,
    pub properties: i32,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// f32: (2018.3.0b1 - 6000.2.0a6)
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
    /// u32: (2017.3.0b1 - 6000.2.0a6)
    pub baseVertex: Option<u32>,
    /// u32: (3.4.0 - 3.5.7)
    pub isTriStrip: Option<u32>,
    /// i32: (4.0.0 - 6000.2.0a6)
    pub topology: Option<i32>,
    /// u32: (3.4.0 - 3.5.7)
    pub triangleCount: Option<u32>,
}

/// SubModule is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubModule {
    pub enabled: bool,
    /// PPtr<[`ParticleSystem`]>: (3.5.0 - 5.4.6f3)
    pub subEmitterBirth: Option<PPtr>,
    /// PPtr<[`ParticleSystem`]>: (4.0.0 - 5.4.6f3)
    pub subEmitterBirth1: Option<PPtr>,
    /// PPtr<[`ParticleSystem`]>: (3.5.0 - 5.4.6f3)
    pub subEmitterCollision: Option<PPtr>,
    /// PPtr<[`ParticleSystem`]>: (4.0.0 - 5.4.6f3)
    pub subEmitterCollision1: Option<PPtr>,
    /// PPtr<[`ParticleSystem`]>: (3.5.0 - 5.4.6f3)
    pub subEmitterDeath: Option<PPtr>,
    /// PPtr<[`ParticleSystem`]>: (4.0.0 - 5.4.6f3)
    pub subEmitterDeath1: Option<PPtr>,
    /// Vec<SubEmitterData>: (5.5.0f3 - 6000.2.0a6)
    pub subEmitters: Option<Vec<SubEmitterData>>,
}

/// SubPassDescriptor is a sub class of the Unity engine since version 6000.0.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Rendering.SubPassDescriptor.html):
/**
Structure discribing a single native subpass.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SubPassDescriptor {
    /**Array of attachment indices to be used as the color render targets in this sub pass. These are specificed as indices into the attachment array passed to CommandBuffer.BeginRenderPass.*/
    pub colorOutputs: AttachmentIndexArray,
    /**Flags controlling specific reading behaviour of depth and stencil attachments.*/
    pub flags: i32,
    /**Array of attachment indices to be used as input attachments in this sub pass. These are specificed as indices into the attachment array passed to CommandBuffer.BeginRenderPass.*/
    pub inputs: AttachmentIndexArray,
}

/// SubstanceArchive is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubstanceArchive {
    pub m_Name: String,
    /// Vec<u8>: (3.4.0 - 2017.4.40f1)
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
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleName: Option<String>,
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleVariant: Option<String>,
    /// Vec<String>: (3.5.2 - 2017.4.40f1)
    pub m_DeletedPrototypes: Option<Vec<String>>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.2)
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    /// i32: (3.4.0 - 3.5.1)
    pub m_IsFirstImport: Option<i32>,
    /// Vec<MaterialImportOutput>: (4.0.0 - 2017.4.40f1)
    pub m_MaterialImportOutputs: Option<Vec<MaterialImportOutput>>,
    /// Vec<MaterialInstanceSettings>: (3.4.0 - 2017.4.40f1)
    pub m_MaterialInstances: Option<Vec<MaterialInstanceSettings>>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_NewHashIdentity: Option<MdFour>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_OldHashIdentity: Option<MdFour>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /// String: (4.0.0 - 6000.2.0a6)
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
    /// i32: (3.4.0 - 2017.4.40f1)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// Vec<String>: (4.5.0 - 2017.4.40f1)
    pub componentLabels: Option<Vec<String>>,
    /// String: (3.5.0 - 2017.4.40f1)
    pub group: Option<String>,
    /// u32: (3.5.0 - 2017.4.40f1)
    pub internalIdentifier: Option<u32>,
    /// String: (4.1.0 - 2017.4.40f1)
    pub label: Option<String>,
    /// String: (5.2.0f2 - 2017.4.40f1)
    pub visibleIf: Option<String>,
}

/// SubstanceValue is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubstanceValue {
    /// PPtr<[`Texture2D`]>: (3.4.0 - 2017.4.40f1)
    pub texture: PPtr,
    /// f32: (3.4.0 - 2017.4.40f1)
    #[serde(alias = "scalar[0]")]
    pub scalar_0_: Option<f32>,
    /// f32: (3.4.0 - 2017.4.40f1)
    #[serde(alias = "scalar[1]")]
    pub scalar_1_: Option<f32>,
    /// f32: (3.4.0 - 2017.4.40f1)
    #[serde(alias = "scalar[2]")]
    pub scalar_2_: Option<f32>,
    /// f32: (3.4.0 - 2017.4.40f1)
    #[serde(alias = "scalar[3]")]
    pub scalar_3_: Option<f32>,
    /// String: (2017.2.0f1 - 2017.4.40f1)
    pub stringvalue: Option<String>,
}

/// SurfaceEffector2D is a  class of the Unity engine since version 5.0.0f4.
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
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The speed to be maintained along the surface.*/
    pub m_Speed: f32,
    /**The speed variation (from zero to the variation) added to base speed to be applied.*/
    pub m_SpeedVariation: f32,
    /**The scale of the impulse force applied while attempting to reach the surface speed.*/
    /// f32: (5.0.1f1 - 6000.2.0a6)
    pub m_ForceScale: Option<f32>,
    /**Should bounce be used for any contact with the surface?*/
    /// bool: (5.0.1f1 - 6000.2.0a6)
    pub m_UseBounce: Option<bool>,
    /**Should the collider-mask be used or the global collision matrix?*/
    /// bool: (5.0.2f1 - 6000.2.0a6)
    pub m_UseColliderMask: Option<bool>,
    /**Should the impulse force but applied to the contact point?*/
    /// bool: (5.0.1f1 - 6000.2.0a6)
    pub m_UseContactForce: Option<bool>,
    /**Should friction be used for any contact with the surface?*/
    /// bool: (5.0.1f1 - 6000.2.0a6)
    pub m_UseFriction: Option<bool>,
}

/// TagManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct TagManager {
    pub tags: Vec<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "Builtin Layer 0")]
    pub Builtin_Layer_0: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "Builtin Layer 1")]
    pub Builtin_Layer_1: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "Builtin Layer 2")]
    pub Builtin_Layer_2: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "Builtin Layer 3")]
    pub Builtin_Layer_3: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "Builtin Layer 4")]
    pub Builtin_Layer_4: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "Builtin Layer 5")]
    pub Builtin_Layer_5: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "Builtin Layer 6")]
    pub Builtin_Layer_6: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "Builtin Layer 7")]
    pub Builtin_Layer_7: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 10")]
    pub User_Layer_10: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 11")]
    pub User_Layer_11: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 12")]
    pub User_Layer_12: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 13")]
    pub User_Layer_13: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 14")]
    pub User_Layer_14: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 15")]
    pub User_Layer_15: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 16")]
    pub User_Layer_16: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 17")]
    pub User_Layer_17: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 18")]
    pub User_Layer_18: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 19")]
    pub User_Layer_19: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 20")]
    pub User_Layer_20: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 21")]
    pub User_Layer_21: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 22")]
    pub User_Layer_22: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 23")]
    pub User_Layer_23: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 24")]
    pub User_Layer_24: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 25")]
    pub User_Layer_25: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 26")]
    pub User_Layer_26: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 27")]
    pub User_Layer_27: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 28")]
    pub User_Layer_28: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 29")]
    pub User_Layer_29: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 30")]
    pub User_Layer_30: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 31")]
    pub User_Layer_31: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 8")]
    pub User_Layer_8: Option<String>,
    /// String: (3.4.0 - 4.7.2)
    #[serde(alias = "User Layer 9")]
    pub User_Layer_9: Option<String>,
    /// Vec<String>: (5.0.0f4 - 6000.2.0a6)
    pub layers: Option<Vec<String>>,
    /// Vec<String>: (2023.3.0b1 - 6000.2.0a6)
    pub m_RenderingLayers: Option<Vec<String>>,
    /// Vec<SortingLayerEntry>: (4.3.0 - 6000.2.0a6)
    pub m_SortingLayers: Option<Vec<SortingLayerEntry>>,
}

/// TakeInfo is a sub class of the Unity engine since version 4.0.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TakeInfo.html):
/**
A Takeinfo object contains all the information needed to describe a take.
Additional resources: ModelImporter.importedTakeInfos.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TakeInfo {
    /**Start time in second.*/
    pub bakeStartTime: f32,
    /**Stop time in second.*/
    pub bakeStopTime: f32,
    /// PPtr<[`AnimationClip`]>: (4.0.0 - 6000.2.0a6)
    pub clip: PPtr,
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
    /// i64: (2019.1.0b1 - 6000.2.0a6)
    pub internalID: Option<i64>,
}

/// TargetJoint2D is a  class of the Unity engine since version 5.3.0f1.
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
    /// PPtr<[`Rigidbody2D`]>: (5.3.0f1 - 6000.2.0a6)
    pub m_ConnectedRigidBody: PPtr,
    /**The amount by which the target spring force is reduced in proportion to the movement speed.*/
    pub m_DampingRatio: f32,
    /**Should the two Rigidbody2D connected with this joint collide with each other?*/
    pub m_EnableCollision: bool,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The frequency at which the target spring oscillates around the target position.*/
    pub m_Frequency: f32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (5.3.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The maximum force that can be generated when trying to maintain the target joint constraint.*/
    pub m_MaxForce: f32,
    /**The world-space position that the joint will attempt to move the body to.*/
    pub m_Target: Vector2f,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_BreakAction: Option<i32>,
}

/// Terrain is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Terrain.html):
/**
The Terrain component renders the terrain.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Terrain {
    /**Whether to bake an array of internal light probes for Tree Editor trees (Editor only).*/
    pub m_BakeLightProbesForTrees: bool,
    pub m_ChunkDynamicUVST: Vector4f,
    /**Density of detail objects.*/
    pub m_DetailObjectDensity: f32,
    /**Detail objects will be displayed up to this distance.*/
    pub m_DetailObjectDistance: f32,
    /**Indicates whether Unity draws the Terrain geometry itself.*/
    pub m_DrawHeightmap: bool,
    /**Specify if terrain trees and details should be drawn. If disabled, this will also disable updates to renderer positions for trees and details. Tree and detail renderer positions will update again once this setting is re-enabled.*/
    pub m_DrawTreesAndFoliage: bool,
    pub m_DynamicUVST: Vector4f,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Limits the Terrain's maximum rendering resolution.*/
    pub m_HeightmapMaximumLOD: i32,
    /**An approximation of how many pixels the terrain will pop in the worst case when switching lod.*/
    pub m_HeightmapPixelError: f32,
    /**The index of the baked lightmap applied to this terrain.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    /**The custom material Unity uses to render the Terrain.*/
    /// PPtr<[`Material`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_MaterialTemplate: PPtr,
    /**How reflection probes are used for terrain. See ReflectionProbeUsage.*/
    pub m_ReflectionProbeUsage: i32,
    pub m_SplatMapDistance: f32,
    /**The Terrain Data that stores heightmaps, terrain textures, detail meshes and trees.*/
    /// PPtr<[`TerrainData`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_TerrainData: PPtr,
    /**Distance from the camera where trees will be rendered as billboards only.*/
    pub m_TreeBillboardDistance: f32,
    /**Total distance delta that trees will use to transition from billboard orientation to mesh orientation.*/
    pub m_TreeCrossFadeLength: f32,
    /**The maximum distance at which trees are rendered.*/
    pub m_TreeDistance: f32,
    /**Maximum number of trees rendered at full LOD.*/
    pub m_TreeMaximumFullLODCount: i32,
    /**Specifies if the terrain tile will be automatically connected to adjacent tiles.*/
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_AllowAutoConnect: Option<bool>,
    /// bool: (5.0.0f4 - 2018.4.36f1)
    pub m_CastShadows: Option<bool>,
    /// f32: (5.0.0f4 - 5.0.0f4)
    pub m_DefaultSmoothness: Option<f32>,
    /**Set to true to enable the terrain instance renderer. The default value is false.*/
    /// bool: (2018.3.0b1 - 6000.2.0a6)
    pub m_DrawInstanced: Option<bool>,
    /**When this options is enabled, Terrain heightmap geometries will be added in acceleration structures used for Ray Tracing.*/
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_EnableHeightmapRayTracing: Option<bool>,
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_EnableTreesAndDetailsRayTracing: Option<bool>,
    /// Hash128: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExplicitProbeSetHash: Option<Hash128>,
    /**Grouping ID for auto connect.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_GroupingID: Option<i32>,
    /**Limits how simplified the rendered terrain can be.*/
    /// i32: (2022.3.31f1 - 6000.2.0a6)
    pub m_HeightmapMinimumLODSimplification: Option<i32>,
    /**When enabled, the terrain ignores the terrain overrides set in the QualitySettings.*/
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_IgnoreQualitySettings: Option<bool>,
    /// f32: (5.0.0f4 - 2019.2.0a7)
    pub m_LegacyShininess: Option<f32>,
    /// ColorRGBA: (5.0.0f4 - 2019.2.0a7)
    pub m_LegacySpecular: Option<ColorRGBA>,
    /// i32: (5.0.0f4 - 2019.2.0a7)
    pub m_MaterialType: Option<i32>,
    /**Allows you to specify how Unity chooses the layer for tree instances.*/
    /// bool: (2018.2.0f1 - 6000.2.0a6)
    pub m_PreserveTreePrototypeLayers: Option<bool>,
    /**Determines which rendering layers the Terrain renderer lives on.*/
    /// u32: (2019.3.0b1 - 6000.2.0a6)
    pub m_RenderingLayerMask: Option<u32>,
    /**Allows you to set the shadow casting mode for the terrain.*/
    /// i32: (2019.1.0b1 - 6000.2.0a6)
    pub m_ShadowCastingMode: Option<i32>,
    /// bool: (2021.1.0b1 - 6000.2.0a6)
    pub m_StaticShadowCaster: Option<bool>,
    /**The motion vector rendering mode for all SpeedTree models painted on the terrain.*/
    /// i32: (2022.3.3f1 - 6000.2.0a6)
    pub m_TreeMotionVectorModeOverride: Option<i32>,
    /// bool: (5.0.0f4 - 5.0.0f4)
    pub m_UseDefaultSmoothness: Option<bool>,
}

/// TerrainCollider is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TerrainCollider.html):
/**
A heightmap based collider.
Additional resources: SphereCollider, CapsuleCollider, PhysicsMaterial, Rigidbody.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainCollider {
    /**Enabled Colliders will collide with other Colliders, disabled Colliders won't.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The terrain that stores the heightmap.*/
    /// PPtr<[`TerrainData`]>: (3.4.0 - 6000.2.0a6)
    pub m_TerrainData: PPtr,
    /// bool: (3.4.0 - 4.7.2)
    pub m_CreateTreeColliders: Option<bool>,
    /// bool: (5.0.0f4 - 6000.2.0a6)
    pub m_EnableTreeColliders: Option<bool>,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**Specify if this collider is configured as a trigger.*/
    /// bool: (3.4.0 - 4.7.2)
    pub m_IsTrigger: Option<bool>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /**The material used by the collider.*/
    /// PPtr<[`PhysicMaterial`]>: (3.4.0 - 2023.3.0a8); PPtr<[`PhysicsMaterial`]>: (2023.3.0b1 - 6000.2.0a6)
    pub m_Material: Option<PPtr>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b1 - 6000.2.0a6)
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
    /// Vec<PPtr<[`Shader`]>>: (2018.4.14f1 - 6000.2.0a6)
    pub m_PreloadShaders: Option<Vec<PPtr>>,
}

/// TerrainLayer is a  class of the Unity engine since version 2018.3.0b1.
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
    /// PPtr<[`Texture2D`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_DiffuseTexture: PPtr,
    /**A Vector4 value specifying the maximum RGBA value that the mask map texture maps to when the value of the channel is 1.*/
    pub m_MaskMapRemapMax: Vector4f,
    /**A Vector4 value specifying the minimum RGBA value that the mask map texture maps to when the value of the channel is 0.*/
    pub m_MaskMapRemapMin: Vector4f,
    /**The mask map texture used by the terrain layer.*/
    /// PPtr<[`Texture2D`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_MaskMapTexture: PPtr,
    /**Metallic factor used by the terrain layer.*/
    pub m_Metallic: f32,
    /**The name of the object.*/
    pub m_Name: String,
    /**Normal map texture used by the terrain layer.*/
    /// PPtr<[`Texture2D`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_NormalMapTexture: PPtr,
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
    /**Choose the source for smoothness value.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_SmoothnessSource: Option<i32>,
}

/// TestObjectVectorPairStringBool is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectVectorPairStringBool {
    pub m_Map: Vec<(String, bool)>,
    pub m_String: String,
}

/// TestObjectWithSerializedAnimationCurve is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectWithSerializedAnimationCurve {
    pub m_Curve: AnimationCurve,
}

/// TestObjectWithSerializedArray is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectWithSerializedArray {
    pub m_ClampTestValue: f32,
    pub m_IntegerArray: Vec<i32>,
}

/// TestObjectWithSerializedMapStringBool is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectWithSerializedMapStringBool {
    pub m_Map: Vec<(String, bool)>,
    pub m_String: String,
}

/// TestObjectWithSerializedMapStringNonAlignedStruct is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectWithSerializedMapStringNonAlignedStruct {
    pub m_Map: Vec<(String, NonAlignedStruct)>,
    pub m_String: String,
}

/// TestObjectWithSpecialLayoutOne is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectWithSpecialLayoutOne {
    pub differentLayout: LayoutDataOne,
    pub sameLayout: LayoutDataOne,
}

/// TestObjectWithSpecialLayoutTwo is a  class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestObjectWithSpecialLayoutTwo {
    pub differentLayout: LayoutDataTwo,
    pub sameLayout: LayoutDataThree,
}

/// Tetrahedron is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tetrahedron {
    pub matrix: Matrix3x4f,
    /// i32: (3.5.0 - 6000.2.0a6)
    #[serde(alias = "indices[0]")]
    pub indices_0_: Option<i32>,
    /// i32: (3.5.0 - 6000.2.0a6)
    #[serde(alias = "indices[1]")]
    pub indices_1_: Option<i32>,
    /// i32: (3.5.0 - 6000.2.0a6)
    #[serde(alias = "indices[2]")]
    pub indices_2_: Option<i32>,
    /// i32: (3.5.0 - 6000.2.0a6)
    #[serde(alias = "indices[3]")]
    pub indices_3_: Option<i32>,
    /// i32: (3.5.0 - 6000.2.0a6)
    #[serde(alias = "neighbors[0]")]
    pub neighbors_0_: Option<i32>,
    /// i32: (3.5.0 - 6000.2.0a6)
    #[serde(alias = "neighbors[1]")]
    pub neighbors_1_: Option<i32>,
    /// i32: (3.5.0 - 6000.2.0a6)
    #[serde(alias = "neighbors[2]")]
    pub neighbors_2_: Option<i32>,
    /// i32: (3.5.0 - 6000.2.0a6)
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
    /// String: (3.4.0 - 2017.1.0b1)
    pub m_PathName: Option<String>,
}

/// TextMesh is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TextMesh.html):
/**
The TextMesh component allows you to display text in 3D text mesh component.This component dynamically generates a mesh that fits the text specified as input, it is great to make world space UI like displaying names above characters like the example below.Note that Text Mesh Pro is now the preferred solution for creating 3D text as it's more feature complete compared to TextMesh.
Additional resources: text mesh component.
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
    /// PPtr<[`Font`]>: (3.4.0 - 6000.2.0a6)
    pub m_Font: PPtr,
    /**The font size to use (for dynamic fonts).*/
    pub m_FontSize: i32,
    /**The font style to use (for dynamic fonts).*/
    pub m_FontStyle: i32,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**How much space will be in-between lines of text.*/
    pub m_LineSpacing: f32,
    /**How far should the text be offset from the transform.position.z when drawing.*/
    pub m_OffsetZ: f32,
    /**How much space will be inserted for a tab '\t' character. This is a multiplum of the 'spacebar' character offset.*/
    pub m_TabSize: f32,
    /**The specified input text to display.*/
    pub m_Text: String,
    /**The color used to render the text.*/
    /// ColorRGBA: (4.2.0 - 6000.2.0a6)
    pub m_Color: Option<ColorRGBA>,
    /**Enable HTML-style tags for Text Formatting Markup.*/
    /// bool: (4.0.0 - 6000.2.0a6)
    pub m_RichText: Option<bool>,
}

/// TextScriptImporter is a  class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextScriptImporter {
    pub m_Name: String,
    pub m_UserData: String,
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleName: Option<String>,
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleVariant: Option<String>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// Texture is a  class of the Unity engine since version 5.0.0f4.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Texture.html):
/**
Base class for Texture handling.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Texture {
    /**The name of the object.*/
    pub m_Name: String,
    /// bool: (2017.3.0b1 - 2023.2.0a17)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.3.0b1 - 2023.2.0a17)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
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
    /// Vec<u8>: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "image data")]
    pub image_data: Option<Vec<u8>>,
    /// i32: (3.5.0 - 6000.2.0a6)
    pub m_ColorSpace: Option<i32>,
    /// bool: (2017.3.0b1 - 2023.2.0a17)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.3.0b1 - 2023.2.0a17)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2019.3.0f5 - 2023.1.0a6)
    pub m_IgnoreMasterTextureLimit: Option<bool>,
    /**This property causes a texture to ignore all texture mipmap limit settings.*/
    /// bool: (2022.2.0f1 - 6000.2.0a6)
    pub m_IgnoreMipmapLimit: Option<bool>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// bool: (2019.4.9f1 - 6000.2.0a6)
    pub m_IsPreProcessed: Option<bool>,
    /// i32: (5.2.0f2 - 6000.2.0a6)
    pub m_MipCount: Option<i32>,
    /// bool: (3.4.0 - 5.1.5f1)
    pub m_MipMap: Option<bool>,
    /// String: (2022.2.0f1 - 6000.2.0a6)
    pub m_MipmapLimitGroupName: Option<String>,
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub m_MipsStripped: Option<i32>,
    /// Vec<u8>: (2020.2.0b1 - 6000.2.0a6)
    pub m_PlatformBlob: Option<Vec<u8>>,
    /// bool: (3.4.0 - 5.4.6f3)
    pub m_ReadAllowed: Option<bool>,
    /// StreamingInfo: (5.3.0f1 - 6000.2.0a6)
    pub m_StreamData: Option<StreamingInfo>,
    /**Determines whether mipmap streaming is enabled for this Texture.*/
    /// bool: (2018.2.0b1 - 6000.2.0a6)
    pub m_StreamingMipmaps: Option<bool>,
    /**Sets the relative priority for this Texture when reducing memory size to fit within the memory budget.*/
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub m_StreamingMipmapsPriority: Option<i32>,
}

/// Texture2DArray is a  class of the Unity engine since version 5.4.0f3.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Texture2DArray.html):
/**
Class for handling 2D texture arrays.
Modern graphics APIs (e.g. D3D10 and later, OpenGL ES 3.0 and later, Metal etc.) support "texture arrays", which is an array of same size & format textures.
From the shader side, they are treated as a single resource, and sampling them needs an extra coordinate that indicates which array element to sample from.Typically texture arrays are useful as an alternative for texture atlases, or in other cases where objects use a set of same-sized textures (e.g. terrains).Currently in Unity texture arrays do not have an import pipeline for them, and must be created from code, either at runtime or in editor scripts.
Using Graphics.CopyTexture is useful for fast copying of pixel data from regular 2D textures into elements of a texture array. From editor scripts,
a common way of creating serialized texture array is to create it, fill with data (either via Graphics.CopyTexture from regular 2D textures, or via SetPixels or
SetPixels32) and save it as an asset via AssetDatabase.CreateAsset.Note that not all platforms and GPUs support texture arrays; for example Direct3D9 does not. Use SystemInfo.supports2DArrayTextures to check. Also, this class does not support Texture2DArray creation with a Crunch compression TextureFormat.
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
    pub m_TextureSettings: GLTextureSettings,
    /**Width of the Texture in pixels (Read Only).*/
    pub m_Width: i32,
    /// Vec<u8>: (5.4.0f3 - 6000.2.0a6)
    #[serde(alias = "image data")]
    pub image_data: Option<Vec<u8>>,
    /// bool: (2017.3.0b1 - 2023.2.0a17)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.3.0b1 - 2023.2.0a17)
    pub m_ForcedFallbackFormat: Option<i32>,
    /**This property causes a texture to ignore all texture mipmap limit settings.*/
    /// bool: (2023.2.0b1 - 6000.2.0a6)
    pub m_IgnoreMipmapLimit: Option<bool>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /// String: (2023.2.0b1 - 6000.2.0a6)
    pub m_MipmapLimitGroupName: Option<String>,
    /// i32: (2023.2.0b1 - 6000.2.0a6)
    pub m_MipsStripped: Option<i32>,
    /// StreamingInfo: (5.6.0b1 - 6000.2.0a6)
    pub m_StreamData: Option<StreamingInfo>,
    /// i32: (2020.2.0b1 - 6000.2.0a6)
    pub m_UsageMode: Option<i32>,
}

/// Texture3D is a  class of the Unity engine since version 4.0.0.
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
    pub m_Format: i64,
    /**Height of the Texture in pixels (Read Only).*/
    pub m_Height: i32,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_TextureSettings: GLTextureSettings,
    /**Width of the Texture in pixels (Read Only).*/
    pub m_Width: i32,
    /// Vec<u8>: (4.0.0 - 6000.2.0a6)
    #[serde(alias = "image data")]
    pub image_data: Option<Vec<u8>>,
    /// i32: (2019.1.0b1 - 6000.2.0a6)
    pub m_ColorSpace: Option<i32>,
    /// bool: (2017.3.0b1 - 2023.2.0a17)
    pub m_DownscaleFallback: Option<bool>,
    /// i32: (2017.3.0b1 - 2023.2.0a17)
    pub m_ForcedFallbackFormat: Option<i32>,
    /// bool: (2020.2.0b1 - 6000.2.0a6)
    pub m_IsAlphaChannelOptional: Option<bool>,
    /**Whether Unity stores an additional copy of this texture's pixel data in CPU-addressable memory.*/
    /// bool: (5.4.0f3 - 6000.2.0a6)
    pub m_IsReadable: Option<bool>,
    /// i32: (5.2.0f2 - 6000.2.0a6)
    pub m_MipCount: Option<i32>,
    /// bool: (4.0.0 - 5.1.5f1)
    pub m_MipMap: Option<bool>,
    /// StreamingInfo: (5.6.0b1 - 6000.2.0a6)
    pub m_StreamData: Option<StreamingInfo>,
    /// i32: (2020.2.0b1 - 6000.2.0a6)
    pub m_UsageMode: Option<i32>,
}

/// TextureImportInstructions is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextureImportInstructions {
    pub colorSpace: i32,
    pub compressedFormat: i32,
    pub compressionQuality: i32,
    pub height: i32,
    pub uncompressedFormat: i32,
    pub usageMode: i32,
    pub width: i32,
    /// bool: (2017.3.0b1 - 2023.2.0a17)
    pub androidETC2FallbackDownscale: Option<bool>,
    /// i32: (2017.3.0b1 - 2023.2.0a17)
    pub androidETC2FallbackFormat: Option<i32>,
    /// i32: (2020.2.0b1 - 6000.2.0a6)
    pub cubeIntermediateSize: Option<i32>,
    /// i32: (2020.2.0b1 - 6000.2.0a6)
    pub cubeLayout: Option<i32>,
    /// i32: (2020.2.0b1 - 6000.2.0a6)
    pub cubeMode: Option<i32>,
    /// i32: (2020.2.0b1 - 6000.2.0a6)
    pub depth: Option<i32>,
    /// i32: (4.3.0 - 6000.2.0a6)
    pub desiredFormat: Option<i32>,
    /// i32: (4.0.0 - 5.4.6f3)
    pub recommendedFormat: Option<i32>,
    /// bool: (2020.1.0b1 - 6000.2.0a6)
    pub vtOnly: Option<bool>,
}

/// TextureImportOutput is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextureImportOutput {
    pub sourceTextureInformation: SourceTextureInformation,
    pub textureImportInstructions: TextureImportInstructions,
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub importInspectorWarnings: Option<String>,
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
    /// i32: (3.5.0 - 5.4.6f3)
    pub correctGamma: Option<i32>,
    /// i32: (4.3.0 - 6000.2.0a6)
    pub m_Alignment: Option<i32>,
    /// i32: (5.2.0f2 - 5.4.3f1)
    pub m_AllowsAlphaSplitting: Option<i32>,
    /**If the alpha channel of your texture represents transparency, enable this property to dilate the color channels of visible texels into fully transparent areas. This effectively adds padding around transparent areas that prevents filtering artifacts from forming on their edges. Unity does not support this property for HDR textures.This property makes the color data of invisible texels undefined. Disable this property to preserve invisible texels' original color data.*/
    /// i32: (4.2.0 - 6000.2.0a6)
    pub m_AlphaIsTransparency: Option<i32>,
    /**Returns or assigns the alpha test reference value.*/
    /// f32: (2017.1.0b1 - 6000.2.0a6)
    pub m_AlphaTestReferenceValue: Option<f32>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_AlphaUsage: Option<i32>,
    /// i32: (2019.3.6f1 - 6000.2.0a6)
    pub m_ApplyGammaDecoding: Option<i32>,
    /**Get or set the AssetBundle name.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleName: Option<String>,
    /**Get or set the AssetBundle variant.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleVariant: Option<String>,
    /// Vec<BuildTargetSettings>: (3.4.0 - 5.4.6f3)
    pub m_BuildTargetSettings: Option<Vec<BuildTargetSettings>>,
    /**The quality of the texture after Crunch compresses it. The range is 0 through 100. A higher value means a larger, better-quality texture, but a longer compression time because Crunch needs more time to try different texture encodings.*/
    /// i32: (3.5.0 - 6000.2.0a6)
    pub m_CompressionQuality: Option<i32>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_CompressionQualitySet: Option<i32>,
    /// i32: (2020.3.38f1 - 6000.2.0a6)
    pub m_CookieLightType: Option<i32>,
    /// i32: (3.4.0 - 3.4.2)
    pub m_CorrectGamma: Option<i32>,
    /// i32: (5.0.0f4 - 6000.2.0a6)
    pub m_CubemapConvolution: Option<i32>,
    /// f32: (5.0.0f4 - 5.4.6f3)
    pub m_CubemapConvolutionExponent: Option<f32>,
    /// i32: (5.0.0f4 - 5.4.6f3)
    pub m_CubemapConvolutionSteps: Option<i32>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// Vec<(i32, String)>: (3.4.0 - 4.7.2); Vec<(i64, String)>: (5.0.0f4 - 2018.4.36f1)
    pub m_FileIDToRecycleName: Option<Vec<(i64, String)>>,
    /**Indicates whether to invert the green channel values of a normal map.*/
    /// i32: (2022.1.0b1 - 6000.2.0a6)
    pub m_FlipGreenChannel: Option<i32>,
    /// i32: (2020.2.0b1 - 6000.2.0a6)
    pub m_FlipbookColumns: Option<i32>,
    /// i32: (2020.2.0b1 - 6000.2.0a6)
    pub m_FlipbookRows: Option<i32>,
    /// i32: (2021.2.0b1 - 2023.1.0a6)
    pub m_IgnoreMasterTextureLimit: Option<i32>,
    /**Enable this flag for textures that should ignore mipmap limit settings.*/
    /// i32: (2022.2.0f1 - 6000.2.0a6)
    pub m_IgnoreMipmapLimit: Option<i32>,
    /**Ignore the Gamma attribute in PNG files. This property does not effect other file formats.*/
    /// i32: (2020.1.0b1 - 6000.2.0a6); bool: (2020.1.0a3 - 2020.1.0a9)
    pub m_IgnorePngGamma: Option<Enum_i32__bool>,
    /// Vec<((i32, i64), String)>: (2019.1.0b1 - 6000.2.0a6)
    pub m_InternalIDToNameTable: Option<Vec<((i32, i64), String)>>,
    /// i32: (3.5.0 - 6000.2.0a6)
    pub m_LinearTexture: Option<i32>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_MaxTextureSizeSet: Option<i32>,
    /**Enables or disables coverage-preserving alpha mipmapping.*/
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub m_MipMapsPreserveCoverage: Option<i32>,
    /**Name of the texture mipmap limit group to which this texture belongs.*/
    /// String: (2022.2.0f1 - 6000.2.0a6)
    pub m_MipmapLimitGroupName: Option<String>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_NewHashIdentity: Option<MdFour>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_OldHashIdentity: Option<MdFour>,
    /// TextureImportOutput: (4.0.0 - 6000.2.0a6)
    pub m_Output: Option<TextureImportOutput>,
    /// bool: (2018.2.1f1 - 6000.2.0a6)
    pub m_PSDRemoveMatte: Option<bool>,
    /// bool: (2018.2.1f1 - 2022.1.0a12)
    pub m_PSDShowRemoveMatteOption: Option<bool>,
    /// Vec<PlatformSettings>: (5.5.0f3 - 2017.2.5f1); Vec<TextureImporterPlatformSettings>: (2017.3.0b1 - 6000.2.0a6)
    pub m_PlatformSettings: Option<Vec<Enum_PlatformSettings__TextureImporterPlatformSettings>>,
    /// i32: (2018.2.0b2 - 2018.2.0b2)
    pub m_PushPullDilation: Option<i32>,
    /// i32: (5.0.0f4 - 5.4.6f3)
    pub m_RGBM: Option<i32>,
    /// i32: (3.4.0 - 3.5.7)
    pub m_RecommendedTextureFormat: Option<i32>,
    /// i32: (4.0.0 - 6000.2.0a6)
    pub m_SeamlessCubemap: Option<i32>,
    /// i32: (2018.1.0f1 - 6000.2.0a6)
    pub m_SingleChannelComponent: Option<i32>,
    /// SourceTextureInformation: (3.4.0 - 3.5.7)
    pub m_SourceTextureInformation: Option<SourceTextureInformation>,
    /**Border sizes of the generated sprites.*/
    /// Vector4f: (4.5.0 - 6000.2.0a6)
    pub m_SpriteBorder: Option<Vector4f>,
    /// u32: (4.3.0 - 6000.2.0a6)
    pub m_SpriteExtrude: Option<u32>,
    /// i32: (2017.3.1p2 - 6000.2.0a6)
    pub m_SpriteGenerateFallbackPhysicsShape: Option<i32>,
    /// i32: (4.3.0 - 6000.2.0a6)
    pub m_SpriteMeshType: Option<i32>,
    /// i32: (4.3.0 - 6000.2.0a6)
    pub m_SpriteMode: Option<i32>,
    /// String: (4.3.0 - 2022.1.24f1)
    pub m_SpritePackingTag: Option<String>,
    /**The point in the Sprite object's coordinate space where the graphic is located.*/
    /// Vector2f: (4.3.0 - 6000.2.0a6)
    pub m_SpritePivot: Option<Vector2f>,
    /// f32: (4.3.0 - 6000.2.0a6)
    pub m_SpritePixelsToUnits: Option<f32>,
    /// SpriteSheetMetaData: (4.3.0 - 6000.2.0a6)
    pub m_SpriteSheet: Option<SpriteSheetMetaData>,
    /// f32: (5.4.0f3 - 6000.2.0a6)
    pub m_SpriteTessellationDetail: Option<f32>,
    /**Enable mipmap streaming for this texture.*/
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub m_StreamingMipmaps: Option<i32>,
    /**Relative priority for this texture when reducing memory size in order to hit the memory budget.*/
    /// i32: (2018.2.0b1 - 6000.2.0a6)
    pub m_StreamingMipmapsPriority: Option<i32>,
    /// u32: (2022.1.0b1 - 6000.2.0a6)
    pub m_Swizzle: Option<u32>,
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_TextureFormatSet: Option<i32>,
    /**The shape of the imported texture.*/
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_TextureShape: Option<i32>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /**Get or set any user data.*/
    /// String: (4.0.0 - 6000.2.0a6)
    pub m_UserData: Option<String>,
    /**When enabled, this texture can solely be used in combination with a Texture Stack for Virtual Texturing. When enabled the texture is not guaranteed to be available as a Texture2D in the Player (e.g., not accessible from a script). When disabled, the Player includes the texture both as a Texture2D (e.g., accessible from script) and as a streamable texture in a Texture Stack.*/
    /// i32: (2020.1.0b1 - 6000.2.0a6)
    pub m_VTOnly: Option<i32>,
    /**Whether this texture stores data in sRGB (also called gamma) color space.*/
    /// i32: (5.5.0f3 - 6000.2.0a6)
    pub m_sRGBTexture: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_i32__bool {
    i32(i32),
    bool(bool),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_PlatformSettings__TextureImporterPlatformSettings {
    PlatformSettings(PlatformSettings),
    TextureImporterPlatformSettings(TextureImporterPlatformSettings),
}

/// TextureImporterPlatformSettings is a sub class of the Unity engine since version 2017.3.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/TextureImporterPlatformSettings.html):
/**
Stores platform specifics settings of a TextureImporter.
Additional resources: TextureImporter.
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
    /// bool: (2019.2.0b1 - 6000.2.0a6)
    pub m_ForceMaximumCompressionQuality_BC6H_BC7: Option<bool>,
    /**Ignores platform support checks for the selected texture format.*/
    /// bool: (2022.2.20f1 - 6000.2.0a6)
    pub m_IgnorePlatformSupport: Option<bool>,
}

/// TextureParameter is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextureParameter {
    pub m_Dim: i8,
    pub m_Index: i32,
    pub m_NameIndex: i32,
    pub m_SamplerIndex: i32,
    /// bool: (2017.3.0b1 - 6000.2.0a6)
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

/// TextureSettings is a sub class of the Unity engine since version 2020.1.0b1.
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

/// TicksPerSecond is a sub class of the Unity engine since version 2023.1.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Unity.IntegerTime.RationalTime.TicksPerSecond.html):
/**
The number of ticks per second ( rate ) of the discrete time, expressed as a rational number.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TicksPerSecond {
    /**Returns the denominator.*/
    pub m_Denominator: u32,
    /**Returns the numerator.*/
    pub m_Numerator: u32,
}

/// TierGraphicsSettings is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct TierGraphicsSettings {
    pub renderingPath: i32,
    pub useCascadedShadowMaps: bool,
    /// bool: (5.6.3f1 - 6000.2.0a6)
    pub enableLPPV: Option<bool>,
    /// i32: (5.6.0b1 - 6000.2.0a6)
    pub hdrMode: Option<i32>,
    /// bool: (2017.1.0p4 - 6000.2.0a6)
    pub prefer32BitShadowMaps: Option<bool>,
    /// i32: (5.6.0f1 - 6000.2.0a6)
    pub realtimeGICPUUsage: Option<i32>,
    /// bool: (5.6.0b1 - 6000.2.0a6)
    pub useHDR: Option<bool>,
}

/// Tile is a sub class of the Unity engine since version 2017.2.0f1.
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
    /// u16: (2019.4.24f1 - 6000.2.0a6)
    pub dummyAlignment: Option<u16>,
    /// u32: (2019.3.0b1 - 6000.2.0a6)
    pub m_AllTileFlags: Option<u32>,
    /// i32: (2017.2.0f1 - 2019.3.0a11)
    pub m_ColliderType: Option<i32>,
    /// PPtr<[`GameObject`]>: (2017.2.0f1 - 2019.3.0a11)
    pub m_ObjectToInstantiate: Option<PPtr>,
    /// i32: (2017.2.0f1 - 2019.3.0a11); u32: (2017.2.0b2 - 2017.2.0b6)
    pub m_TileFlags: Option<i64>,
    /// u16: (2019.3.0b1 - 6000.2.0a6)
    pub m_TileObjectToInstantiateIndex: Option<u16>,
}

/// TileAnimationData is a sub class of the Unity engine since version 2017.2.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Tilemaps.TileAnimationData.html):
/**
A Struct for the required data for animating a Tile.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TileAnimationData {
    /**The array of sprites that are ordered by appearance in the animation.*/
    /// Vec<PPtr<[`Sprite`]>>: (2017.2.0f1 - 6000.2.0a6)
    pub m_AnimatedSprites: Vec<PPtr>,
    /**The animation speed.*/
    pub m_AnimationSpeed: f32,
    pub m_AnimationTimeOffset: f32,
    /**TileAnimationFlags for controlling the Tile Animation.*/
    /// u32: (2022.2.0b1 - 6000.2.0a6)
    pub m_Flags: Option<u32>,
    /// bool: (2017.2.0f1 - 2022.1.24f1)
    pub m_IsLooping: Option<bool>,
}

/// Tilemap is a  class of the Unity engine since version 2017.2.0f1.
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
    /// PPtr<[`GameObject`]>: (2017.2.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /// Vec<TilemapRefCountedData>: (2019.3.0b1 - 6000.2.0a6)
    pub m_TileObjectToInstantiateArray: Option<Vec<TilemapRefCountedData>>,
}

/// TilemapCollider2D is a  class of the Unity engine since version 2017.2.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Tilemaps.TilemapCollider2D.html):
/**
Collider for 2D physics representing shapes defined by the corresponding Tilemap.
Additional resources: BoxCollider2D, CircleCollider2D, EdgeCollider2D, PolygonCollider2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TilemapCollider2D {
    /**The density of the collider used to calculate its mass (when auto mass is enabled).*/
    pub m_Density: f32,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (2017.2.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Is this collider configured as a trigger?*/
    pub m_IsTrigger: bool,
    /// PPtr<[`PhysicsMaterial2D`]>: (2017.2.0f1 - 6000.2.0a6)
    pub m_Material: PPtr,
    /**The local offset of the collider geometry.*/
    pub m_Offset: Vector2f,
    /**Whether the collider is used by an attached effector or not.*/
    pub m_UsedByEffector: bool,
    /**The Layers that this Collider2D will report collision or trigger callbacks for during a contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_CallbackLayers: Option<BitField>,
    /**The composite operation to be used by a CompositeCollider2D.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOperation: Option<i32>,
    /**The composite operation order to be used when a CompositeCollider2D is used.*/
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub m_CompositeOrder: Option<i32>,
    /**The layers of other Collider2D involved in contacts with this Collider2D that will be captured.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ContactCaptureLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should exclude when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**The amount of Collider shapes each Tile extrudes to facilitate compositing with neighboring Tiles. This eliminates fine gaps between Tiles when using a CompositeCollider2D. This is calculated in Unity units within world space.*/
    /// f32: (2019.1.3f1 - 6000.2.0a6)
    pub m_ExtrusionFactor: Option<f32>,
    /**The Layers that this Collider2D can receive forces from during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceReceiveLayers: Option<BitField>,
    /**The Layers that this Collider2D is allowed to send forces to during a Collision contact with another Collider2D.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ForceSendLayers: Option<BitField>,
    /**The additional Layers that this Collider2D should include when deciding if a contact with another Collider2D should happen or not.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider2D used when there is a conflicting decision on whether a contact between itself and another Collision2D should happen or not.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /**Maximum number of Tile Changes accumulated before doing a full collider rebuild instead of an incremental rebuild.*/
    /// u32: (2019.1.0b1 - 6000.2.0a6)
    pub m_MaximumTileChangeCount: Option<u32>,
    /**When the value is true, the Collider uses an additional Delaunay triangulation step to produce the Collider mesh. When the value is false, this additional step does not occur.*/
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_UseDelaunayMesh: Option<bool>,
    /// bool: (2017.2.0f1 - 2023.1.0a21)
    pub m_UsedByComposite: Option<bool>,
}

/// TilemapEditorUserSettings is a  class of the Unity engine since version 2017.2.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct TilemapEditorUserSettings {
    pub m_FocusMode: i32,
    /// PPtr<[`GameObject`]>: (2017.2.0f1 - 2019.2.0a6)
    pub m_LastUsedPalette: PPtr,
}

/// TilemapRefCountedData is a sub class of the Unity engine since version 2017.2.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct TilemapRefCountedData {
    /// PPtr<[`Object`]>: (2017.2.0f1 - 6000.2.0a6)
    pub m_Data: PPtr,
    pub m_RefCount: u32,
}

/// TilemapRenderer is a  class of the Unity engine since version 2017.2.0f1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Tilemaps.TilemapRenderer.html):
/**
The tile map renderer is used to render the tile map marked out by a tile map component and a grid component.
This class is a script interface for a tile map renderer component.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TilemapRenderer {
    pub m_CastShadows: u8,
    /**Size in number of tiles of each chunk created by the TilemapRenderer.*/
    pub m_ChunkSize: int3_storage,
    pub m_DynamicOccludee: u8,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (2017.2.0f1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The light probe interpolation type.*/
    pub m_LightProbeUsage: u8,
    /// PPtr<[`GameObject`]>: (2017.2.0f1 - 6000.2.0a6)
    pub m_LightProbeVolumeOverride: PPtr,
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    /**Specifies how the Tilemap interacts with the masks.*/
    pub m_MaskInteraction: i32,
    /**Returns all the instantiated materials of this object.*/
    /// Vec<PPtr<[`Material`]>>: (2017.2.0f1 - 6000.2.0a6)
    pub m_Materials: Vec<PPtr>,
    /**Maximum number of chunks the TilemapRenderer caches in memory.*/
    pub m_MaxChunkCount: u32,
    /**Maximum number of frames the TilemapRenderer keeps unused chunks in memory.*/
    pub m_MaxFrameAge: u32,
    pub m_MotionVectors: u8,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr<[`Transform`]>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ProbeAnchor: PPtr,
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
    /// PPtr<[`Transform`]>: (2017.2.0f1 - 6000.2.0a6)
    pub m_StaticBatchRoot: PPtr,
    /**Bounds used for culling of Tilemap chunks.*/
    /// Vector3f: (2017.4.33f1 - 6000.2.0a6)
    pub m_ChunkCullingBounds: Option<Vector3f>,
    /**Returns whether the TilemapRenderer automatically detects the bounds to extend chunk culling by.*/
    /// i32: (2018.1.0f1 - 6000.2.0a6)
    pub m_DetectChunkCullingBounds: Option<i32>,
    /**The mode in which the TileMapRenderer batches the tiles for rendering.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_Mode: Option<i32>,
    /// u8: (2020.1.0b1 - 6000.2.0a6)
    pub m_RayTraceProcedural: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlags: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlagsOverride: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0b1 - 6000.2.0a6)
    pub m_RayTracingMode: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on, if you use a scriptable render pipeline.*/
    /// u32: (2018.1.0f1 - 6000.2.0a6)
    pub m_RenderingLayerMask: Option<u32>,
    /// u8: (2023.3.0b1 - 6000.2.0a6)
    pub m_SmallMeshCulling: Option<u8>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.1.0b1 - 6000.2.0a6)
    pub m_StaticShadowCaster: Option<u8>,
}

/// TimeManager is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeManager {
    pub m_TimeScale: f32,
    /// f32: (3.4.0 - 2023.1.0a14); RationalTime: (2023.1.0b1 - 6000.2.0a6)
    #[serde(alias = "Fixed Timestep")]
    pub Fixed_Timestep: Option<Enum_f32__RationalTime>,
    /// f32: (6000.2.0a6 - 6000.2.0a6)
    #[serde(alias = "Maximum Particle Timestep")]
    pub Maximum_Particle_Timestep: Option<f32>,
    /// f32: (3.4.0 - 6000.2.0a6)
    #[serde(alias = "Maximum Allowed Timestep")]
    pub Maximum_Allowed_Timestep: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_f32__RationalTime {
    f32(f32),
    RationalTime(RationalTime),
}

/// TrailModule is a sub class of the Unity engine since version 5.5.0f3.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.TrailModule.html):
/**
Script interface for the TrailsModule.
This module adds trails to your particles. For example, you can make the trails stay in the wake of particles as they move, or make them connect each particle in the system together.Additional resources: ParticleSystem, ParticleSystem.trails.
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
    /// bool: (2018.3.0f1 - 6000.2.0a6)
    pub attachRibbonsToTransform: Option<bool>,
    /**Configures the trails to generate Normals and Tangents. With this data, Scene lighting can affect the trails via Normal Maps and the Unity Standard Shader, or your own custom-built Shaders.*/
    /// bool: (2017.1.0f1 - 6000.2.0a6)
    pub generateLightingData: Option<bool>,
    /**Choose how the system generates the particle trails.*/
    /// i32: (2017.3.0b1 - 6000.2.0a6)
    pub mode: Option<i32>,
    /**Select how many lines to create through the Particle System.*/
    /// i32: (2017.3.0b1 - 6000.2.0a6)
    pub ribbonCount: Option<i32>,
    /**Apply a shadow bias to prevent self-shadowing artifacts. The specified value is the proportion of the trail width at each segment.*/
    /// f32: (2018.3.0b1 - 6000.2.0a6)
    pub shadowBias: Option<f32>,
    /**Specifies whether, if you use this system as a sub-emitter, ribbons connect particles from each parent particle independently.*/
    /// bool: (2017.3.0b1 - 6000.2.0a6)
    pub splitSubEmitterRibbons: Option<bool>,
    /**A multiplier for the UV coordinates of the trail texture.*/
    /// Vector2f: (2022.1.0b1 - 6000.2.0a6)
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapTilingOffset: Vector4f,
    /**Returns all the instantiated materials of this object.*/
    /// Vec<PPtr<[`Material`]>>: (3.4.0 - 6000.2.0a6)
    pub m_Materials: Vec<PPtr>,
    /**Set the minimum distance the trail can travel before a new vertex is added to it.*/
    pub m_MinVertexDistance: f32,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: Enum_bool__u8,
    /// PPtr<[`Transform`]>: (3.4.0 - 6000.2.0a6)
    pub m_StaticBatchRoot: PPtr,
    /**How long does the trail take to fade out.*/
    pub m_Time: f32,
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_ApplyActiveColorSpace: Option<bool>,
    /// Gradient: (3.4.0 - 5.4.6f3)
    pub m_Colors: Option<Gradient>,
    /// u8: (2017.2.0f1 - 6000.2.0a6)
    pub m_DynamicOccludee: Option<u8>,
    /**Creates trails when the GameObject moves.*/
    /// bool: (2018.2.0b1 - 6000.2.0a6)
    pub m_Emitting: Option<bool>,
    /**The width of the trail at the end of the trail.*/
    /// f32: (3.4.0 - 5.4.6f3)
    pub m_EndWidth: Option<f32>,
    /// PPtr<[`Transform`]>: (3.5.0 - 4.7.2)
    pub m_LightProbeAnchor: Option<PPtr>,
    /**The light probe interpolation type.*/
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeUsage: Option<u8>,
    /// PPtr<[`GameObject`]>: (5.4.0f3 - 6000.2.0a6)
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    /// u16: (5.0.0f4 - 6000.2.0a6)
    pub m_LightmapIndexDynamic: Option<u16>,
    /// Vector4f: (5.0.0f4 - 6000.2.0a6)
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    /**Specifies how the TrailRenderer interacts with SpriteMask.*/
    /// i32: (2022.1.0b1 - 6000.2.0a6)
    pub m_MaskInteraction: Option<i32>,
    /// u8: (5.4.0f3 - 6000.2.0a6)
    pub m_MotionVectors: Option<u8>,
    /// LineParameters: (5.5.0f3 - 6000.2.0a6)
    pub m_Parameters: Option<LineParameters>,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr<[`Transform`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_ProbeAnchor: Option<PPtr>,
    /// u8: (2020.1.0b1 - 6000.2.0a6)
    pub m_RayTraceProcedural: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlags: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlagsOverride: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0b1 - 6000.2.0a6)
    pub m_RayTracingMode: Option<u8>,
    /**Should reflection probes be used for this Renderer?*/
    /// i32: (5.0.0f4 - 5.3.8f2); u8: (5.4.0f3 - 6000.2.0a6)
    pub m_ReflectionProbeUsage: Option<i32>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on, if you use a scriptable render pipeline.*/
    /// u32: (2018.1.0f1 - 6000.2.0a6)
    pub m_RenderingLayerMask: Option<u32>,
    /// u8: (2023.3.0b1 - 6000.2.0a6)
    pub m_SmallMeshCulling: Option<u8>,
    /// i16: (4.3.0 - 6000.2.0a6)
    pub m_SortingLayer: Option<i16>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// u32: (4.5.0 - 4.7.2); i32: (5.0.0f4 - 6000.2.0a6)
    pub m_SortingLayerID: Option<i64>,
    /**Renderer's order within a sorting layer.*/
    /// i16: (4.3.0 - 6000.2.0a6)
    pub m_SortingOrder: Option<i16>,
    /**The width of the trail at the spawning point.*/
    /// f32: (3.4.0 - 5.4.6f3)
    pub m_StartWidth: Option<f32>,
    /// StaticBatchInfo: (5.5.0f3 - 6000.2.0a6)
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.1.0b1 - 6000.2.0a6)
    pub m_StaticShadowCaster: Option<u8>,
    /// Vec<u32>: (3.4.0 - 5.4.6f3)
    pub m_SubsetIndices: Option<Vec<u32>>,
    /// bool: (3.5.0 - 5.3.8f2)
    pub m_UseLightProbes: Option<bool>,
}

/// Transform is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Transform.html):
/**
Position, rotation and scale of an object.
Every object in a Scene has a Transform.
It's used to store and manipulate the position, rotation and scale of the object.
Every Transform can have a parent, which allows you to apply position, rotation and scale hierarchically. This is the hierarchy seen in the Hierarchy pane.
They also support enumerators so you can loop through children using:
Additional resources: The component reference, Physics class.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Transform {
    /// Vec<PPtr<[`Transform`]>>: (3.4.0 - 6000.2.0a6)
    pub m_Children: Vec<PPtr>,
    /// PPtr<[`Transform`]>: (3.4.0 - 6000.2.0a6)
    pub m_Father: PPtr,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Position of the transform relative to the parent transform.*/
    pub m_LocalPosition: Vector3f,
    /**The rotation of the transform relative to the transform rotation of the parent.*/
    pub m_LocalRotation: Quaternionf,
    /**The scale of the transform relative to the GameObjects parent.*/
    pub m_LocalScale: Vector3f,
}

/// TransformMaskElement is a sub class of the Unity engine since version 4.1.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransformMaskElement {
    pub m_Path: String,
    pub m_Weight: f32,
}

/// Transition is a  class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Transition {
    pub m_Atomic: bool,
    pub m_Conditions: Vec<Condition>,
    /// PPtr<[`State`]>: (4.0.0 - 4.7.2)
    pub m_DstState: PPtr,
    pub m_Mute: bool,
    pub m_Name: String,
    pub m_Solo: bool,
    /// PPtr<[`State`]>: (4.0.0 - 4.7.2)
    pub m_SrcState: PPtr,
    pub m_TransitionDuration: f32,
    pub m_TransitionOffset: f32,
    /// bool: (4.5.0 - 4.7.2)
    pub m_CanTransitionToSelf: Option<bool>,
}

/// Tree is a  class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Tree.html):
/**
Tree Component for the tree creator.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /// PPtr<[`SpeedTreeWindAsset`]>: (5.0.0f4 - 6000.2.0a6)
    pub m_SpeedTreeWindAsset: Option<PPtr>,
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
    /**The position of the tree in the local space of the terrain. The value is a Vector3 clamped to 0-1, and describes a percentage of the terrain width, length, and height.*/
    pub position: Vector3f,
    /**Width scale of this instance (compared to the prototype's size).*/
    pub widthScale: f32,
    /**Read-only.Rotation of the tree on X-Z plane (in radians).*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub prefab: PPtr,
    /**The LOD index of a Tree LODGroup that Unity uses to generate a NavMesh. It uses this value only for Trees with a LODGroup, and ignores this value for regular Trees.*/
    /// i32: (2020.2.0b1 - 6000.2.0a6)
    pub navMeshLod: Option<i32>,
}

/// TriggerModule is a sub class of the Unity engine since version 5.4.0f3.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/ParticleSystem.TriggerModule.html):
/**
Script interface for the TriggerModule.
This module is useful for killing particles when they touch a set of collision shapes, or for calling a script command to let you apply custom particle behaviors when the trigger is activated.The example code for MonoBehaviour.OnParticleTrigger shows how the callback type action works.Additional resources: ParticleSystem, ParticleSystem.trigger.
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
    /// i32: (2020.2.0b1 - 6000.2.0a6)
    pub colliderQueryMode: Option<i32>,
    /// PPtr<[`Component`]>: (5.4.0f3 - 2020.2.0a13)
    pub collisionShape0: Option<PPtr>,
    /// PPtr<[`Component`]>: (5.4.0f3 - 2020.2.0a13)
    pub collisionShape1: Option<PPtr>,
    /// PPtr<[`Component`]>: (5.4.0f3 - 2020.2.0a13)
    pub collisionShape2: Option<PPtr>,
    /// PPtr<[`Component`]>: (5.4.0f3 - 2020.2.0a13)
    pub collisionShape3: Option<PPtr>,
    /// PPtr<[`Component`]>: (5.4.0f3 - 2020.2.0a13)
    pub collisionShape4: Option<PPtr>,
    /// PPtr<[`Component`]>: (5.4.0f3 - 2020.2.0a13)
    pub collisionShape5: Option<PPtr>,
    /// Vec<PPtr<[`Component`]>>: (2020.2.0b1 - 6000.2.0a6)
    pub primitives: Option<Vec<PPtr>>,
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
    /// i32: (5.3.4f1 - 6000.2.0a6)
    pub m_AscentCalculationMode: Option<i32>,
    /**Get or set the AssetBundle name.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleName: Option<String>,
    /**Get or set the AssetBundle variant.*/
    /// String: (5.0.0f4 - 6000.2.0a6)
    pub m_AssetBundleVariant: Option<String>,
    /**Border pixels added to character images for padding. This is useful if you want to render text using a shader which needs to render outside of the character area (like an outline shader).*/
    /// i32: (4.0.0 - 6000.2.0a6)
    pub m_CharacterPadding: Option<i32>,
    /**Spacing between character images in the generated texture in pixels. This is useful if you want to render text using a shader which samples pixels outside of the character area (like an outline shader).*/
    /// i32: (4.0.0 - 6000.2.0a6)
    pub m_CharacterSpacing: Option<i32>,
    /**A custom set of characters to be included in the Font Texture.*/
    /// String: (3.5.0 - 6000.2.0a6)
    pub m_CustomCharacters: Option<String>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /// Vec<PPtr<[`Font`]>>: (5.2.0f2 - 6000.2.0a6)
    pub m_FallbackFontReferences: Option<Vec<PPtr>>,
    /// Vec<(i32, String)>: (3.4.0 - 3.4.2)
    pub m_FileIDToRecycleName: Option<Vec<(i32, String)>>,
    /// ColorRGBA: (3.5.0 - 4.1.5)
    pub m_FontColor: Option<ColorRGBA>,
    /// String: (5.5.0f3 - 6000.2.0a6)
    pub m_FontName: Option<String>,
    /**Font rendering mode to use for this font.*/
    /// i32: (4.0.0 - 6000.2.0a6)
    pub m_FontRenderingMode: Option<i32>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_NewHashIdentity: Option<MdFour>,
    /// MdFour: (3.4.0 - 3.4.2)
    pub m_OldHashIdentity: Option<MdFour>,
    /// Output: (4.0.0 - 6000.2.0a6)
    pub m_Output: Option<Output>,
    /// i32: (3.4.0 - 3.5.7)
    pub m_RenderMode: Option<i32>,
    /**Set this property to true if you want to round the internal advance width of the font to the nearest integer.*/
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub m_ShouldRoundAdvanceValue: Option<bool>,
    /// i32: (3.4.0 - 3.5.7)
    pub m_Style: Option<i32>,
    /// bool: (3.4.0 - 5.3.3f1)
    pub m_Use2xBehaviour: Option<bool>,
    /// bool: (5.6.5f1 - 6000.2.0a6)
    pub m_UseLegacyBoundsCalculation: Option<bool>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
    /**Get or set any user data.*/
    /// String: (4.0.0 - 6000.2.0a6)
    pub m_UserData: Option<String>,
}

/// UAVParameter is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct UAVParameter {
    pub m_Index: i32,
    pub m_NameIndex: i32,
    pub m_OriginalIndex: i32,
}

/// UIRenderer is a  class of the Unity engine since version 4.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/UIElements.UIRenderer.html):
/**
A renderer Component that should be added next to a UIDocument Component to allow
 world-space rendering. This Component is added automatically by the UIDocument when
 the PanelSettings asset is configured in world-space.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UIRenderer {
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.5.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_CastShadows: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_DynamicOccludee: Option<u8>,
    /**Makes the rendered 3D object visible if enabled.*/
    /// bool: (2023.2.0b1 - 6000.2.0a6)
    pub m_Enabled: Option<bool>,
    /**The light probe interpolation type.*/
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_LightProbeUsage: Option<u8>,
    /// PPtr<[`GameObject`]>: (2023.2.0b1 - 6000.2.0a6)
    pub m_LightProbeVolumeOverride: Option<PPtr>,
    /**The index of the baked lightmap applied to this renderer.*/
    /// u16: (2023.2.0b1 - 6000.2.0a6)
    pub m_LightmapIndex: Option<u16>,
    /// u16: (2023.2.0b1 - 6000.2.0a6)
    pub m_LightmapIndexDynamic: Option<u16>,
    /// Vector4f: (2023.2.0b1 - 6000.2.0a6)
    pub m_LightmapTilingOffset: Option<Vector4f>,
    /// Vector4f: (2023.2.0b1 - 6000.2.0a6)
    pub m_LightmapTilingOffsetDynamic: Option<Vector4f>,
    /**Returns all the instantiated materials of this object.*/
    /// Vec<PPtr<[`Material`]>>: (2023.2.0b1 - 6000.2.0a6)
    pub m_Materials: Option<Vec<PPtr>>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_MotionVectors: Option<u8>,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr<[`Transform`]>: (2023.2.0b1 - 6000.2.0a6)
    pub m_ProbeAnchor: Option<PPtr>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTraceProcedural: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlags: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlagsOverride: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingMode: Option<u8>,
    /**Does this object receive shadows?*/
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_ReceiveShadows: Option<u8>,
    /**Should reflection probes be used for this Renderer?*/
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_ReflectionProbeUsage: Option<u8>,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    /// i32: (2023.2.0b1 - 6000.2.0a6)
    pub m_RendererPriority: Option<i32>,
    /**Determines which rendering layer this renderer lives on, if you use a scriptable render pipeline.*/
    /// u32: (2023.2.0b1 - 6000.2.0a6)
    pub m_RenderingLayerMask: Option<u32>,
    /// u8: (2023.3.0b1 - 6000.2.0a6)
    pub m_SmallMeshCulling: Option<u8>,
    /// i16: (2023.2.0b1 - 6000.2.0a6)
    pub m_SortingLayer: Option<i16>,
    /**Unique ID of the Renderer's sorting layer.*/
    /// i32: (2023.2.0b1 - 6000.2.0a6)
    pub m_SortingLayerID: Option<i32>,
    /**Renderer's order within a sorting layer.*/
    /// i16: (2023.2.0b1 - 6000.2.0a6)
    pub m_SortingOrder: Option<i16>,
    /// StaticBatchInfo: (2023.2.0b1 - 6000.2.0a6)
    pub m_StaticBatchInfo: Option<StaticBatchInfo>,
    /// PPtr<[`Transform`]>: (2023.2.0b1 - 6000.2.0a6)
    pub m_StaticBatchRoot: Option<PPtr>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_StaticShadowCaster: Option<u8>,
}

/// UVAnimation is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct UVAnimation {
    pub cycles: f32,
    /// i32: (3.4.0 - 2018.2.21f1)
    #[serde(alias = "x Tile")]
    pub x_Tile: Option<i32>,
    /// i32: (3.4.0 - 2018.2.21f1)
    #[serde(alias = "y Tile")]
    pub y_Tile: Option<i32>,
}

/// UVModule is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct UVModule {
    pub animationType: i32,
    pub cycles: f32,
    pub enabled: bool,
    pub frameOverTime: MinMaxCurve,
    pub rowIndex: i32,
    pub tilesX: i32,
    pub tilesY: i32,
    /// f32: (5.5.0f3 - 6000.2.0a6)
    pub flipU: Option<f32>,
    /// f32: (5.5.0f3 - 6000.2.0a6)
    pub flipV: Option<f32>,
    /// f32: (2018.3.0b1 - 6000.2.0a6)
    pub fps: Option<f32>,
    /// i32: (2017.1.0b1 - 6000.2.0a6)
    pub mode: Option<i32>,
    /// bool: (3.5.0 - 2018.4.36f1)
    pub randomRow: Option<bool>,
    /// i32: (2019.1.0b1 - 6000.2.0a6)
    pub rowMode: Option<i32>,
    /// Vector2f: (2018.3.0b1 - 6000.2.0a6)
    pub speedRange: Option<Vector2f>,
    /// Vec<SpriteData>: (2017.1.0b1 - 6000.2.0a6)
    pub sprites: Option<Vec<SpriteData>>,
    /// MinMaxCurve: (5.4.0f3 - 6000.2.0a6)
    pub startFrame: Option<MinMaxCurve>,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    pub timeMode: Option<i32>,
    /// i32: (5.4.0f3 - 6000.2.0a6)
    pub uvChannelMask: Option<i32>,
}

/// UnityAdsManager is a  class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityAdsManager {}

/// UnityAdsSettings is a  class of the Unity engine since version 5.2.0f2.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityAdsSettings {
    pub m_Enabled: bool,
    pub m_InitializeOnStartup: bool,
    pub m_TestMode: bool,
    /// String: (5.2.0f2 - 5.6.7f1)
    pub m_AndroidGameId: Option<String>,
    /// u32: (5.2.0f2 - 2017.1.5f1)
    pub m_EnabledPlatforms: Option<u32>,
    /// String: (2017.1.0b1 - 6000.2.0a6)
    pub m_GameId: Option<String>,
    /// String: (5.2.0f2 - 5.6.7f1)
    pub m_IosGameId: Option<String>,
}

/// UnityAnalyticsManager is a  class of the Unity engine since version 5.2.0f2.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityAnalyticsManager {
    /// bool: (5.2.0f2 - 5.2.5f1)
    pub m_Enabled: Option<bool>,
    /// bool: (5.2.0f2 - 5.2.5f1)
    pub m_InitializeOnStartup: Option<bool>,
    /// String: (5.2.0f2 - 5.2.5f1)
    pub m_TestConfigUrl: Option<String>,
    /// String: (5.2.0f2 - 5.2.5f1)
    pub m_TestEventUrl: Option<String>,
    /// bool: (5.2.0f2 - 5.2.5f1)
    pub m_TestMode: Option<bool>,
}

/// UnityAnalyticsSettings is a sub class of the Unity engine since version 5.3.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityAnalyticsSettings {
    pub m_Enabled: bool,
    pub m_TestMode: bool,
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_InitializeOnStartup: Option<bool>,
    /// bool: (2020.3.41f1 - 6000.2.0a6)
    pub m_PackageRequiringCoreStatsPresent: Option<bool>,
    /// String: (5.3.0f1 - 2018.2.21f1)
    pub m_TestConfigUrl: Option<String>,
    /// String: (5.3.0f1 - 2018.2.21f1)
    pub m_TestEventUrl: Option<String>,
}

/// UnityConnectSettings is a  class of the Unity engine since version 5.3.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityConnectSettings {
    pub UnityAnalyticsSettings: UnityAnalyticsSettings,
    pub UnityPurchasingSettings: UnityPurchasingSettings,
    /// CrashReportingSettings: (5.4.0f3 - 6000.2.0a6)
    pub CrashReportingSettings: Option<CrashReportingSettings>,
    /// PerformanceReportingSettings: (5.6.0b1 - 6000.2.0a6)
    pub PerformanceReportingSettings: Option<PerformanceReportingSettings>,
    /// UnityAdsSettings: (5.5.0f3 - 6000.2.0a6)
    pub UnityAdsSettings: Option<UnityAdsSettings>,
    /// String: (2018.3.0b1 - 6000.2.0a6)
    pub m_ConfigUrl: Option<String>,
    /// String: (2020.3.5f1 - 6000.2.0a6)
    pub m_DashboardUrl: Option<String>,
    /// bool: (5.4.0f3 - 6000.2.0a6)
    pub m_Enabled: Option<bool>,
    /// String: (2018.3.0b1 - 6000.2.0a6)
    pub m_EventOldUrl: Option<String>,
    /// String: (2018.3.0b1 - 6000.2.0a6)
    pub m_EventUrl: Option<String>,
    /// String: (5.4.0f3 - 2018.2.21f1)
    pub m_TestConfigUrl: Option<String>,
    /// String: (5.4.0f3 - 2018.2.21f1)
    pub m_TestEventUrl: Option<String>,
    /// i32: (5.6.0f1 - 6000.2.0a6)
    pub m_TestInitMode: Option<i32>,
    /// bool: (5.4.0f3 - 6000.2.0a6)
    pub m_TestMode: Option<bool>,
}

/// UnityPropertySheet is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnityPropertySheet {
    pub m_Colors: Vec<(Enum_FastPropertyName__String, ColorRGBA)>,
    pub m_Floats: Vec<(Enum_FastPropertyName__String, f32)>,
    pub m_TexEnvs: Vec<(Enum_FastPropertyName__String, UnityTexEnv)>,
    /// Vec<(String, i32)>: (2021.1.0b1 - 6000.2.0a6)
    pub m_Ints: Option<Vec<(String, i32)>>,
}

/// UnityPurchasingSettings is a sub class of the Unity engine since version 5.3.0f1.
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
    /// PPtr<[`Texture`]>: (3.4.0 - 6000.2.0a6)
    pub m_Texture: PPtr,
}

/// UpdateZoneInfo is a sub class of the Unity engine since version 5.6.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateZoneInfo {
    pub needSwap: bool,
    pub passIndex: i32,
    pub rotation: f32,
    pub updateZoneCenter: Vector3f,
    pub updateZoneSize: Vector3f,
}

/// VFXCPUBufferData is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXCPUBufferData {
    pub data: Vec<u32>,
}

/// VFXCPUBufferDesc is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXCPUBufferDesc {
    pub capacity: u32,
    pub initialData: VFXCPUBufferData,
    pub layout: Vec<VFXLayoutElementDesc>,
    pub stride: u32,
}

/// VFXEditorSystemDesc is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXEditorSystemDesc {
    pub buffers: Vec<VFXMapping>,
    pub capacity: u32,
    pub flags: i32,
    pub layer: u32,
    pub tasks: Vec<VFXEditorTaskDesc>,
    pub values: Vec<VFXMapping>,
    /// i32: (2018.3.0b1 - 2020.1.0a14)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// String: (2020.1.0a13 - 2020.1.0a14)
    pub name: Option<String>,
}

/// VFXEditorTaskDesc is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXEditorTaskDesc {
    pub buffers: Vec<VFXMapping>,
    pub params: Vec<VFXMapping>,
    /// PPtr<[`NamedObject`]>: (2018.3.0b1 - 2020.1.0a14)
    pub processor: PPtr,
    pub shaderSourceIndex: i32,
    pub values: Vec<VFXMapping>,
    /// i32: (2018.3.0b1 - 2020.1.0a14)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// Vec<VFXMappingTemporary>: (2019.2.0b1 - 2020.1.0a14)
    pub temporaryBuffers: Option<Vec<VFXMappingTemporary>>,
}

/// VFXEntryExpressionValue is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXEntryExpressionValue {
    pub m_ExpressionIndex: u32,
    pub m_Value: f32,
}

/// VFXEventDesc is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXEventDesc {
    pub name: String,
    pub playSystems: Vec<u32>,
    pub stopSystems: Vec<u32>,
    /// Vec<u32>: (2021.2.0b1 - 6000.2.0a6)
    pub initSystems: Option<Vec<u32>>,
}

/// VFXExposedMapping is a sub class of the Unity engine since version 2023.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXExposedMapping {
    pub mapping: VFXMapping,
    pub space: i32,
}

/// VFXExpressionContainer is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXExpressionContainer {
    pub m_Expressions: Vec<Expression>,
    pub m_NeedsLocalToWorld: bool,
    pub m_NeedsWorldToLocal: bool,
    /// u32: (2022.2.0f1 - 6000.2.0a6)
    pub m_ConstantBakeCurveCount: Option<u32>,
    /// u32: (2022.2.0f1 - 6000.2.0a6)
    pub m_ConstantBakeGradientCount: Option<u32>,
    /// u32: (2022.2.0f1 - 6000.2.0a6)
    pub m_DynamicBakeCurveCount: Option<u32>,
    /// u32: (2022.2.0f1 - 6000.2.0a6)
    pub m_DynamicBakeGradientCount: Option<u32>,
    /// u32: (2020.1.0b1 - 6000.2.0a6)
    pub m_MaxCommonExpressionsIndex: Option<u32>,
    /// i32: (2019.1.0b1 - 6000.2.0a6)
    pub m_NeededMainCameraBuffers: Option<i32>,
    /// bool: (2022.1.0b1 - 6000.2.0a6)
    pub m_NeedsMainCamera: Option<bool>,
}

/// VFXField is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXField {
    pub m_Array: Vec<VFXEntryExpressionValue>,
}

/// VFXGPUBufferDesc is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXGPUBufferDesc {
    pub capacity: u32,
    pub layout: Vec<VFXLayoutElementDesc>,
    pub size: u32,
    pub stride: u32,
    /// i32: (2018.3.0b1 - 2023.3.0a18)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// i32: (6000.0.0f1 - 6000.2.0a6)
    pub mode: Option<i32>,
    /// i32: (6000.0.0f1 - 6000.2.0a6)
    pub target: Option<i32>,
}

/// VFXInstanceSplitDesc is a sub class of the Unity engine since version 2022.3.6f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXInstanceSplitDesc {
    pub values: Vec<u32>,
}

/// VFXLayoutElementDesc is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXLayoutElementDesc {
    pub name: String,
    pub offset: VFXLayoutOffset,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
}

/// VFXLayoutOffset is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXLayoutOffset {
    pub bucket: u32,
    pub element: u32,
    pub structure: u32,
}

/// VFXManager is a  class of the Unity engine since version 2018.3.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/VFX.VFXManager.html):
/**
Use this class to set a number of properties that control VisualEffect behavior within your Unity Project.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXManager {
    /// PPtr<[`ComputeShader`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_CopyBufferShader: PPtr,
    pub m_FixedTimeStep: f32,
    /// PPtr<[`ComputeShader`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_IndirectShader: PPtr,
    pub m_MaxDeltaTime: f32,
    pub m_RenderPipeSettingsPath: String,
    /// PPtr<[`ComputeShader`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_SortShader: PPtr,
    /// u32: (2022.2.0f1 - 6000.2.0a6)
    pub m_BatchEmptyLifetime: Option<u32>,
    /// u32: (2020.1.0b1 - 6000.2.0a6)
    pub m_CompiledVersion: Option<u32>,
    /// PPtr<[`Shader`]>: (2021.3.32f1 - 6000.2.0a6)
    pub m_EmptyShader: Option<PPtr>,
    /// u32: (2023.1.0b1 - 6000.2.0a6)
    pub m_MaxCapacity: Option<u32>,
    /// f32: (2022.1.0b1 - 6000.2.0a6)
    pub m_MaxScrubTime: Option<f32>,
    /// PPtr<[`ComputeShader`]>: (6000.1.0b1 - 6000.2.0a6)
    pub m_PrefixSumShader: Option<PPtr>,
    /// PPtr<[`MonoBehaviour`]>: (2021.2.0b1 - 6000.2.0a6)
    pub m_RuntimeResources: Option<PPtr>,
    /// u32: (2020.1.0b1 - 6000.2.0a6)
    pub m_RuntimeVersion: Option<u32>,
    /// PPtr<[`ComputeShader`]>: (2019.3.0b1 - 6000.2.0a6)
    pub m_StripUpdateShader: Option<PPtr>,
}

/// VFXMapping is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXMapping {
    pub index: i32,
    pub nameId: String,
}

/// VFXMappingTemporary is a sub class of the Unity engine since version 2019.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXMappingTemporary {
    pub mapping: VFXMapping,
    pub pastFrameIndex: u32,
    pub perCameraBuffer: bool,
}

/// VFXPropertySheetSerializedBase is a sub class of the Unity engine since version 2018.3.0b1.
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

/// VFXRenderer is a  class of the Unity engine since version 2018.3.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/VFX.VFXRenderer.html):
/**
Renders a VisualEffect.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXRenderer {
    pub m_CastShadows: u8,
    pub m_DynamicOccludee: u8,
    /**Makes the rendered 3D object visible if enabled.*/
    pub m_Enabled: bool,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**The light probe interpolation type.*/
    pub m_LightProbeUsage: u8,
    /// PPtr<[`GameObject`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_LightProbeVolumeOverride: PPtr,
    /**The index of the baked lightmap applied to this renderer.*/
    pub m_LightmapIndex: u16,
    pub m_LightmapIndexDynamic: u16,
    pub m_LightmapTilingOffset: Vector4f,
    pub m_LightmapTilingOffsetDynamic: Vector4f,
    pub m_MotionVectors: u8,
    /**If set, Renderer will use this Transform's position to find the light or reflection probe.*/
    /// PPtr<[`Transform`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_ProbeAnchor: PPtr,
    /**Does this object receive shadows?*/
    pub m_ReceiveShadows: u8,
    /**Should reflection probes be used for this Renderer?*/
    pub m_ReflectionProbeUsage: u8,
    /**This value sorts renderers by priority. Lower values are rendered first and higher values are rendered last.*/
    pub m_RendererPriority: i32,
    /**Determines which rendering layer this renderer lives on, if you use a scriptable render pipeline.*/
    pub m_RenderingLayerMask: u32,
    pub m_SortingLayer: i16,
    /**Unique ID of the Renderer's sorting layer.*/
    pub m_SortingLayerID: i32,
    /**Renderer's order within a sorting layer.*/
    pub m_SortingOrder: i16,
    pub m_StaticBatchInfo: StaticBatchInfo,
    /// PPtr<[`Transform`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_StaticBatchRoot: PPtr,
    /**Returns all the instantiated materials of this object.*/
    /// Vec<PPtr<[`Material`]>>: (2018.3.0b1 - 2021.1.28f1)
    pub m_Materials: Option<Vec<PPtr>>,
    /// u8: (2020.1.0b1 - 6000.2.0a6)
    pub m_RayTraceProcedural: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlags: Option<u8>,
    /// u8: (2023.2.0b1 - 6000.2.0a6)
    pub m_RayTracingAccelStructBuildFlagsOverride: Option<u8>,
    /**Describes how this renderer is updated for ray tracing.*/
    /// u8: (2019.3.0b1 - 6000.2.0a6)
    pub m_RayTracingMode: Option<u8>,
    /// u8: (2023.3.0b1 - 6000.2.0a6)
    pub m_SmallMeshCulling: Option<u8>,
    /**Is this renderer a static shadow caster?*/
    /// u8: (2021.1.0b1 - 6000.2.0a6)
    pub m_StaticShadowCaster: Option<u8>,
}

/// VFXRendererSettings is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXRendererSettings {
    pub lightProbeUsage: i32,
    pub motionVectorGenerationMode: i32,
    pub receiveShadows: bool,
    pub reflectionProbeUsage: i32,
    pub shadowCastingMode: i32,
    /// i32: (2023.1.0b1 - 6000.2.0a6)
    pub rayTracingMode: Option<i32>,
    /// i32: (2018.3.0b4 - 2018.3.0b6)
    pub transparencyPriority: Option<i32>,
}

/// VFXShaderSourceDesc is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXShaderSourceDesc {
    pub compute: bool,
    pub name: String,
    pub source: String,
}

/// VFXSystemDesc is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXSystemDesc {
    pub buffers: Vec<VFXMapping>,
    pub capacity: u32,
    pub flags: i32,
    pub layer: u32,
    pub tasks: Vec<VFXTaskDesc>,
    pub values: Vec<VFXMapping>,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// Vec<VFXInstanceSplitDesc>: (2022.3.6f1 - 6000.2.0a6)
    pub instanceSplitDescs: Option<Vec<VFXInstanceSplitDesc>>,
    /// String: (2020.1.0b1 - 6000.2.0a6)
    pub name: Option<String>,
}

/// VFXTaskDesc is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXTaskDesc {
    pub buffers: Vec<VFXMapping>,
    pub params: Vec<VFXMapping>,
    /// PPtr<[`NamedObject`]>: (2018.3.0b1 - 6000.2.0a6)
    pub processor: PPtr,
    pub values: Vec<VFXMapping>,
    /// i32: (2018.3.0b1 - 6000.2.0a6)
    #[serde(alias = "type")]
    pub _type: Option<i32>,
    /// u32: (2022.3.6f1 - 6000.2.0a6)
    pub instanceSplitIndex: Option<u32>,
    /// Vec<VFXMappingTemporary>: (2019.2.0b1 - 6000.2.0a6)
    pub temporaryBuffers: Option<Vec<VFXMappingTemporary>>,
}

/// VFXTemplate is a sub class of the Unity engine since version 2023.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXTemplate {
    pub category: String,
    pub description: String,
    /// PPtr<[`Texture2D`]>: (2023.2.0b1 - 6000.2.0a6)
    pub icon: PPtr,
    pub name: String,
    /// PPtr<[`Texture2D`]>: (2023.2.0b1 - 6000.2.0a6)
    pub thumbnail: PPtr,
}

/// VFXTemporaryGPUBufferDesc is a sub class of the Unity engine since version 2019.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VFXTemporaryGPUBufferDesc {
    pub desc: VFXGPUBufferDesc,
    pub frameCount: u32,
}

/// VRSettings is a sub class of the Unity engine since version 5.6.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VRSettings {
    /// Google: (5.6.0b1 - 2020.2.0a15)
    pub cardboard: Option<Google>,
    /// Google: (5.6.0b1 - 2020.2.0a15)
    pub daydream: Option<Google>,
    /// bool: (2018.1.0f1 - 6000.2.0a6)
    pub enable360StereoCapture: Option<bool>,
    /// HoloLens: (5.6.0f1 - 2020.2.0a15)
    pub hololens: Option<HoloLens>,
    /// Lumin: (2019.1.0b1 - 2020.2.0a15)
    pub lumin: Option<Lumin>,
    /// DeviceNone: (5.6.0b1 - 2020.2.0a17)
    pub none: Option<DeviceNone>,
    /// Oculus: (2017.3.0f1 - 2020.2.0a15)
    pub oculus: Option<Oculus>,
}

/// ValueArrayConstant is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ValueArrayConstant {
    pub m_ValueArray: Vec<ValueConstant>,
}

/// ValueConstant is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ValueConstant {
    pub m_ID: u32,
    pub m_Index: u32,
    pub m_Type: u32,
    pub m_TypeID: u32,
}

/// ValueDelta is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct ValueDelta {
    pub m_Start: f32,
    pub m_Stop: f32,
}

/// VariableBoneCountWeights is a sub class of the Unity engine since version 2019.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VariableBoneCountWeights {
    pub m_Data: Vec<u32>,
}

/// VariantInfo is a sub class of the Unity engine since version 5.0.0f4.
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

/// VectorParameter is a sub class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct VectorParameter {
    pub m_ArraySize: i32,
    pub m_Dim: i8,
    pub m_Index: i32,
    pub m_NameIndex: i32,
    pub m_Type: i8,
}

/// VelocityModule is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct VelocityModule {
    pub enabled: bool,
    pub inWorldSpace: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    /// MinMaxCurve: (2018.1.0f1 - 6000.2.0a6)
    pub orbitalOffsetX: Option<MinMaxCurve>,
    /// MinMaxCurve: (2018.1.0f1 - 6000.2.0a6)
    pub orbitalOffsetY: Option<MinMaxCurve>,
    /// MinMaxCurve: (2018.1.0f1 - 6000.2.0a6)
    pub orbitalOffsetZ: Option<MinMaxCurve>,
    /// MinMaxCurve: (2018.1.0f1 - 6000.2.0a6)
    pub orbitalX: Option<MinMaxCurve>,
    /// MinMaxCurve: (2018.1.0f1 - 6000.2.0a6)
    pub orbitalY: Option<MinMaxCurve>,
    /// MinMaxCurve: (2018.1.0f1 - 6000.2.0a6)
    pub orbitalZ: Option<MinMaxCurve>,
    /// MinMaxCurve: (2018.1.0f1 - 6000.2.0a6)
    pub radial: Option<MinMaxCurve>,
    /// MinMaxCurve: (2017.3.0b1 - 6000.2.0a6)
    pub speedModifier: Option<MinMaxCurve>,
}

/// VersionControlSettings is a  class of the Unity engine since version 2020.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionControlSettings {
    pub m_Mode: String,
    /// CollabEditorSettings: (2020.1.0b1 - 6000.0.0b13)
    pub m_CollabEditorSettings: Option<CollabEditorSettings>,
    /// bool: (2023.1.0f1 - 6000.2.0a6)
    pub m_TrackPackagesOutsideProject: Option<bool>,
}

/// VertexData is a sub class of the Unity engine since version 3.5.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct VertexData {
    pub m_DataSize: Vec<u8>,
    pub m_VertexCount: u32,
    /// Vec<ChannelInfo>: (4.0.0 - 6000.2.0a6)
    pub m_Channels: Option<Vec<ChannelInfo>>,
    /// u32: (3.5.0 - 5.5.6f1); i32: (5.6.0b1 - 2017.4.40f1)
    pub m_CurrentChannels: Option<i64>,
    /// Vec<StreamInfo>: (4.0.0 - 4.7.2)
    pub m_Streams: Option<Vec<StreamInfo>>,
    /// StreamInfo: (3.5.0 - 3.5.7)
    #[serde(alias = "m_Streams[0]")]
    pub m_Streams_0_: Option<StreamInfo>,
    /// StreamInfo: (3.5.0 - 3.5.7)
    #[serde(alias = "m_Streams[1]")]
    pub m_Streams_1_: Option<StreamInfo>,
    /// StreamInfo: (3.5.0 - 3.5.7)
    #[serde(alias = "m_Streams[2]")]
    pub m_Streams_2_: Option<StreamInfo>,
    /// StreamInfo: (3.5.0 - 3.5.7)
    #[serde(alias = "m_Streams[3]")]
    pub m_Streams_3_: Option<StreamInfo>,
}

/// VertexLayoutInfo is a sub class of the Unity engine since version 6000.0.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VertexLayoutInfo {
    pub vertexChannelsInfo: Vec<ChannelInfo>,
    pub vertexStreamCount: i32,
    pub vertexStrides: Vec<u8>,
}

/// VideoBuildInfo is a  class of the Unity engine since version 2019.4.38f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoBuildInfo {
    pub m_IsVideoModuleDisabled: bool,
    pub m_VideoClipCount: i32,
}

/// VideoClip is a  class of the Unity engine since version 5.6.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Video.VideoClip.html):
/**
A container for video assets that can be used in the Vide.VideoPlayer component.
A VideoClip stores the video portion of a movie file using a codec that is appropriate for the target platform. The VideoPlayer class references VideoClips.The following example shows how to assign a video clip to a video player and play it.Additional resources: VideoPlayer, class-VideoClip.
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
    /**The length of the video clip in frames. (Read Only).*/
    pub m_FrameCount: u64,
    /**The frame rate of the clip in frames per second. (Read Only).*/
    pub m_FrameRate: f64,
    pub m_HasSplitAlpha: bool,
    /**The name of the object.*/
    pub m_Name: String,
    /**Gets the original video clip file path as it was imported into Unity. (Read Only).*/
    pub m_OriginalPath: String,
    pub m_ProxyHeight: u32,
    pub m_ProxyWidth: u32,
    /// u32: (2017.2.0f1 - 6000.2.0a6)
    pub m_PixelAspecRatioDen: Option<u32>,
    /// u32: (2017.2.0f1 - 6000.2.0a6)
    pub m_PixelAspecRatioNum: Option<u32>,
    /// Vec<PPtr<[`Shader`]>>: (2020.1.0b1 - 6000.2.0a6)
    pub m_VideoShaders: Option<Vec<PPtr>>,
    /**Whether the imported clip contains sRGB color data (Read Only).*/
    /// bool: (2019.2.0b1 - 6000.2.0a6)
    pub m_sRGB: Option<bool>,
}

/// VideoClipImporter is a  class of the Unity engine since version 5.6.0b1.
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
    pub m_TargetSettings: Vec<(Enum_i32__String, VideoClipImporterTargetSettings)>,
    /**Get or set any user data.*/
    pub m_UserData: String,
    /// i32: (5.6.0b1 - 5.6.0b2)
    pub m_AudioImportMode: Option<i32>,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2017.2.0f1 - 6000.2.0a6)
    pub m_ExternalObjects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    /**Number of frames in the clip.*/
    /// i32: (5.6.0b1 - 2017.2.1f1)
    pub m_FrameCount: Option<i32>,
    /**Frame rate of the clip.*/
    /// f64: (5.6.0b1 - 2017.2.1f1)
    pub m_FrameRate: Option<f64>,
    /**Import audio tracks from source file.*/
    /// bool: (5.6.0f1 - 6000.2.0a6)
    pub m_ImportAudio: Option<bool>,
    /// bool: (5.6.0b1 - 2019.2.21f1)
    pub m_IsColorLinear: Option<bool>,
    /// i32: (5.6.0b1 - 2017.2.1f1)
    pub m_OriginalHeight: Option<i32>,
    /// i32: (5.6.0b1 - 2017.2.1f1)
    pub m_OriginalWidth: Option<i32>,
    /**Denominator of the pixel aspect ratio (num:den).*/
    /// u32: (2017.2.0f1 - 2017.2.1f1)
    pub m_PixelAspectRatioDenominator: Option<u32>,
    /**Numerator of the pixel aspect ratio (num:den).*/
    /// u32: (2017.2.0f1 - 2017.2.1f1)
    pub m_PixelAspectRatioNumerator: Option<u32>,
    /// f32: (5.6.0b1 - 2019.2.21f1)
    pub m_Quality: Option<f32>,
    /// Vec<u16>: (5.6.0b1 - 2017.2.1f1)
    pub m_SourceAudioChannelCount: Option<Vec<u16>>,
    /// Vec<u32>: (5.6.0b1 - 2017.2.1f1)
    pub m_SourceAudioSampleRate: Option<Vec<u32>>,
    /**Size in bytes of the file before importing.*/
    /// u64: (5.6.0b1 - 2017.2.1f1)
    pub m_SourceFileSize: Option<u64>,
    /**True if the source file has a channel for per-pixel transparency.*/
    /// bool: (5.6.0b1 - 2017.2.1f1)
    pub m_SourceHasAlpha: Option<bool>,
    /// bool: (5.6.0b1 - 2019.2.21f1)
    pub m_UseLegacyImporter: Option<bool>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// VideoClipImporterOutput is a sub class of the Unity engine since version 5.6.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoClipImporterOutput {
    /// i32: (5.6.0b1 - 2017.2.1f1)
    pub encodedEndFrame: Option<i32>,
    /// i32: (5.6.0b1 - 2017.2.1f1)
    pub encodedHeight: Option<i32>,
    /// VideoClipImporterTargetSettings: (2017.1.3p1 - 6000.2.0a6)
    pub encodedSettings: Option<VideoClipImporterTargetSettings>,
    /// i32: (5.6.0b1 - 2017.2.1f1)
    pub encodedStartFrame: Option<i32>,
    /// i32: (5.6.0b1 - 2017.2.1f1)
    pub encodedWidth: Option<i32>,
    /// i32: (5.6.0b1 - 2017.2.1f1)
    pub format: Option<i32>,
    /// i32: (2017.1.3p1 - 6000.2.0a6)
    pub originalFrameCount: Option<i32>,
    /// i32: (2017.1.3p1 - 6000.2.0a6)
    pub originalHeight: Option<i32>,
    /// i32: (2017.1.3p1 - 6000.2.0a6)
    pub originalWidth: Option<i32>,
    /// VideoClipImporterTargetSettings: (5.6.0b1 - 2017.2.1f1)
    pub settings: Option<VideoClipImporterTargetSettings>,
    /// Vec<u16>: (2017.1.3p1 - 6000.2.0a6)
    pub sourceAudioChannelCount: Option<Vec<u16>>,
    /// Vec<u32>: (2017.1.3p1 - 6000.2.0a6)
    pub sourceAudioSampleRate: Option<Vec<u32>>,
    /// u64: (2017.1.3p1 - 6000.2.0a6)
    pub sourceFileSize: Option<u64>,
    /// f64: (2017.1.3p1 - 6000.2.0a6)
    pub sourceFrameRate: Option<f64>,
    /// bool: (2017.1.3p1 - 6000.2.0a6)
    pub sourceHasAlpha: Option<bool>,
    /// u32: (2017.2.1p1 - 6000.2.0a6)
    pub sourcePixelAspectRatioDenominator: Option<u32>,
    /// u32: (2017.2.1p1 - 6000.2.0a6)
    pub sourcePixelAspectRatioNumerator: Option<u32>,
    /// StreamedResource: (5.6.0b1 - 2017.2.1f1)
    pub streamedResource: Option<StreamedResource>,
    /// bool: (2017.3.0b1 - 6000.2.0a6)
    pub transcodeSkipped: Option<bool>,
}

/// VideoClipImporterTargetSettings is a sub class of the Unity engine since version 5.6.0b1.
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

/// VideoPlayer is a  class of the Unity engine since version 5.6.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/Video.VideoPlayer.html):
/**
Plays video content onto a target.
Content can be either a VideoClip imported asset or a URL such as file:// or http://. Video content will be projected onto one of the supported targets, such as camera background or RenderTexture.
If the video content includes transparency, this transparency will be present in the target, allowing objects behind the video target to be visible. When the data VideoPlayer.source is set to URL, the audio and video description of what is being played will only be initialized once the VideoPlayer preparation is completed. You can test this with VideoPlayer.isPrepared.Refer to Video file compatibility for more information on supported video file formats.The following demonstrates a few features of the VideoPlayer:
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
    /// PPtr<[`GameObject`]>: (5.6.0b1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub m_Looping: bool,
    /**Whether the content will start playing back as soon as the component awakes.*/
    pub m_PlayOnAwake: bool,
    /**Factor by which the basic playback rate will be multiplied.*/
    pub m_PlaybackSpeed: f32,
    /**Where the video content will be drawn.*/
    pub m_RenderMode: i32,
    /**Whether the VideoPlayer is allowed to skip frames to catch up with current time.*/
    pub m_SkipOnDrop: bool,
    /// Vec<PPtr<[`AudioSource`]>>: (5.6.0b1 - 6000.2.0a6)
    pub m_TargetAudioSources: Vec<PPtr>,
    /**Camera component to draw to when VideoPlayer.renderMode is set to either VideoRenderMode.CameraFarPlane or VideoRenderMode.CameraNearPlane.*/
    /// PPtr<[`Camera`]>: (5.6.0b1 - 6000.2.0a6)
    pub m_TargetCamera: PPtr,
    /**Overall transparency level of the target camera plane video.*/
    pub m_TargetCameraAlpha: f32,
    /**Material texture property which is targeted when VideoPlayer.renderMode is set to Video.VideoTarget.MaterialOverride.*/
    pub m_TargetMaterialProperty: String,
    /**Renderer which is targeted when VideoPlayer.renderMode is set to Video.VideoTarget.MaterialOverride*/
    /// PPtr<[`Renderer`]>: (5.6.0b1 - 6000.2.0a6)
    pub m_TargetMaterialRenderer: PPtr,
    /**RenderTexture to draw to when VideoPlayer.renderMode is set to Video.VideoTarget.RenderTexture.*/
    /// PPtr<[`RenderTexture`]>: (5.6.0b1 - 6000.2.0a6)
    pub m_TargetTexture: PPtr,
    /**The file URL or web URL that the VideoPlayer reads content from.*/
    pub m_Url: String,
    /// PPtr<[`VideoClip`]>: (5.6.0b1 - 6000.2.0a6)
    pub m_VideoClip: PPtr,
    /**Determines whether the VideoPlayer will wait for the first frame to be loaded into the texture before starting playback when VideoPlayer.playOnAwake is on.*/
    pub m_WaitForFirstFrame: bool,
    /**Type of 3D content contained in the source video media.*/
    /// i32: (2017.3.0b1 - 6000.2.0a6)
    pub m_TargetCamera3DLayout: Option<i32>,
    /// String: (5.6.0b1 - 5.6.0b10)
    pub m_TargetMaterialName: Option<String>,
    /**The clock that the VideoPlayer observes to detect and correct drift.*/
    /// i32: (2017.1.0f1 - 6000.2.0a6)
    pub m_TimeReference: Option<i32>,
    /**The clock source used by the VideoPlayer to derive its current time.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_TimeUpdateMode: Option<i32>,
    /// Vec<PPtr<[`Shader`]>>: (2020.1.0b1 - 6000.2.0a6)
    pub m_VideoShaders: Option<Vec<PPtr>>,
}

/// VisualEffect is a  class of the Unity engine since version 2018.3.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/VFX.VisualEffect.html):
/**
The visual effect class that references an VisualEffectAsset instance within the Scene.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffect {
    /// PPtr<[`VisualEffectAsset`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_Asset: PPtr,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    pub m_PropertySheet: VFXPropertySheetSerializedBase,
    /**This property controls whether the visual effect generates a new seed for the random number generator with each call to VisualEffect.Play function.*/
    pub m_ResetSeedOnPlay: Enum_bool__u8,
    /**The initial seed used for internal random number generator.*/
    pub m_StartSeed: u32,
    /// u8: (2022.2.0b1 - 6000.2.0a6)
    pub m_AllowInstancing: Option<u8>,
    /**The default event name. Unity calls this event when the VisualEffect awakes, or when you call VisualEffect.Reinit.*/
    /// String: (2019.3.0b1 - 6000.2.0a6)
    pub m_InitialEventName: Option<String>,
    /// u8: (2019.3.0b1 - 6000.2.0a6)
    pub m_InitialEventNameOverriden: Option<u8>,
}

/// VisualEffectAsset is a  class of the Unity engine since version 2018.3.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/VFX.VisualEffectAsset.html):
/**
This class contains a graph of the elements needed to describe a visual effect. These include: the visual effects system, generated shaders, and compiled data.
Additional resources: VisualEffect.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectAsset {
    pub m_Infos: VisualEffectInfo,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_Systems: Vec<VFXSystemDesc>,
}

/// VisualEffectImporter is a  class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectImporter {
    pub m_AssetBundleName: String,
    pub m_AssetBundleVariant: String,
    /// Vec<(SourceAssetIdentifier, PPtr<[`Object`]>)>: (2018.3.0b1 - 6000.2.0a6)
    pub m_ExternalObjects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_Name: String,
    pub m_UserData: String,
    /// VFXTemplate: (2023.2.0b1 - 6000.2.0a6)
    pub m_Template: Option<VFXTemplate>,
    /// Vec<i64>: (2019.1.0b1 - 6000.2.0a6)
    pub m_UsedFileIDs: Option<Vec<i64>>,
}

/// VisualEffectInfo is a sub class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectInfo {
    pub m_Buffers: Vec<VFXGPUBufferDesc>,
    pub m_CPUBuffers: Vec<VFXCPUBufferDesc>,
    pub m_CullingFlags: i32,
    pub m_Events: Vec<VFXEventDesc>,
    pub m_ExposedExpressions: Vec<Enum_VFXMapping__VFXExposedMapping>,
    pub m_Expressions: VFXExpressionContainer,
    pub m_PropertySheet: VFXPropertySheetSerializedBase,
    pub m_RendererSettings: VFXRendererSettings,
    pub m_UpdateMode: i32,
    /// u32: (2021.2.0b1 - 6000.2.0a6)
    pub m_CompilationVersion: Option<u32>,
    /// String: (2019.3.0b1 - 6000.2.0a6)
    pub m_InitialEventName: Option<String>,
    /// u32: (2022.2.0b1 - 6000.2.0a6)
    pub m_InstancingCapacity: Option<u32>,
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_InstancingDisabledReason: Option<i32>,
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_InstancingMode: Option<i32>,
    /// f32: (2019.1.0b1 - 6000.2.0a6)
    pub m_PreWarmDeltaTime: Option<f32>,
    /// u32: (2019.1.0b1 - 6000.2.0a6)
    pub m_PreWarmStepCount: Option<u32>,
    /// u32: (2018.4.15f1 - 6000.2.0a6)
    pub m_RuntimeVersion: Option<u32>,
    /// Vec<VFXTemporaryGPUBufferDesc>: (2019.2.0b1 - 6000.2.0a6)
    pub m_TemporaryBuffers: Option<Vec<VFXTemporaryGPUBufferDesc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_VFXMapping__VFXExposedMapping {
    VFXMapping(VFXMapping),
    VFXExposedMapping(VFXExposedMapping),
}

/// VisualEffectResource is a  class of the Unity engine since version 2018.3.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectResource {
    /// PPtr<[`MonoBehaviour`]>: (2018.3.0b1 - 6000.2.0a6)
    pub m_Graph: PPtr,
    pub m_Infos: Enum_VisualEffectInfo__VisualEffectSettings,
    pub m_Name: String,
    /// Vec<VFXShaderSourceDesc>: (2018.3.0b1 - 2020.1.0a14)
    pub m_ShaderSources: Option<Vec<VFXShaderSourceDesc>>,
    /// Vec<VFXEditorSystemDesc>: (2018.3.0b1 - 2020.1.0a14)
    pub m_Systems: Option<Vec<VFXEditorSystemDesc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enum_VisualEffectInfo__VisualEffectSettings {
    VisualEffectInfo(VisualEffectInfo),
    VisualEffectSettings(VisualEffectSettings),
}

/// VisualEffectSettings is a sub class of the Unity engine since version 2020.1.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectSettings {
    pub m_CullingFlags: i32,
    pub m_InitialEventName: String,
    pub m_PreWarmDeltaTime: f32,
    pub m_PreWarmStepCount: u32,
    pub m_RendererSettings: VFXRendererSettings,
    pub m_UpdateMode: i32,
    /// u32: (2022.2.0b1 - 6000.2.0a6)
    pub m_InstancingCapacity: Option<u32>,
    /// i32: (2022.2.0b1 - 2023.1.0a4)
    pub m_InstancingDisabledReason: Option<i32>,
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_InstancingMode: Option<i32>,
}

/// VisualEffectSubgraphBlock is a  class of the Unity engine since version 2019.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectSubgraphBlock {
    pub m_Name: String,
}

/// VisualEffectSubgraphOperator is a  class of the Unity engine since version 2019.2.0b1.
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualEffectSubgraphOperator {
    pub m_Name: String,
}

/// VulkanDeviceFilterLists is a  class of the Unity engine since version 6000.1.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/VulkanDeviceFilterLists.html):
/**
Set of parameters used to define an Android device or group of Android devices.
Specify a list of parameters to identify an Android device or set of Android devices. Enter values for one or multiple parameters. The parameter values are processed using logical AND operation to check if the device properties match with all the specified values.Unity ignores the filter if all parameters are empty.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct VulkanDeviceFilterLists {
    pub m_GfxJobFilterList: Vec<VulkanGraphicsJobsDeviceFilterData>,
    /**The name of the object.*/
    pub m_Name: String,
    pub m_VulkanAllowFilterList: Vec<AndroidDeviceFilterData>,
    pub m_VulkanDenyFilterList: Vec<AndroidDeviceFilterData>,
}

/// VulkanGraphicsJobsDeviceFilterData is a sub class of the Unity engine since version 6000.1.0b1.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/VulkanGraphicsJobsDeviceFilterData.html):
/**
Set of parameters used to define Android device criteria for selecting specified Graphics Jobs modes.
Specify a list of parameters to identify an Android device or set of Android devices and a preferred Graphics Jobs mode those devices should use. Enter values for one or multiple parameters. The parameter values are processed using logical AND operation to check if the device properties match with all the specified values.Unity ignores the filter if all parameters are empty.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct VulkanGraphicsJobsDeviceFilterData {
    /**The set of parameters used to define Android device filtering criteria.*/
    pub filter: AndroidDeviceFilterData,
    /**The preferred Graphics Jobs mode.*/
    pub preferredMode: u32,
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
wheel colliders ignore standard PhysicsMaterial settings. Simulation of different road materials
is done by changing the forwardFriction and sidewaysFriction
based on what material the wheel is hitting. Additional resources: GetGroundHit and WheelFrictionCurve.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct WheelCollider {
    /**The center of the wheel, measured in the object's local space.*/
    pub m_Center: Vector3f,
    /**Properties of tire friction in the direction the wheel is pointing in.*/
    pub m_ForwardFriction: WheelFrictionCurve,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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
    /// bool: (3.5.0 - 6000.2.0a6)
    pub m_Enabled: Option<bool>,
    /**The additional layers that this Collider should exclude when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_ExcludeLayers: Option<BitField>,
    /**Application point of the suspension and tire forces measured from the base of the resting wheel.*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_ForceAppPointDistance: Option<f32>,
    /**The additional layers that this Collider should include when deciding if the Collider can contact another Collider.*/
    /// BitField: (2022.2.0b1 - 6000.2.0a6)
    pub m_IncludeLayers: Option<BitField>,
    /**A decision priority assigned to this Collider used when there is a conflicting decision on whether a Collider can contact another Collider.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_LayerOverridePriority: Option<i32>,
    /**Whether or not this Collider generates contacts for Physics.ContactEvent.*/
    /// bool: (2022.2.0b1 - 6000.2.0a6)
    pub m_ProvidesContacts: Option<bool>,
    /**The damping rate of the wheel. Must be larger than zero.*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_WheelDampingRate: Option<f32>,
}

/// WheelFrictionCurve is a sub class of the Unity engine since version 3.4.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/WheelFrictionCurve.html):
/**
WheelFrictionCurve is used by the WheelCollider to describe friction properties of the wheel tire.
The curve takes a measure of tire slip as an input and gives a force as output. The curve is approximated by
a two-piece spline. The first section goes from (0,0) to (extremumSlip,extremumValue), at which
point the curve's tangent is zero. The second section goes from (extremumSlip,extremumValue)
to (asymptoteSlip,asymptoteValue), where curve's tangent is again zero:In the previous image a graph displays the wheel slip curve with force on the y-axis and slip on the x-axis. Force increases as slip increases up to the extremum point, after which force declines as slip increases up to the asymptote point. Beyond the asymptote the curve becomes a flat line as additional slip gives no further change in force.Wheel collider computes friction separately from the rest of physics engine, using a slip based
friction model. It separates the overall friction force into a "forwards" component (in the
direction of rolling, and responsible for acceleration and braking) and "sideways" component
(orthogonal to rolling, responsible for keeping the car oriented). Tire friction is described
separately in these directions using WheelCollider.forwardFriction and WheelCollider.sidewaysFriction.
In both directions it is first determined how much the tire is slipping (what is the speed difference between
the rubber and the road). Then this slip value is used to find out tire force exerted on the contact.The property of real tires is that for low slip they can exert high forces as the rubber compensates
for the slip by stretching. Later when the slip gets really high, the forces are reduced as the tire
starts to slide or spin. Thus tire friction curves have a shape like in the image above.Because the friction for the tires is computed separately, the PhysicsMaterial of the ground
does not affect the wheels. Simulation of different road materials is done by changing
the WheelCollider.forwardFriction and WheelCollider.sidewaysFriction of the wheel,
based on what material the wheel is hitting.Additional resources: WheelCollider, WheelCollider.forwardFriction, WheelCollider.sidewaysFriction.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct WheelFrictionCurve {
    /**Asymptote point slip (default 2).*/
    /// f32: (3.4.0 - 4.7.2)
    pub asymptoteSlip: Option<f32>,
    /**Force at the asymptote slip (default 10000).*/
    /// f32: (3.4.0 - 4.7.2)
    pub asymptoteValue: Option<f32>,
    /**Extremum point slip (default 1).*/
    /// f32: (3.4.0 - 4.7.2)
    pub extremumSlip: Option<f32>,
    /**Force at the extremum slip (default 20000).*/
    /// f32: (3.4.0 - 4.7.2)
    pub extremumValue: Option<f32>,
    /**Asymptote point slip (default 2).*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_AsymptoteSlip: Option<f32>,
    /**Force at the asymptote slip (default 10000).*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_AsymptoteValue: Option<f32>,
    /**Extremum point slip (default 1).*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_ExtremumSlip: Option<f32>,
    /**Force at the extremum slip (default 20000).*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_ExtremumValue: Option<f32>,
    /**Multiplier for the extremumValue and asymptoteValue values (default 1).*/
    /// f32: (5.0.0f4 - 6000.2.0a6)
    pub m_Stiffness: Option<f32>,
    /// f32: (3.4.0 - 4.7.2)
    pub stiffnessFactor: Option<f32>,
}

/// WheelJoint2D is a  class of the Unity engine since version 4.5.0.
/// Exert from [Unity's scripting documentation](https://docs.unity3d.com/ScriptReference/WheelJoint2D.html):
/**
The wheel joint allows the simulation of wheels by providing a constraining suspension motion with an optional motor.
Additional resources: JointSuspension2D.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct WheelJoint2D {
    /**The joint's anchor point on the object that has the joint component.*/
    pub m_Anchor: Vector2f,
    /**The joint's anchor point on the second object (ie, the one which doesn't have the joint component).*/
    pub m_ConnectedAnchor: Vector2f,
    /// PPtr<[`Rigidbody2D`]>: (4.5.0 - 6000.2.0a6)
    pub m_ConnectedRigidBody: PPtr,
    /**Enabled Behaviours are Updated, disabled Behaviours are not.*/
    pub m_Enabled: u8,
    /**The game object this component is attached to. A component is always attached to a game object.*/
    /// PPtr<[`GameObject`]>: (4.5.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
    /**Parameters for a motor force that is applied automatically to the Rigidbody2D along the line.*/
    pub m_Motor: JointMotor2D,
    /**Set the joint suspension configuration.*/
    pub m_Suspension: JointSuspension2D,
    /**Should a motor force be applied automatically to the Rigidbody2D?*/
    pub m_UseMotor: bool,
    /**Should the connectedAnchor be calculated automatically?*/
    /// bool: (5.3.0f1 - 6000.2.0a6)
    pub m_AutoConfigureConnectedAnchor: Option<bool>,
    /**The action to take when the joint breaks the breakForce or breakTorque.*/
    /// i32: (2022.2.0b1 - 6000.2.0a6)
    pub m_BreakAction: Option<i32>,
    /**The force that needs to be applied for this joint to break.*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_BreakForce: Option<f32>,
    /**The torque that needs to be applied for this joint to break.*/
    /// f32: (5.3.0f1 - 6000.2.0a6)
    pub m_BreakTorque: Option<f32>,
    /// bool: (4.5.0 - 5.0.0f4)
    pub m_CollideConnected: Option<bool>,
    /**Should the two Rigidbody2D connected with this joint collide with each other?*/
    /// bool: (5.0.1f1 - 6000.2.0a6)
    pub m_EnableCollision: Option<bool>,
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
    /// PPtr<[`GameObject`]>: (3.4.0 - 6000.2.0a6)
    pub m_GameObject: PPtr,
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

/// WorldAnchor is a  class of the Unity engine since version 5.5.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorldAnchor {
    /// PPtr<[`GameObject`]>: (5.5.0f3 - 2020.2.0a15)
    pub m_GameObject: PPtr,
}

/// WorldParticleCollider is a  class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorldParticleCollider {
    pub m_BounceFactor: f32,
    pub m_CollidesWith: BitField,
    pub m_CollisionEnergyLoss: f32,
    /// PPtr<[`GameObject`]>: (3.4.0 - 2018.2.21f1)
    pub m_GameObject: PPtr,
    pub m_MinKillVelocity: f32,
    pub m_SendCollisionMessage: bool,
}

/// bitset is a sub class of the Unity engine since version 3.4.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct bitset {
    pub bitCount: i32,
    pub bitblocks: Vec<u8>,
}

/// float3 is a sub class of the Unity engine since version 5.4.0f3.
#[derive(Debug, Serialize, Deserialize)]
pub struct float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// float4 is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct float4 {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// int2_storage is a sub class of the Unity engine since version 2018.1.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct int2_storage {
    pub x: i32,
    pub y: i32,
}

/// int3_storage is a sub class of the Unity engine since version 2017.2.0f1.
#[derive(Debug, Serialize, Deserialize)]
pub struct int3_storage {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// xform is a sub class of the Unity engine since version 4.0.0.
#[derive(Debug, Serialize, Deserialize)]
pub struct xform {
    pub q: float4,
    pub s: Enum_float4__float3,
    pub t: Enum_float4__float3,
}
