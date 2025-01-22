import { Settings } from "@/providers/SettingsProvider";
import fs from 'fs';
import path from "path";

export const dynamic = 'force-dynamic'

export async function POST(request: Request) {
  try {
    const { name } = await request.json();

    if (!name) {
      return new Response(JSON.stringify({ error: 'File name is required' }), {
        status: 400,
        headers: { 'Content-Type': 'application/json' },
      });
    }

    const rootDir = path.join(process.cwd(), '..');
    const filePath = path.join(rootDir, name);

    // Create default empty settings
    const defaultSettings = {
      client_configs: {},
      enabled_clients: [],
      completion_provider: "",
      embedding_provider: "", 
      db: "",
    };

    fs.writeFileSync(filePath, JSON.stringify(defaultSettings, null, 2));
    
    const savedContent = fs.readFileSync(filePath, 'utf8');
    const savedSettings = JSON.parse(savedContent);
    savedSettings.path_name = name;

    return new Response(JSON.stringify(savedSettings), {
      headers: { 'Content-Type': 'application/json' },
    });

  } catch (error) {
    return new Response(JSON.stringify({ error: 'Failed to save settings' }), {
      status: 500,
      headers: { 'Content-Type': 'application/json' },
    });
  }
}