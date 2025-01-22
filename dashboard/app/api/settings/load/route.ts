import { Character } from "@/providers/CharacterProvider";
import fs from 'fs';
import path from 'path';

export const dynamic = 'force-dynamic'

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const fileName = searchParams.get('file');

    if (!fileName) {
      return new Response(JSON.stringify({ error: 'File name is required' }), {
        status: 400,
        headers: { 'Content-Type': 'application/json' },
      });
    }

    const charactersDir = path.join(process.cwd(), '..', 'characters');
    const filePath = path.join(charactersDir, fileName);

    if (!fs.existsSync(filePath) || !filePath.startsWith(charactersDir)) {
      return new Response(JSON.stringify({ error: 'Character not found' }), {
        status: 404,
        headers: { 'Content-Type': 'application/json' },
      });
    }

    const content = fs.readFileSync(filePath, 'utf8');
    const character = JSON.parse(content);

    if (
      typeof character.alias === 'string' &&
      typeof character.bio === 'string' &&
      Array.isArray(character.adjectives) &&
      Array.isArray(character.lore) &&
      Array.isArray(character.styles) &&
      Array.isArray(character.topics) &&
      Array.isArray(character.inspirations)
    ) {
      character.path_name = fileName;
      return new Response(JSON.stringify(character), {
        headers: { 'Content-Type': 'application/json' },
      });
    }

    return new Response(JSON.stringify({ error: 'Invalid character format' }), {
      status: 400,
      headers: { 'Content-Type': 'application/json' },
    });

  } catch (error) {
    return new Response(JSON.stringify({ error: 'Failed to load character' }), {
      status: 500,
      headers: { 'Content-Type': 'application/json' },
    });
  }
}