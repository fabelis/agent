import { Character } from "@/providers/CharacterProvider";
import fs from 'fs';
import path from 'path';

export const dynamic = 'force-dynamic'

export async function GET(request: Request) {
  try {
    const charactersDir = path.join(process.cwd(), '..', 'characters');

    const files = fs.readdirSync(charactersDir);
    
    const characters = files
      .filter(file => file.endsWith('.json'))
      .map(file => {
        const content = fs.readFileSync(path.join(charactersDir, file), 'utf8');
        try {
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
            character.path_name = file;
            return character;
          }
          return null;
        } catch (e) {
          return null;
        }
      })
      .filter(Boolean) as Character[];

    return new Response(JSON.stringify(characters), {
      headers: { 'Content-Type': 'application/json' },
    });

  } catch (error) {
    return new Response(JSON.stringify({ error: 'Failed to load characters' }), {
      status: 500,
      headers: { 'Content-Type': 'application/json' },
    });
  }
}