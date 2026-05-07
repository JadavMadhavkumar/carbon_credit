# Setting up PostgreSQL on Windows for Plastic Waste Backend

This guide will help you set up PostgreSQL manually on Windows for the Plastic Waste Backend project.

## Step 1: Download and Install PostgreSQL

1. Go to the [PostgreSQL Windows download page](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads)
2. Download the latest version for Windows x86-64
3. Run the installer with default settings
4. During installation, you'll be prompted to set a password for the postgres user - remember this password
5. Keep the default port (5432)
6. Complete the installation

## Step 2: Add PostgreSQL to Your PATH (Optional but Recommended)

1. Open the Start Menu and search for "Environment Variables"
2. Click "Edit the system environment variables"
3. Click "Environment Variables"
4. Under "System Variables", scroll down and select "Path", then click "Edit"
5. Click "New" and add the PostgreSQL bin directory (typically `C:\Program Files\PostgreSQL\[version]\bin`)
6. Click OK to save

## Step 3: Verify Installation

1. Open Command Prompt or PowerShell
2. Run the following command:
   ```bash
   psql --version
   ```
   
   If you get a version number, PostgreSQL is correctly installed and in your PATH.
   
   If you get an error, you'll need to use the full path to the PostgreSQL binaries.

## Step 4: Connect to PostgreSQL

1. Open Command Prompt or PowerShell
2. Connect to PostgreSQL as the postgres user:
   ```bash
   psql -U postgres
   ```
   
   If PostgreSQL is not in your PATH, use:
   ```bash
   "C:\Program Files\PostgreSQL\[version]\bin\psql.exe" -U postgres
   ```
   
3. Enter the password you set during installation

## Step 5: Create Database and Schema

Once connected to PostgreSQL, run the following commands:

```sql
-- Create the database
CREATE DATABASE plastic_waste_db;

-- Connect to the database
\c plastic_waste_db;

-- Enable uuid extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the tables using the schema from setup_db.sql
-- You can copy and paste the contents of setup_db.sql here, or use:
\i 'C:/plastic_west_mng/plastic_waste_backend/setup_db.sql'

-- Exit psql
\q
```

## Step 6: Update Your Environment File

Make sure your [.env](file:///c:/plastic_west_mng/plastic_waste_backend/.env) file has the correct database connection string:

```
DATABASE_URL=postgresql://postgres:your_password@localhost:5432/plastic_waste_db
```

Replace `your_password` with the password you set during PostgreSQL installation.

## Step 7: Compile and Run the Application

Now you should be able to compile and run the application:

```bash
# Compile the project
cargo build

# Run the project
cargo run
```

## Troubleshooting

### Connection Refused Errors

If you're still getting "connection refused" errors:

1. Make sure the PostgreSQL service is running:
   - Open Services (services.msc)
   - Find "postgresql" service
   - Ensure it's running (start it if it's not)

2. Check that PostgreSQL is listening on the correct port:
   - In Command Prompt, run: `netstat -an | findstr 5432`
   - You should see a line showing that port 5432 is LISTENING

3. Verify your connection string in the [.env](file:///c:/plastic_west_mng/plastic_waste_backend/.env) file:
   - Make sure the username, password, host, and port are correct
   - Try using `127.0.0.1` instead of `localhost` if you have issues

### SQLx Compile-time Query Checking

SQLx validates queries at compile time by connecting to the database. If you continue to have issues:

1. Make sure the database is running and accessible
2. Make sure the database schema has been created
3. Make sure your DATABASE_URL in [.env](file:///c:/plastic_west_mng/plastic_waste_backend/.env) is correct
4. Try running `cargo clean` and then `cargo build` again

## Alternative: Using Docker

If you continue to have issues with manual PostgreSQL installation, you can try using Docker:

1. Install Docker Desktop for Windows
2. From the project directory, run:
   ```bash
   docker-compose up -d
   ```

This will start a PostgreSQL container with the database properly configured.