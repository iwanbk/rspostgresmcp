# rspostgresmcp

## Overview
A PostgreSQL Model Context Protocol (MCP) server that provides tools for interacting with PostgreSQL databases. This server allows AI models to query database schema information and table data through a standardized interface.

## Features
- Connect to PostgreSQL databases
- List all tables in a database
- Get detailed schema information for specific tables, including columns and indexes
- Structured JSON responses for easy consumption by AI models

## Available MCP Tools

### List Tables
Lists all tables in the connected PostgreSQL database.

**Tool Name:** `list_tables`
**Parameters:** None
**Returns:** JSON array of table names

### Get Table Schema
Returns detailed schema information for a specific table, including columns and indexes.

**Tool Name:** `get_schema`
**Parameters:** 
- `name`: Name of the table to get schema for
**Returns:** JSON object containing column and index information

## Schema Information
The schema information includes:

### Columns
- Name
- Data type
- Maximum length (if applicable)
- Nullability
- Default value

### Indexes
- Name
- Columns included in the index
- Whether the index is unique
- Whether the index is a primary key

## Usage

```bash
./target/release/rspostgresmcp --dsn 'postgres://username:password@localhost:5432/database' --addr '127.0.0.1:9000'
```

## License

MIT
