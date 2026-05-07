#!/bin/bash

echo "Setting up development environment for Plastic Waste Backend"

# Create a .env file if it doesn't exist
if [ ! -f .env ]; then
    echo "DATABASE_URL=postgresql://postgres:password@localhost:5432/plastic_waste_db" > .env
    echo "Created .env file with default database configuration"
fi

echo "Setup complete. Please ensure PostgreSQL is running with the plastic_waste_db database created."
echo "You can now run 'cargo build' to compile the project."