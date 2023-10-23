import type { Category, Claim, User } from "./models";
import { dev } from '$app/environment';
import type { Item } from "./models";
import type { as } from "vitest/dist/reporters-5f784f42";

let API_URL = "";
if (dev) {
  console.log("Using dev backend");
  API_URL = "http://localhost:8080";
} else {
  console.log("Using prod backend");
  API_URL = "https://re-calc-backend.fly.dev";
}

export async function fetchUsersCount() {
  const res = await fetch(`${API_URL}/users/count`);
  const data = await res.json();
  return data;
};


export async function login(username: string, password: string): Promise<boolean> {
  const res = await fetch(`${API_URL}/auth/login`, {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ username, password }),
  });
  if (res.status === 200) {
    let data = await res.json();
    return data.status == "success";
  }
  return false;
}

export async function signup(username: string, mail: string, password: string): Promise<boolean> {
  console.log("Hello");
  const res = await fetch(`${API_URL}/auth/register`, {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ username, mail, password }),
  });

  if (res.status === 200) {
    let data = await res.json();
    return data.status == "success";
  }
  return false;
}

export async function getMe(): Promise<User | null> {
  const res = await fetch(`${API_URL}/users/me`, {
    method: "GET",
    credentials: "include",
  });
  if (res.status === 200) {
    let data = await res.json();
    console.log(data);
    return data as User;
  }
  return null;
}


export async function logout(): Promise<boolean> {
  const res = await fetch(`${API_URL}/auth/logout`, {
    method: "GET",
    credentials: "include",
  });
  if (res.status === 200) {
    let data = await res.json();
    return data.status == "success";
  }
  return false;
}


export async function getUsers(startswith: string): Promise<User[]> {
  const res = await fetch(`${API_URL}/users/startswith?start=${startswith}`, {
    method: "GET",
    credentials: "include",
  });
  if (res.status === 200) {
    let data = await res.json();
    return data as User[];
  }
  return [];
}

export async function makeUserManager(userId: number): Promise<boolean> {
  const res = await fetch(`${API_URL}/users/make_manager/${userId}`, {
    method: "GET",
    credentials: "include",
  });
  if (res.status === 200) {
    let data = await res.json();
    return data.status == "success";
  }
  return false;

}

export async function getCategories(): Promise<Category[]> {
  const res = await fetch(`${API_URL}/categories/list`, {
    method: "GET",
    credentials: "include",
  });
  if (res.status === 200) {
    let data = await res.json();
    return data as Category[];
  }
  return [];
}

// .route(
//     "/categories/create",
//     authorized!(post(handlers::create_category)),
// )

export async function createCategory(name: string, reimbursementPercentage: number, maxReimburstment: number): Promise<boolean> {
  const res = await fetch(`${API_URL}/categories/create`, {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ name, reimbursementPercentage, maxReimburstment }),
  });
  if (res.status === 200) {
    let data = await res.json();
    return data?.status === "success";
  }
  return false;
}

export async function deleteCategory(id: number): Promise<boolean> {
  const res = await fetch(`${API_URL}/categories/delete/${id}`, {
    method: "DELETE",
    credentials: "include",
  });
  if (res.status === 200) {
    let data = await res.json();
    return data?.status === "success";
  }
  return false;
}


export async function verifyEmail(code: string): Promise<boolean> {
  const res = await fetch(`${API_URL}/auth/verifyemail/${code}`, {
    method: "GET",
    credentials: "include",
  });
  if (res.status === 200) {
    let data = await res.json();
    return data.status == "success";
  }
  return false;
}


export async function estimateItem(categoryId: number, cost: number): Promise<number> {
  const res = await fetch(`${API_URL}/claims/estimate_item`, {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ categoryId, cost }),
  });
  if (res.status === 200) {
    let data = await res.json();
    console.log(data);
    return Number(data?.reimbursement);
  }
  return 0;
}


export async function createClaim(userId: number, items: Item[]): Promise<boolean> {
  const res = await fetch(`${API_URL}/claims/create`, {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ userId, items }),
  });
  if (res.status === 200) {
    let data = await res.json();
    return data?.status === "success";
  }
  return false;
}

export async function getMyClaims(): Promise<Claim[]> {
  const res = await fetch(`${API_URL}/claims/my`, {
    method: "GET",
    credentials: "include",
  });
  if (res.status === 200) {
    let data = await res.json();
    return data as Claim[];
  }
  return [];
}

