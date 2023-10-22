import type { Category, User } from "./models";

// const API_URL = "http://localhost:8080"
const API_URL = "https://re-calc-backend.fly.dev"

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
