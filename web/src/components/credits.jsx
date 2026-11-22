import { useEffect, useState } from 'react';
import notices from '../../../THIRD_PARTY.md?raw';
import { Button } from '@/components/ui/button';
import { Alert, AlertDescription } from '@/components/ui/alert';

export function Credits({ showLicenses }) {
  const [licenses, setLicenses] = useState(null);
  const [error, setError] = useState(null);
  useEffect(() => {
    if (!showLicenses) return;
    const controller = new AbortController();
    fetch(`${import.meta.env.BASE_URL}licenses/index.json`, { signal: controller.signal })
      .then(response => {
        if (!response.ok) throw new Error('Dependency licenses could not be loaded.');
        return response.json();
      })
      .then(setLicenses)
      .catch(error => { if (error.name !== 'AbortError') setError(error.message); });
    return () => controller.abort();
  }, [showLicenses]);

  return (
    <main className="credits">
      <Button asChild variant="link"><a href="#">Back to the game</a></Button>
      <h1>{showLicenses ? 'Dependency licenses' : 'Credits & licenses'}</h1>
      {showLicenses ? (
        <>
          {error && <Alert><AlertDescription>{error}</AlertDescription></Alert>}
          {!licenses && !error && <p role="status">Loading dependency licenses…</p>}
          <ul>{licenses?.map(({ name, license, files }) => (
            <li key={name}>{name} ({license}): {files.map((file, index) => (
              <span key={file.url}>{index > 0 && ', '}<a href={`${import.meta.env.BASE_URL}licenses/${file.url}`}>{file.name}</a></span>
            ))}</li>
          ))}</ul>
          <p><a href="#credits">Credits</a></p>
        </>
      ) : (
        <>
          <pre>{notices}</pre>
          <p><a href={`${import.meta.env.BASE_URL}assets/audio/LICENSE-GeneralUser-GS.txt`}>GeneralUser GS license</a></p>
          <p><a href="#licenses">Bundled dependency licenses</a></p>
          <p><a href="https://github.com/carlosmoran97/zelda-mobile">Source code and dependency notices</a></p>
        </>
      )}
    </main>
  );
}
