#!/usr/bin/env bash
# Quick test runner for E2E tests

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default: run Docker-based tests only
TEST_TAGS="--tags=~cli"
VERBOSE=""
SCENARIO_NAME=""

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --all)
      TEST_TAGS=""
      shift
      ;;
    --cli)
      TEST_TAGS="--tags=cli"
      shift
      ;;
    --name)
      SCENARIO_NAME="$2"
      shift 2
      ;;
    --verbose|-v)
      VERBOSE="--format pretty"
      shift
      ;;
    --help|-h)
      echo "Usage: run-tests.sh [OPTIONS]"
      echo ""
      echo "Options:"
      echo "  --all       Run all tests including CLI tests"
      echo "  --cli       Run only CLI tests"
      echo "  --name NAME Run specific scenario by name"
      echo "  --verbose   Show verbose output"
      echo "  --help      Show this help message"
      exit 0
      ;;
    *)
      echo -e "${RED}Unknown option: $1${NC}"
      exit 1
      ;;
  esac
done

# Ensure we're in the e2e directory
cd "$(dirname "$0")"

# Check if behave is installed
if ! command -v behave &> /dev/null; then
    echo -e "${YELLOW}Behave not found. Installing dependencies...${NC}"
    pip install --break-system-packages -r requirements.txt
fi

# Build the behave command
if [ -n "$SCENARIO_NAME" ]; then
    BEHAVE_CMD="behave --name \"$SCENARIO_NAME\" $VERBOSE"
else
    BEHAVE_CMD="behave $TEST_TAGS $VERBOSE"
fi

echo -e "${GREEN}Running: $BEHAVE_CMD${NC}"
echo ""

# Run the tests
eval $BEHAVE_CMD

# Exit with behave's exit code
exit $?
