# Activation du CLI nou
# Usage : source activate.sh
#    ou : . activate.sh

_NOU_PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

export PATH="$_NOU_PROJECT_DIR/scripts:$PATH"

echo "✅ nou activé ($(nou version 2>/dev/null || echo 'scripts/nou'))"
