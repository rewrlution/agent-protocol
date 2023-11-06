/*
 * Agent Protocol
 *
 * Specification of the API protocol for communication with an agent.
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// TaskRequestBody : Body of the task request.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskRequestBody {
	/// Input prompt for the task.
	#[serde(
		rename = "input",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub input: Option<Option<String>>,
	/// Input parameters for the task. Any value is allowed.
	#[serde(rename = "additional_input", skip_serializing_if = "Option::is_none")]
	pub additional_input: Option<serde_json::Value>,
}

impl TaskRequestBody {
	/// Body of the task request.
	pub fn new() -> TaskRequestBody {
		TaskRequestBody { input: None, additional_input: None }
	}
}
