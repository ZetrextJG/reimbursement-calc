
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
