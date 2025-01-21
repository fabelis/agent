import { Settings } from "@/providers/SettingsProvider";
import fs from 'fs';
import path from 'path';

export const dynamic = 'force-dynamic'

export async function GET(request: Request) {
  try {
    const settingsDir = path.join(process.cwd(), '..');

    const files = fs.readdirSync(settingsDir);
    
    const settingsArr = files
      .filter(file => file === 'config.json' || /^config\..+\.json$/.test(file))
      .map(file => {
        const content = fs.readFileSync(path.join(settingsDir, file), 'utf8');
        try {
          const settings = JSON.parse(content);
          if (
            typeof settings.client_configs === 'object' &&
            Array.isArray(settings.enabled_clients) &&
            typeof settings.completion_provider === 'string' &&
            typeof settings.embedding_provider === 'string' &&
            typeof settings.db === 'string'
          ) {
            settings.path_name = file;
            return settings;
          }
          return null;
        } catch (e) {
          return null;
        }
      })
      .filter(Boolean) as Settings[];

    return new Response(JSON.stringify(settingsArr), {
      headers: { 'Content-Type': 'application/json' },
    });

  } catch (error) {
    return new Response(JSON.stringify({ error: 'Failed to load settings' }), {
      status: 500,
      headers: { 'Content-Type': 'application/json' },
    });
  }
}