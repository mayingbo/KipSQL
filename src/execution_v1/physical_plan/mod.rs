use crate::execution_v1::physical_plan::physical_create_table::PhysicalCreateTable;
use crate::execution_v1::physical_plan::physical_insert::PhysicalInsert;
use crate::execution_v1::physical_plan::physical_projection::PhysicalProjection;
use crate::execution_v1::physical_plan::physical_table_scan::PhysicalTableScan;
use crate::execution_v1::physical_plan::physical_values::PhysicalValues;

pub(crate) mod physical_create_table;
pub(crate) mod physical_plan_builder;
pub(crate) mod physical_projection;
pub(crate) mod physical_table_scan;
pub(crate) mod physical_insert;
pub(crate) mod physical_values;

#[derive(Debug)]
pub enum PhysicalOperator {
    Insert(PhysicalInsert),
    CreateTable(PhysicalCreateTable),
    TableScan(PhysicalTableScan),
    Projection(PhysicalProjection),
    Values(PhysicalValues)
}