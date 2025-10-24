const { Client } = require('pg');

const client = new Client({
  connectionString: "postgresql://funify_owner:8Q8Q8Q8Q8Q8Q@ep-perfect-happiness-12345678.us-east-1.aws.neon.tech/funify?sslmode=require"
});

async function checkDatabase() {
  try {
    await client.connect();
    console.log("🔗 Connected to Neondb successfully!");
    
    // Check if campaigns table exists
    const tableExists = await client.query(`
      SELECT EXISTS (
        SELECT FROM information_schema.tables 
        WHERE table_name = 'campaigns'
      );
    `);
    
    console.log(`📋 Campaigns table exists: ${tableExists.rows[0].exists}`);
    
    if (tableExists.rows[0].exists) {
      // Get table structure
      const columns = await client.query(`
        SELECT column_name, data_type, is_nullable, column_default 
        FROM information_schema.columns 
        WHERE table_name = 'campaigns' 
        ORDER BY ordinal_position;
      `);
      
      console.log("\n📊 Campaigns table structure:");
      columns.rows.forEach(col => {
        console.log(`   - ${col.column_name}: ${col.data_type} (nullable: ${col.is_nullable}, default: ${col.column_default})`);
      });
      
      // Check if cover_image column exists
      const coverImageExists = columns.rows.some(col => col.column_name === 'cover_image');
      console.log(`\n🖼️  cover_image column exists: ${coverImageExists}`);
      
      // Get row count
      const rowCount = await client.query("SELECT COUNT(*) FROM campaigns;");
      console.log(`📊 Total campaigns: ${rowCount.rows[0].count}`);
      
      // Get sample data
      if (rowCount.rows[0].count > 0) {
        const sampleData = await client.query("SELECT id, title, description, slug FROM campaigns LIMIT 3;");
        console.log("\n📝 Sample campaigns:");
        sampleData.rows.forEach(row => {
          console.log(`   - ${row.title} (slug: ${row.slug})`);
        });
      } else {
        console.log("\n📝 No campaigns found in database");
      }
    }
    
    await client.end();
    console.log("\n✅ Database check completed!");
    
  } catch (error) {
    console.error("❌ Database error:", error.message);
    process.exit(1);
  }
}

checkDatabase();
