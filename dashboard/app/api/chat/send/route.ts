export const dynamic = 'force-dynamic'

export async function POST(request: Request) {
  try {
    const body = await request.json();
    
    if (!body.path_name || !body.content) {
      return new Response(JSON.stringify({ error: 'Missing required fields: path_name and content' }), {
        status: 400,
        headers: { 'Content-Type': 'application/json' },
      });
    }

    if (typeof body.path_name !== 'string' || typeof body.content !== 'string') {
      return new Response(JSON.stringify({ error: 'Invalid types: path_name and content must be strings' }), {
        status: 400,
        headers: { 'Content-Type': 'application/json' },
      });
    }

    // TODO: Add your chat message handling logic here
    
    return new Response(JSON.stringify({ success: true }), {
      headers: { 'Content-Type': 'application/json' },
    });

  } catch (error) {
    return new Response(JSON.stringify({ error: 'Failed to process chat message' }), {
      status: 500,
      headers: { 'Content-Type': 'application/json' },
    });
  }
}