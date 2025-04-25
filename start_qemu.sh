# Make sure you are in your project's root directory (/workspace)

# --- Define File Paths ---
ESP_IDF_SYS_HASH="*" # <-- UPDATE THIS HASH IF NEEDED
INTERMEDIATE_BUILD_DIR="target/xtensa-esp32-espidf/debug/build/esp-idf-sys-${ESP_IDF_SYS_HASH}/out/build"

# *** IMPORTANT: Manually set the correct application binary name found in step 2 ***
APP_BIN_NAME="rust-esp-starter.bin" # <-- REPLACE if different!
APP_BIN_PATH="${INTERMEDIATE_BUILD_DIR}/${APP_BIN_NAME}"

PART_TABLE_BIN="${INTERMEDIATE_BUILD_DIR}/partition_table/partition-table.bin"
BOOTLOADER_BIN="${INTERMEDIATE_BUILD_DIR}/bootloader/bootloader.bin"

# --- Verify Files Exist ---
echo "Using Application Binary: ${APP_BIN_PATH}"
echo "Using Partition Table:    ${PART_TABLE_BIN}"
echo "Using Bootloader:         ${BOOTLOADER_BIN}"
if [ ! -f "$APP_BIN_PATH" ]; then echo "ERROR: Application binary not found!"; exit 1; fi
if [ ! -f "$PART_TABLE_BIN" ]; then echo "ERROR: Partition table not found!"; exit 1; fi
if [ ! -f "$BOOTLOADER_BIN" ]; then echo "ERROR: Bootloader not found!"; exit 1; fi

# --- Standard ESP32 Load Addresses ---
BOOTLOADER_ADDR=0x1000
PART_TABLE_ADDR=0x8000
APP_ADDR=0x10000

# --- QEMU Command (No explicit flash size) ---
qemu-system-xtensa -nographic \
  -machine esp32 \
  -monitor none \
  -serial stdio \
  -device loader,file=${BOOTLOADER_BIN},addr=${BOOTLOADER_ADDR} \
  -device loader,file=${PART_TABLE_BIN},addr=${PART_TABLE_ADDR} \
  -device loader,file=${APP_BIN_PATH},addr=${APP_ADDR}