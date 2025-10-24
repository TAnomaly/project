#!/usr/bin/env python3
import psycopg2
import json
import sys

# Database connection
DATABASE_URL = "postgresql://funify_owner:8Q8Q8Q8Q8Q8Q@ep-perfect-happiness-12345678.us-east-1.aws.neon.tech/funify?sslmode=require"

try:
    # Connect to database
    conn = psycopg2.connect(DATABASE_URL)
    cur = conn.cursor()
    
    print("🔗 Connected to Neondb successfully!")
    
    # Check if campaigns table exists
    cur.execute("""
        SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_name = 'campaigns'
        );
    """)
    table_exists = cur.fetchone()[0]
    print(f"📋 Campaigns table exists: {table_exists}")
    
    if table_exists:
        # Get table structure
        cur.execute("""
            SELECT column_name, data_type, is_nullable, column_default 
            FROM information_schema.columns 
            WHERE table_name = 'campaigns' 
            ORDER BY ordinal_position;
        """)
        
        columns = cur.fetchall()
        print("\n📊 Campaigns table structure:")
        for col in columns:
            print(f"   - {col[0]}: {col[1]} (nullable: {col[2]}, default: {col[3]})")
        
        # Check if cover_image column exists
        cover_image_exists = any(col[0] == 'cover_image' for col in columns)
        print(f"\n🖼️  cover_image column exists: {cover_image_exists}")
        
        # Get row count
        cur.execute("SELECT COUNT(*) FROM campaigns;")
        row_count = cur.fetchone()[0]
        print(f"📊 Total campaigns: {row_count}")
        
        # Get sample data
        if row_count > 0:
            cur.execute("SELECT id, title, description, slug FROM campaigns LIMIT 3;")
            sample_data = cur.fetchall()
            print("\n📝 Sample campaigns:")
            for row in sample_data:
                print(f"   - {row[1]} (slug: {row[3]})")
        else:
            print("\n📝 No campaigns found in database")
    
    cur.close()
    conn.close()
    print("\n✅ Database check completed!")
    
except Exception as e:
    print(f"❌ Database error: {e}")
    sys.exit(1)
