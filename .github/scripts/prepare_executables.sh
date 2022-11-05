# Figure out suffix of binary
EXE_suffix=""
case $MATRIX_TARGET in
*-pc-windows-*) EXE_suffix=".exe" ;;
esac;

# Setup paths
BIN_DIR="${DEPLOY_DIR}"

mkdir -p "${BIN_DIR}"

BIN_NAME="${PROJECT_NAME}${EXE_suffix}"
BIN_PUBLISH_NAME="${PROJECT_NAME}-${MATRIX_TARGET}${EXE_suffix}"

BIN_PATH="${BIN_DIR}/${BIN_PUBLISH_NAME}"

# Copy the release build binary to deploy location and name
cp "target/${MATRIX_TARGET}/release/${BIN_NAME}" "${BIN_PATH}"

# Let subsequent steps know where to find the bin
echo "BIN_DIR=$BIN_DIR" >> $GITHUB_OUTPUT
echo "BIN_PATH=$BIN_PATH" >> $GITHUB_OUTPUT
echo "BIN_PUBLISH_NAME=$BIN_PUBLISH_NAME" >> $GITHUB_OUTPUT
