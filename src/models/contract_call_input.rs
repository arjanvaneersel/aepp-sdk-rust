/* 
 * Aeternity Epoch
 *
 * This is the [Aeternity](https://www.aeternity.com/) Epoch API.
 *
 * OpenAPI spec version: 0.25.0
 * Contact: apiteam@aeternity.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractCallInput {
  #[serde(rename = "abi")]
  abi: String,
  #[serde(rename = "code")]
  code: String,
  #[serde(rename = "function")]
  function: Option<String>,
  #[serde(rename = "arg")]
  arg: Option<String>,
  /// Source code for a contract with a function __call() = f(args), if calling a function f
  #[serde(rename = "call")]
  call: Option<String>
}

impl ContractCallInput {
  pub fn new(abi: String, code: String) -> ContractCallInput {
    ContractCallInput {
      abi: abi,
      code: code,
      function: None,
      arg: None,
      call: None
    }
  }

  pub fn set_abi(&mut self, abi: String) {
    self.abi = abi;
  }

  pub fn with_abi(mut self, abi: String) -> ContractCallInput {
    self.abi = abi;
    self
  }

  pub fn abi(&self) -> &String {
    &self.abi
  }


  pub fn set_code(&mut self, code: String) {
    self.code = code;
  }

  pub fn with_code(mut self, code: String) -> ContractCallInput {
    self.code = code;
    self
  }

  pub fn code(&self) -> &String {
    &self.code
  }


  pub fn set_function(&mut self, function: String) {
    self.function = Some(function);
  }

  pub fn with_function(mut self, function: String) -> ContractCallInput {
    self.function = Some(function);
    self
  }

  pub fn function(&self) -> Option<&String> {
    self.function.as_ref()
  }

  pub fn reset_function(&mut self) {
    self.function = None;
  }

  pub fn set_arg(&mut self, arg: String) {
    self.arg = Some(arg);
  }

  pub fn with_arg(mut self, arg: String) -> ContractCallInput {
    self.arg = Some(arg);
    self
  }

  pub fn arg(&self) -> Option<&String> {
    self.arg.as_ref()
  }

  pub fn reset_arg(&mut self) {
    self.arg = None;
  }

  pub fn set_call(&mut self, call: String) {
    self.call = Some(call);
  }

  pub fn with_call(mut self, call: String) -> ContractCallInput {
    self.call = Some(call);
    self
  }

  pub fn call(&self) -> Option<&String> {
    self.call.as_ref()
  }

  pub fn reset_call(&mut self) {
    self.call = None;
  }

}


