const BASE_URL = 'http://127.0.0.1:3000/api/v1';
const MOCK_USER = {
    username: 'Chain93',
    password: 'anypassword'
};

async function fetchWithTimeout(resource, options = {}) {
    const { timeout = 60000 } = options; // Default 60s timeout
    const controller = new AbortController();
    const id = setTimeout(() => controller.abort(), timeout);
    const response = await fetch(resource, {
        ...options,
        signal: controller.signal
    });
    clearTimeout(id);
    return response;
}

async function runTests() {
    console.log('🚀 Starting Comprehensive API Test Suite...\n');
    let authToken = '';

    // 1. Login
    try {
        console.log('--- [POST] /authentication/login ---');
        const loginRes = await fetchWithTimeout(`${BASE_URL}/authentication/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(MOCK_USER)
        });
        const loginData = await loginRes.json();
        
        if (!loginRes.ok) throw new Error(JSON.stringify(loginData));
        
        authToken = loginData.access_token;
        console.log('✅ Login Successful');
        console.log(`👤 User: ${loginData.display_name}\n`);
    } catch (error) {
        console.error('❌ Login Failed:', error.message);
        return;
    }

    const authHeaders = { 
        'Authorization': `Bearer ${authToken}`,
        'Content-Type': 'application/json'
    };

    // 2. Seed Data (Optional, but helps populate DB)
    try {
        console.log('--- [POST] /debug/seed ---');
        const seedRes = await fetchWithTimeout(`${BASE_URL}/debug/seed`, { method: 'POST' });
        if (seedRes.ok) {
            console.log('✅ Seed Data Successful\n');
        } else {
            console.log(`⚠️ Seed Data failed or already seeded: ${seedRes.status}\n`);
        }
    } catch (error) {
        console.log('⚠️ Seed Data failed (likely DB issue):', error.message, '\n');
    }

    const endpoints = [
        { method: 'GET', url: '/authentication/google/url', name: 'Google Auth URL' },
        { method: 'POST', url: '/brawlers/register', name: 'Register Brawler', body: { username: 'testuser_' + Date.now(), password: 'password', display_name: 'Test User' } },
        { method: 'GET', url: '/missions', name: 'Get Missions' },
        { method: 'GET', url: '/cards', name: 'Get Cards' },
        { method: 'GET', url: '/fortune/draw', name: 'Draw Fortune' },
        { method: 'GET', url: '/fortune/daily', name: 'Daily Fortune', headers: authHeaders },
    ];

    for (const ep of endpoints) {
        try {
            console.log(`--- [${ep.method}] ${ep.url} (${ep.name}) ---`);
            const res = await fetchWithTimeout(`${BASE_URL}${ep.url}`, {
                method: ep.method,
                headers: ep.headers || { 'Content-Type': 'application/json' },
                body: ep.body ? JSON.stringify(ep.body) : undefined
            });
            
            const data = await res.json().catch(() => null);
            
            if (res.ok) {
                console.log(`✅ Success (Status: ${res.status})`);
                const dataStr = JSON.stringify(data).substring(0, 100);
                console.log(`📄 Data: ${dataStr}${dataStr.length >= 100 ? '...' : ''}\n`);
            } else {
                console.error(`❌ Failed: ${res.status} ${JSON.stringify(data)}\n`);
            }
        } catch (error) {
            console.error(`❌ Error: ${error.message}\n`);
        }
    }

    // 3. Mission Management (Add -> Edit -> Join)
    try {
        console.log('--- [POST] /mission-management (Add Mission) ---');
        const missionData = {
            name: "Automated Test Mission " + new Date().getTime(),
            description: "Created by automated test script",
            mission_date: "2026-02-12T10:00:00",
            time: "10:00 AM",
            email: "test@example.com",
            phone: "0812345678",
            location: "Test Location",
            rewards: "Test Rewards"
        };
        const res = await fetchWithTimeout(`${BASE_URL}/mission-management`, {
            method: 'POST',
            headers: authHeaders,
            body: JSON.stringify(missionData)
        });
        const data = await res.json();
        
        if (res.ok) {
            console.log(`✅ Mission Created (ID: ${data.mission_id})\n`);
            const missionId = data.mission_id;

            // Test Join Mission
            console.log(`--- [POST] /missions/${missionId}/join ---`);
            const joinRes = await fetchWithTimeout(`${BASE_URL}/missions/${missionId}/join`, {
                method: 'POST',
                headers: authHeaders
            });
            const joinData = await joinRes.json();
            if (joinRes.ok) {
                console.log(`✅ Joined Mission Successful: ${JSON.stringify(joinData)}\n`);
            } else {
                console.error(`❌ Join Failed: ${joinRes.status} ${JSON.stringify(joinData)}\n`);
            }
        } else {
            console.error(`❌ Mission Creation Failed: ${res.status} ${JSON.stringify(data)}\n`);
        }
    } catch (error) {
        console.error(`❌ Error in Mission Management: ${error.message}\n`);
    }

    console.log('🏁 API Test Suite Finished.');
}

runTests();
