
export type User = {
  id: number,
  username: string;
  role: string;
  verified: boolean;
  createdAt: string;
};

export type Category = {
  id: number,
  name: string;
  reimbursementPercentage: number;
  maxReimburstment: number;
}

export type Item = {
  categoryId: number;
  cost: number;
}

export type Claim = {
  id: number;
  userId: number;
  totalCost: number;
  reimbursement: number;
  status: string;
}
