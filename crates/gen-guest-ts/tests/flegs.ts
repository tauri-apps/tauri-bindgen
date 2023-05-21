

export enum Flag1 { 
B0 = 2,
 }

export enum Flag2 { 
B0 = 2,

B1 = 4,
 }

export enum Flag4 { 
B0 = 2,

B1 = 4,

B2 = 8,

B3 = 16,
 }

export enum Flag8 { 
B0 = 2,

B1 = 4,

B2 = 8,

B3 = 16,

B4 = 32,

B5 = 64,

B6 = 128,

B7 = 256,
 }

export enum Flag16 { 
B0 = 2,

B1 = 4,

B2 = 8,

B3 = 16,

B4 = 32,

B5 = 64,

B6 = 128,

B7 = 256,

B8 = 512,

B9 = 1024,

B10 = 2048,

B11 = 4096,

B12 = 8192,

B13 = 16384,

B14 = 32768,

B15 = 65536,
 }

export enum Flag32 { 
B0 = 2,

B1 = 4,

B2 = 8,

B3 = 16,

B4 = 32,

B5 = 64,

B6 = 128,

B7 = 256,

B8 = 512,

B9 = 1024,

B10 = 2048,

B11 = 4096,

B12 = 8192,

B13 = 16384,

B14 = 32768,

B15 = 65536,

B16 = 131072,

B17 = 262144,

B18 = 524288,

B19 = 1048576,

B20 = 2097152,

B21 = 4194304,

B22 = 8388608,

B23 = 16777216,

B24 = 33554432,

B25 = 67108864,

B26 = 134217728,

B27 = 268435456,

B28 = 536870912,

B29 = 1073741824,

B30 = 2147483648,

B31 = 4294967296,
 }

export enum Flag64 { 
B0 = 2,

B1 = 4,

B2 = 8,

B3 = 16,

B4 = 32,

B5 = 64,

B6 = 128,

B7 = 256,

B8 = 512,

B9 = 1024,

B10 = 2048,

B11 = 4096,

B12 = 8192,

B13 = 16384,

B14 = 32768,

B15 = 65536,

B16 = 131072,

B17 = 262144,

B18 = 524288,

B19 = 1048576,

B20 = 2097152,

B21 = 4194304,

B22 = 8388608,

B23 = 16777216,

B24 = 33554432,

B25 = 67108864,

B26 = 134217728,

B27 = 268435456,

B28 = 536870912,

B29 = 1073741824,

B30 = 2147483648,

B31 = 4294967296,

B32 = 8589934592,

B33 = 17179869184,

B34 = 34359738368,

B35 = 68719476736,

B36 = 137438953472,

B37 = 274877906944,

B38 = 549755813888,

B39 = 1099511627776,

B40 = 2199023255552,

B41 = 4398046511104,

B42 = 8796093022208,

B43 = 17592186044416,

B44 = 35184372088832,

B45 = 70368744177664,

B46 = 140737488355328,

B47 = 281474976710656,

B48 = 562949953421312,

B49 = 1125899906842624,

B50 = 2251799813685248,

B51 = 4503599627370496,

B52 = 9007199254740992,

B53 = 18014398509481984,

B54 = 36028797018963968,

B55 = 72057594037927936,

B56 = 144115188075855872,

B57 = 288230376151711744,

B58 = 576460752303423488,

B59 = 1152921504606846976,

B60 = 2305843009213693952,

B61 = 4611686018427387904,

B62 = 9223372036854775808,

B63 = 0,
 }


            
            export async function roundtripFlag1 (x: Flag1) : Promise<Flag1> {
                return fetch('ipc://localhost/flegs/roundtrip_flag1', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function roundtripFlag2 (x: Flag2) : Promise<Flag2> {
                return fetch('ipc://localhost/flegs/roundtrip_flag2', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function roundtripFlag4 (x: Flag4) : Promise<Flag4> {
                return fetch('ipc://localhost/flegs/roundtrip_flag4', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function roundtripFlag8 (x: Flag8) : Promise<Flag8> {
                return fetch('ipc://localhost/flegs/roundtrip_flag8', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function roundtripFlag16 (x: Flag16) : Promise<Flag16> {
                return fetch('ipc://localhost/flegs/roundtrip_flag16', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function roundtripFlag32 (x: Flag32) : Promise<Flag32> {
                return fetch('ipc://localhost/flegs/roundtrip_flag32', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function roundtripFlag64 (x: Flag64) : Promise<Flag64> {
                return fetch('ipc://localhost/flegs/roundtrip_flag64', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        