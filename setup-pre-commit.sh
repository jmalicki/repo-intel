#!/bin/bash
# Setup pre-commit hooks for documentation CI

echo "🔧 Setting up pre-commit hooks for documentation CI..."

# Install pre-commit if not already installed
if ! command -v pre-commit &> /dev/null; then
    echo "📦 Installing pre-commit..."
    pip install pre-commit
fi

# Install the hooks
echo "🔗 Installing pre-commit hooks..."
pre-commit install

# Run on all files to check current state
echo "🔍 Running pre-commit on all files..."
pre-commit run --all-files

echo "✅ Pre-commit setup complete!"
echo ""
echo "💡 To run pre-commit manually:"
echo "   pre-commit run --all-files"
echo ""
echo "💡 To run specific hooks:"
echo "   pre-commit run markdownlint --all-files"
echo "   pre-commit run markdown-link-check --all-files"
echo "   pre-commit run cspell --all-files"
