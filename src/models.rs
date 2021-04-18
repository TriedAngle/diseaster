use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Symptom {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct NewSymptom {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Disease {
    pub id: i32,
    pub name: String,
    // symptom ids
    pub symptoms: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtendedDisease {
    pub id: i32,
    pub name: String,
    pub symptoms: Vec<Symptom>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct NewDisease {
    pub name: String,
    // symptom ids
    pub symptoms: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Department {
    pub id: i32,
    pub name: String,
    // disease ids
    pub diseases: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct NewDepartment {
    pub name: String,
    // disease ids
    pub diseases: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Doctor {
    pub id: i32,
    pub name: String,
    pub occupied: bool,
    pub department: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct NewDoctor {
    pub name: String,
    pub occupied: bool,
    pub department: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Chance {
    pub disease: i32,
    pub chance: i32,
}
