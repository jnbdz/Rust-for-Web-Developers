use super::base::Base;
use super::super::enums::TaskStatus;

use super::super::traits::get::Get;
use super::super::traits::delete::Delete;
use super::super::traits::edit::Edit;


/// This struct is responsible for defing a completed task. 
/// 
/// # Attributes 
/// * super_struct (Base): the super struct that hosts the core functionality and fields
pub struct Done {
    pub super_struct: Base
}

impl Done {

    /// The constructor for the Done struct. 
    /// 
    /// # Arguements
    /// * input_title (&str): the title of the completed task being created
    /// 
    /// # Returns 
    /// (Done): the constructed struct
    pub fn new(input_title: &str) -> Done {
        let base = Base::new(input_title, 
            TaskStatus::DONE);
        return Done{super_struct: base}
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}