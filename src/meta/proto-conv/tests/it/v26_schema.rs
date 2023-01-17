// Copyright 2023 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_expression::types::NumberDataType;
use common_expression::TableDataType;
use common_expression::TableField;
use common_expression::TableSchema;

use crate::common;

// These bytes are built when a new version in introduced,
// and are kept for backward compatibility test.
//
// *************************************************************
// * These messages should never be updated,                   *
// * only be added when a new version is added,                *
// * or be removed when an old version is no longer supported. *
// *************************************************************
//
// The message bytes are built from the output of `test_build_pb_buf()`
#[test]
fn test_decode_v26_schema() -> anyhow::Result<()> {
    let schema_v26 = [
        10, 28, 10, 1, 97, 26, 17, 154, 2, 8, 34, 0, 160, 6, 26, 168, 6, 24, 160, 6, 26, 168, 6,
        24, 160, 6, 26, 168, 6, 24, 10, 102, 10, 1, 98, 26, 91, 202, 2, 82, 10, 2, 98, 49, 10, 2,
        98, 50, 18, 47, 202, 2, 38, 10, 3, 98, 49, 49, 10, 3, 98, 49, 50, 18, 9, 138, 2, 0, 160, 6,
        26, 168, 6, 24, 18, 9, 146, 2, 0, 160, 6, 26, 168, 6, 24, 160, 6, 26, 168, 6, 24, 160, 6,
        26, 168, 6, 24, 18, 17, 154, 2, 8, 66, 0, 160, 6, 26, 168, 6, 24, 160, 6, 26, 168, 6, 24,
        160, 6, 26, 168, 6, 24, 160, 6, 26, 168, 6, 24, 160, 6, 26, 168, 6, 24, 10, 28, 10, 1, 99,
        26, 17, 154, 2, 8, 34, 0, 160, 6, 26, 168, 6, 24, 160, 6, 26, 168, 6, 24, 160, 6, 26, 168,
        6, 24, 24, 5, 34, 3, 10, 1, 97, 34, 5, 10, 1, 98, 16, 1, 34, 8, 10, 4, 98, 58, 98, 49, 16,
        1, 34, 12, 10, 8, 98, 58, 98, 49, 58, 98, 49, 49, 16, 1, 34, 12, 10, 8, 98, 58, 98, 49, 58,
        98, 49, 50, 16, 2, 34, 8, 10, 4, 98, 58, 98, 50, 16, 3, 34, 5, 10, 1, 99, 16, 4, 160, 6,
        26, 168, 6, 24,
    ];

    let b1 = TableDataType::Tuple {
        fields_name: vec!["b11".to_string(), "b12".to_string()],
        fields_type: vec![TableDataType::Boolean, TableDataType::String],
    };
    let b = TableDataType::Tuple {
        fields_name: vec!["b1".to_string(), "b2".to_string()],
        fields_type: vec![b1.clone(), TableDataType::Number(NumberDataType::Int64)],
    };
    let fields = vec![
        TableField::new("a", TableDataType::Number(NumberDataType::UInt64)),
        TableField::new("b", b.clone()),
        TableField::new("c", TableDataType::Number(NumberDataType::UInt64)),
    ];
    let want = TableSchema::new(fields);

    common::test_load_old(func_name!(), schema_v26.as_slice(), 26, want.clone())?;
    common::test_pb_from_to(func_name!(), want)?;

    Ok(())
}
