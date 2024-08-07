/*
 * Copyright 2020 The SkyWater PDK Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
*/

specify
if ((A2&!B1&!B2&!C1&!C2)) (A1 +=> X) = (0:0:0,0:0:0);
if ((A2&!B1&!B2&!C1&C2)) (A1 +=> X) = (0:0:0,0:0:0);
if ((A2&!B1&!B2&C1&!C2)) (A1 +=> X) = (0:0:0,0:0:0);
if ((A2&!B1&B2&!C1&!C2)) (A1 +=> X) = (0:0:0,0:0:0);
if ((A2&!B1&B2&!C1&C2)) (A1 +=> X) = (0:0:0,0:0:0);
if ((A2&!B1&B2&C1&!C2)) (A1 +=> X) = (0:0:0,0:0:0);
if ((A2&B1&!B2&!C1&!C2)) (A1 +=> X) = (0:0:0,0:0:0);
if ((A2&B1&!B2&!C1&C2)) (A1 +=> X) = (0:0:0,0:0:0);
if ((A2&B1&!B2&C1&!C2)) (A1 +=> X) = (0:0:0,0:0:0);
if ((A1&!B1&!B2&!C1&!C2)) (A2 +=> X) = (0:0:0,0:0:0);
if ((A1&!B1&!B2&!C1&C2)) (A2 +=> X) = (0:0:0,0:0:0);
if ((A1&!B1&!B2&C1&!C2)) (A2 +=> X) = (0:0:0,0:0:0);
if ((A1&!B1&B2&!C1&!C2)) (A2 +=> X) = (0:0:0,0:0:0);
if ((A1&!B1&B2&!C1&C2)) (A2 +=> X) = (0:0:0,0:0:0);
if ((A1&!B1&B2&C1&!C2)) (A2 +=> X) = (0:0:0,0:0:0);
if ((A1&B1&!B2&!C1&!C2)) (A2 +=> X) = (0:0:0,0:0:0);
if ((A1&B1&!B2&!C1&C2)) (A2 +=> X) = (0:0:0,0:0:0);
if ((A1&B1&!B2&C1&!C2)) (A2 +=> X) = (0:0:0,0:0:0);
if ((!A1&!A2&B2&!C1&!C2)) (B1 +=> X) = (0:0:0,0:0:0);
if ((!A1&!A2&B2&!C1&C2)) (B1 +=> X) = (0:0:0,0:0:0);
if ((!A1&!A2&B2&C1&!C2)) (B1 +=> X) = (0:0:0,0:0:0);
if ((!A1&A2&B2&!C1&!C2)) (B1 +=> X) = (0:0:0,0:0:0);
if ((!A1&A2&B2&!C1&C2)) (B1 +=> X) = (0:0:0,0:0:0);
if ((!A1&A2&B2&C1&!C2)) (B1 +=> X) = (0:0:0,0:0:0);
if ((A1&!A2&B2&!C1&!C2)) (B1 +=> X) = (0:0:0,0:0:0);
if ((A1&!A2&B2&!C1&C2)) (B1 +=> X) = (0:0:0,0:0:0);
if ((A1&!A2&B2&C1&!C2)) (B1 +=> X) = (0:0:0,0:0:0);
if ((!A1&!A2&B1&!C1&!C2)) (B2 +=> X) = (0:0:0,0:0:0);
if ((!A1&!A2&B1&!C1&C2)) (B2 +=> X) = (0:0:0,0:0:0);
if ((!A1&!A2&B1&C1&!C2)) (B2 +=> X) = (0:0:0,0:0:0);
if ((!A1&A2&B1&!C1&!C2)) (B2 +=> X) = (0:0:0,0:0:0);
if ((!A1&A2&B1&!C1&C2)) (B2 +=> X) = (0:0:0,0:0:0);
if ((!A1&A2&B1&C1&!C2)) (B2 +=> X) = (0:0:0,0:0:0);
if ((A1&!A2&B1&!C1&!C2)) (B2 +=> X) = (0:0:0,0:0:0);
if ((A1&!A2&B1&!C1&C2)) (B2 +=> X) = (0:0:0,0:0:0);
if ((A1&!A2&B1&C1&!C2)) (B2 +=> X) = (0:0:0,0:0:0);
if ((!A1&!A2&!B1&!B2&C2)) (C1 +=> X) = (0:0:0,0:0:0);
if ((!A1&!A2&!B1&B2&C2)) (C1 +=> X) = (0:0:0,0:0:0);
if ((!A1&!A2&B1&!B2&C2)) (C1 +=> X) = (0:0:0,0:0:0);
if ((!A1&A2&!B1&!B2&C2)) (C1 +=> X) = (0:0:0,0:0:0);
if ((!A1&A2&!B1&B2&C2)) (C1 +=> X) = (0:0:0,0:0:0);
if ((!A1&A2&B1&!B2&C2)) (C1 +=> X) = (0:0:0,0:0:0);
if ((A1&!A2&!B1&!B2&C2)) (C1 +=> X) = (0:0:0,0:0:0);
if ((A1&!A2&!B1&B2&C2)) (C1 +=> X) = (0:0:0,0:0:0);
if ((A1&!A2&B1&!B2&C2)) (C1 +=> X) = (0:0:0,0:0:0);
if ((!A1&!A2&!B1&!B2&C1)) (C2 +=> X) = (0:0:0,0:0:0);
if ((!A1&!A2&!B1&B2&C1)) (C2 +=> X) = (0:0:0,0:0:0);
if ((!A1&!A2&B1&!B2&C1)) (C2 +=> X) = (0:0:0,0:0:0);
if ((!A1&A2&!B1&!B2&C1)) (C2 +=> X) = (0:0:0,0:0:0);
if ((!A1&A2&!B1&B2&C1)) (C2 +=> X) = (0:0:0,0:0:0);
if ((!A1&A2&B1&!B2&C1)) (C2 +=> X) = (0:0:0,0:0:0);
if ((A1&!A2&!B1&!B2&C1)) (C2 +=> X) = (0:0:0,0:0:0);
if ((A1&!A2&!B1&B2&C1)) (C2 +=> X) = (0:0:0,0:0:0);
if ((A1&!A2&B1&!B2&C1)) (C2 +=> X) = (0:0:0,0:0:0);
endspecify