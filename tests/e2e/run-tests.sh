#!/usr/bin/env bash
# Quick test runner for E2E tests

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
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
    --presets)
      TEST_TAGS="--tags=preset"
      shift
      ;;
    --versions)
      TEST_TAGS="--tags=version"
      shift
      ;;
    --edge)
      TEST_TAGS="--tags=edge-case"
      shift
      ;;
    --config)
      TEST_TAGS="--tags=config"
      shift
      ;;
    --concurrent)
      TEST_TAGS="--tags=concurrent"
      shift
      ;;
    --tags)
      TEST_TAGS="--tags=$2"
      shift 2
      ;;
    --layer-1)
      TEST_TAGS="--tags=layer-1"
      shift
      ;;
    --layer-2)
      TEST_TAGS="--tags=layer-2"
      shift
      ;;
    --layer-3)
      TEST_TAGS="--tags=layer-3"
      shift
      ;;
    --fast)
      TEST_TAGS="--tags=fast"
      shift
      ;;
    --slow)
      TEST_TAGS="--tags=slow"
      shift
      ;;
    --medium)
      TEST_TAGS="--tags=medium"
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
      echo "  --all        Run all tests including CLI tests"
      echo "  --cli        Run only CLI tests"
      echo "  --presets    Run preset coverage tests"
      echo "  --versions   Run multi-version language tests"
      echo "  --edge       Run edge case and negative tests"
      echo "  --config     Run configuration option tests"
      echo "  --concurrent Run concurrent operation tests"
      echo ""
      echo "Three-Layer Testing:"
      echo "  --layer-1    Build template images (slow)"
      echo "  --layer-2    Test isolde.yaml scenarios (medium)"
      echo "  --layer-3    In-container verification (fast)"
      echo "  --fast       Run only fast tests"
      echo "  --medium     Run fast + medium tests"
      echo "  --slow       Run only slow tests"
      echo "  --tags TAGS  Custom tag expression (e.g. 'layer-3 or fast')"
      echo ""
      echo "  --name NAME  Run specific scenario by name"
      echo "  --verbose    Show verbose output"
      echo "  --help       Show this help message"
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
