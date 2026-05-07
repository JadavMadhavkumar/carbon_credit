pub mod schema;

use sqlx::PgPool;

pub async fn init_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await?;

    Ok(())
}

pub mod migrations {
    pub const CREATE_USERS: &str = r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            full_name VARCHAR(255) NOT NULL,
            wallet_address VARCHAR(42),
            state_code VARCHAR(10) NOT NULL,
            municipality_id UUID,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );
    "#;

    pub const CREATE_HOUSEHOLDS: &str = r#"
        CREATE TABLE IF NOT EXISTS households (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id),
            wallet_address VARCHAR(42),
            state_code VARCHAR(10) NOT NULL,
            address JSONB NOT NULL DEFAULT '{}',
            municipality_id UUID,
            created_at TIMESTAMPTZ DEFAULT NOW()
        );
    "#;

    pub const CREATE_WASTE_SUBMISSIONS: &str = r#"
        CREATE TABLE IF NOT EXISTS waste_submissions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            household_id UUID NOT NULL REFERENCES households(id),
            submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            waste_category VARCHAR(50) NOT NULL,
            waste_subtype VARCHAR(100) NOT NULL,
            weight_kg NUMERIC(10, 3) NOT NULL CHECK (weight_kg > 0),
            verification_mode VARCHAR(20) NOT NULL DEFAULT 'self',
            verification_score NUMERIC(3, 2) NOT NULL DEFAULT 0.50,
            quality_factor NUMERIC(3, 2) NOT NULL DEFAULT 1.00,
            raw_credits_kg NUMERIC(12, 4),
            processing_method VARCHAR(30) DEFAULT 'recycling',
            blockchain_tx_hash VARCHAR(66),
            status VARCHAR(20) NOT NULL DEFAULT 'pending',
            ai_confidence NUMERIC(3, 2),
            ai_category_match BOOLEAN,
            created_at TIMESTAMPTZ DEFAULT NOW()
        );
    "#;

    pub const CREATE_CREDIT_LEDGER: &str = r#"
        CREATE TABLE IF NOT EXISTS credit_ledger (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            household_id UUID NOT NULL REFERENCES households(id),
            submission_id UUID REFERENCES waste_submissions(id),
            credits_kg NUMERIC(12, 4) NOT NULL,
            token_id VARCHAR(100),
            ledger_type VARCHAR(20) NOT NULL CHECK (ledger_type IN ('earn', 'redeem', 'transfer')),
            created_at TIMESTAMPTZ DEFAULT NOW()
        );
    "#;

    pub const CREATE_BIOCHAR_RECORDS: &str = r#"
        CREATE TABLE IF NOT EXISTS biochar_records (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            household_id UUID NOT NULL REFERENCES households(id),
            biomass_type VARCHAR(50) NOT NULL,
            biomass_input_kg NUMERIC(10, 3) NOT NULL,
            biochar_output_kg NUMERIC(10, 3) NOT NULL,
            carbon_sequestered_kg NUMERIC(12, 4) NOT NULL,
            carbon_credits_kg NUMERIC(12, 4) NOT NULL,
            pyrolysis_temp_c NUMERIC(6, 2) NOT NULL,
            residence_time_min INTEGER NOT NULL,
            status VARCHAR(20) NOT NULL DEFAULT 'pending',
            created_at TIMESTAMPTZ DEFAULT NOW()
        );
    "#;

    pub const CREATE_REPORTS: &str = r#"
        CREATE TABLE IF NOT EXISTS reports (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            household_id UUID REFERENCES households(id),
            municipality_id UUID,
            report_type VARCHAR(30) NOT NULL,
            period_start TIMESTAMPTZ NOT NULL,
            period_end TIMESTAMPTZ NOT NULL,
            total_credits_kg NUMERIC(12, 4) NOT NULL,
            total_submissions INTEGER NOT NULL DEFAULT 0,
            category_breakdown JSONB,
            created_at TIMESTAMPTZ DEFAULT NOW()
        );
    "#;

    pub const CREATE_MUNICIPALITIES: &str = r#"
        CREATE TABLE IF NOT EXISTS municipalities (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL,
            state_code VARCHAR(10) NOT NULL,
            total_households INTEGER NOT NULL DEFAULT 0,
            created_at TIMESTAMPTZ DEFAULT NOW()
        );
    "#;

    pub const CREATE_INDEXES: &str = r#"
        CREATE INDEX IF NOT EXISTS idx_submissions_household ON waste_submissions(household_id);
        CREATE INDEX IF NOT EXISTS idx_submissions_submitted_at ON waste_submissions(submitted_at);
        CREATE INDEX IF NOT EXISTS idx_submissions_category ON waste_submissions(waste_category);
        CREATE INDEX IF NOT EXISTS idx_ledger_household ON credit_ledger(household_id);
        CREATE INDEX IF NOT EXISTS idx_reports_period ON reports(period_start, period_end);
        CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
    "#;
}