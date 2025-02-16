#!/bin/bash

valgrind --tool=massif --massif-out-file=./massif.one_log --time-unit=B ./target/debug/batch-processor
ms_print massif.one_log
