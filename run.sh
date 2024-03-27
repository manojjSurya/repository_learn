#!/usr/bin/env bash

LOG_FILE=$"/home/manojj/Rust/package/SuperDB-Batch-1/SuperDB-Batch-1/surya/utility/test-bed/log.txt"
DIAGNOSTICS_FILE=$"/home/manojj/Rust/package/SuperDB-Batch-1/SuperDB-Batch-1/surya/utility/test-bed/diagnostics.txt"
I1_FILE=$"I1.txt"
I2_FILE=$"I2.txt"
I3_FILE=$"I3.txt"
I4_FILE=$"I4.xlsx"
I5_FILE=$"I5.txt"
I6_FILE=$"I6.txt"
I7_FILE=$"I7.txt"
OUTPUT_FILE=$"Output.txt"

cargo run  -- \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-file ${LOG_FILE} \
--log-level trace \
--I1-file ${I1_FILE} \
--I2-file ${I2_FILE} \
--I3-file ${I3_FILE} \
--I4-file ${I4_FILE} \
--I5-file ${I5_FILE} \
--I6-file ${I6_FILE} \
--I7-file ${I7_FILE} \
--output-file ${OUTPUT_FILE} \
--diagnostics-flag false
