import { Character } from "@/providers/CharacterProvider";
import fs from 'fs';
import path from 'path';

export const dynamic = 'force-dynamic'

export async function POST(request: Request) {
  try {
    const character: Character = await request.json();

    if (!character.path_name) {
      return new Response(JSON.stringify({ error: 'File name is required' }), {
        status: 400,
        headers: { 'Content-Type': 'application/json' },
      });
    }

    const charactersDir = path.join(process.cwd(), '..', 'characters');
    const { path_name, ...characterWithoutPath } = character;
    const filePath = path.join(charactersDir, path_name);

    if (!filePath.startsWith(charactersDir)) {
      return new Response(JSON.stringify({ error: 'Invalid file path' }), {
        status: 400,
        headers: { 'Content-Type': 'application/json' },
      });
    }

    if (
      typeof characterWithoutPath.alias === 'string' &&
      typeof characterWithoutPath.bio === 'string' &&
      Array.isArray(characterWithoutPath.adjectives) &&
      Array.isArray(characterWithoutPath.lore) &&
      Array.isArray(characterWithoutPath.styles) &&
      Array.isArray(characterWithoutPath.topics) &&
      Array.isArray(characterWithoutPath.inspirations)
    ) {
      fs.writeFileSync(filePath, JSON.stringify(characterWithoutPath, null, 2));
      
      const savedContent = fs.readFileSync(filePath, 'utf8');
      const savedCharacter = JSON.parse(savedContent);
      savedCharacter.path_name = path_name;
      return new Response(JSON.stringify(savedCharacter), {
        headers: { 'Content-Type': 'application/json' },
      });
    }

    return new Response(JSON.stringify({ error: 'Invalid character format' }), {
      status: 400,
      headers: { 'Content-Type': 'application/json' },
    });

  } catch (error) {
    return new Response(JSON.stringify({ error: 'Failed to save character' }), {
      status: 500,
      headers: { 'Content-Type': 'application/json' },
    });
  }
}