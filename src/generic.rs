use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BudgetHours {
    pub amount_budget: Option<f64>,
    pub amount_spent: Option<f64>,
    pub value_budget: Option<f64>,
    pub value_spent: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BudgetCosts {
    pub value_budget: Option<f64>,
    pub value_spent: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BudgetTotal {
    pub value_budget: Option<f64>,
    pub value_spent: Option<f64>,
    pub value_invoiced: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Budget {
    pub hours: Option<BudgetHours>,
    pub costs: Option<BudgetCosts>,
    pub total: Option<BudgetTotal>,
}

#[derive(Deserialize)]
pub struct MetaData {
    pub count: Option<u32>,
    pub total_count: Option<u32>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub upper_limit: Option<u32>,
}

#[derive(Deserialize)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub metadata: Option<MetaData>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CustomField {
    pub name: String,
    pub value: String,
    pub label: Option<String>
}