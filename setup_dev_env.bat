@echo off
echo Setting up development environment for Plastic Waste Backend

REM Create a .env file if it doesn't exist
if not exist .env (
    echo DATABASE_URL=postgresql://postgres:password@localhost:5432/plastic_waste_db > .env
    echo Created .env file with default database configuration
)

echo Setup complete. Please ensure PostgreSQL is running with the plastic_waste_db database created.
echo You can now run 'cargo build' to compile the project.