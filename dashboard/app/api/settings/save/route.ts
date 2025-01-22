import { Settings } from "@/providers/SettingsProvider";
import fs from 'fs';
import path from "path";

export const dynamic = 'force-dynamic'

export async function POST(request: Request) {
  try {
    const settings: Settings = await request.json();

    if (!settings.path_name) {
      return new Response(JSON.stringify({ error: 'File name is required' }), {
        status: 400,
        headers: { 'Content-Type': 'application/json' },
      });
    }

    const rootDir = path.join(process.cwd(), '..');
    const { path_name, ...settingsWithoutPath } = settings;
    const filePath = path.join(rootDir, path_name);

    if (
      typeof settingsWithoutPath.client_configs === 'object' &&
      Array.isArray(settingsWithoutPath.enabled_clients) &&
      typeof settingsWithoutPath.completion_provider === 'string' &&
      typeof settingsWithoutPath.embedding_provider === 'string' &&
      typeof settingsWithoutPath.db === 'string'
    ) {
      fs.writeFileSync(filePath, JSON.stringify(settingsWithoutPath, null, 2));
      
      const savedContent = fs.readFileSync(filePath, 'utf8');
      const savedSettings = JSON.parse(savedContent);
      savedSettings.path_name = path_name;
      return new Response(JSON.stringify(savedSettings), {
        headers: { 'Content-Type': 'application/json' },
      });
    }

    return new Response(JSON.stringify({ error: 'Invalid settings format' }), {
      status: 400,
      headers: { 'Content-Type': 'application/json' },
    });

  } catch (error) {
    return new Response(JSON.stringify({ error: 'Failed to save settings' }), {
      status: 500,
      headers: { 'Content-Type': 'application/json' },
    });
  }
}